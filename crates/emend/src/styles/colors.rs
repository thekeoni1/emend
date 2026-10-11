//! A colour resolved to the RGB the workbook defines for it: the default
//! palette, the styles part's `<indexedColors>`, the theme part's colour
//! scheme, and the tint (`specs/0032-read-breadth-styles.md`, "Colours").
//! The Cargo feature `read`.
//!
//! The theme part is streamed and only `a:theme/a:themeElements/a:clrScheme`
//! is read, its twelve colours kept; everything else in it is walked past.
//! Nothing else is cached: a resolve is a lookup.

use std::io::Read;

use quick_xml::events::{BytesStart, Event};

use crate::archive::Error;
use crate::cells::skip_subtree;
use crate::styles::Color;
use crate::styles::read::{COLOR_UNRESOLVED, THEME_UNREADABLE};
use crate::xml::{Scope, Stream};

/// The default palette of ECMA-376 Part 1, 18.8.27 (`indexedColors`),
/// indexes 0 to 63, transcribed from the standard's table. The table writes
/// each entry `00RRGGBB`; here the top byte is `FF`, so an entry is the
/// opaque colour a writer would write, as [`Color::Rgb`] documents
/// (`specs/0032-read-breadth-styles.md`, "Planning rulings during P2",
/// ruling 8).
pub(crate) const PALETTE: [u32; 64] = [
    0xFF000000, 0xFFFFFFFF, 0xFFFF0000, 0xFF00FF00, 0xFF0000FF, 0xFFFFFF00, 0xFFFF00FF, 0xFF00FFFF,
    0xFF000000, 0xFFFFFFFF, 0xFFFF0000, 0xFF00FF00, 0xFF0000FF, 0xFFFFFF00, 0xFFFF00FF, 0xFF00FFFF,
    0xFF800000, 0xFF008000, 0xFF000080, 0xFF808000, 0xFF800080, 0xFF008080, 0xFFC0C0C0, 0xFF808080,
    0xFF9999FF, 0xFF993366, 0xFFFFFFCC, 0xFFCCFFFF, 0xFF660066, 0xFFFF8080, 0xFF0066CC, 0xFFCCCCFF,
    0xFF000080, 0xFFFF00FF, 0xFFFFFF00, 0xFF00FFFF, 0xFF800080, 0xFF800000, 0xFF008080, 0xFF0000FF,
    0xFF00CCFF, 0xFFCCFFFF, 0xFFCCFFCC, 0xFFFFFF99, 0xFF99CCFF, 0xFFFF99CC, 0xFFCC99FF, 0xFFFFCC99,
    0xFF3366FF, 0xFF33CCCC, 0xFF99CC00, 0xFFFFCC00, 0xFFFF9900, 0xFFFF6600, 0xFF666699, 0xFF969696,
    0xFF003366, 0xFF339966, 0xFF003300, 0xFF333300, 0xFF993300, 0xFF993366, 0xFF333399, 0xFF333333,
];

/// The scheme's children in element order: [`read_theme`] keeps the colours
/// in this order, `dk1` first.
const SCHEME: [&[u8]; 12] = [
    b"dk1",
    b"lt1",
    b"dk2",
    b"lt2",
    b"accent1",
    b"accent2",
    b"accent3",
    b"accent4",
    b"accent5",
    b"accent6",
    b"hlink",
    b"folHlink",
];

/// The scheme entry a colour's `theme` index names: the first two pairs
/// swapped against the element order, so theme 0 is `lt1` and theme 1 `dk1`
/// (`specs/0032-read-breadth-styles.md`, "Facts planning checked").
const THEME_ORDER: [usize; 12] = [1, 0, 3, 2, 4, 5, 6, 7, 8, 9, 10, 11];

/// A colour resolved: its ARGB value and the tint the colour carries, which
/// [`ResolvedColor::tinted`] applies. The Cargo feature `read`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedColor {
    /// ARGB: an `rgb` as written, a palette entry, or a scheme colour.
    pub rgb: u32,
    /// A theme colour's `tint`, a fraction in `-1.0..=1.0`, when given.
    pub tint: Option<f64>,
}

