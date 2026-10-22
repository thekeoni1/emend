//! The typed read of a styles part: its fonts, fills and borders, each cell
//! format's alignment and protection, and a cell format resolved into them
//! (`specs/0027-read-back.md`, "Cell formats"). The Cargo feature `read`.
//!
//! Everything here is read in the one read of the part [`Styles::read`]
//! makes, into the types [`crate::package::Package::add_style`] takes, so
//! every value read is one the writer accepts. What the types cannot hold is
//! read past (decision 3): a font's `charset`, `outline`, `shadow`,
//! `condense` and `extend` (by the font reader runs share), an alignment's
//! `justifyLastLine`, `readingOrder` and `relativeIndent`; a `<fill>` that
//! holds no child reads as `None`, and a `gradientFill` as a [`Fill`] whose
//! `gradient` is `Some` (`specs/0042-gradient-fills-and-named-styles.md`,
//! "The styles part").
//! Everything else this layer does not know is refused with "a style cannot
//! be read".
//!
//! The same read gives the named styles of `<cellStyles>` with the formats of
//! `<cellStyleXfs>` they name, and the differential styles of `<dxfs>` as the
//! [`Style`] values `Package::add_differential_style` takes
//! (`specs/0032-read-breadth-styles.md`, "Named styles" and "Differential
//! styles"); a named style's format resolves through the one resolver a
//! cell format's does.

use std::io::Read;

use quick_xml::events::{BytesStart, Event};

use crate::archive::Error;
use crate::cells::{
    ColorAttributes, color_from, decimal, read_properties, same_color, same_font, skip_subtree,
};
use crate::package::MAIN;
use crate::styles::{
    Alignment, Border, BorderStyle, Color, Fill, Font, Gradient, GradientKind, Horizontal,
    NumberFormat, Pattern, Protection, Side, Stop, Style, Styles, Vertical, Xf,
};
use crate::xml::{Scope, Stream};

/// What a styles part this layer cannot read into the writers' types is
/// refused with, at the read of the part and at [`Styles::cell_style`];
/// catalogued in [`crate::package`]'s module header.
pub(crate) const STYLE_UNREADABLE: Error = Error::Package("a style cannot be read");

/// What a theme part whose colour scheme this layer cannot read is refused
/// with, at [`crate::package::Package::theme_colors`]
/// (`specs/0032-read-breadth-styles.md`, "Errors"); catalogued in
/// [`crate::package`]'s module header.
pub(crate) const THEME_UNREADABLE: Error = Error::Package("a theme cannot be read");

/// What a colour the theme cannot resolve is refused with, at
/// [`crate::package::Package::resolve_color`]: a theme index past 11, a tint
/// not finite or outside `-1.0..=1.0` (`specs/0032-read-breadth-styles.md`,
/// "Errors"); catalogued in [`crate::package`]'s module header.
pub(crate) const COLOR_UNRESOLVED: Error = Error::Package("a colour cannot be resolved");

fn refusal() -> Error {
    STYLE_UNREADABLE
}

/// A cell format resolved through `cellXfs` into the types
/// [`crate::package::Package::add_style`] takes
/// (`specs/0027-read-back.md`, "Cell formats"). The Cargo feature `read`.
#[derive(Clone, Debug, PartialEq)]
pub struct CellStyle {
    /// `Code` when the cell format's `numFmtId` names a `numFmt` of the
    /// part, else `BuiltIn` with the id, `BuiltIn(0)` without one.
    pub number_format: NumberFormat,
    /// The font at `fontId`, font 0 without one.
    pub font: Font,
    /// The fill at `fillId`, fill 0 without one: `None` for a fill holding
    /// no child.
    pub fill: Option<Fill>,
    /// The border at `borderId`, border 0 without one.
    pub border: Border,
    /// The cell format's first `alignment` child, `None` without one.
    pub alignment: Option<Alignment>,
    /// The cell format's first `protection` child, `None` without one.
    pub protection: Option<Protection>,
}

