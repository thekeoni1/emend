//! Cell formats read back as typed values on byte-built styles parts
//! (`specs/0027-read-back.md`, "P1: cell formats and the round trip"): each
//! test builds the smallest package with a given styles part, resolves a
//! cell format with `cell_style`, and compares the value with the one the
//! part's bytes give; the round trips give each value to `add_style`, write
//! the package, and read the added cell format back equal, the appended
//! element byte-identical to the one read where the part writes it in the
//! writer's own form. The Cargo feature `read`.

use super::*;
use crate::styles::CellStyle;

/// The package whose styles part holds `body`.
fn with_styles_part(body: &str) -> Package<Cursor<Vec<u8>>> {
    opened(styled(body, "")).1
}

/// Cell format 0 of a styles part holding one font, one fill, one border
/// (each given) and `xf` as its one cell format.
fn style_of(font: &str, fill: &str, border: &str, xf: &str) -> Result<CellStyle, Error> {
    let body = format!(
        "<fonts count=\"1\">{font}</fonts><fills count=\"1\">{fill}</fills>\
         <borders count=\"1\">{border}</borders><cellXfs count=\"1\">{xf}</cellXfs>"
    );
    with_styles_part(&body).cell_style(0)
}

const FONT: &str = "<font/>";
const FILL: &str = "<fill><patternFill patternType=\"none\"/></fill>";
const BORDER: &str = "<border><left/><right/><top/><bottom/><diagonal/></border>";
const XF: &str = "<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\"/>";

#[track_caller]
fn font(element: &str) -> Font {
    style_of(element, FILL, BORDER, XF).expect("read").font
}

#[track_caller]
fn fill(element: &str) -> Option<Fill> {
    style_of(FONT, element, BORDER, XF).expect("read").fill
}

#[track_caller]
fn border(element: &str) -> Border {
    style_of(FONT, FILL, element, XF).expect("read").border
}

#[track_caller]
fn xf(element: &str) -> CellStyle {
    style_of(FONT, FILL, BORDER, element).expect("read")
}

#[track_caller]
fn refused(font: &str, fill: &str, border: &str, xf: &str) {
    match style_of(font, fill, border, xf) {
        Err(error) => assert_package(error, "a style cannot be read"),
        Ok(style) => panic!("read as {style:?}"),
    }
}

// --- fonts ---

#[test]
fn each_font_field_is_read() {
    assert_eq!(font("<font/>"), Font::default());
    assert_eq!(
        font(
            "<font><b/><i/><strike/><u/><vertAlign val=\"superscript\"/><sz val=\"11.5\"/>\
             <color rgb=\"FF112233\"/><name val=\"Arial\"/><family val=\"2\"/>\
             <scheme val=\"minor\"/></font>"
        ),
        Font {
            name: Some("Arial".into()),
            size: Some(11.5),
            bold: true,
            italic: true,
            strike: true,
            underline: Some(Underline::Single),
            vertical_align: Some(VerticalAlign::Superscript),
            color: Some(Color::Rgb(0xFF11_2233)),
            family: Some(2),
            scheme: Some(Scheme::Minor),
        }
    );
    // A typeface child written `rFont`, as a run writes it, is not a styles
    // part's: the font's name child is `name`.
    refused("<font><rFont val=\"Arial\"/></font>", FILL, BORDER, XF);
}

#[test]
fn each_underline_vertical_alignment_and_scheme_is_read() {
    for (val, expected) in [
        ("<u/>", Some(Underline::Single)),
        ("<u val=\"single\"/>", Some(Underline::Single)),
        ("<u val=\"double\"/>", Some(Underline::Double)),
        (
            "<u val=\"singleAccounting\"/>",
            Some(Underline::SingleAccounting),
        ),
        (
            "<u val=\"doubleAccounting\"/>",
            Some(Underline::DoubleAccounting),
        ),
        ("<u val=\"none\"/>", None),
    ] {
        assert_eq!(font(&format!("<font>{val}</font>")).underline, expected);
    }
    for (val, expected) in [
        ("superscript", VerticalAlign::Superscript),
        ("subscript", VerticalAlign::Subscript),
        ("baseline", VerticalAlign::Baseline),
    ] {
        let element = format!("<font><vertAlign val=\"{val}\"/></font>");
        assert_eq!(font(&element).vertical_align, Some(expected));
    }
    for (val, expected) in [
        ("major", Some(Scheme::Major)),
        ("minor", Some(Scheme::Minor)),
        ("none", None),
    ] {
        let element = format!("<font><scheme val=\"{val}\"/></font>");
        assert_eq!(font(&element).scheme, expected);
    }
}