impl ResolvedColor {
    /// `rgb` with the tint applied by ECMA-376 Part 1, 18.8.19: in HLS space
    /// with HLSMAX 255, the luminance multiplied by `1.0 + tint` for a tint
    /// below zero, and by `1.0 - tint` then raised by HLSMAX times the tint
    /// above zero; each channel rounded to nearest, halves away from zero,
    /// and the alpha byte kept. `rgb` unchanged with no tint.
    pub fn tinted(&self) -> u32 {
        let Some(tint) = self.tint else {
            return self.rgb;
        };
        const HLSMAX: f64 = 255.0;
        let channel = |shift: u32| f64::from((self.rgb >> shift) & 0xFF) / 255.0;
        let (hue, luminance, saturation) = to_hls(channel(16), channel(8), channel(0));
        let mut luminance = luminance * HLSMAX;
        if tint < 0.0 {
            luminance *= 1.0 + tint;
        } else {
            luminance = luminance * (1.0 - tint) + HLSMAX * tint;
        }
        let (red, green, blue) = from_hls(hue, luminance / HLSMAX, saturation);
        let byte = |value: f64| (value * 255.0).round().clamp(0.0, 255.0) as u32;
        (self.rgb & 0xFF00_0000) | (byte(red) << 16) | (byte(green) << 8) | byte(blue)
    }
}

/// Hue (a fraction of the circle), luminance and saturation, each in
/// `0.0..=1.0`, of a colour whose channels are in `0.0..=1.0`.
fn to_hls(red: f64, green: f64, blue: f64) -> (f64, f64, f64) {
    let max = red.max(green).max(blue);
    let min = red.min(green).min(blue);
    let luminance = (max + min) / 2.0;
    if max == min {
        return (0.0, luminance, 0.0);
    }
    let delta = max - min;
    let saturation = if luminance <= 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };
    let hue = if max == red {
        (green - blue) / delta
    } else if max == green {
        2.0 + (blue - red) / delta
    } else {
        4.0 + (red - green) / delta
    };
    ((hue / 6.0).rem_euclid(1.0), luminance, saturation)
}

/// The channels, each in `0.0..=1.0`, of a colour in HLS as [`to_hls`] gives.
fn from_hls(hue: f64, luminance: f64, saturation: f64) -> (f64, f64, f64) {
    if saturation == 0.0 {
        return (luminance, luminance, luminance);
    }
    let high = if luminance <= 0.5 {
        luminance * (1.0 + saturation)
    } else {
        luminance + saturation - luminance * saturation
    };
    let low = 2.0 * luminance - high;
    let channel = |offset: f64| {
        let h = (hue + offset).rem_euclid(1.0);
        if h < 1.0 / 6.0 {
            low + (high - low) * h * 6.0
        } else if h < 0.5 {
            high
        } else if h < 2.0 / 3.0 {
            low + (high - low) * (2.0 / 3.0 - h) * 6.0
        } else {
            low
        }
    };
    (channel(1.0 / 3.0), channel(0.0), channel(-1.0 / 3.0))
}

/// `color` resolved against the part's `indexedColors` and the theme's
/// scheme colours in element order: `None` for `Auto`, an index past 63
/// and a theme colour with no theme; "a colour cannot be resolved" for a
/// theme index past 11 or a tint not finite or outside `-1.0..=1.0`.
pub(crate) fn resolve(
    color: &Color,
    indexed: Option<&[u32]>,
    theme: Option<&[u32; 12]>,
) -> Result<Option<ResolvedColor>, Error> {
    Ok(match *color {
        Color::Rgb(rgb) => Some(ResolvedColor { rgb, tint: None }),
        Color::Indexed(at) => {
            let at = usize::try_from(at).unwrap_or(usize::MAX);
            match indexed.and_then(|list| list.get(at)) {
                Some(&rgb) => Some(ResolvedColor { rgb, tint: None }),
                None => PALETTE
                    .get(at)
                    .map(|&rgb| ResolvedColor { rgb, tint: None }),
            }
        }
        Color::Theme { index, tint } => {
            let at = usize::try_from(index)
                .ok()
                .and_then(|at| THEME_ORDER.get(at))
                .ok_or(COLOR_UNRESOLVED)?;
            if tint.is_some_and(|tint| !tint.is_finite() || !(-1.0..=1.0).contains(&tint)) {
                return Err(COLOR_UNRESOLVED);
            }
            theme.map(|scheme| ResolvedColor {
                rgb: scheme[*at],
                tint,
            })
        }
        Color::Auto => None,
    })
}