/// One `<cellStyle>` of `<cellStyles>`: a named style
/// (`specs/0032-read-breadth-styles.md`, "Named styles"). The Cargo feature
/// `read`.
///
/// Its format is `cell_style_xfs[xf_id]` of [`Styles`], which
/// [`crate::package::Package::named_style`] resolves; a cell format names
/// its named style by the same index, its own `xf_id`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedStyle {
    /// `name`.
    pub name: String,
    /// `xfId`: an index into the part's `<cellStyleXfs>`.
    pub xf_id: u32,
    /// `builtinId`, `None` when absent.
    pub builtin_id: Option<u32>,
    /// `customBuiltin`, absent read as `false`.
    pub custom_builtin: bool,
    /// `hidden`, absent read as `false`.
    pub hidden: bool,
    /// `iLevel`, `None` when absent.
    pub outline_level: Option<u32>,
}

/// The fonts, fills and borders of a styles part, read typed, in part order,
/// and the differential styles of its `<dxfs>`.
///
/// Two compare equal when their elements do, each `f64` (a size, a tint) by
/// its bits, as [`crate::cells::Run`] compares fonts, so the comparison is an
/// equivalence and [`Styles`] stays `Eq`.
#[derive(Clone, Debug, Default)]
pub(crate) struct Typed {
    pub(crate) fonts: Vec<Font>,
    pub(crate) fills: Vec<Option<Fill>>,
    pub(crate) borders: Vec<Border>,
    /// The `dxf` children of the main namespace's `<dxfs>`, in part order.
    pub(crate) dxfs: Vec<Style>,
}

impl PartialEq for Typed {
    fn eq(&self, other: &Self) -> bool {
        self.fonts.len() == other.fonts.len()
            && self
                .fonts
                .iter()
                .zip(&other.fonts)
                .all(|(a, b)| same_font(a, b))
            && self.fills.len() == other.fills.len()
            && self
                .fills
                .iter()
                .zip(&other.fills)
                .all(|(a, b)| match (a, b) {
                    (None, None) => true,
                    (Some(a), Some(b)) => same_fill(a, b),
                    _ => false,
                })
            && self.borders.len() == other.borders.len()
            && self
                .borders
                .iter()
                .zip(&other.borders)
                .all(|(a, b)| same_border(a, b))
            && self.dxfs.len() == other.dxfs.len()
            && self
                .dxfs
                .iter()
                .zip(&other.dxfs)
                .all(|(a, b)| same_style(a, b))
    }
}

/// Whether two styles are equal, each `f64` by its bits.
fn same_style(a: &Style, b: &Style) -> bool {
    let Style {
        number_format,
        font,
        fill,
        border,
        alignment,
        protection,
    } = a;
    let both = |a: bool, b: bool| a == b;
    *number_format == b.number_format
        && match (font, &b.font) {
            (Some(a), Some(b)) => same_font(a, b),
            (a, b) => both(a.is_none(), b.is_none()),
        }
        && match (fill, &b.fill) {
            (Some(a), Some(b)) => same_fill(a, b),
            (a, b) => both(a.is_none(), b.is_none()),
        }
        && match (border, &b.border) {
            (Some(a), Some(b)) => same_border(a, b),
            (a, b) => both(a.is_none(), b.is_none()),
        }
        && *alignment == b.alignment
        && *protection == b.protection
}

impl Eq for Typed {}

/// Whether two fills are equal, each colour as [`same_color`] compares it.
fn same_fill(a: &Fill, b: &Fill) -> bool {
    let Fill {
        pattern,
        foreground,
        background,
        gradient,
    } = a;
    *pattern == b.pattern
        && same_color(foreground.as_ref(), b.foreground.as_ref())
        && same_color(background.as_ref(), b.background.as_ref())
        && match (gradient, &b.gradient) {
            (Some(a), Some(b)) => {
                kind_bits(a.kind) == kind_bits(b.kind)
                    && a.stops.len() == b.stops.len()
                    && a.stops.iter().zip(&b.stops).all(|(a, b)| {
                        a.position.to_bits() == b.position.to_bits()
                            && same_color(Some(&a.color), Some(&b.color))
                    })
            }
            (a, b) => a.is_none() && b.is_none(),
        }
}

