//! Named styles, differential styles and the default font read on byte-built
//! styles parts (`specs/0032-read-breadth-styles.md`, "P1: named styles and
//! differential styles", and "Planning rulings during P1"): each test builds
//! the smallest package with a given styles part and reads it through
//! `styles`, `named_style`, `differential_styles` or `default_font`; the
//! round trip gives a differential style read to `add_differential_style`,
//! writes the package and compares the appended element with the one read.
//! The Cargo feature `read`.

use super::*;
use crate::styles::{
    Alignment, BorderStyle, CellStyle, Color, Fill, Font, Horizontal, NamedStyle, Pattern,
    Protection, Side, Vertical,
};

/// The package whose styles part holds `body`.
fn with_styles_part(body: &str) -> Package<Cursor<Vec<u8>>> {
    opened(styled(body, "")).1
}

/// One font, one fill, one border and one cell format, each in the
/// writer's own form.
const BASE: &str = "<fonts count=\"2\"><font><sz val=\"11\"/><name val=\"Calibri\"/></font>\
                    <font><b/><sz val=\"9\"/></font></fonts>\
                    <fills count=\"1\"><fill><patternFill patternType=\"none\"/></fill></fills>\
                    <borders count=\"1\"><border><left/><right/><top/><bottom/><diagonal/>\
                    </border></borders>";

/// `BASE` with `<cellStyleXfs>` holding `xfs`, `<cellXfs>` one plain cell
/// format, then `rest`.
fn part(xfs: &str, rest: &str) -> String {
    format!(
        "{BASE}<cellStyleXfs count=\"1\">{xfs}</cellStyleXfs>\
         <cellXfs count=\"1\"><xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\" \
         xfId=\"0\"/></cellXfs>{rest}"
    )
}

/// The differential styles of the package whose styles part is `BASE`'s
/// with `<dxfs>` holding `dxfs`.
fn dxfs_of(dxfs: &str) -> Result<Vec<Style>, Error> {
    let body = part("<xf/>", &format!("<dxfs count=\"1\">{dxfs}</dxfs>"));
    with_styles_part(&body)
        .differential_styles()
        .map(<[Style]>::to_vec)
}

/// The one differential style `dxf` reads as.
#[track_caller]
fn dxf(dxf: &str) -> Style {
    let read = dxfs_of(dxf).expect("read");
    assert_eq!(read.len(), 1, "one dxf");
    read[0].clone()
}

#[track_caller]
fn refused(result: Result<impl std::fmt::Debug, Error>) {
    match result {
        Err(error) => assert_package(error, "a style cannot be read"),
        Ok(value) => panic!("read as {value:?}"),
    }
}

// --- named styles ---

#[test]
fn a_cell_style_with_every_attribute_and_with_the_two_required_alone_is_read() {
    let body = part(
        "<xf/><xf/>",
        "<cellStyles count=\"2\"><cellStyle name=\"A &amp; B\" xfId=\"1\" builtinId=\"3\" \
         customBuiltin=\"1\" hidden=\"true\" iLevel=\"2\"/>\
         <cellStyle name=\"Plain\" xfId=\"0\"><extLst/></cellStyle></cellStyles>",
    );
    let mut package = with_styles_part(&body);
    let styles = package.styles().expect("read");
    assert_eq!(styles.cell_style_xfs.len(), 2);
    assert_eq!(
        styles.cell_styles,
        vec![
            NamedStyle {
                name: "A & B".into(),
                xf_id: 1,
                builtin_id: Some(3),
                custom_builtin: true,
                hidden: true,
                outline_level: Some(2),
            },
            NamedStyle {
                name: "Plain".into(),
                xf_id: 0,
                builtin_id: None,
                custom_builtin: false,
                hidden: false,
                outline_level: None,
            },
        ]
    );
}

