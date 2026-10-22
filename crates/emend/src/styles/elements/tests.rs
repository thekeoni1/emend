//! The elements a style addition composes, byte for byte.
//!
//! `specs/0017-fuller-styles.md`, "P1": every keyword against the schema's
//! spelling (printed, so the log carries the table), each colour form, the
//! number forms, the refusals and every element's shape. What reaches a
//! styles part through `add_style` is `package::tests`'s.

use super::*;

/// The element with no prefix, as text.
#[track_caller]
fn text(composed: Composed) -> String {
    String::from_utf8(composed.element(b"")).expect("an element is UTF-8")
}

#[track_caller]
fn assert_package(error: Error, message: &str) {
    match error {
        Error::Package(what) => assert_eq!(what, message, "the message"),
        other => panic!("expected a package error, got {other:?}"),
    }
}

/// Prints one table line per keyword and asserts it.
#[track_caller]
fn pinned<T: std::fmt::Debug + Copy>(
    table: &str,
    rows: &[(T, &str)],
    keyword: impl Fn(T) -> &'static str,
) {
    for (variant, spelling) in rows {
        println!("keyword {table}::{variant:?} {}", keyword(*variant));
        assert_eq!(keyword(*variant), *spelling, "{table}::{variant:?}");
    }
}

#[test]
fn every_keyword_is_the_schemas_spelling() {
    // ECMA-376 Part 1, 18.18: ST_UnderlineValues, ST_VerticalAlignRun,
    // ST_FontScheme, ST_PatternType, ST_BorderStyle, ST_HorizontalAlignment
    // and ST_VerticalAlignment, each less the `none` a `None` field stands
    // for where the spec says so.
    pinned(
        "Underline",
        &[
            (Underline::Single, "single"),
            (Underline::Double, "double"),
            (Underline::SingleAccounting, "singleAccounting"),
            (Underline::DoubleAccounting, "doubleAccounting"),
        ],
        Underline::keyword,
    );
    pinned(
        "VerticalAlign",
        &[
            (VerticalAlign::Superscript, "superscript"),
            (VerticalAlign::Subscript, "subscript"),
            (VerticalAlign::Baseline, "baseline"),
        ],
        VerticalAlign::keyword,
    );
    pinned(
        "Scheme",
        &[(Scheme::Major, "major"), (Scheme::Minor, "minor")],
        Scheme::keyword,
    );
    let patterns = [
        (Pattern::None, "none"),
        (Pattern::Solid, "solid"),
        (Pattern::MediumGray, "mediumGray"),
        (Pattern::DarkGray, "darkGray"),
        (Pattern::LightGray, "lightGray"),
        (Pattern::DarkHorizontal, "darkHorizontal"),
        (Pattern::DarkVertical, "darkVertical"),
        (Pattern::DarkDown, "darkDown"),
        (Pattern::DarkUp, "darkUp"),
        (Pattern::DarkGrid, "darkGrid"),
        (Pattern::DarkTrellis, "darkTrellis"),
        (Pattern::LightHorizontal, "lightHorizontal"),
        (Pattern::LightVertical, "lightVertical"),
        (Pattern::LightDown, "lightDown"),
        (Pattern::LightUp, "lightUp"),
        (Pattern::LightGrid, "lightGrid"),
        (Pattern::LightTrellis, "lightTrellis"),
        (Pattern::Gray125, "gray125"),
        (Pattern::Gray0625, "gray0625"),
    ];
    pinned("Pattern", &patterns, Pattern::keyword);
    let styles = [
        (BorderStyle::Thin, "thin"),
        (BorderStyle::Medium, "medium"),
        (BorderStyle::Thick, "thick"),
        (BorderStyle::Dashed, "dashed"),
        (BorderStyle::Dotted, "dotted"),
        (BorderStyle::Double, "double"),
        (BorderStyle::Hair, "hair"),
        (BorderStyle::MediumDashed, "mediumDashed"),
        (BorderStyle::DashDot, "dashDot"),
        (BorderStyle::MediumDashDot, "mediumDashDot"),
        (BorderStyle::DashDotDot, "dashDotDot"),
        (BorderStyle::MediumDashDotDot, "mediumDashDotDot"),
        (BorderStyle::SlantDashDot, "slantDashDot"),
    ];
    pinned("BorderStyle", &styles, BorderStyle::keyword);
    pinned(
        "Horizontal",
        &[
            (Horizontal::General, "general"),
            (Horizontal::Left, "left"),
            (Horizontal::Center, "center"),
            (Horizontal::Right, "right"),
            (Horizontal::Fill, "fill"),
            (Horizontal::Justify, "justify"),
            (Horizontal::CenterContinuous, "centerContinuous"),
            (Horizontal::Distributed, "distributed"),
        ],
        Horizontal::keyword,
    );
    pinned(
        "Vertical",
        &[
            (Vertical::Top, "top"),
            (Vertical::Center, "center"),
            (Vertical::Bottom, "bottom"),
            (Vertical::Justify, "justify"),
            (Vertical::Distributed, "distributed"),
        ],
        Vertical::keyword,
    );
}