#[test]
fn each_colour_form_is_read() {
    for (attributes, expected) in [
        ("rgb=\"FFAABBCC\"", Color::Rgb(0xFFAA_BBCC)),
        (
            "theme=\"1\"",
            Color::Theme {
                index: 1,
                tint: None,
            },
        ),
        (
            "theme=\"4\" tint=\"-0.25\"",
            Color::Theme {
                index: 4,
                tint: Some(-0.25),
            },
        ),
        ("indexed=\"64\"", Color::Indexed(64)),
        ("auto=\"1\"", Color::Auto),
    ] {
        let element = format!("<font><color {attributes}/></font>");
        assert_eq!(font(&element).color, Some(expected), "{attributes}");
    }
}

#[test]
fn a_flag_val_of_zero_reads_false_and_the_five_unmodelled_children_are_read_past() {
    assert_eq!(
        font("<font><b val=\"0\"/><i val=\"false\"/><strike val=\"1\"/></font>"),
        Font {
            strike: true,
            ..Font::default()
        }
    );
    assert_eq!(
        font(
            "<font><b/><charset val=\"238\"/><outline/><shadow val=\"1\"/>\
             <condense val=\"0\"/><extend/><sz val=\"10\"/></font>"
        ),
        Font {
            bold: true,
            size: Some(10.0),
            ..Font::default()
        }
    );
}

// --- fills ---

#[test]
fn each_pattern_and_both_colours_are_read() {
    for pattern in [
        Pattern::None,
        Pattern::Solid,
        Pattern::MediumGray,
        Pattern::DarkGray,
        Pattern::LightGray,
        Pattern::DarkHorizontal,
        Pattern::DarkVertical,
        Pattern::DarkDown,
        Pattern::DarkUp,
        Pattern::DarkGrid,
        Pattern::DarkTrellis,
        Pattern::LightHorizontal,
        Pattern::LightVertical,
        Pattern::LightDown,
        Pattern::LightUp,
        Pattern::LightGrid,
        Pattern::LightTrellis,
        Pattern::Gray125,
        Pattern::Gray0625,
    ] {
        let element = format!(
            "<fill><patternFill patternType=\"{}\"/></fill>",
            pattern.keyword()
        );
        assert_eq!(
            fill(&element),
            Some(Fill {
                pattern,
                foreground: None,
                background: None
            })
        );
    }
    assert_eq!(
        fill(
            "<fill><patternFill patternType=\"solid\"><fgColor theme=\"5\" tint=\"0.5\"/>\
             <bgColor indexed=\"64\"/></patternFill></fill>"
        ),
        Some(Fill {
            pattern: Pattern::Solid,
            foreground: Some(Color::Theme {
                index: 5,
                tint: Some(0.5)
            }),
            background: Some(Color::Indexed(64)),
        })
    );
    // No `patternType` reads as `none`.
    assert_eq!(
        fill("<fill><patternFill><bgColor auto=\"1\"/></patternFill></fill>"),
        Some(Fill {
            pattern: Pattern::None,
            foreground: None,
            background: Some(Color::Auto),
        })
    );
}

#[test]
fn a_gradient_fill_and_a_fill_with_no_pattern_read_as_none() {
    assert_eq!(
        fill(
            "<fill><gradientFill degree=\"90\"><stop position=\"0\"><color theme=\"0\"/></stop>\
             <stop position=\"1\"><color theme=\"4\"/></stop></gradientFill></fill>"
        ),
        None
    );
    assert_eq!(fill("<fill/>"), None);
    assert_eq!(fill("<fill></fill>"), None);
}

// --- borders ---

