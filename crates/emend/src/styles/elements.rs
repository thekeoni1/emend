//! What a new cell format can carry beyond its number format, and the
//! elements a styles part gets for it.
//!
//! `specs/0017-fuller-styles.md`, "Module `styles`: what can be added". A
//! [`Style`] is given to [`crate::package::Package::add_style`]; each `Some`
//! field is what the new cell format gets. A [`Font`], a [`Fill`] and a
//! [`Border`] each become one element appended to the part's `<fonts>`,
//! `<fills>` or `<borders>`; an [`Alignment`] and a [`Protection`] are
//! children of the cell format itself.
//!
//! Every keyword a variant writes is ECMA-376 Part 1's, from the simple
//! types of 18.18, in their lower camel case spelling.
//!
//! An element is composed when the addition is recorded, with every
//! attribute double-quoted, no space before `/>`, its children in the
//! schema's order and only the children and attributes the given fields ask
//! for. Its name, and the name of each child, takes the qualified-name prefix
//! of the container it goes into, which only the styles rewrite reads.

use crate::archive::Error;
use crate::cells::{forbidden_in_xml, number_text};
use crate::styles::attribute_text;

#[cfg(test)]
mod tests;

/// What [`crate::package::Package::add_style`] refuses a field with,
/// catalogued in [`crate::package`]'s module header.
pub(crate) const FONT_NAME_REFUSED: Error =
    Error::Package("a font name is empty or holds a character XML cannot carry");
pub(crate) const FIELD_OUT_OF_RANGE: Error = Error::Package("a style field is out of range");

/// What a new cell format gets: each `Some` field is given to it, and each
/// `None` field is what its base has, or the plain form's value with no base.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    /// The number format, as [`crate::package::Package::add_cell_style`]
    /// takes one.
    pub number_format: Option<crate::styles::NumberFormat>,
    /// A font, appended to `<fonts>`.
    pub font: Option<Font>,
    /// A fill, appended to `<fills>`.
    pub fill: Option<Fill>,
    /// A border, appended to `<borders>`.
    pub border: Option<Border>,
    /// The cell format's `alignment` child, in place of its base's.
    pub alignment: Option<Alignment>,
    /// The cell format's `protection` child, in place of its base's.
    pub protection: Option<Protection>,
}

/// A font: the children of a `<font>` element.
///
/// A font with every field `None` and `false` is written `<font/>`, which the
/// schema allows; a consumer shows its default font for it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Font {
    /// `name`: the typeface. Refused when empty or holding a character XML
    /// cannot carry.
    pub name: Option<String>,
    /// `sz`: the size in points, finite and above zero.
    pub size: Option<f64>,
    /// `b`.
    pub bold: bool,
    /// `i`.
    pub italic: bool,
    /// `strike`.
    pub strike: bool,
    /// `u`.
    pub underline: Option<Underline>,
    /// `vertAlign`.
    pub vertical_align: Option<VerticalAlign>,
    /// `color`.
    pub color: Option<Color>,
    /// `family`: the font family number.
    pub family: Option<u32>,
    /// `scheme`: the theme font this font stands for.
    pub scheme: Option<Scheme>,
}

/// An underline, `ST_UnderlineValues` less `none`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Underline {
    /// `single`.
    Single,
    /// `double`.
    Double,
    /// `singleAccounting`.
    SingleAccounting,
    /// `doubleAccounting`.
    DoubleAccounting,
}

/// A run's vertical position, `ST_VerticalAlignRun`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerticalAlign {
    /// `superscript`.
    Superscript,
    /// `subscript`.
    Subscript,
    /// `baseline`.
    Baseline,
}

/// A theme font, `ST_FontScheme` less `none`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scheme {
    /// `major`.
    Major,
    /// `minor`.
    Minor,
}

/// A colour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    /// `rgb`: ARGB, written as eight upper-case hex digits. An opaque colour
    /// has `FF` in the top byte.
    Rgb(u32),
    /// `theme`: whatever the workbook's theme defines at `index`, with
    /// `tint`, a fraction in `-1.0..=1.0`, when given.
    Theme {
        /// The theme colour's index.
        index: u32,
        /// Lighter above zero, darker below.
        tint: Option<f64>,
    },
    /// `indexed`: whatever the workbook's `colors` or the default palette
    /// defines at that index.
    Indexed(u32),
    /// `auto`: the consumer's automatic colour.
    Auto,
}

