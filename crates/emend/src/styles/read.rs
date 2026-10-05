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
//! holds no `patternFill`, a gradient among them, reads as `None`.
//! Everything else this layer does not know is refused with "a style cannot
//! be read".

use std::io::Read;

use quick_xml::events::{BytesStart, Event};

use crate::archive::Error;
use crate::cells::{
    ColorAttributes, color_from, decimal, read_properties, same_color, same_font, skip_subtree,
};
use crate::package::MAIN;
use crate::styles::{
    Alignment, Border, BorderStyle, Color, Fill, Font, Horizontal, NumberFormat, Pattern,
    Protection, Side, Styles, Vertical,
};
use crate::xml::{Scope, Stream};

/// What a styles part this layer cannot read into the writers' types is
/// refused with, at the read of the part and at [`Styles::cell_style`];
/// catalogued in [`crate::package`]'s module header.
pub(crate) const STYLE_UNREADABLE: Error = Error::Package("a style cannot be read");

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
    /// no `patternFill`, which a gradient fill is.
    pub fill: Option<Fill>,
    /// The border at `borderId`, border 0 without one.
    pub border: Border,
    /// The cell format's first `alignment` child, `None` without one.
    pub alignment: Option<Alignment>,
    /// The cell format's first `protection` child, `None` without one.
    pub protection: Option<Protection>,
}

/// The fonts, fills and borders of a styles part, read typed, in part order.
///
/// Two compare equal when their elements do, each `f64` (a size, a tint) by
/// its bits, as [`crate::cells::Run`] compares fonts, so the comparison is an
/// equivalence and [`Styles`] stays `Eq`.
#[derive(Clone, Debug, Default)]
pub(crate) struct Typed {
    pub(crate) fonts: Vec<Font>,
    pub(crate) fills: Vec<Option<Fill>>,
    pub(crate) borders: Vec<Border>,
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
    }
}

impl Eq for Typed {}

/// Whether two fills are equal, each colour as [`same_color`] compares it.
fn same_fill(a: &Fill, b: &Fill) -> bool {
    let Fill {
        pattern,
        foreground,
        background,
    } = a;
    *pattern == b.pattern
        && same_color(foreground.as_ref(), b.foreground.as_ref())
        && same_color(background.as_ref(), b.background.as_ref())
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
        let at = |id: Option<u32>| usize::try_from(id.unwrap_or(0)).ok();
        let xf = usize::try_from(index)
            .ok()
            .and_then(|at| self.cell_xfs.get(at))
            .ok_or(STYLE_UNREADABLE)?;
        let number_format = self.number_format(index).ok_or(STYLE_UNREADABLE)?;
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
            fill: *fill,
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
/// end tag: its `patternFill` as a [`Fill`], `None` for a fill holding no
/// `patternFill` (a `gradientFill`, read past with its subtree, or no child).
pub(crate) fn read_fill<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    empty: bool,
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
            b"patternFill" => fill = Some(read_pattern(stream, scope, index, &child)?),
            b"gradientFill" => {
                if !child.empty {
                    skip_subtree(stream, scope, index)?;
                }
            }
            _ => return Err(STYLE_UNREADABLE),
        }
    }
    Ok(fill)
}

/// A `<patternFill>`: `patternType` (absent reads as `none`) and its
/// `fgColor` and `bgColor`.
fn read_pattern<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Child,
) -> Result<Fill, Error> {
    let pattern = match element.value(b"patternType") {
        None => Pattern::None,
        Some(text) => Pattern::parse(text).ok_or(STYLE_UNREADABLE)?,
    };
    let mut fill = Fill {
        pattern,
        foreground: None,
        background: None,
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

/// A `<border>` of `<borders>`, its start tag's attributes in `element`,
/// read through its end tag: `diagonalUp`, `diagonalDown` and the five
/// sides, each `Some` when it carries a `style` other than `none`.
pub(crate) fn read_border<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Child,
) -> Result<Border, Error> {
    let mut border = Border {
        diagonal_up: element.flag(b"diagonalUp")?.unwrap_or(false),
        diagonal_down: element.flag(b"diagonalDown")?.unwrap_or(false),
        ..Border::default()
    };
    if element.empty {
        return Ok(border);
    }
    let mut seen = [false; 5];
    while let Step::Child(child) = step(stream, scope, index)? {
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