#[test]
fn a_font_with_every_field_is_written_in_the_schemas_order() {
    let font = Font {
        name: Some("A & <B>".to_owned()),
        size: Some(11.5),
        bold: true,
        italic: true,
        strike: true,
        underline: Some(Underline::DoubleAccounting),
        vertical_align: Some(VerticalAlign::Superscript),
        color: Some(Color::Theme {
            index: 1,
            tint: Some(-0.25),
        }),
        family: Some(2),
        scheme: Some(Scheme::Minor),
    };
    assert_eq!(
        text(font.compose().expect("the font composes")),
        "<font><b/><i/><strike/><u val=\"doubleAccounting\"/><vertAlign val=\"superscript\"/>\
         <sz val=\"11.5\"/><color theme=\"1\" tint=\"-0.25\"/><name val=\"A &amp; &lt;B&gt;\"/>\
         <family val=\"2\"/><scheme val=\"minor\"/></font>"
    );
}

#[test]
fn a_font_writes_only_what_is_given() {
    let font = Font {
        name: Some("emend".to_owned()),
        size: Some(12.0),
        bold: true,
        color: Some(Color::Rgb(0xFF33_6699)),
        ..Font::default()
    };
    assert_eq!(
        text(font.compose().expect("the font composes")),
        "<font><b/><sz val=\"12\"/><color rgb=\"FF336699\"/><name val=\"emend\"/></font>"
    );
    let single = Font {
        underline: Some(Underline::Single),
        ..Font::default()
    };
    assert_eq!(
        text(single.compose().expect("the font composes")),
        "<font><u/></font>",
        "a single underline is the val-less form"
    );
    assert_eq!(
        text(Font::default().compose().expect("the font composes")),
        "<font/>"
    );
}

#[test]
fn a_colour_is_written_in_each_of_its_four_forms() {
    let written = |color: Color| {
        let font = Font {
            color: Some(color),
            ..Font::default()
        };
        text(font.compose().expect("the font composes"))
    };
    assert_eq!(
        written(Color::Rgb(0x0000_00AB)),
        "<font><color rgb=\"000000AB\"/></font>",
        "eight upper-case hex digits, leading zeros kept"
    );
    assert_eq!(
        written(Color::Theme {
            index: 4,
            tint: None
        }),
        "<font><color theme=\"4\"/></font>"
    );
    assert_eq!(
        written(Color::Theme {
            index: 4,
            tint: Some(0.5999938962981048)
        }),
        "<font><color theme=\"4\" tint=\"0.5999938962981048\"/></font>"
    );
    assert_eq!(
        written(Color::Indexed(64)),
        "<font><color indexed=\"64\"/></font>"
    );
    assert_eq!(written(Color::Auto), "<font><color auto=\"1\"/></font>");
}