/// A pattern fill, or a gradient fill. A solid fill shows its foreground,
/// not its background, as in Excel.
///
/// The schema gives a fill one child, so a fill whose `gradient` is `Some`
/// has `pattern` [`Pattern::None`] and both colours `None`, and is written
/// as a `gradientFill` (`specs/0042-gradient-fills-and-named-styles.md`,
/// "Types").
#[derive(Clone, Debug, PartialEq)]
pub struct Fill {
    /// `patternType`.
    pub pattern: Pattern,
    /// `fgColor`: the pattern's colour, and a solid fill's.
    pub foreground: Option<Color>,
    /// `bgColor`: the colour behind the pattern.
    pub background: Option<Color>,
    /// `gradientFill`: the fill is a gradient, in place of the pattern.
    pub gradient: Option<Gradient>,
}

/// A gradient fill, `gradientFill`: its kind and its stops, at least two,
/// their positions non-decreasing.
#[derive(Clone, Debug, PartialEq)]
pub struct Gradient {
    /// Linear at an angle, or a path from a rectangle.
    pub kind: GradientKind,
    /// The `stop` children, in order.
    pub stops: Vec<Stop>,
}

/// A gradient's kind, `ST_GradientType`, with its attributes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GradientKind {
    /// `linear`, at `degree`, finite and in `0.0..=360.0`.
    Linear {
        /// `degree`.
        degree: f64,
    },
    /// `path`: the colours run out from the rectangle the four give, each a
    /// fraction of the cell, finite and in `0.0..=1.0`.
    Path {
        /// `left`.
        left: f64,
        /// `right`.
        right: f64,
        /// `top`.
        top: f64,
        /// `bottom`.
        bottom: f64,
    },
}

/// One `stop` of a gradient.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stop {
    /// `position`: finite and in `0.0..=1.0`.
    pub position: f64,
    /// The stop's `color`, checked as a fill's colour is.
    pub color: Color,
}

/// A fill pattern, `ST_PatternType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// `none`.
    None,
    /// `solid`.
    Solid,
    /// `mediumGray`.
    MediumGray,
    /// `darkGray`.
    DarkGray,
    /// `lightGray`.
    LightGray,
    /// `darkHorizontal`.
    DarkHorizontal,
    /// `darkVertical`.
    DarkVertical,
    /// `darkDown`.
    DarkDown,
    /// `darkUp`.
    DarkUp,
    /// `darkGrid`.
    DarkGrid,
    /// `darkTrellis`.
    DarkTrellis,
    /// `lightHorizontal`.
    LightHorizontal,
    /// `lightVertical`.
    LightVertical,
    /// `lightDown`.
    LightDown,
    /// `lightUp`.
    LightUp,
    /// `lightGrid`.
    LightGrid,
    /// `lightTrellis`.
    LightTrellis,
    /// `gray125`.
    Gray125,
    /// `gray0625`.
    Gray0625,
}

/// A cell's border: four sides and a diagonal, each written, the `None` ones
/// as empty elements.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Border {
    /// `left`.
    pub left: Option<Side>,
    /// `right`.
    pub right: Option<Side>,
    /// `top`.
    pub top: Option<Side>,
    /// `bottom`.
    pub bottom: Option<Side>,
    /// `diagonal`.
    pub diagonal: Option<Side>,
    /// `diagonalUp`: the diagonal runs from bottom left to top right.
    pub diagonal_up: bool,
    /// `diagonalDown`: the diagonal runs from top left to bottom right.
    pub diagonal_down: bool,
}

/// One side of a border.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Side {
    /// `style`.
    pub style: BorderStyle,
    /// `color`.
    pub color: Option<Color>,
}

/// A border line, `ST_BorderStyle` less `none`, which is a `None` side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorderStyle {
    /// `thin`.
    Thin,
    /// `medium`.
    Medium,
    /// `thick`.
    Thick,
    /// `dashed`.
    Dashed,
    /// `dotted`.
    Dotted,
    /// `double`.
    Double,
    /// `hair`.
    Hair,
    /// `mediumDashed`.
    MediumDashed,
    /// `dashDot`.
    DashDot,
    /// `mediumDashDot`.
    MediumDashDot,
    /// `dashDotDot`.
    DashDotDot,
    /// `mediumDashDotDot`.
    MediumDashDotDot,
    /// `slantDashDot`.
    SlantDashDot,
}