/// A gradient's kind as the bits of its numbers, so two compare as
/// [`same_color`] compares a tint.
fn kind_bits(kind: GradientKind) -> (bool, [u64; 4]) {
    match kind {
        GradientKind::Linear { degree } => (false, [degree.to_bits(), 0, 0, 0]),
        GradientKind::Path {
            left,
            right,
            top,
            bottom,
        } => (
            true,
            [
                left.to_bits(),
                right.to_bits(),
                top.to_bits(),
                bottom.to_bits(),
            ],
        ),
    }
}

/// Whether two borders are equal, each side's colour as [`same_color`]
/// compares it.
fn same_border(a: &Border, b: &Border) -> bool {
    let Border {
        left,
        right,
        top,
        bottom,
        diagonal,
        diagonal_up,
        diagonal_down,
    } = a;
    let same_side = |a: &Option<Side>, b: &Option<Side>| match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a.style == b.style && same_color(a.color.as_ref(), b.color.as_ref()),
        _ => false,
    };
    same_side(left, &b.left)
        && same_side(right, &b.right)
        && same_side(top, &b.top)
        && same_side(bottom, &b.bottom)
        && same_side(diagonal, &b.diagonal)
        && *diagonal_up == b.diagonal_up
        && *diagonal_down == b.diagonal_down
}

impl Styles {
    /// The cell format `index` resolved, as [`CellStyle`] says.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a style cannot be read" for an `index` past
    /// `cellXfs`, and for a `fontId`, `fillId` or `borderId` (or the 0 its
    /// absence reads as) past its list.
    pub(crate) fn cell_style(&self, index: u32) -> Result<CellStyle, Error> {
        self.resolve(Self::entry(&self.cell_xfs, index)?)
    }

    /// The format of the named style at `<cellStyleXfs>` index `index`,
    /// resolved by the one resolver [`Styles::cell_style`] uses.
    ///
    /// # Errors
    ///
    /// As [`Styles::cell_style`], with `cellStyleXfs` for `cellXfs`.
    pub(crate) fn named_style(&self, index: u32) -> Result<CellStyle, Error> {
        self.resolve(Self::entry(&self.cell_style_xfs, index)?)
    }

    /// The differential styles of the main namespace's `<dxfs>`, in part
    /// order.
    pub(crate) fn dxfs(&self) -> &[Style] {
        &self.typed.dxfs
    }

    /// The `font` children of `<fonts>` read typed, in part order: what a
    /// `fontId` names (`specs/0032-read-breadth-styles.md`, "Planning
    /// rulings during P2", ruling 9). The Cargo feature `read`.
    pub fn typed_fonts(&self) -> &[Font] {
        &self.typed.fonts
    }

    /// The `fill` children of `<fills>` read typed, in part order, a fill
    /// holding no child as `None`: what a
    /// `fillId` names (ruling 9 as above). The Cargo feature `read`.
    pub fn typed_fills(&self) -> &[Option<Fill>] {
        &self.typed.fills
    }

    /// The `border` children of `<borders>` read typed, in part order: what
    /// a `borderId` names (ruling 9 as above). The Cargo feature `read`.
    pub fn typed_borders(&self) -> &[Border] {
        &self.typed.borders
    }

    /// The font the format at `<cellStyleXfs>` index 0 names, font 0 when
    /// that format has no `fontId` or the part has no such format
    /// (`specs/0032-read-breadth-styles.md`, "The default font").
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a style cannot be read" for a part with no
    /// `<fonts>`, or a `fontId` past it.
    pub(crate) fn default_font(&self) -> Result<Font, Error> {
        if self.fonts().is_none() {
            return Err(STYLE_UNREADABLE);
        }
        let id = self.cell_style_xfs.first().and_then(|xf| xf.font_id);
        usize::try_from(id.unwrap_or(0))
            .ok()
            .and_then(|at| self.typed.fonts.get(at))
            .cloned()
            .ok_or(STYLE_UNREADABLE)
    }