#[test]
fn a_named_style_resolves_as_a_cell_format_with_equal_ids_does() {
    // The same ids, alignment and protection in both lists: the one
    // resolver gives one value.
    let xf = "<xf numFmtId=\"164\" fontId=\"1\" fillId=\"0\" borderId=\"0\">\
              <alignment horizontal=\"center\" wrapText=\"1\"/><protection locked=\"0\"/></xf>";
    let body = format!(
        "<numFmts count=\"1\"><numFmt numFmtId=\"164\" formatCode=\"0.0\"/></numFmts>{BASE}\
         <cellStyleXfs count=\"2\"><xf/>{xf}</cellStyleXfs><cellXfs count=\"2\"><xf/>{xf}\
         </cellXfs>"
    );
    let mut package = with_styles_part(&body);
    let named = package.named_style(1).expect("named");
    assert_eq!(named, package.cell_style(1).expect("cell"));
    assert_eq!(
        named,
        CellStyle {
            number_format: NumberFormat::Code("0.0".into()),
            font: Font {
                bold: true,
                size: Some(9.0),
                ..Font::default()
            },
            fill: Some(Fill {
                pattern: Pattern::None,
                foreground: None,
                background: None,
            }),
            border: Border::default(),
            alignment: Some(Alignment {
                horizontal: Some(Horizontal::Center),
                wrap_text: true,
                ..Alignment::default()
            }),
            protection: Some(Protection {
                locked: Some(false),
                hidden: None,
            }),
        }
    );
}

#[test]
fn a_named_style_reads_cell_style_xfs_not_cell_xfs() {
    // The two lists differ at index 1: the named style is the
    // `cellStyleXfs` format's, the cell format the `cellXfs` one's.
    let body = format!(
        "{BASE}<cellStyleXfs count=\"2\"><xf/><xf fontId=\"1\"/></cellStyleXfs>\
         <cellXfs count=\"2\"><xf/><xf fontId=\"0\"/></cellXfs>"
    );
    let mut package = with_styles_part(&body);
    assert!(package.named_style(1).expect("named").font.bold);
    assert!(!package.cell_style(1).expect("cell").font.bold);
}

#[test]
fn a_cell_style_without_name_or_xf_id_or_past_the_list_and_an_index_past_it_are_refused() {
    for cell_style in [
        "<cellStyle xfId=\"0\"/>",
        "<cellStyle name=\"Normal\"/>",
        "<cellStyle name=\"Normal\" xfId=\"1\"/>",
        "<cellStyle name=\"Normal\" xfId=\"x\"/>",
    ] {
        let body = part("<xf/>", &format!("<cellStyles>{cell_style}</cellStyles>"));
        refused(with_styles_part(&body).styles().map(|_| ()));
    }
    // A part with `<cellStyles>` and no `<cellStyleXfs>`.
    let body = format!(
        "{BASE}<cellXfs count=\"1\"><xf/></cellXfs><cellStyles>\
         <cellStyle name=\"Normal\" xfId=\"0\"/></cellStyles>"
    );
    refused(with_styles_part(&body).styles().map(|_| ()));
    // An index past `<cellStyleXfs>`.
    let mut package = with_styles_part(&part("<xf/>", ""));
    assert!(package.named_style(0).is_ok());
    refused(package.named_style(1));
    refused(package.named_style(u32::MAX));
}

#[test]
fn a_second_cell_style_xfs_cell_styles_or_dxfs_is_refused() {
    // P1 ruling 3: one of each, or the indexes are ambiguous.
    for second in [
        "<cellStyleXfs count=\"1\"><xf/></cellStyleXfs>",
        "<cellStyles count=\"0\"/><cellStyles count=\"0\"/>",
        "<dxfs count=\"0\"/><dxfs count=\"0\"/>",
    ] {
        refused(
            with_styles_part(&part("<xf/>", second))
                .styles()
                .map(|_| ()),
        );
    }
    // One of each is read; a foreign container of the name is not one.
    let one = "<cellStyles count=\"0\"/><dxfs count=\"0\"/>\
               <o:dxfs xmlns:o=\"urn:other\"/><o:cellStyles xmlns:o=\"urn:other\"/>";
    assert!(with_styles_part(&part("<xf/>", one)).styles().is_ok());
}