/// A cell format's `alignment` child.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Alignment {
    /// `horizontal`.
    pub horizontal: Option<Horizontal>,
    /// `vertical`.
    pub vertical: Option<Vertical>,
    /// `wrapText`.
    pub wrap_text: bool,
    /// `shrinkToFit`.
    pub shrink_to_fit: bool,
    /// `indent`.
    pub indent: Option<u32>,
    /// `textRotation`: 0 to 180 degrees, or 255 for vertical text.
    pub text_rotation: Option<u32>,
}

/// A horizontal alignment, `ST_HorizontalAlignment`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Horizontal {
    /// `general`.
    General,
    /// `left`.
    Left,
    /// `center`.
    Center,
    /// `right`.
    Right,
    /// `fill`.
    Fill,
    /// `justify`.
    Justify,
    /// `centerContinuous`.
    CenterContinuous,
    /// `distributed`.
    Distributed,
}

/// A vertical alignment, `ST_VerticalAlignment`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vertical {
    /// `top`.
    Top,
    /// `center`.
    Center,
    /// `bottom`.
    Bottom,
    /// `justify`.
    Justify,
    /// `distributed`.
    Distributed,
}

/// A cell format's `protection` child.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Protection {
    /// `locked`.
    pub locked: Option<bool>,
    /// `hidden`.
    pub hidden: Option<bool>,
}

impl Underline {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Double => "double",
            Self::SingleAccounting => "singleAccounting",
            Self::DoubleAccounting => "doubleAccounting",
        }
    }
}

impl VerticalAlign {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::Superscript => "superscript",
            Self::Subscript => "subscript",
            Self::Baseline => "baseline",
        }
    }
}

impl Scheme {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
        }
    }
}

impl Pattern {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Solid => "solid",
            Self::MediumGray => "mediumGray",
            Self::DarkGray => "darkGray",
            Self::LightGray => "lightGray",
            Self::DarkHorizontal => "darkHorizontal",
            Self::DarkVertical => "darkVertical",
            Self::DarkDown => "darkDown",
            Self::DarkUp => "darkUp",
            Self::DarkGrid => "darkGrid",
            Self::DarkTrellis => "darkTrellis",
            Self::LightHorizontal => "lightHorizontal",
            Self::LightVertical => "lightVertical",
            Self::LightDown => "lightDown",
            Self::LightUp => "lightUp",
            Self::LightGrid => "lightGrid",
            Self::LightTrellis => "lightTrellis",
            Self::Gray125 => "gray125",
            Self::Gray0625 => "gray0625",
        }
    }
}

impl BorderStyle {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::Thin => "thin",
            Self::Medium => "medium",
            Self::Thick => "thick",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Double => "double",
            Self::Hair => "hair",
            Self::MediumDashed => "mediumDashed",
            Self::DashDot => "dashDot",
            Self::MediumDashDot => "mediumDashDot",
            Self::DashDotDot => "dashDotDot",
            Self::MediumDashDotDot => "mediumDashDotDot",
            Self::SlantDashDot => "slantDashDot",
        }
    }
}

impl Horizontal {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Fill => "fill",
            Self::Justify => "justify",
            Self::CenterContinuous => "centerContinuous",
            Self::Distributed => "distributed",
        }
    }
}

impl Vertical {
    pub(crate) fn keyword(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Center => "center",
            Self::Bottom => "bottom",
            Self::Justify => "justify",
            Self::Distributed => "distributed",
        }
    }
}

/// The variant whose [`keyword`](Pattern::keyword) is `text`, for each enum
/// a typed read parses (`specs/0027-read-back.md`, "Cell formats"): every
/// variant listed once, so a keyword reads back as the variant that writes
/// it, and the match below fails to compile when a variant is left out.
/// The Cargo feature `read`.
#[cfg(feature = "read")]
macro_rules! parse_keyword {
    ($type:ident: $($variant:ident),+ $(,)?) => {
        const _: fn($type) = |variant| match variant {
            $($type::$variant)|+ => {}
        };
        impl $type {
            pub(crate) fn parse(text: &str) -> Option<Self> {
                [$(Self::$variant),+]
                    .into_iter()
                    .find(|variant| variant.keyword() == text)
            }
        }
    };
}

#[cfg(feature = "read")]
parse_keyword!(Pattern: None, Solid, MediumGray, DarkGray, LightGray, DarkHorizontal,
    DarkVertical, DarkDown, DarkUp, DarkGrid, DarkTrellis, LightHorizontal, LightVertical,
    LightDown, LightUp, LightGrid, LightTrellis, Gray125, Gray0625);