    /// The entry `index` of `list`, refused past its end.
    fn entry(list: &[Xf], index: u32) -> Result<&Xf, Error> {
        usize::try_from(index)
            .ok()
            .and_then(|at| list.get(at))
            .ok_or(STYLE_UNREADABLE)
    }

    /// The format `xf`, of `<cellXfs>` or `<cellStyleXfs>`, resolved into
    /// the types [`CellStyle`] holds.
    fn resolve(&self, xf: &Xf) -> Result<CellStyle, Error> {
        let at = |id: Option<u32>| usize::try_from(id.unwrap_or(0)).ok();
        let number_format = self.format_of(xf);
        let font = at(xf.font_id)
            .and_then(|at| self.typed.fonts.get(at))
            .ok_or(STYLE_UNREADABLE)?;
        let fill = at(xf.fill_id)
            .and_then(|at| self.typed.fills.get(at))
            .ok_or(STYLE_UNREADABLE)?;
        let border = at(xf.border_id)
            .and_then(|at| self.typed.borders.get(at))
            .ok_or(STYLE_UNREADABLE)?;
        Ok(CellStyle {
            number_format,
            font: font.clone(),
            fill: fill.clone(),
            border: *border,
            alignment: xf.alignment_value,
            protection: xf.protection_value,
        })
    }
}

/// A `<font>` of `<fonts>`, its start tag given to `scope`, read through its
/// end tag by the reader runs share, its typeface child `name`.
pub(crate) fn read_font<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    empty: bool,
) -> Result<Font, Error> {
    if empty {
        return Ok(Font::default());
    }
    read_properties(stream, scope, index, b"name", refusal)
}

/// A `<fill>` of `<fills>`, its start tag given to `scope`, read through its
/// end tag: its `patternFill` or its `gradientFill` as a [`Fill`], `None`
/// for a fill holding no child
/// (`specs/0042-gradient-fills-and-named-styles.md`, "The styles part").
///
/// A `patternFill` with no `patternType` reads as `untyped`: `None` in
/// `<fills>`, `Solid` in a `dxf` (`specs/0032-read-breadth-styles.md`,
/// decision 2).
pub(crate) fn read_fill<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    empty: bool,
    untyped: Pattern,
) -> Result<Option<Fill>, Error> {
    let mut fill = None;
    if empty {
        return Ok(fill);
    }
    let mut seen = false;
    while let Step::Child(child) = step(stream, scope, index)? {
        // The schema gives a fill one child, a pattern or a gradient.
        if !child.main || std::mem::replace(&mut seen, true) {
            return Err(STYLE_UNREADABLE);
        }
        match child.local.as_slice() {
            b"patternFill" => fill = Some(read_pattern(stream, scope, index, &child, untyped)?),
            b"gradientFill" => {
                fill = Some(Fill {
                    pattern: Pattern::None,
                    foreground: None,
                    background: None,
                    gradient: Some(read_gradient(stream, scope, index, &child)?),
                });
            }
            _ => return Err(STYLE_UNREADABLE),
        }
    }
    Ok(fill)
}

/// A `<patternFill>`: `patternType` (absent reads as `untyped`) and its
/// `fgColor` and `bgColor`.
fn read_pattern<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Child,
    untyped: Pattern,
) -> Result<Fill, Error> {
    let pattern = match element.value(b"patternType") {
        None => untyped,
        Some(text) => Pattern::parse(text).ok_or(STYLE_UNREADABLE)?,
    };
    let mut fill = Fill {
        pattern,
        foreground: None,
        background: None,
        gradient: None,
    };
    if element.empty {
        return Ok(fill);
    }
    while let Step::Child(child) = step(stream, scope, index)? {
        let slot = match child.local.as_slice() {
            b"fgColor" if child.main => &mut fill.foreground,
            b"bgColor" if child.main => &mut fill.background,
            _ => return Err(STYLE_UNREADABLE),
        };
        if slot.is_some() {
            return Err(STYLE_UNREADABLE);
        }
        *slot = Some(read_color(stream, scope, index, &child)?);
    }
    Ok(fill)
}