#[test]
fn each_side_and_both_diagonals_are_read() {
    let side = |style, color| Some(Side { style, color });
    assert_eq!(
        border(
            "<border diagonalUp=\"1\" diagonalDown=\"true\"><left style=\"thin\"/>\
             <right style=\"medium\"><color rgb=\"FF000000\"/></right><top style=\"dashed\"/>\
             <bottom style=\"double\"><color indexed=\"8\"/></bottom>\
             <diagonal style=\"hair\"/></border>"
        ),
        Border {
            left: side(BorderStyle::Thin, None),
            right: side(BorderStyle::Medium, Some(Color::Rgb(0xFF00_0000))),
            top: side(BorderStyle::Dashed, None),
            bottom: side(BorderStyle::Double, Some(Color::Indexed(8))),
            diagonal: side(BorderStyle::Hair, None),
            diagonal_up: true,
            diagonal_down: true,
        }
    );
    // A side with no style, or `none`, is `None`, a colour on it with it.
    assert_eq!(
        border(
            "<border diagonalUp=\"0\"><left style=\"none\"/><right><color auto=\"1\"/></right>\
             </border>"
        ),
        Border::default()
    );
    for style in [
        BorderStyle::Thin,
        BorderStyle::Medium,
        BorderStyle::Thick,
        BorderStyle::Dashed,
        BorderStyle::Dotted,
        BorderStyle::Double,
        BorderStyle::Hair,
        BorderStyle::MediumDashed,
        BorderStyle::DashDot,
        BorderStyle::MediumDashDot,
        BorderStyle::DashDotDot,
        BorderStyle::MediumDashDotDot,
        BorderStyle::SlantDashDot,
    ] {
        let element = format!("<border><top style=\"{}\"/></border>", style.keyword());
        assert_eq!(border(&element).top, side(style, None));
    }
}

// --- alignment, protection, number formats ---

#[test]
fn each_alignment_field_is_read_and_three_attributes_read_past() {
    assert_eq!(
        xf(
            "<xf><alignment horizontal=\"centerContinuous\" vertical=\"distributed\" \
            wrapText=\"1\" shrinkToFit=\"true\" indent=\"3\" textRotation=\"255\" \
            justifyLastLine=\"1\" readingOrder=\"2\" relativeIndent=\"-1\"/></xf>"
        )
        .alignment,
        Some(Alignment {
            horizontal: Some(Horizontal::CenterContinuous),
            vertical: Some(Vertical::Distributed),
            wrap_text: true,
            shrink_to_fit: true,
            indent: Some(3),
            text_rotation: Some(255),
        })
    );
    assert_eq!(
        xf("<xf><alignment wrapText=\"0\"/></xf>").alignment,
        Some(Alignment::default())
    );
    assert_eq!(xf("<xf/>").alignment, None);
    for horizontal in [
        Horizontal::General,
        Horizontal::Left,
        Horizontal::Center,
        Horizontal::Right,
        Horizontal::Fill,
        Horizontal::Justify,
        Horizontal::CenterContinuous,
        Horizontal::Distributed,
    ] {
        let element = format!(
            "<xf><alignment horizontal=\"{}\"/></xf>",
            horizontal.keyword()
        );
        assert_eq!(
            xf(&element).alignment.and_then(|a| a.horizontal),
            Some(horizontal)
        );
    }
    for vertical in [
        Vertical::Top,
        Vertical::Center,
        Vertical::Bottom,
        Vertical::Justify,
        Vertical::Distributed,
    ] {
        let element = format!("<xf><alignment vertical=\"{}\"/></xf>", vertical.keyword());
        assert_eq!(
            xf(&element).alignment.and_then(|a| a.vertical),
            Some(vertical)
        );
    }
}