// --- differential styles ---

#[test]
fn each_dxf_child_kind_is_read_alone_and_all_six_together() {
    let font = "<font><b/><color rgb=\"FF9C0006\"/></font>";
    let num_fmt = "<numFmt numFmtId=\"170\" formatCode=\"0.00&quot;x&quot;\"/>";
    let fill = "<fill><patternFill patternType=\"gray125\"><fgColor theme=\"4\" tint=\"0.5\"/>\
                <bgColor indexed=\"64\"/></patternFill></fill>";
    let alignment = "<alignment horizontal=\"left\" vertical=\"top\" indent=\"2\" \
                     justifyLastLine=\"1\"/>";
    let protection = "<protection locked=\"1\" hidden=\"0\"/>";
    let border = "<border diagonalUp=\"1\"><left style=\"thin\"><color auto=\"1\"/></left>\
                  <bottom style=\"double\"/></border>";
    let font_value = Font {
        bold: true,
        color: Some(Color::Rgb(0xFF9C_0006)),
        ..Font::default()
    };
    let num_fmt_value = NumberFormat::Code("0.00\"x\"".into());
    let fill_value = Fill {
        pattern: Pattern::Gray125,
        foreground: Some(Color::Theme {
            index: 4,
            tint: Some(0.5),
        }),
        background: Some(Color::Indexed(64)),
    };
    let alignment_value = Alignment {
        horizontal: Some(Horizontal::Left),
        vertical: Some(Vertical::Top),
        indent: Some(2),
        ..Alignment::default()
    };
    let protection_value = Protection {
        locked: Some(true),
        hidden: Some(false),
    };
    let border_value = Border {
        left: Some(Side {
            style: BorderStyle::Thin,
            color: Some(Color::Auto),
        }),
        bottom: Some(Side {
            style: BorderStyle::Double,
            color: None,
        }),
        diagonal_up: true,
        ..Border::default()
    };
    assert_eq!(dxf("<dxf/>"), Style::default());
    assert_eq!(dxf("<dxf></dxf>"), Style::default());
    let all = Style {
        number_format: Some(num_fmt_value),
        font: Some(font_value),
        fill: Some(fill_value),
        border: Some(border_value),
        alignment: Some(alignment_value),
        protection: Some(protection_value),
    };
    assert_eq!(
        dxf(&format!("<dxf>{font}</dxf>")),
        Style {
            font: all.font.clone(),
            ..Style::default()
        }
    );
    assert_eq!(
        dxf(&format!("<dxf>{num_fmt}</dxf>")),
        Style {
            number_format: all.number_format.clone(),
            ..Style::default()
        }
    );
    assert_eq!(
        dxf(&format!("<dxf>{fill}</dxf>")),
        Style {
            fill: all.fill,
            ..Style::default()
        }
    );
    assert_eq!(
        dxf(&format!("<dxf>{alignment}</dxf>")),
        Style {
            alignment: all.alignment,
            ..Style::default()
        }
    );
    assert_eq!(
        dxf(&format!("<dxf>{protection}</dxf>")),
        Style {
            protection: all.protection,
            ..Style::default()
        }
    );
    assert_eq!(
        dxf(&format!("<dxf>{border}</dxf>")),
        Style {
            border: all.border,
            ..Style::default()
        }
    );
    // The schema's order, with whitespace between the children.
    assert_eq!(
        dxf(&format!(
            "<dxf>\n {font}\n {num_fmt}\n {fill}\n {alignment}\n {protection}\n {border}\n</dxf>"
        )),
        all
    );
    // Several, in part order.
    let read = dxfs_of(&format!("<dxf>{font}</dxf><dxf/><dxf>{border}</dxf>")).expect("read");
    assert_eq!(read.len(), 3);
    assert_eq!(read[1], Style::default());
    assert_eq!(read[2].border, all.border);
}