/// A `<gradientFill>`, read through its end tag: `type` absent or `linear`
/// as [`GradientKind::Linear`], its `degree` absent read as 0, `path` with
/// each of `left`, `right`, `top` and `bottom` absent read as 0; each
/// `stop`'s `position` and its one `color`. A number reads as a tint does,
/// any finite value.
fn read_gradient<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Child,
) -> Result<Gradient, Error> {
    let number = |name: &[u8]| element.value(name).map_or(Ok(0.0), finite);
    let kind = match element.value(b"type") {
        None | Some("linear") => GradientKind::Linear {
            degree: number(b"degree")?,
        },
        Some("path") => GradientKind::Path {
            left: number(b"left")?,
            right: number(b"right")?,
            top: number(b"top")?,
            bottom: number(b"bottom")?,
        },
        Some(_) => return Err(STYLE_UNREADABLE),
    };
    let mut stops = Vec::new();
    if !element.empty {
        while let Step::Child(child) = step(stream, scope, index)? {
            if !child.main || child.local != b"stop" {
                return Err(STYLE_UNREADABLE);
            }
            let position = finite(child.value(b"position").ok_or(STYLE_UNREADABLE)?)?;
            let mut color = None;
            if !child.empty {
                while let Step::Child(inner) = step(stream, scope, index)? {
                    if !inner.main || inner.local != b"color" || color.is_some() {
                        return Err(STYLE_UNREADABLE);
                    }
                    color = Some(read_color(stream, scope, index, &inner)?);
                }
            }
            let color = color.ok_or(STYLE_UNREADABLE)?;
            stops.push(Stop { position, color });
        }
    }
    Ok(Gradient { kind, stops })
}

/// A number attribute's value as a finite `f64`, as a tint is read.
fn finite(text: &str) -> Result<f64, Error> {
    text.parse::<f64>()
        .ok()
        .filter(|number| number.is_finite())
        .ok_or(STYLE_UNREADABLE)
}

/// A `<border>` of `<borders>`, its start tag's attributes in `element`,
/// read through its end tag: `diagonalUp`, `diagonalDown` and the five
/// sides, each `Some` when it carries a `style` other than `none`.
///
/// With `inside`, a `dxf`'s border, the inside-of-range sides `vertical`
/// and `horizontal`, which the type has no field for, are read past with
/// their subtrees, once each; in `<borders>` they are refused.
pub(crate) fn read_border<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Child,
    inside: bool,
) -> Result<Border, Error> {
    let mut border = Border {
        diagonal_up: element.flag(b"diagonalUp")?.unwrap_or(false),
        diagonal_down: element.flag(b"diagonalDown")?.unwrap_or(false),
        ..Border::default()
    };
    if element.empty {
        return Ok(border);
    }
    let mut seen = [false; 7];
    while let Step::Child(child) = step(stream, scope, index)? {
        let past = match child.local.as_slice() {
            b"vertical" if inside && child.main => Some(5),
            b"horizontal" if inside && child.main => Some(6),
            _ => None,
        };
        if let Some(at) = past {
            if std::mem::replace(&mut seen[at], true) {
                return Err(STYLE_UNREADABLE);
            }
            if !child.empty {
                skip_subtree(stream, scope, index)?;
            }
            continue;
        }
        let (at, slot) = match child.local.as_slice() {
            b"left" if child.main => (0, &mut border.left),
            b"right" if child.main => (1, &mut border.right),
            b"top" if child.main => (2, &mut border.top),
            b"bottom" if child.main => (3, &mut border.bottom),
            b"diagonal" if child.main => (4, &mut border.diagonal),
            _ => return Err(STYLE_UNREADABLE),
        };
        if std::mem::replace(&mut seen[at], true) {
            return Err(STYLE_UNREADABLE);
        }
        *slot = read_side(stream, scope, index, &child)?;
    }
    Ok(border)
}