#[test]
fn both_protection_fields_are_read_and_the_first_child_of_each_kind_counts() {
    assert_eq!(
        xf("<xf><protection locked=\"0\" hidden=\"1\"/></xf>").protection,
        Some(Protection {
            locked: Some(false),
            hidden: Some(true),
        })
    );
    assert_eq!(
        xf("<xf><protection/></xf>").protection,
        Some(Protection::default())
    );
    // The first `alignment` and `protection` are the ones the writer
    // replaces, so they are the ones read.
    let style = xf(
        "<xf><alignment indent=\"1\"></alignment><protection locked=\"1\"/>\
                    <alignment indent=\"2\"/><protection locked=\"0\"/></xf>",
    );
    assert_eq!(style.alignment.and_then(|a| a.indent), Some(1));
    assert_eq!(style.protection.and_then(|p| p.locked), Some(true));
}

#[test]
fn a_built_in_and_a_coded_number_format_and_an_xf_with_no_ids() {
    let body = "<numFmts count=\"2\"><numFmt numFmtId=\"164\" formatCode=\"0.0%\"/>\
                <numFmt numFmtId=\"14\" formatCode=\"d/m/yyyy\"/></numFmts>\
                <fonts count=\"2\"><font><b/></font><font><i/></font></fonts>\
                <fills count=\"1\"><fill/></fills><borders count=\"1\"><border/></borders>\
                <cellXfs count=\"4\"><xf numFmtId=\"164\" fontId=\"1\"/><xf numFmtId=\"14\"/>\
                <xf numFmtId=\"3\"/><xf/></cellXfs>";
    let mut package = with_styles_part(body);
    let read = |package: &mut Package<_>, at| package.cell_style(at).expect("read");
    let first = read(&mut package, 0);
    assert_eq!(first.number_format, NumberFormat::Code("0.0%".into()));
    assert!(first.font.italic && !first.font.bold);
    // A producer's spelling of a built-in id is its code.
    assert_eq!(
        read(&mut package, 1).number_format,
        NumberFormat::Code("d/m/yyyy".into())
    );
    assert_eq!(
        read(&mut package, 2).number_format,
        NumberFormat::BuiltIn(3)
    );
    // No ids: built-in 0, and font, fill and border 0.
    let last = read(&mut package, 3);
    assert_eq!(last.number_format, NumberFormat::BuiltIn(0));
    assert!(last.font.bold);
    assert_eq!(last.fill, None);
    assert_eq!(last.border, Border::default());
}

#[test]
fn the_counts_stand_beside_the_typed_read() {
    // `styles/tests.rs`'s count test without the nested font this read
    // refuses: the counts are still the main-namespace children at depth 3.
    // The `dxf`'s font holds a main-namespace child, since a `dxf` is read
    // typed too (`specs/0032-read-breadth-styles.md`, "Differential
    // styles").
    let mut package = with_styles_part(
        "<fonts count=\"9\"><font><b/></font><o:font xmlns:o=\"urn:other\"/><font/></fonts>\
         <fills count=\"0\"><fill><patternFill patternType=\"none\"/></fill></fills>\
         <borders><border><left/></border><border/><border numFmtId=\"x\"/></borders>\
         <cellXfs><xf/></cellXfs>\
         <dxfs><dxf><font><b/></font><fill/><border/></dxf></dxfs>\
         <extLst><ext uri=\"urn:x\"><fonts><font><o:b xmlns:o=\"urn:other\"/></font></fonts>\
         </ext></extLst>",
    );
    let styles = package.styles().expect("read");
    assert_eq!(
        (styles.fonts(), styles.fills(), styles.borders()),
        (Some(2), Some(1), Some(3))
    );
    assert!(package.cell_style(0).expect("read").font.bold);
}

// --- refusals ---

#[test]
fn an_id_past_its_list_is_refused() {
    refused(FONT, FILL, BORDER, "<xf fontId=\"1\"/>");
    refused(FONT, FILL, BORDER, "<xf fillId=\"1\"/>");
    refused(FONT, FILL, BORDER, "<xf borderId=\"1\"/>");
    // A cell format past `cellXfs`.
    let mut package = with_styles_part(&format!(
        "<fonts>{FONT}</fonts><fills>{FILL}</fills><borders>{BORDER}</borders>\
         <cellXfs>{XF}</cellXfs>"
    ));
    assert!(package.cell_style(0).is_ok());
    assert_package(
        package.cell_style(1).expect_err("past cellXfs"),
        "a style cannot be read",
    );
    // No `<fonts>` at all: font 0 is past an empty list.
    let mut package = with_styles_part("<cellXfs><xf/></cellXfs>");
    assert_package(
        package.cell_style(0).expect_err("no fonts"),
        "a style cannot be read",
    );
}