#[test]
fn a_dxf_pattern_fill_with_no_pattern_type_reads_solid_beside_a_cell_format_s_none() {
    // Decision 2 A: Excel renders it solid from `bgColor`.
    let body = "<fonts count=\"1\"><font/></fonts><fills count=\"1\"><fill><patternFill>\
         <bgColor rgb=\"FFFFC7CE\"/></patternFill></fill></fills><borders count=\"1\">\
         <border/></borders><cellXfs count=\"1\"><xf/></cellXfs><dxfs count=\"2\">\
         <dxf><fill><patternFill><bgColor rgb=\"FFFFC7CE\"/></patternFill></fill></dxf>\
         <dxf><fill><patternFill patternType=\"none\"/></fill></dxf></dxfs>";
    let mut package = with_styles_part(body);
    let background = Some(Color::Rgb(0xFFFF_C7CE));
    assert_eq!(
        package.cell_style(0).expect("read").fill,
        Some(Fill {
            pattern: Pattern::None,
            foreground: None,
            background,
        })
    );
    let read = package.differential_styles().expect("read");
    assert_eq!(
        read[0].fill,
        Some(Fill {
            pattern: Pattern::Solid,
            foreground: None,
            background,
        })
    );
    // A `patternType` given is read as given.
    assert_eq!(read[1].fill.map(|fill| fill.pattern), Some(Pattern::None));
    // An empty `patternFill` is solid too.
    assert_eq!(
        dxf("<dxf><fill><patternFill/></fill></dxf>").fill,
        Some(Fill {
            pattern: Pattern::Solid,
            foreground: None,
            background: None,
        })
    );
}

#[test]
fn a_gradient_dxf_fill_and_an_empty_fill_read_as_none() {
    let gradient = "<dxf><font><i/></font><fill><gradientFill degree=\"90\">\
                    <stop position=\"0\"><color theme=\"0\"/></stop>\
                    <stop position=\"1\"><color theme=\"4\"/></stop></gradientFill></fill></dxf>";
    let read = dxf(gradient);
    assert_eq!(read.fill, None);
    assert!(read.font.is_some_and(|font| font.italic));
    assert_eq!(dxf("<dxf><fill/></dxf>").fill, None);
}

#[test]
fn a_dxf_border_reads_its_inside_sides_past_and_a_cell_border_still_refuses_them() {
    // P1 ruling 1: in a `dxf`, `vertical` and `horizontal` are read past
    // with their subtrees, and `outline` with the other unnamed attributes.
    let read = dxf(
        "<dxf><border diagonalUp=\"0\" diagonalDown=\"0\" outline=\"0\">\
         <left style=\"thin\"><color rgb=\"FF000000\"/></left><right/><top/><bottom/>\
         <vertical style=\"thin\"><color rgb=\"FF000000\"/></vertical>\
         <horizontal style=\"dashed\"><color theme=\"1\"/></horizontal></border></dxf>",
    );
    assert_eq!(
        read.border,
        Some(Border {
            left: Some(Side {
                style: BorderStyle::Thin,
                color: Some(Color::Rgb(0xFF00_0000)),
            }),
            ..Border::default()
        })
    );
    // Two of one inside side are refused as two of any side are.
    refused(dxfs_of(
        "<dxf><border><vertical/><vertical/></border></dxf>",
    ));
    // In `<borders>`, either one is refused, as before this part.
    for side in ["<vertical/>", "<horizontal style=\"thin\"/>"] {
        let body = format!(
            "<fonts count=\"1\"><font/></fonts><fills count=\"1\"><fill/></fills>\
             <borders count=\"1\"><border><left/>{side}</border></borders>\
             <cellXfs count=\"1\"><xf/></cellXfs>"
        );
        refused(with_styles_part(&body).styles().map(|_| ()));
    }
}