/// One side of a border, read through its end tag: `Some` when it carries a
/// `style` other than `none`, with its `color`.
fn read_side<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Child,
) -> Result<Option<Side>, Error> {
    let style = match element.value(b"style") {
        None | Some("none") => None,
        Some(text) => Some(BorderStyle::parse(text).ok_or(STYLE_UNREADABLE)?),
    };
    let mut color = None;
    if !element.empty {
        while let Step::Child(child) = step(stream, scope, index)? {
            if !child.main || child.local != b"color" || color.is_some() {
                return Err(STYLE_UNREADABLE);
            }
            color = Some(read_color(stream, scope, index, &child)?);
        }
    }
    Ok(style.map(|style| Side { style, color }))
}

/// A colour element, read through its end tag, its form as a run's colour
/// is read.
fn read_color<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Child,
) -> Result<Color, Error> {
    let owned = |name: &[u8]| element.value(name).map(str::to_owned);
    let color = color_from(
        ColorAttributes {
            rgb: owned(b"rgb"),
            theme: owned(b"theme"),
            tint: owned(b"tint"),
            indexed: owned(b"indexed"),
            auto: owned(b"auto"),
        },
        refusal,
    )?;
    if !element.empty && matches!(step(stream, scope, index)?, Step::Child(_)) {
        return Err(STYLE_UNREADABLE);
    }
    Ok(color)
}

/// An `<indexedColors>` of the main namespace's `<colors>`, its start tag
/// given to `scope`, read through its end tag: each `rgbColor`'s `rgb`, in
/// order, as [`Color::Rgb`] holds one (`specs/0032-read-breadth-styles.md`,
/// "Colours", and "Planning rulings during P2", ruling 1). A child outside
/// the main namespace is walked past.
///
/// # Errors
///
/// [`Error::Package`] "a style cannot be read" for a main-namespace child
/// other than `rgbColor`, and an `rgbColor` with no `rgb` or one that is not
/// eight hex digits.
pub(crate) fn read_indexed_colors<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    empty: bool,
) -> Result<Vec<u32>, Error> {
    let mut colors = Vec::new();
    if empty {
        return Ok(colors);
    }
    while let Step::Child(child) = step(stream, scope, index)? {
        if !child.main {
            if !child.empty {
                skip_subtree(stream, scope, index)?;
            }
            continue;
        }
        if child.local != b"rgbColor" {
            return Err(STYLE_UNREADABLE);
        }
        let rgb = child.value(b"rgb").ok_or(STYLE_UNREADABLE)?;
        match color_from(
            ColorAttributes {
                rgb: Some(rgb.to_owned()),
                theme: None,
                tint: None,
                indexed: None,
                auto: None,
            },
            refusal,
        )? {
            Color::Rgb(value) => colors.push(value),
            _ => return Err(STYLE_UNREADABLE),
        }
        if !child.empty {
            skip_subtree(stream, scope, index)?;
        }
    }
    Ok(colors)
}