#[test]
fn a_colour_with_no_form_is_refused() {
    refused("<font><color/></font>", FILL, BORDER, XF);
    refused("<font><color rgb=\"FF00\"/></font>", FILL, BORDER, XF);
    refused(
        FONT,
        "<fill><patternFill><fgColor tint=\"0.5\"/></patternFill></fill>",
        BORDER,
        XF,
    );
    refused(
        FONT,
        FILL,
        "<border><left style=\"thin\"><color theme=\"x\"/></left></border>",
        XF,
    );
}

#[test]
fn a_value_that_does_not_parse_is_refused() {
    refused("<font><sz val=\"x\"/></font>", FILL, BORDER, XF);
    refused("<font><b val=\"2\"/></font>", FILL, BORDER, XF);
    refused("<font><u val=\"wavy\"/></font>", FILL, BORDER, XF);
    refused(
        "<font><color theme=\"1\" tint=\"2\"/></font>",
        FILL,
        BORDER,
        XF,
    );
    refused(
        FONT,
        "<fill><patternFill patternType=\"stripes\"/></fill>",
        BORDER,
        XF,
    );
    refused(FONT, FILL, "<border><left style=\"wavy\"/></border>", XF);
    refused(FONT, FILL, "<border diagonalUp=\"yes\"/>", XF);
    refused(FONT, FILL, BORDER, "<xf><alignment wrapText=\"2\"/></xf>");
    refused(
        FONT,
        FILL,
        BORDER,
        "<xf><alignment horizontal=\"middle\"/></xf>",
    );
    refused(FONT, FILL, BORDER, "<xf><alignment indent=\"-1\"/></xf>");
    refused(
        FONT,
        FILL,
        BORDER,
        "<xf><alignment textRotation=\"181\"/></xf>",
    );
    refused(
        FONT,
        FILL,
        BORDER,
        "<xf><protection locked=\"maybe\"/></xf>",
    );
}

#[test]
fn a_child_outside_the_namespace_or_unknown_to_the_type_is_refused() {
    let other = "xmlns:o=\"urn:other\"";
    refused(&format!("<font><o:b {other}/></font>"), FILL, BORDER, XF);
    refused("<font><glow/></font>", FILL, BORDER, XF);
    refused(
        FONT,
        &format!("<fill><o:patternFill {other}/></fill>"),
        BORDER,
        XF,
    );
    refused(
        FONT,
        "<fill><patternFill/><patternFill/></fill>",
        BORDER,
        XF,
    );
    refused(
        FONT,
        "<fill><patternFill><stop/></patternFill></fill>",
        BORDER,
        XF,
    );
    refused(
        FONT,
        FILL,
        &format!("<border><o:left {other} style=\"thin\"/></border>"),
        XF,
    );
    // An inside-of-range side, which no writer writes.
    refused(
        FONT,
        FILL,
        "<border><vertical style=\"thin\"/></border>",
        XF,
    );
    refused(FONT, FILL, "<border><left/><left/></border>", XF);
    refused(
        FONT,
        FILL,
        "<border><left style=\"thin\"><color auto=\"1\"><b/></color></left></border>",
        XF,
    );
}

#[test]
fn a_part_with_473_fonts_is_read() {
    // The corpus's largest font count (`specs/0027-read-back.md`, "Facts
    // planning checked"), byte-built.
    let fonts: String = (0..473)
        .map(|at| {
            format!(
                "<font><sz val=\"{}\"/><name val=\"F{at}\"/></font>",
                8 + at % 20
            )
        })
        .collect();
    let xfs: String = (0..473)
        .map(|at| format!("<xf fontId=\"{at}\"/>"))
        .collect();
    let mut package = with_styles_part(&format!(
        "<fonts count=\"473\">{fonts}</fonts><fills><fill/></fills><borders><border/></borders>\
         <cellXfs count=\"473\">{xfs}</cellXfs>"
    ));
    assert_eq!(package.styles().expect("read").fonts(), Some(473));
    for at in [0u32, 1, 236, 472] {
        let font = package.cell_style(at).expect("read").font;
        assert_eq!(font.name, Some(format!("F{at}")));
        assert_eq!(font.size, Some(f64::from(8 + at % 20)));
    }
}