/// The DrawingML main namespace, transitional then strict, as `package`
/// gives it.
type Namespaces = [&'static [u8]; 2];

/// An element met in the theme part: whether it is in the DrawingML
/// namespace, and its local name.
fn drawingml(
    scope: &Scope,
    start: &BytesStart<'_>,
    namespaces: Namespaces,
) -> Result<(bool, Vec<u8>), Error> {
    let (namespace, local) = scope.resolve_element(start.name())?;
    Ok((
        namespace.is_some_and(|namespace| namespaces.contains(&namespace)),
        local.to_vec(),
    ))
}

/// The unprefixed attribute `name` of `start`, as written.
fn attribute(
    start: &BytesStart<'_>,
    scope: &Scope,
    index: usize,
    name: &[u8],
) -> Result<Option<Vec<u8>>, Error> {
    for found in start.attributes() {
        let found = found.map_err(|_| Error::Xml(index))?;
        if found.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(found.key)?;
        if namespace.is_none() && local == name {
            return Ok(Some(found.value.as_bytes().to_vec()));
        }
    }
    Ok(None)
}

/// Six hex digits, either case, as an opaque ARGB value.
fn six_hex(value: Option<Vec<u8>>) -> Result<u32, Error> {
    let value = value.ok_or(THEME_UNREADABLE)?;
    if value.len() != 6 || !value.iter().all(u8::is_ascii_hexdigit) {
        return Err(THEME_UNREADABLE);
    }
    let text = std::str::from_utf8(&value).map_err(|_| THEME_UNREADABLE)?;
    let rgb = u32::from_str_radix(text, 16).map_err(|_| THEME_UNREADABLE)?;
    Ok(0xFF00_0000 | rgb)
}

/// Reads a theme part's twelve scheme colours, in element order (`dk1`
/// first), each an `srgbClr`'s `val` or a `sysClr`'s `lastClr` as an opaque
/// ARGB value.
///
/// # Errors
///
/// [`Error::Package`] "a theme cannot be read" for a root that is not a
/// DrawingML `theme`, no `clrScheme` under its `themeElements` or a second
/// one there, a scheme child outside the DrawingML namespace or unknown to
/// the scheme or met twice, one of the twelve missing, a scheme child with
/// no `srgbClr` or `sysClr` or with another colour form, a colour element
/// holding children, a `val` or `lastClr` that is not six hex digits, and a
/// `sysClr` with no `lastClr` (`specs/0032-read-breadth-styles.md`,
/// "Errors", and "Planning rulings during P2", rulings 2 and 3); and
/// whatever the part's XML refuses.
pub(crate) fn read_theme<R: Read>(
    stream: &mut Stream<R>,
    index: usize,
    namespaces: Namespaces,
) -> Result<[u32; 12], Error> {
    let mut scope = Scope::new(index);
    let mut depth = 0usize;
    let mut root = false;
    let mut scheme: Option<[u32; 12]> = None;
    while let Some(token) = stream.next()? {
        // What the token asks for, acted on after the match because `token`
        // borrows the stream.
        let mut skip = false;
        let mut read_scheme = false;
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                depth += 1;
                scope.start(start)?;
                let (ours, local) = drawingml(&scope, start, namespaces)?;
                if depth == 1 {
                    if !ours || local != b"theme" {
                        return Err(THEME_UNREADABLE);
                    }
                    root = true;
                } else if depth == 2 && ours && local == b"themeElements" {
                    // Entered: its `clrScheme` is read below.
                } else if depth == 3 && ours && local == b"clrScheme" {
                    if scheme.is_some() {
                        return Err(THEME_UNREADABLE);
                    }
                    if empty {
                        return Err(THEME_UNREADABLE);
                    }
                    read_scheme = true;
                } else {
                    skip = !empty;
                }
                if empty {
                    scope.end();
                    depth -= 1;
                }
            }
            Event::End(_) => {
                scope.end();
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
        if read_scheme {
            scheme = Some(read_clr_scheme(stream, &mut scope, index, namespaces)?);
            depth -= 1;
        } else if skip {
            skip_subtree(stream, &mut scope, index)?;
            depth -= 1;
        }
    }
    if !root {
        return Err(THEME_UNREADABLE);
    }
    scheme.ok_or(THEME_UNREADABLE)
}

/// A `clrScheme`, its start tag given to `scope`, read through its end tag.
fn read_clr_scheme<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    namespaces: Namespaces,
) -> Result<[u32; 12], Error> {
    let mut colors: [Option<u32>; 12] = [None; 12];
    loop {
        let Some(token) = stream.next()? else {
            return Err(Error::Xml(index));
        };
        // A scheme child to read, and whether it was empty; or `a:extLst`.
        let mut child: Option<(usize, bool)> = None;
        let mut skip = false;
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                let (ours, local) = drawingml(scope, start, namespaces)?;
                if !ours {
                    return Err(THEME_UNREADABLE);
                }
                if local == b"extLst" {
                    skip = !empty;
                } else {
                    let at = SCHEME
                        .iter()
                        .position(|name| *name == local.as_slice())
                        .ok_or(THEME_UNREADABLE)?;
                    if colors[at].is_some() {
                        return Err(THEME_UNREADABLE);
                    }
                    child = Some((at, empty));
                }
                if empty {
                    scope.end();
                }
            }
            Event::End(_) => {
                scope.end();
                break;
            }
            _ => {}
        }
        if let Some((at, empty)) = child {
            if empty {
                return Err(THEME_UNREADABLE);
            }
            colors[at] = Some(read_scheme_color(stream, scope, index, namespaces)?);
        } else if skip {
            skip_subtree(stream, scope, index)?;
        }
    }
    let mut scheme = [0u32; 12];
    for (slot, color) in scheme.iter_mut().zip(colors) {
        *slot = color.ok_or(THEME_UNREADABLE)?;
    }
    Ok(scheme)
}