/// A `<dxf>` of the main namespace's `<dxfs>`, its start tag given to
/// `scope`, read through its end tag into the [`Style`] the writers take
/// (`specs/0032-read-breadth-styles.md`, "Differential styles"): `numFmt`'s
/// `formatCode` as [`NumberFormat::Code`], `font`, `fill` and `border`
/// through the readers `<fonts>`, `<fills>` and `<borders>` use, a
/// `patternFill` with no `patternType` as [`Pattern::Solid`] and a border's
/// inside-of-range sides read past, and `alignment` and `protection` from
/// their start tags as an `xf`'s are; each `None` without its child.
///
/// # Errors
///
/// [`Error::Package`] "a style cannot be read" for a child outside the main
/// namespace, one the type has no field for, a second of one kind, a
/// `numFmt` with no `formatCode`, and whatever the readers refuse.
pub(crate) fn read_dxf<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    empty: bool,
) -> Result<Style, Error> {
    let mut style = Style::default();
    if empty {
        return Ok(style);
    }
    // The kinds met, in the schema's order: a second of one is refused.
    let mut seen = [false; 6];
    loop {
        let Some(token) = stream.next()? else {
            return Err(Error::Xml(index));
        };
        // A font, a fill or a border, read after the match, because `token`
        // borrows the stream its reader reads.
        let child = match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                let child = Child::of(start, scope, index, empty)?;
                let kind = [
                    b"font".as_slice(),
                    b"numFmt",
                    b"fill",
                    b"alignment",
                    b"protection",
                    b"border",
                ]
                .iter()
                .position(|name| *name == child.local.as_slice());
                let Some(kind) = kind.filter(|_| child.main) else {
                    return Err(STYLE_UNREADABLE);
                };
                if std::mem::replace(&mut seen[kind], true) {
                    return Err(STYLE_UNREADABLE);
                }
                let read = match child.local.as_slice() {
                    b"numFmt" => {
                        let code = text_attribute(start, scope, index, b"formatCode")?
                            .ok_or(STYLE_UNREADABLE)?;
                        style.number_format = Some(NumberFormat::Code(code));
                        true
                    }
                    b"alignment" => {
                        style.alignment = Some(read_alignment(start, scope, index)?);
                        true
                    }
                    b"protection" => {
                        style.protection = Some(read_protection(start, scope, index)?);
                        true
                    }
                    // A font, a fill or a border.
                    _ => false,
                };
                if empty {
                    scope.end();
                }
                if read {
                    // Read from its start tag; anything inside is not
                    // examined.
                    if !empty {
                        skip_subtree(stream, scope, index)?;
                    }
                    continue;
                }
                child
            }
            Event::End(_) => {
                scope.end();
                return Ok(style);
            }
            // Whitespace between the children.
            _ => continue,
        };
        let empty = child.empty;
        match child.local.as_slice() {
            b"font" => style.font = Some(read_font(stream, scope, index, empty)?),
            // A fill holding no child reads as no fill.
            b"fill" => style.fill = read_fill(stream, scope, index, empty, Pattern::Solid)?,
            _ => style.border = Some(read_border(stream, scope, index, &child, true)?),
        }
    }
}

/// A `<cellStyle>` from its start tag (`specs/0032-read-breadth-styles.md`,
/// "Named styles").
///
/// # Errors
///
/// [`Error::Package`] "a style cannot be read" for a `cellStyle` with no
/// `name` or no `xfId`, or a value that does not parse.
pub(crate) fn read_named_style(
    start: &BytesStart<'_>,
    scope: &Scope,
    index: usize,
) -> Result<NamedStyle, Error> {
    let element = Child::of(start, scope, index, true)?;
    let number = |name: &[u8]| -> Result<Option<u32>, Error> {
        element
            .value(name)
            .map(|text| decimal(text).ok_or(STYLE_UNREADABLE))
            .transpose()
    };
    let name = text_attribute(start, scope, index, b"name")?.ok_or(STYLE_UNREADABLE)?;
    Ok(NamedStyle {
        name,
        xf_id: number(b"xfId")?.ok_or(STYLE_UNREADABLE)?,
        builtin_id: number(b"builtinId")?,
        custom_builtin: element.flag(b"customBuiltin")?.unwrap_or(false),
        hidden: element.flag(b"hidden")?.unwrap_or(false),
        outline_level: number(b"iLevel")?,
    })
}

/// The unprefixed attribute `name` of `start` as text, unescaped and
/// normalised as a `formatCode` of `<numFmts>` is (the `styles` module
/// header), `None` when absent.
fn text_attribute(
    start: &BytesStart<'_>,
    scope: &Scope,
    index: usize,
    name: &[u8],
) -> Result<Option<String>, Error> {
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| Error::Xml(index))?;
        if attribute.key.as_namespace_binding().is_some() {
            continue;
        }
        let (namespace, local) = scope.resolve_attribute(attribute.key)?;
        if namespace.is_none() && local == name {
            let text = attribute
                .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                .map_err(|_| Error::Xml(index))?;
            return Ok(Some(text.into_owned()));
        }
    }
    Ok(None)
}