#[test]
fn a_dxf_child_outside_the_namespace_unknown_to_the_type_or_twice_is_refused() {
    for dxf in [
        // P1 ruling 5: an `extLst` is a child the type has no field for.
        "<dxf><font/><extLst><ext uri=\"urn:x\"/></extLst></dxf>",
        "<dxf><o:font xmlns:o=\"urn:other\"/></dxf>",
        "<dxf><font><o:b xmlns:o=\"urn:other\"/></font></dxf>",
        "<dxf><cellStyle/></dxf>",
        "<dxf><font/><font/></dxf>",
        "<dxf><fill/><fill/></dxf>",
        "<dxf><border/><border/></dxf>",
        "<dxf><numFmt numFmtId=\"1\" formatCode=\"0\"/><numFmt numFmtId=\"2\" formatCode=\"0\"/></dxf>",
        "<dxf><alignment/><alignment/></dxf>",
        "<dxf><protection/><protection/></dxf>",
        // P1 ruling 5: a `numFmt` with no `formatCode`.
        "<dxf><numFmt numFmtId=\"170\"/></dxf>",
        // Values that do not parse.
        "<dxf><fill><patternFill patternType=\"stripes\"/></fill></dxf>",
        "<dxf><alignment horizontal=\"middle\"/></dxf>",
        "<dxf><protection locked=\"yes\"/></dxf>",
        "<dxf><border><left style=\"wavy\"/></border></dxf>",
        "<dxf><font><color/></font></dxf>",
    ] {
        refused(dxfs_of(dxf));
    }
}

#[test]
fn an_x14_dxfs_in_ext_lst_is_not_counted() {
    let body = part(
        "<xf/>",
        "<dxfs count=\"1\"><dxf><font><b/></font></dxf></dxfs><extLst>\
         <ext uri=\"{46F421CA-312F-682f-3DD2-61675219B42D}\" \
         xmlns:x14=\"http://schemas.microsoft.com/office/spreadsheetml/2009/9/main\">\
         <x14:dxfs count=\"2\"><dxf><fill><gradientFill/></fill></dxf><dxf><o:x \
         xmlns:o=\"urn:other\"/></dxf></x14:dxfs></ext></extLst>",
    );
    let mut package = with_styles_part(&body);
    let read = package.differential_styles().expect("read");
    assert_eq!(read.len(), 1);
    assert!(read[0].font.as_ref().is_some_and(|font| font.bold));
}

#[test]
fn a_part_with_440_dxfs_is_read() {
    // The corpus's largest `<dxfs>` holds 440 `dxf`.
    let dxfs: String = (0..440)
        .map(|at| format!("<dxf><font><sz val=\"{}\"/></font></dxf>", at % 40 + 1))
        .collect();
    let read = dxfs_of(&dxfs).expect("read");
    assert_eq!(read.len(), 440);
    assert_eq!(
        read[439].font.as_ref().and_then(|font| font.size),
        Some(f64::from(439 % 40 + 1))
    );
}

// --- the default font ---

#[test]
fn the_default_font_is_the_normal_style_s_then_font_zero() {
    let normal = Font {
        bold: true,
        size: Some(9.0),
        ..Font::default()
    };
    let zero = Font {
        size: Some(11.0),
        name: Some("Calibri".into()),
        ..Font::default()
    };
    // Through the `Normal` style's format.
    let mut package = with_styles_part(&part("<xf fontId=\"1\"/>", ""));
    assert_eq!(package.default_font().expect("read"), normal);
    // Font 0 when that format has no `fontId`.
    let mut package = with_styles_part(&part("<xf numFmtId=\"0\"/>", ""));
    assert_eq!(package.default_font().expect("read"), zero);
    // Font 0 with an empty `<cellStyleXfs>` (P1 ruling 5) and with none.
    let mut package = with_styles_part(&part("", ""));
    assert_eq!(package.default_font().expect("read"), zero);
    let body = format!("{BASE}<cellXfs count=\"1\"><xf/></cellXfs>");
    assert_eq!(with_styles_part(&body).default_font().expect("read"), zero);
}