#[test]
fn a_size_and_a_tint_take_the_number_text_of_a_cell() {
    let sized = |size: f64| {
        let font = Font {
            size: Some(size),
            ..Font::default()
        };
        text(font.compose().expect("the font composes"))
    };
    assert_eq!(sized(12.0), "<font><sz val=\"12\"/></font>");
    assert_eq!(sized(11.5), "<font><sz val=\"11.5\"/></font>");
    assert_eq!(sized(1e-8), "<font><sz val=\"1E-8\"/></font>");
    let tinted = |tint: f64| {
        let fill = Fill {
            pattern: Pattern::Solid,
            foreground: Some(Color::Theme {
                index: 0,
                tint: Some(tint),
            }),
            background: None,
            gradient: None,
        };
        text(fill.compose().expect("the fill composes"))
    };
    for (tint, written) in [(-1.0, "-1"), (0.0, "0"), (1.0, "1"), (-0.25, "-0.25")] {
        assert_eq!(
            tinted(tint),
            format!(
                "<fill><patternFill patternType=\"solid\"><fgColor theme=\"0\" tint=\"{written}\"/>\
                 </patternFill></fill>"
            )
        );
    }
}

#[test]
fn a_font_name_empty_or_holding_a_forbidden_character_is_refused() {
    for name in ["", "a\u{1}b", "a\u{FFFE}"] {
        let font = Font {
            name: Some(name.to_owned()),
            ..Font::default()
        };
        assert_package(
            font.compose().expect_err("refused"),
            "a font name is empty or holds a character XML cannot carry",
        );
    }
    let tab = Font {
        name: Some("a\tb".to_owned()),
        ..Font::default()
    };
    assert!(tab.compose().is_ok(), "a tab is a character XML carries");
}

#[test]
fn a_font_name_takes_the_number_format_codes_path() {
    // specs/0017-fuller-styles.md, P1 verification ruling 4: escaped, then a
    // tab, a line feed and a carriage return as character references, so the
    // name reads back as itself rather than with spaces.
    let font = Font {
        name: Some("a\tb\nc\rd \"&".to_owned()),
        ..Font::default()
    };
    assert_eq!(
        text(font.compose().expect("the font composes")),
        "<font><name val=\"a&#9;b&#10;c&#13;d &quot;&amp;\"/></font>"
    );
}

#[test]
fn a_field_out_of_range_is_refused() {
    for size in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let font = Font {
            size: Some(size),
            ..Font::default()
        };
        assert_package(
            font.compose().expect_err("refused"),
            "a style field is out of range",
        );
    }
    for tint in [1.5, -1.5, f64::NAN, f64::NEG_INFINITY] {
        let color = Some(Color::Theme {
            index: 0,
            tint: Some(tint),
        });
        let font = Font {
            color,
            ..Font::default()
        };
        let fill = Fill {
            pattern: Pattern::Solid,
            foreground: None,
            background: color,
            gradient: None,
        };
        let side = Some(Side {
            style: BorderStyle::Thin,
            color,
        });
        let border = Border {
            diagonal: side,
            ..Border::default()
        };
        for error in [
            font.compose().expect_err("a font refused"),
            fill.compose().expect_err("a fill refused"),
            border.compose().expect_err("a border refused"),
        ] {
            assert_package(error, "a style field is out of range");
        }
    }
    for rotation in [181, 254, 256, u32::MAX] {
        let alignment = Alignment {
            text_rotation: Some(rotation),
            ..Alignment::default()
        };
        assert_package(
            alignment.compose().expect_err("refused"),
            "a style field is out of range",
        );
    }
    for rotation in [0, 180, 255] {
        let alignment = Alignment {
            text_rotation: Some(rotation),
            ..Alignment::default()
        };
        assert_eq!(
            text(alignment.compose().expect("accepted")),
            format!("<alignment textRotation=\"{rotation}\"/>")
        );
    }
}