/// An `xf`'s `<alignment>` from its start tag: each field from its
/// attribute; `justifyLastLine`, `readingOrder` and `relativeIndent` read
/// past.
pub(crate) fn read_alignment(
    start: &BytesStart<'_>,
    scope: &Scope,
    index: usize,
) -> Result<Alignment, Error> {
    let element = Child::of(start, scope, index, true)?;
    let number = |name: &[u8]| -> Result<Option<u32>, Error> {
        element
            .value(name)
            .map(|text| decimal(text).ok_or(STYLE_UNREADABLE))
            .transpose()
    };
    let text_rotation = number(b"textRotation")?;
    // The writer's range: 0 to 180 degrees, or 255.
    if text_rotation.is_some_and(|rotation| rotation > 180 && rotation != 255) {
        return Err(STYLE_UNREADABLE);
    }
    Ok(Alignment {
        horizontal: element
            .value(b"horizontal")
            .map(|text| Horizontal::parse(text).ok_or(STYLE_UNREADABLE))
            .transpose()?,
        vertical: element
            .value(b"vertical")
            .map(|text| Vertical::parse(text).ok_or(STYLE_UNREADABLE))
            .transpose()?,
        wrap_text: element.flag(b"wrapText")?.unwrap_or(false),
        shrink_to_fit: element.flag(b"shrinkToFit")?.unwrap_or(false),
        indent: number(b"indent")?,
        text_rotation,
    })
}

/// An `xf`'s `<protection>` from its start tag.
pub(crate) fn read_protection(
    start: &BytesStart<'_>,
    scope: &Scope,
    index: usize,
) -> Result<Protection, Error> {
    let element = Child::of(start, scope, index, true)?;
    Ok(Protection {
        locked: element.flag(b"locked")?,
        hidden: element.flag(b"hidden")?,
    })
}

/// An element met: whether it is in the main namespace, its local name, its
/// unprefixed attributes as written, and whether it was an empty-element tag.
pub(crate) struct Child {
    main: bool,
    local: Vec<u8>,
    attributes: Vec<(Vec<u8>, String)>,
    empty: bool,
}

impl Child {
    /// The element `start`, which `scope` has been given.
    pub(crate) fn of(
        start: &BytesStart<'_>,
        scope: &Scope,
        index: usize,
        empty: bool,
    ) -> Result<Self, Error> {
        let (namespace, local) = scope.resolve_element(start.name())?;
        let main = MAIN.contains(&namespace.unwrap_or_default());
        let local = local.to_vec();
        let mut attributes = Vec::new();
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|_| Error::Xml(index))?;
            if attribute.key.as_namespace_binding().is_some() {
                continue;
            }
            let (namespace, name) = scope.resolve_attribute(attribute.key)?;
            if namespace.is_some() {
                continue;
            }
            attributes.push((name.to_vec(), attribute.value.as_ref().to_owned()));
        }
        Ok(Self {
            main,
            local,
            attributes,
            empty,
        })
    }

    /// Whether it was an empty-element tag.
    pub(crate) fn empty(&self) -> bool {
        self.empty
    }

    /// The attribute `name`'s value, as written.
    fn value(&self, name: &[u8]) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    /// The attribute `name` as a flag: `1` or `true`, `0` or `false`.
    fn flag(&self, name: &[u8]) -> Result<Option<bool>, Error> {
        match self.value(name) {
            None => Ok(None),
            Some("1") | Some("true") => Ok(Some(true)),
            Some("0") | Some("false") => Ok(Some(false)),
            Some(_) => Err(STYLE_UNREADABLE),
        }
    }
}

/// One step through an element's children.
enum Step {
    /// A child's start tag, given to the scope, and given back when it was
    /// an empty-element tag.
    Child(Child),
    /// The element's own end tag, given back to the scope.
    End,
}

/// The next child of the element being read, or its end. Text and comments
/// between the children are passed over.
fn step<R: Read>(stream: &mut Stream<R>, scope: &mut Scope, index: usize) -> Result<Step, Error> {
    loop {
        let Some(token) = stream.next()? else {
            return Err(Error::Xml(index));
        };
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                let child = Child::of(start, scope, index, empty)?;
                if empty {
                    scope.end();
                }
                return Ok(Step::Child(child));
            }
            Event::End(_) => {
                scope.end();
                return Ok(Step::End);
            }
            _ => {}
        }
    }
}