#[test]
fn the_default_font_is_refused_with_no_fonts_or_a_font_id_past_them() {
    let body = "<cellStyleXfs count=\"1\"><xf/></cellStyleXfs><cellXfs count=\"1\"><xf/></cellXfs>";
    refused(with_styles_part(body).default_font());
    refused(with_styles_part(&part("<xf fontId=\"2\"/>", "")).default_font());
}

// --- the round trip ---

/// A font, fill and border `dxf` in the writer's own form, its
/// `patternFill` with no `patternType`.
#[cfg(feature = "rules")]
const READ_DXF: &str = "<dxf><font><b/><color rgb=\"FF9C0006\"/></font><fill><patternFill>\
                        <bgColor rgb=\"FFFFC7CE\"/></patternFill></fill><border><left/><right/>\
                        <top/><bottom style=\"thin\"><color theme=\"1\" tint=\"-0.25\"/></bottom>\
                        <diagonal/></border></dxf>";

#[cfg(feature = "rules")]
#[test]
fn a_font_fill_and_border_dxf_round_trips_through_add_differential_style() {
    let body = part(
        "<xf/>",
        &format!(
            "<cellStyles count=\"1\"><cellStyle name=\"Normal\" xfId=\"0\"/></cellStyles><dxfs count=\"1\">{READ_DXF}</dxfs>"
        ),
    );
    let (_, mut package) = opened(styled(&body, ""));
    let read = package.differential_styles().expect("read").to_vec();
    assert_eq!(read.len(), 1);
    let added = package.add_differential_style(&read[0]).expect("added");
    assert_eq!(added, 1);
    let output = written(package);
    let styles = String::from_utf8(stored_data(&output, STYLES_PART)).expect("UTF-8");
    // The appended element is the one read, child by child, with the one
    // named difference: `patternType="solid"` written where none was read.
    let written_dxf = READ_DXF.replace("<patternFill>", "<patternFill patternType=\"solid\">");
    let want = body.replace(
        &format!("<dxfs count=\"1\">{READ_DXF}</dxfs>"),
        &format!("<dxfs count=\"2\">{READ_DXF}{written_dxf}</dxfs>"),
    );
    assert_eq!(styles, style_sheet(&want));
    let mut reopened = Package::open(Cursor::new(output)).expect("the output opens");
    let back = reopened.differential_styles().expect("read back");
    assert_eq!(back, [read[0].clone(), read[0].clone()].as_slice());
}

#[cfg(feature = "rules")]
#[test]
fn a_dxf_with_inside_sides_round_trips_without_them() {
    // P1 ruling 1: the two sides are unmodelled, absent on the written side.
    let read_dxf = "<dxf><border><left style=\"thin\"/><right/><top/><bottom/><diagonal/>\
                    <vertical style=\"thin\"/><horizontal style=\"thin\"/></border></dxf>";
    let body = part("<xf/>", &format!("<dxfs count=\"1\">{read_dxf}</dxfs>"));
    let (_, mut package) = opened(styled(&body, ""));
    let read = package.differential_styles().expect("read").to_vec();
    package.add_differential_style(&read[0]).expect("added");
    let output = written(package);
    let styles = String::from_utf8(stored_data(&output, STYLES_PART)).expect("UTF-8");
    let written_dxf = "<dxf><border><left style=\"thin\"/><right/><top/><bottom/><diagonal/>\
                       </border></dxf>";
    let want = body.replace(
        &format!("<dxfs count=\"1\">{read_dxf}</dxfs>"),
        &format!("<dxfs count=\"2\">{read_dxf}{written_dxf}</dxfs>"),
    );
    assert_eq!(styles, style_sheet(&want));
}