#[cfg(feature = "read")]
parse_keyword!(BorderStyle: Thin, Medium, Thick, Dashed, Dotted, Double, Hair, MediumDashed,
    DashDot, MediumDashDot, DashDotDot, MediumDashDotDot, SlantDashDot);
#[cfg(feature = "read")]
parse_keyword!(Horizontal: General, Left, Center, Right, Fill, Justify, CenterContinuous,
    Distributed);
#[cfg(feature = "read")]
parse_keyword!(Vertical: Top, Center, Bottom, Justify, Distributed);

/// An element composed when its addition is recorded, and the positions in
/// it where its container's prefix goes: after each `<` and `</` that opens
/// a name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Composed {
    bytes: Vec<u8>,
    names: Vec<usize>,
}

impl Composed {
    pub(crate) fn new() -> Self {
        Self {
            bytes: Vec::new(),
            names: Vec::new(),
        }
    }

    /// `<name`.
    fn open(&mut self, name: &str) {
        self.bytes.push(b'<');
        self.names.push(self.bytes.len());
        self.bytes.extend_from_slice(name.as_bytes());
    }

    /// ` name="value"`; `value` is already escaped.
    fn attribute(&mut self, name: &str, value: &str) {
        self.bytes.push(b' ');
        self.bytes.extend_from_slice(name.as_bytes());
        self.bytes.extend_from_slice(b"=\"");
        self.bytes.extend_from_slice(value.as_bytes());
        self.bytes.push(b'"');
    }

    /// `>` or `/>`.
    fn end_tag(&mut self, empty: bool) {
        self.bytes
            .extend_from_slice(if empty { b"/>".as_slice() } else { b">" });
    }

    /// `</name>`.
    fn close(&mut self, name: &str) {
        self.bytes.extend_from_slice(b"</");
        self.names.push(self.bytes.len());
        self.bytes.extend_from_slice(name.as_bytes());
        self.bytes.push(b'>');
    }

    /// `<name/>`.
    fn empty(&mut self, name: &str) {
        self.open(name);
        self.end_tag(true);
    }

    /// `<name val="value"/>`.
    fn val(&mut self, name: &str, value: &str) {
        self.open(name);
        self.attribute("val", value);
        self.end_tag(true);
    }

    /// A colour element named `name`.
    fn color(&mut self, name: &str, color: &Color) {
        self.open(name);
        match *color {
            Color::Rgb(argb) => self.attribute("rgb", &format!("{argb:08X}")),
            Color::Theme { index, tint } => {
                self.attribute("theme", &index.to_string());
                if let Some(tint) = tint {
                    self.attribute("tint", &number_text(tint));
                }
            }
            Color::Indexed(index) => self.attribute("indexed", &index.to_string()),
            Color::Auto => self.attribute("auto", "1"),
        }
        self.end_tag(true);
    }

    /// `bytes` as they are: a base's own bytes, which carry their own names.
    pub(crate) fn raw(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// `other` after these bytes, its names still taking the prefix.
    pub(crate) fn append(&mut self, other: &Composed) {
        let at = self.bytes.len();
        self.names.extend(other.names.iter().map(|name| name + at));
        self.bytes.extend_from_slice(&other.bytes);
    }

    /// Whether nothing has been composed.
    pub(crate) fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The element, with `prefix` (with its colon, or empty) on every name.
    pub(crate) fn element(&self, prefix: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.bytes.len() + self.names.len() * prefix.len());
        let mut from = 0;
        for &at in &self.names {
            out.extend_from_slice(&self.bytes[from..at]);
            out.extend_from_slice(prefix);
            from = at;
        }
        out.extend_from_slice(&self.bytes[from..]);
        out
    }
}

/// A colour element named `name`, as a font or a fill writes one, for a
/// conditional format's visual rule
/// (`specs/0020-validation-and-conditional-formatting.md`, "Conditional
/// formatting"), and for a worksheet's `<tabColor>`
/// (`specs/0025-sheet-and-print-settings.md`, "Types"). The Cargo feature
/// `rules` or `sheetprops`.
#[cfg(any(feature = "rules", feature = "sheetprops"))]
pub(crate) fn color_element(name: &str, color: &Color) -> Composed {
    let mut out = Composed::new();
    out.color(name, color);
    out
}

/// Refuses a tint that is not finite or lies outside `-1.0..=1.0`.
fn check_color(color: Option<&Color>) -> Result<(), Error> {
    match color {
        Some(Color::Theme {
            tint: Some(tint), ..
        }) if !(-1.0..=1.0).contains(tint) => Err(FIELD_OUT_OF_RANGE),
        _ => Ok(()),
    }
}

