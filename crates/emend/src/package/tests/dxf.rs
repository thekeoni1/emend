//! Differential styles on byte-built styles parts
//! (`specs/0020-validation-and-conditional-formatting.md`, "Differential
//! styles", "P2", and the P2 rulings): each test builds the smallest package
//! with its styles part holding a given body, records differential styles,
//! and compares the written styles part with the bytes the design gives,
//! every other part asserted untouched.

use super::*;
use crate::styles::{BorderStyle, Color, Fill, Font, Pattern, Side};

/// The collections every fixture holds, each with its `count` right, so a
/// rewrite leaves them byte for byte.
const BASE: &str = "<fonts count=\"1\"><font/></fonts><fills count=\"2\"><fill>\
                    <patternFill patternType=\"none\"/></fill><fill>\
                    <patternFill patternType=\"gray125\"/></fill></fills>\
                    <borders count=\"1\"><border/></borders><cellXfs count=\"1\"><xf/></cellXfs>";

/// A bold dark red font, openpyxl's light red fill's companion.
fn red_font() -> Font {
    Font {
        bold: true,
        color: Some(Color::Rgb(0xFF9C_0006)),
        ..Font::default()
    }
}

/// A solid fill with its colour in `bgColor`, as a differential style
/// wants it (decision 5 A).
fn red_fill() -> Fill {
    Fill {
        pattern: Pattern::Solid,
        foreground: None,
        background: Some(Color::Rgb(0xFFFF_C7CE)),
    }
}

/// A style of `red_font` and `red_fill`.
fn red() -> Style {
    Style {
        font: Some(red_font()),
        fill: Some(red_fill()),
        ..Style::default()
    }
}

/// The `<dxf>` [`red`] is written as, with `p` on every name.
fn red_dxf(p: &str) -> String {
    format!(
        "<{p}dxf><{p}font><{p}b/><{p}color rgb=\"FF9C0006\"/></{p}font><{p}fill>\
         <{p}patternFill patternType=\"solid\"><{p}bgColor rgb=\"FFFFC7CE\"/>\
         </{p}patternFill></{p}fill></{p}dxf>"
    )
}

/// Records `style` and asserts the index it returns.
#[track_caller]
fn dxf(package: &mut Package<Cursor<Vec<u8>>>, style: &Style, index: u32) {
    assert_eq!(
        package.add_differential_style(style).expect("recorded"),
        index,
        "the index the addition returns"
    );
}

#[test]
fn a_dxf_is_appended_to_a_dxfs_the_kept_children_kept_and_count_set() {
    let kept = "<dxf><font><i/></font></dxf><dxf><fill><patternFill/></fill></dxf>";
    let body = format!(
        "{BASE}<cellStyles count=\"1\"><cellStyle name=\"Normal\" xfId=\"0\" builtinId=\"0\"/>\
         </cellStyles><dxfs count=\"2\">{kept}</dxfs><tableStyles count=\"0\"/>"
    );
    let border = Style {
        border: Some(Border {
            bottom: Some(Side {
                style: BorderStyle::Thin,
                color: None,
            }),
            ..Border::default()
        }),
        ..Style::default()
    };
    let part = added_styles(&body, |p| {
        dxf(p, &red(), 2);
        dxf(p, &border, 3);
    });
    let want = body.replace(
        &format!("<dxfs count=\"2\">{kept}</dxfs>"),
        &format!(
            "<dxfs count=\"4\">{kept}{}<dxf><border><left/><right/><top/>\
             <bottom style=\"thin\"/><diagonal/></border></dxf></dxfs>",
            red_dxf("")
        ),
    );
    assert_eq!(part, style_sheet(&want));
    // An empty container is opened up, a prefixed one gives its prefix.
    let part = added_styles(&format!("{BASE}<dxfs count=\"0\"/>"), |p| dxf(p, &red(), 0));
    assert_eq!(
        part,
        style_sheet(&format!("{BASE}<dxfs count=\"1\">{}</dxfs>", red_dxf("")))
    );
    let prefixed = format!("{BASE}<x:dxfs xmlns:x=\"{MAIN_NS}\" count=\"1\"><x:dxf/></x:dxfs>");
    let part = added_styles(&prefixed, |p| dxf(p, &red(), 1));
    assert_eq!(
        part,
        style_sheet(&format!(
            "{BASE}<x:dxfs xmlns:x=\"{MAIN_NS}\" count=\"2\"><x:dxf/>{}</x:dxfs>",
            red_dxf("x:")
        ))
    );
}