/// A scheme child's content, its start tag given to `scope`, read through
/// its end tag: one `srgbClr` or `sysClr` with no children.
fn read_scheme_color<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    namespaces: Namespaces,
) -> Result<u32, Error> {
    let mut color: Option<u32> = None;
    loop {
        let Some(token) = stream.next()? else {
            return Err(Error::Xml(index));
        };
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                let (ours, local) = drawingml(scope, start, namespaces)?;
                if !ours || color.is_some() {
                    return Err(THEME_UNREADABLE);
                }
                color = Some(match local.as_slice() {
                    b"srgbClr" => six_hex(attribute(start, scope, index, b"val")?)?,
                    b"sysClr" => six_hex(attribute(start, scope, index, b"lastClr")?)?,
                    _ => return Err(THEME_UNREADABLE),
                });
                if empty {
                    scope.end();
                } else {
                    childless(stream, scope, index)?;
                }
            }
            Event::End(_) => {
                scope.end();
                break;
            }
            _ => {}
        }
    }
    color.ok_or(THEME_UNREADABLE)
}

/// Reads through the end tag of an element whose start tag `scope` has been
/// given, refusing any child: a transform such as `a:lumMod` would make the
/// colour element's value not the colour.
fn childless<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
) -> Result<(), Error> {
    loop {
        let Some(token) = stream.next()? else {
            return Err(Error::Xml(index));
        };
        match token.event {
            Event::Start(_) | Event::Empty(_) => return Err(THEME_UNREADABLE),
            Event::End(_) => {
                scope.end();
                return Ok(());
            }
            _ => {}
        }
    }
}