impl Font {
    /// The `<font>` element, or the refusal of a field.
    pub(crate) fn compose(&self) -> Result<Composed, Error> {
        self.compose_as("font", "name")
    }

    /// The font's element named `element`, its typeface child named `name`,
    /// or the refusal of a field: `<font>` with `name` for a styles part, and
    /// with the Cargo feature `richtext` a run's `<rPr>` with `rFont`
    /// (`specs/0024-rich-text.md`, "The cell write"). One writer, so the
    /// children come in one order for both.
    pub(crate) fn compose_as(&self, element: &str, name: &str) -> Result<Composed, Error> {
        self.check()?;
        Ok(self.composed(element, name))
    }

    /// The element [`Font::compose_as`] writes, for a font [`Font::check`]
    /// has passed.
    pub(crate) fn composed(&self, element: &str, name: &str) -> Composed {
        let mut out = Composed::new();
        out.open(element);
        if *self == Font::default() {
            out.end_tag(true);
            return out;
        }
        out.end_tag(false);
        for (on, child) in [
            (self.bold, "b"),
            (self.italic, "i"),
            (self.strike, "strike"),
        ] {
            if on {
                out.empty(child);
            }
        }
        match self.underline {
            Some(Underline::Single) => out.empty("u"),
            Some(underline) => out.val("u", underline.keyword()),
            None => {}
        }
        if let Some(align) = self.vertical_align {
            out.val("vertAlign", align.keyword());
        }
        if let Some(size) = self.size {
            out.val("sz", &number_text(size));
        }
        if let Some(color) = &self.color {
            out.color("color", color);
        }
        if let Some(typeface) = &self.name {
            out.val(name, &attribute_text(typeface));
        }
        if let Some(family) = self.family {
            out.val("family", &family.to_string());
        }
        if let Some(scheme) = self.scheme {
            out.val("scheme", scheme.keyword());
        }
        out.close(element);
        out
    }

    /// The refusal of a field [`Font::compose`] cannot write, or nothing.
    pub(crate) fn check(&self) -> Result<(), Error> {
        if let Some(name) = &self.name
            && (name.is_empty() || name.chars().any(forbidden_in_xml))
        {
            return Err(FONT_NAME_REFUSED);
        }
        if let Some(size) = self.size
            && !(size.is_finite() && size > 0.0)
        {
            return Err(FIELD_OUT_OF_RANGE);
        }
        check_color(self.color.as_ref())
    }
}

impl Fill {
    /// The `<fill>` element, or the refusal of a colour or of the gradient.
    pub(crate) fn compose(&self) -> Result<Composed, Error> {
        self.check_colors()?;
        self.check_gradient()?;
        Ok(self.composed())
    }

    /// The refusal of a pattern colour's tint, or nothing.
    pub(crate) fn check_colors(&self) -> Result<(), Error> {
        check_color(self.foreground.as_ref())?;
        check_color(self.background.as_ref())
    }

    /// The refusal of the gradient, or nothing: a pattern or a colour beside
    /// it (the schema's one child), then its fields in declaration order, the
    /// kind's numbers, fewer than two stops, and each stop's position, its
    /// order and its colour.
    pub(crate) fn check_gradient(&self) -> Result<(), Error> {
        let Some(gradient) = &self.gradient else {
            return Ok(());
        };
        if self.pattern != Pattern::None || self.foreground.is_some() || self.background.is_some() {
            return Err(FIELD_OUT_OF_RANGE);
        }
        let within = |number: f64, top: f64| number.is_finite() && (0.0..=top).contains(&number);
        let kind = match gradient.kind {
            GradientKind::Linear { degree } => within(degree, 360.0),
            GradientKind::Path {
                left,
                right,
                top,
                bottom,
            } => [left, right, top, bottom]
                .into_iter()
                .all(|number| within(number, 1.0)),
        };
        if !kind || gradient.stops.len() < 2 {
            return Err(FIELD_OUT_OF_RANGE);
        }
        let mut last = 0.0;
        for stop in &gradient.stops {
            if !within(stop.position, 1.0) || stop.position < last {
                return Err(FIELD_OUT_OF_RANGE);
            }
            last = stop.position;
            check_color(Some(&stop.color))?;
        }
        Ok(())
    }