#[test]
fn the_index_counts_the_children_not_the_count_attribute() {
    // P2 ruling 3: a `count` that disagrees with the children is replaced,
    // and the index is the element's position.
    let part = added_styles(
        &format!("{BASE}<dxfs count=\"7\"><dxf/><dxf/></dxfs>"),
        |p| {
            dxf(p, &red(), 2);
            dxf(p, &red(), 3);
        },
    );
    assert_eq!(
        part,
        style_sheet(&format!(
            "{BASE}<dxfs count=\"4\"><dxf/><dxf/>{}{}</dxfs>",
            red_dxf(""),
            red_dxf("")
        ))
    );
}

#[test]
fn a_dxfs_is_created_before_table_styles_colors_ext_lst_or_the_root_end() {
    let created = format!("<dxfs count=\"1\">{}</dxfs>", red_dxf(""));
    for (after, before) in [
        ("<tableStyles count=\"0\"/><colors/>", "<tableStyles"),
        ("<colors/><extLst/>", "<colors"),
        ("<extLst><ext uri=\"x\"/></extLst>", "<extLst"),
        ("", "</styleSheet>"),
    ] {
        let body = format!("{BASE}{after}");
        let part = added_styles(&body, |p| dxf(p, &red(), 0));
        println!("dxfs created before {before}: {part}");
        assert_eq!(
            part,
            style_sheet(&body).replacen(before, &format!("{created}{before}"), 1)
        );
    }
}

#[test]
fn two_dxfs_are_refused_at_the_call_and_a_created_then_met_one_at_the_write() {
    let (_, mut package) = opened(styled(&format!("{BASE}<dxfs/><dxfs/>"), ""));
    assert_package(
        package.add_differential_style(&red()).expect_err("refused"),
        "a styles part has two dxfs elements",
    );
    // Created before `colors`, then one met after it: out of schema order.
    let (_, mut package) = opened(styled(&format!("{BASE}<colors/><dxfs/>"), ""));
    package.add_differential_style(&red()).expect("recorded");
    assert_package(refused(package), "a styles part has two dxfs elements");
}

#[test]
fn a_field_this_part_does_not_write_or_no_field_is_refused_and_records_nothing() {
    let parts = styled(BASE, "");
    let (_, mut package) = opened(parts.clone());
    for style in [
        Style {
            number_format: Some(NumberFormat::BuiltIn(14)),
            ..red()
        },
        Style {
            alignment: Some(Alignment::default()),
            ..red()
        },
        Style {
            protection: Some(Protection::default()),
            ..red()
        },
        Style::default(),
    ] {
        assert_package(
            package.add_differential_style(&style).expect_err("refused"),
            "a differential style holds a field this part does not write",
        );
    }
    assert_untouched(&written(package), &parts, &[]);
}

#[test]
fn a_dxf_on_a_replaced_styles_part_is_refused() {
    let (_, mut package) = opened(styled(BASE, ""));
    package.add_differential_style(&red()).expect("recorded");
    package.replace_part(
        STYLES_PART,
        replacement_from_reader(Cursor::new(b"x".to_vec())),
    );
    assert_package(
        refused(package),
        "a style addition to a part this write replaces",
    );
}

#[test]
fn a_cell_style_and_a_dxf_in_one_write_both_go_in() {
    let part = added_styles(BASE, |p| {
        assert_eq!(p.add_style(None, &red()).expect("add_style"), 1);
        dxf(p, &red(), 0);
    });
    assert!(part.contains(&format!("<dxfs count=\"1\">{}</dxfs>", red_dxf(""))));
    assert!(part.contains("<cellXfs count=\"2\">"));
    assert!(part.contains("<fonts count=\"2\">"));
}