#[test]
fn a_fill_is_written_with_both_colours_one_or_none() {
    let fill = |foreground, background| Fill {
        pattern: Pattern::Solid,
        foreground,
        background,
        gradient: None,
    };
    assert_eq!(
        text(
            fill(Some(Color::Rgb(0xFFDD_EEFF)), Some(Color::Indexed(64)))
                .compose()
                .expect("composes")
        ),
        "<fill><patternFill patternType=\"solid\"><fgColor rgb=\"FFDDEEFF\"/>\
         <bgColor indexed=\"64\"/></patternFill></fill>"
    );
    assert_eq!(
        text(fill(None, Some(Color::Auto)).compose().expect("composes")),
        "<fill><patternFill patternType=\"solid\"><bgColor auto=\"1\"/></patternFill></fill>"
    );
    let none = Fill {
        pattern: Pattern::Gray0625,
        foreground: None,
        background: None,
        gradient: None,
    };
    assert_eq!(
        text(none.compose().expect("composes")),
        "<fill><patternFill patternType=\"gray0625\"/></fill>"
    );
}

#[test]
fn a_border_is_written_with_its_five_sides_always() {
    let thin = Some(Side {
        style: BorderStyle::Thin,
        color: Some(Color::Theme {
            index: 1,
            tint: None,
        }),
    });
    let every = Border {
        left: thin,
        right: thin,
        top: thin,
        bottom: thin,
        diagonal: Some(Side {
            style: BorderStyle::MediumDashDotDot,
            color: None,
        }),
        diagonal_up: true,
        diagonal_down: true,
    };
    let side = "style=\"thin\"><color theme=\"1\"/>";
    assert_eq!(
        text(every.compose().expect("composes")),
        format!(
            "<border diagonalUp=\"1\" diagonalDown=\"1\"><left {side}</left><right {side}</right>\
             <top {side}</top><bottom {side}</bottom><diagonal style=\"mediumDashDotDot\"/>\
             </border>"
        )
    );
    let one = Border {
        bottom: thin,
        ..Border::default()
    };
    assert_eq!(
        text(one.compose().expect("composes")),
        format!("<border><left/><right/><top/><bottom {side}</bottom><diagonal/></border>")
    );
    assert_eq!(
        text(Border::default().compose().expect("composes")),
        "<border><left/><right/><top/><bottom/><diagonal/></border>"
    );
}

#[test]
fn an_alignment_is_written_with_every_field_or_none() {
    let every = Alignment {
        horizontal: Some(Horizontal::CenterContinuous),
        vertical: Some(Vertical::Top),
        wrap_text: true,
        shrink_to_fit: true,
        indent: Some(3),
        text_rotation: Some(90),
    };
    assert_eq!(
        text(every.compose().expect("composes")),
        "<alignment horizontal=\"centerContinuous\" vertical=\"top\" wrapText=\"1\" \
         shrinkToFit=\"1\" indent=\"3\" textRotation=\"90\"/>"
    );
    assert_eq!(
        text(Alignment::default().compose().expect("composes")),
        "<alignment/>"
    );
}

#[test]
fn a_protection_is_written_with_each_field() {
    let written = |locked, hidden| text(Protection { locked, hidden }.compose());
    assert_eq!(
        written(Some(true), Some(false)),
        "<protection locked=\"1\" hidden=\"0\"/>"
    );
    assert_eq!(written(Some(false), None), "<protection locked=\"0\"/>");
    assert_eq!(written(None, Some(true)), "<protection hidden=\"1\"/>");
    assert_eq!(written(None, None), "<protection/>");
}

#[test]
fn every_name_in_an_element_takes_the_prefix() {
    let fill = Fill {
        pattern: Pattern::Solid,
        foreground: Some(Color::Indexed(9)),
        background: None,
        gradient: None,
    };
    let element = fill.compose().expect("composes").element(b"x:");
    assert_eq!(
        String::from_utf8(element).expect("UTF-8"),
        "<x:fill><x:patternFill patternType=\"solid\"><x:fgColor indexed=\"9\"/>\
         </x:patternFill></x:fill>"
    );
}