// --- round trips ---

/// The cell format 0 of the package whose styles part holds `body`, given
/// to `add_style` with no base, the package written; the added cell format
/// read back from the output, and the output's styles part as text.
#[track_caller]
fn round_trip(body: &str) -> (CellStyle, CellStyle, String) {
    let (_, mut package) = opened(styled(body, ""));
    let read = package.cell_style(0).expect("read");
    let added = package
        .add_style(
            None,
            &Style {
                number_format: Some(read.number_format.clone()),
                font: Some(read.font.clone()),
                fill: read.fill,
                border: Some(read.border),
                alignment: read.alignment,
                protection: read.protection,
            },
        )
        .expect("added");
    let output = written(package);
    let styles = String::from_utf8(stored_data(&output, STYLES_PART)).expect("UTF-8");
    let mut reopened = Package::open(Cursor::new(output)).expect("the output opens");
    let back = reopened.cell_style(added).expect("read back");
    (read, back, styles)
}

/// A styles part whose cell format 0 names the one font, fill and border
/// given, with `xf`'s attributes and children.
fn one_of_each(font: &str, fill: &str, border: &str, xf: &str) -> String {
    format!(
        "<numFmts count=\"1\"><numFmt numFmtId=\"164\" formatCode=\"#,##0.00&quot;x&quot;\"/>\
         </numFmts><fonts count=\"1\">{font}</fonts><fills count=\"1\">{fill}</fills>\
         <borders count=\"1\">{border}</borders><cellXfs count=\"1\">{xf}</cellXfs>"
    )
}