    /// The element [`Fill::compose`] writes, for a fill its checks have
    /// passed.
    pub(crate) fn composed(&self) -> Composed {
        let mut out = Composed::new();
        out.open("fill");
        out.end_tag(false);
        if let Some(gradient) = &self.gradient {
            gradient.compose_into(&mut out);
            out.close("fill");
            return out;
        }
        out.open("patternFill");
        out.attribute("patternType", self.pattern.keyword());
        if self.foreground.is_none() && self.background.is_none() {
            out.end_tag(true);
        } else {
            out.end_tag(false);
            if let Some(color) = &self.foreground {
                out.color("fgColor", color);
            }
            if let Some(color) = &self.background {
                out.color("bgColor", color);
            }
            out.close("patternFill");
        }
        out.close("fill");
        out
    }
}

impl Gradient {
    /// `<gradientFill ATTRS>` with its stops: `degree` for a linear gradient
    /// whose degree is not 0, the schema's default; `type="path"` and the
    /// four, always written, for a path gradient.
    fn compose_into(&self, out: &mut Composed) {
        out.open("gradientFill");
        match self.kind {
            GradientKind::Linear { degree } => {
                if degree != 0.0 {
                    out.attribute("degree", &number_text(degree));
                }
            }
            GradientKind::Path {
                left,
                right,
                top,
                bottom,
            } => {
                out.attribute("type", "path");
                for (name, number) in [
                    ("left", left),
                    ("right", right),
                    ("top", top),
                    ("bottom", bottom),
                ] {
                    out.attribute(name, &number_text(number));
                }
            }
        }
        out.end_tag(false);
        for stop in &self.stops {
            out.open("stop");
            out.attribute("position", &number_text(stop.position));
            out.end_tag(false);
            out.color("color", &stop.color);
            out.close("stop");
        }
        out.close("gradientFill");
    }
}

impl Border {
    /// The `<border>` element, its five sides always written, or the refusal
    /// of a colour.
    pub(crate) fn compose(&self) -> Result<Composed, Error> {
        let sides = [
            ("left", &self.left),
            ("right", &self.right),
            ("top", &self.top),
            ("bottom", &self.bottom),
            ("diagonal", &self.diagonal),
        ];
        for (_, side) in sides {
            check_color(side.as_ref().and_then(|side| side.color.as_ref()))?;
        }
        let mut out = Composed::new();
        out.open("border");
        if self.diagonal_up {
            out.attribute("diagonalUp", "1");
        }
        if self.diagonal_down {
            out.attribute("diagonalDown", "1");
        }
        out.end_tag(false);
        for (name, side) in sides {
            let Some(side) = side else {
                out.empty(name);
                continue;
            };
            out.open(name);
            out.attribute("style", side.style.keyword());
            match &side.color {
                None => out.end_tag(true),
                Some(color) => {
                    out.end_tag(false);
                    out.color("color", color);
                    out.close(name);
                }
            }
        }
        out.close("border");
        Ok(out)
    }
}

impl Alignment {
    /// The `<alignment>` element, or the refusal of a rotation above 180
    /// that is not 255.
    pub(crate) fn compose(&self) -> Result<Composed, Error> {
        if let Some(rotation) = self.text_rotation
            && rotation > 180
            && rotation != 255
        {
            return Err(FIELD_OUT_OF_RANGE);
        }
        let mut out = Composed::new();
        out.open("alignment");
        if let Some(horizontal) = self.horizontal {
            out.attribute("horizontal", horizontal.keyword());
        }
        if let Some(vertical) = self.vertical {
            out.attribute("vertical", vertical.keyword());
        }
        if self.wrap_text {
            out.attribute("wrapText", "1");
        }
        if self.shrink_to_fit {
            out.attribute("shrinkToFit", "1");
        }
        if let Some(indent) = self.indent {
            out.attribute("indent", &indent.to_string());
        }
        if let Some(rotation) = self.text_rotation {
            out.attribute("textRotation", &rotation.to_string());
        }
        out.end_tag(true);
        Ok(out)
    }
}

impl Protection {
    /// The `<protection>` element.
    pub(crate) fn compose(&self) -> Composed {
        let flag = |on: bool| if on { "1" } else { "0" };
        let mut out = Composed::new();
        out.open("protection");
        if let Some(locked) = self.locked {
            out.attribute("locked", flag(locked));
        }
        if let Some(hidden) = self.hidden {
            out.attribute("hidden", flag(hidden));
        }
        out.end_tag(true);
        out
    }
}