#[test]
fn each_value_read_round_trips_through_add_style_and_its_element_is_the_one_read() {
    // Each element in the writer's own form, so the appended one is
    // byte-identical to it: the composed element equals the read one child
    // by child.
    let fonts = [
        "<font/>",
        "<font><b/><i/><strike/><u val=\"double\"/><vertAlign val=\"superscript\"/>\
         <sz val=\"11.5\"/><color theme=\"1\" tint=\"-0.5\"/><name val=\"A &amp; B\"/>\
         <family val=\"2\"/><scheme val=\"minor\"/></font>",
        "<font><u/><sz val=\"9\"/><color rgb=\"FF00FF00\"/><name val=\"Calibri\"/></font>",
        "<font><vertAlign val=\"baseline\"/><color indexed=\"10\"/><scheme val=\"major\"/></font>",
        "<font><u val=\"doubleAccounting\"/><color auto=\"1\"/></font>",
    ];
    let fills = [
        "<fill><patternFill patternType=\"none\"/></fill>",
        "<fill><patternFill patternType=\"gray125\"/></fill>",
        "<fill><patternFill patternType=\"solid\"><fgColor rgb=\"FFFFFF00\"/>\
         <bgColor indexed=\"64\"/></patternFill></fill>",
        "<fill><patternFill patternType=\"darkTrellis\"><bgColor theme=\"3\" tint=\"0.25\"/>\
         </patternFill></fill>",
    ];
    let borders = [
        "<border><left/><right/><top/><bottom/><diagonal/></border>",
        "<border diagonalUp=\"1\" diagonalDown=\"1\"><left style=\"thin\"/>\
         <right style=\"mediumDashDot\"><color rgb=\"FF123456\"/></right><top/>\
         <bottom style=\"double\"><color auto=\"1\"/></bottom><diagonal style=\"hair\">\
         <color theme=\"2\"/></diagonal></border>",
    ];
    let xfs = [
        "<xf numFmtId=\"164\" fontId=\"0\" fillId=\"0\" borderId=\"0\"/>",
        "<xf numFmtId=\"10\" fontId=\"0\" fillId=\"0\" borderId=\"0\">\
         <alignment horizontal=\"right\" vertical=\"top\" wrapText=\"1\" shrinkToFit=\"1\" \
         indent=\"2\" textRotation=\"90\"/><protection locked=\"0\" hidden=\"1\"/></xf>",
        "<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\">\
         <protection locked=\"1\"/></xf>",
    ];
    let mut cases = 0;
    for font in fonts {
        for fill in fills {
            for border in borders {
                for xf in xfs {
                    let (read, back, styles) = round_trip(&one_of_each(font, fill, border, xf));
                    assert_eq!(back, read, "{font} {fill} {border} {xf}");
                    for element in [font, fill, border] {
                        assert_eq!(styles.matches(element).count(), 2, "{element}");
                    }
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, fonts.len() * fills.len() * borders.len() * xfs.len());
}

#[test]
fn a_value_read_from_another_form_round_trips_equal() {
    // Not the writer's form: attributes in another order, single quotes,
    // flags spelt `true`, a `none` side with a colour, a `charset` read past.
    // The value read back equals the value read, which is the property the
    // corpus sample proves.
    let (read, back, _) = round_trip(&one_of_each(
        "<font><charset val='0'/><name val='Arial'/><sz val='10.0'/><b val='true'/>\
         <color tint='0.1' theme='4'/></font>",
        "<fill><patternFill><fgColor indexed='9'/></patternFill></fill>",
        "<border diagonalDown='true'><left style='none'><color auto='1'/></left>\
         <diagonal style='thick'/></border>",
        "<xf borderId='0' fillId='0' fontId='0' numFmtId='49' applyFont='1'>\
         <alignment readingOrder='1' wrapText='true'/></xf>",
    ));
    assert_eq!(back, read);
    assert_eq!(read.font.size, Some(10.0));
    assert_eq!(read.number_format, NumberFormat::BuiltIn(49));
}

#[test]
fn an_attribute_the_types_do_not_name_is_read_past() {
    // `specs/0027-read-back.md`, "Planning verification and rulings on P1",
    // ruling 2 (R3): on every element the module reads, as the three named
    // alignment attributes are.
    let style = style_of(
        "<font extra=\"1\"><b val=\"1\" extra=\"x\"/><color theme=\"1\" extra=\"y\"/></font>",
        "<fill extra=\"1\"><patternFill patternType=\"solid\" extra=\"2\">\
         <fgColor indexed=\"8\" extra=\"3\"/></patternFill></fill>",
        "<border outline=\"0\" extra=\"1\"><left style=\"thin\" extra=\"2\">\
         <color auto=\"1\" extra=\"3\"/></left></border>",
        "<xf extra=\"1\"><alignment vertical=\"top\" extra=\"2\"/>\
         <protection hidden=\"1\" extra=\"3\"/></xf>",
    )
    .expect("read");
    assert!(style.font.bold);
    assert_eq!(
        style.font.color,
        Some(Color::Theme {
            index: 1,
            tint: None
        })
    );
    assert_eq!(
        style.fill,
        Some(Fill {
            pattern: Pattern::Solid,
            foreground: Some(Color::Indexed(8)),
            background: None,
        })
    );
    assert_eq!(
        style.border.left,
        Some(Side {
            style: BorderStyle::Thin,
            color: Some(Color::Auto),
        })
    );
    assert_eq!(
        style.alignment,
        Some(Alignment {
            vertical: Some(Vertical::Top),
            ..Alignment::default()
        })
    );
    assert_eq!(
        style.protection,
        Some(Protection {
            locked: None,
            hidden: Some(true),
        })
    );
}

#[test]
fn every_enum_variant_round_trips_through_add_style() {
    // `specs/0027-read-back.md`, "Planning verification and rulings on P1",
    // ruling 3 (R4): each underline, vertical alignment, pattern, border
    // style, horizontal and vertical alignment, each element in the
    // writer's own form so the appended one is byte-identical to it.
    let none_fill = "<fill><patternFill patternType=\"none\"/></fill>";
    let empty_border = "<border><left/><right/><top/><bottom/><diagonal/></border>";
    let plain_xf = "<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\"/>";
    let mut fonts = vec!["<font><u/></font>".to_string()];
    for underline in [
        Underline::Double,
        Underline::SingleAccounting,
        Underline::DoubleAccounting,
    ] {
        fonts.push(format!("<font><u val=\"{}\"/></font>", underline.keyword()));
    }
    for align in [
        VerticalAlign::Superscript,
        VerticalAlign::Subscript,
        VerticalAlign::Baseline,
    ] {
        fonts.push(format!(
            "<font><vertAlign val=\"{}\"/></font>",
            align.keyword()
        ));
    }
    let mut cases: Vec<(String, String, String, String)> = fonts
        .into_iter()
        .map(|font| (font, none_fill.into(), empty_border.into(), plain_xf.into()))
        .collect();
    for pattern in [
        Pattern::None,
        Pattern::Solid,
        Pattern::MediumGray,
        Pattern::DarkGray,
        Pattern::LightGray,
        Pattern::DarkHorizontal,
        Pattern::DarkVertical,
        Pattern::DarkDown,
        Pattern::DarkUp,
        Pattern::DarkGrid,
        Pattern::DarkTrellis,
        Pattern::LightHorizontal,
        Pattern::LightVertical,
        Pattern::LightDown,
        Pattern::LightUp,
        Pattern::LightGrid,
        Pattern::LightTrellis,
        Pattern::Gray125,
        Pattern::Gray0625,
    ] {
        let fill = format!(
            "<fill><patternFill patternType=\"{}\"/></fill>",
            pattern.keyword()
        );
        cases.push(("<font/>".into(), fill, empty_border.into(), plain_xf.into()));
    }
    for style in [
        BorderStyle::Thin,
        BorderStyle::Medium,
        BorderStyle::Thick,
        BorderStyle::Dashed,
        BorderStyle::Dotted,
        BorderStyle::Double,
        BorderStyle::Hair,
        BorderStyle::MediumDashed,
        BorderStyle::DashDot,
        BorderStyle::MediumDashDot,
        BorderStyle::DashDotDot,
        BorderStyle::MediumDashDotDot,
        BorderStyle::SlantDashDot,
    ] {
        let border = format!(
            "<border><left/><right/><top style=\"{}\"/><bottom/><diagonal/></border>",
            style.keyword()
        );
        cases.push(("<font/>".into(), none_fill.into(), border, plain_xf.into()));
    }
    let mut alignments = Vec::new();
    for horizontal in [
        Horizontal::General,
        Horizontal::Left,
        Horizontal::Center,
        Horizontal::Right,
        Horizontal::Fill,
        Horizontal::Justify,
        Horizontal::CenterContinuous,
        Horizontal::Distributed,
    ] {
        alignments.push(format!(
            "<alignment horizontal=\"{}\"/>",
            horizontal.keyword()
        ));
    }
    for vertical in [
        Vertical::Top,
        Vertical::Center,
        Vertical::Bottom,
        Vertical::Justify,
        Vertical::Distributed,
    ] {
        alignments.push(format!("<alignment vertical=\"{}\"/>", vertical.keyword()));
    }
    for alignment in &alignments {
        let xf =
            format!("<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\">{alignment}</xf>");
        cases.push(("<font/>".into(), none_fill.into(), empty_border.into(), xf));
    }
    // Every variant: 4 underlines (single as `<u/>`), 3 vertical
    // alignments, 19 patterns, 13 border styles, 8 horizontal and 5
    // vertical alignments.
    assert_eq!(cases.len(), 4 + 3 + 19 + 13 + 8 + 5);
    let mut alignments_written = 0;
    for (font, fill, border, xf) in &cases {
        let (read, back, styles) = round_trip(&one_of_each(font, fill, border, xf));
        assert_eq!(back, read, "{font} {fill} {border} {xf}");
        for element in [font, fill, border] {
            assert_eq!(styles.matches(element.as_str()).count(), 2, "{element}");
        }
        // The alignment child, too, is written as it was read.
        if let Some(alignment) = alignments.iter().find(|a| xf.contains(a.as_str())) {
            assert_eq!(styles.matches(alignment.as_str()).count(), 2, "{alignment}");
            alignments_written += 1;
        }
    }
    assert_eq!(alignments_written, alignments.len());
}
