//! Colours resolved on byte-built packages (`specs/0032-read-breadth-
//! styles.md`, "P2: colours", and "Planning rulings during P2"): the default
//! palette and a styles part's `<indexedColors>`, a theme part's colour
//! scheme read through `theme_colors`, a colour resolved through
//! `resolve_color`, and the tint applied by `ResolvedColor::tinted`. The
//! Cargo feature `read`.

use super::*;
use crate::styles::PALETTE;
use crate::styles::{
    Border, BorderStyle, Color, Fill, Font, Gradient, GradientKind, Pattern, ResolvedColor, Side,
    Stop,
};

/// The DrawingML main namespace, transitional.
const DRAWING_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";

/// The same, strict.
const DRAWING_STRICT: &str = "http://purl.oclc.org/ooxml/drawingml/main";

const THEME_PART: &[u8] = b"xl/theme/theme1.xml";

/// Twelve scheme children in element order, two of them `sysClr`, each
/// colour distinct, fictional.
const SCHEME: &str = "<a:dk1><a:sysClr val=\"windowText\" lastClr=\"000000\"/></a:dk1>\
                      <a:lt1><a:sysClr val=\"window\" lastClr=\"FFFFFF\"/></a:lt1>\
                      <a:dk2><a:srgbClr val=\"102030\"/></a:dk2>\
                      <a:lt2><a:srgbClr val=\"E0E0D0\"/></a:lt2>\
                      <a:accent1><a:srgbClr val=\"4080C0\"/></a:accent1>\
                      <a:accent2><a:srgbClr val=\"C04040\"/></a:accent2>\
                      <a:accent3><a:srgbClr val=\"80C040\"/></a:accent3>\
                      <a:accent4><a:srgbClr val=\"8060A0\"/></a:accent4>\
                      <a:accent5><a:srgbClr val=\"40B0C0\"/></a:accent5>\
                      <a:accent6><a:srgbClr val=\"F09040\"/></a:accent6>\
                      <a:hlink><a:srgbClr val=\"0000FE\"/></a:hlink>\
                      <a:folHlink><a:srgbClr val=\"800081\"/></a:folHlink>";

/// `SCHEME` read: element order, each opaque.
const SCHEME_READ: [u32; 12] = [
    0xFF000000, 0xFFFFFFFF, 0xFF102030, 0xFFE0E0D0, 0xFF4080C0, 0xFFC04040, 0xFF80C040, 0xFF8060A0,
    0xFF40B0C0, 0xFFF09040, 0xFF0000FE, 0xFF800081,
];

/// A theme part in `namespace` whose `themeElements` holds `elements`, with
/// a font scheme and an object-defaults element beside it.
fn theme_in(namespace: &str, elements: &str) -> String {
    format!(
        "{DECLARATION}\n<a:theme xmlns:a=\"{namespace}\" name=\"T\"><a:themeElements>\
         {elements}<a:fontScheme name=\"F\"><a:majorFont><a:latin typeface=\"X\"/>\
         </a:majorFont></a:fontScheme></a:themeElements><a:objectDefaults/></a:theme>"
    )
}

/// A transitional theme part whose one `clrScheme` holds `children`.
fn theme(children: &str) -> String {
    theme_in(
        DRAWING_NS,
        &format!("<a:clrScheme name=\"S\">{children}</a:clrScheme>"),
    )
}

/// The smallest package with its styles part holding `body` and, with
/// `theme`, a theme relationship and the theme part holding it.
fn with_theme(body: &str, theme: Option<String>) -> Package<Cursor<Vec<u8>>> {
    let mut parts = styled(body, "");
    if let Some(theme) = theme {
        set(
            &mut parts,
            b"xl/_rels/workbook.xml.rels",
            rels(&format!(
                "{}{}{}",
                relationship(
                    "rId1",
                    &format!("{ATTR_NS}/worksheet"),
                    "worksheets/sheet1.xml"
                ),
                relationship("rId2", &format!("{ATTR_NS}/styles"), "styles.xml"),
                relationship("rId3", &format!("{ATTR_NS}/theme"), "theme/theme1.xml"),
            )),
        );
        parts.push((THEME_PART.to_vec(), theme.into_bytes()));
    }
    opened(parts).1
}

/// The scheme colours of a package whose theme part is `theme`.
fn scheme_of(theme: String) -> Result<Option<[u32; 12]>, Error> {
    with_theme("", Some(theme))
        .theme_colors()
        .map(|colors| colors.copied())
}

#[track_caller]
fn theme_refused(theme: String) {
    match scheme_of(theme) {
        Err(error) => assert_package(error, "a theme cannot be read"),
        Ok(read) => panic!("read as {read:?}"),
    }
}

/// `color` resolved on a package whose styles part holds `body` and whose
/// theme part is `SCHEME`'s.
fn resolved(body: &str, color: Color) -> Result<Option<ResolvedColor>, Error> {
    with_theme(body, Some(theme(SCHEME))).resolve_color(&color)
}

fn plain(rgb: u32) -> Option<ResolvedColor> {
    Some(ResolvedColor { rgb, tint: None })
}

#[track_caller]
fn style_refused(body: &str) {
    match with_theme(body, None).styles() {
        Err(error) => assert_package(error, "a style cannot be read"),
        Ok(styles) => panic!("read as {styles:?}"),
    }
}

// --- the palette and indexedColors ---

#[test]
fn the_palette_s_first_sixteen_entries_and_its_length() {
    // ECMA-376 Part 1, 18.8.27: black, white, red, green, blue, yellow,
    // magenta and cyan, twice; the top byte FF (P2 ruling 8).
    let eight = [
        0xFF000000, 0xFFFFFFFF, 0xFFFF0000, 0xFF00FF00, 0xFF0000FF, 0xFFFFFF00, 0xFFFF00FF,
        0xFF00FFFF,
    ];
    assert_eq!(PALETTE.len(), 64);
    assert_eq!(PALETTE[..8], eight);
    assert_eq!(PALETTE[8..16], eight);
    assert!(PALETTE.iter().all(|entry| entry >> 24 == 0xFF));
}

#[test]
fn an_indexed_colors_override_is_read_and_consulted_and_an_index_past_it_falls_to_the_default() {
    let body = "<colors><indexedColors><rgbColor rgb=\"00123456\"/>\
                <rgbColor rgb=\"FFabcdef\"/></indexedColors></colors>";
    assert_eq!(
        with_theme(body, None)
            .styles()
            .expect("read")
            .indexed_colors,
        Some(vec![0x00123456, 0xFFABCDEF])
    );
    assert_eq!(
        resolved(body, Color::Indexed(0)).expect("0"),
        plain(0x00123456)
    );
    assert_eq!(
        resolved(body, Color::Indexed(1)).expect("1"),
        plain(0xFFABCDEF)
    );
    assert_eq!(
        resolved(body, Color::Indexed(2)).expect("2"),
        plain(PALETTE[2])
    );
    assert_eq!(
        resolved(body, Color::Indexed(63)).expect("63"),
        plain(PALETTE[63])
    );
    // Without the element, the default throughout.
    assert_eq!(
        with_theme("", None).styles().expect("read").indexed_colors,
        None
    );
    assert_eq!(
        resolved("", Color::Indexed(0)).expect("0"),
        plain(0xFF000000)
    );
}

#[test]
fn indexes_64_and_81_resolve_to_none() {
    for at in [64, 81] {
        assert_eq!(resolved("", Color::Indexed(at)).expect("resolved"), None);
    }
}

#[test]
fn an_override_of_65_entries_resolves_64_to_its_entry_and_65_to_none() {
    // "Planning verification and rulings on P2", ruling 2: an index within
    // the override resolves to it, 64 included; past it and past 63, `None`.
    let mut body = String::from("<colors><indexedColors>");
    for at in 0..65u32 {
        body.push_str(&format!("<rgbColor rgb=\"FF0000{at:02X}\"/>"));
    }
    body.push_str("</indexedColors></colors>");
    assert_eq!(
        resolved(&body, Color::Indexed(64)).expect("64"),
        plain(0xFF000040)
    );
    assert_eq!(resolved(&body, Color::Indexed(65)).expect("65"), None);
}

#[test]
fn mru_colors_and_a_foreign_child_are_walked_past() {
    let body = "<colors><mruColors><color rgb=\"FF000000\"/><x/></mruColors>\
                <o:any xmlns:o=\"urn:other\"><o:rgbColor/></o:any>\
                <indexedColors><o:rgbColor xmlns:o=\"urn:other\"/>\
                <rgbColor rgb=\"FF010203\"/></indexedColors></colors>";
    assert_eq!(
        with_theme(body, None)
            .styles()
            .expect("read")
            .indexed_colors,
        Some(vec![0xFF010203])
    );
}

#[test]
fn a_second_colors_is_refused() {
    style_refused("<colors/><colors/>");
}

#[test]
fn a_second_indexed_colors_is_refused() {
    style_refused("<colors><indexedColors/><indexedColors/></colors>");
}

#[test]
fn an_rgb_color_with_no_rgb_is_refused() {
    style_refused("<colors><indexedColors><rgbColor/></indexedColors></colors>");
}

#[test]
fn an_rgb_that_is_not_eight_hex_digits_is_refused() {
    for rgb in ["FF00000", "FF0000000", "FF00000G"] {
        style_refused(&format!(
            "<colors><indexedColors><rgbColor rgb=\"{rgb}\"/></indexedColors></colors>"
        ));
    }
}

#[test]
fn an_indexed_colors_child_other_than_rgb_color_is_refused() {
    style_refused("<colors><indexedColors><color rgb=\"FF000000\"/></indexedColors></colors>");
}

// --- the theme ---

#[test]
fn a_theme_s_srgb_and_sys_clr_children_are_read_in_order() {
    assert_eq!(scheme_of(theme(SCHEME)).expect("read"), Some(SCHEME_READ));
    // Lower-case hex, an extLst in the scheme, a clrScheme under
    // extraClrSchemeLst and a start-and-end colour element with no child.
    let lower = SCHEME.replace("102030", "a0b0c0").replace(
        "<a:srgbClr val=\"E0E0D0\"/>",
        "<a:srgbClr val=\"E0E0D0\"></a:srgbClr>",
    );
    let elements = format!(
        "<a:clrScheme name=\"S\">{lower}<a:extLst><a:ext uri=\"u\"/></a:extLst></a:clrScheme>"
    );
    let extra = "<a:extraClrSchemeLst><a:extraClrScheme><a:clrScheme name=\"E\">\
                 <a:dk1/></a:clrScheme></a:extraClrScheme></a:extraClrSchemeLst>";
    let read = scheme_of(format!(
        "{DECLARATION}\n<a:theme xmlns:a=\"{DRAWING_NS}\"><a:themeElements>{elements}\
         </a:themeElements>{extra}</a:theme>"
    ))
    .expect("read")
    .expect("a theme");
    assert_eq!(read[2], 0xFFA0B0C0);
    assert_eq!(read[3], 0xFFE0E0D0);
}

#[test]
fn the_theme_index_swaps_the_first_two_pairs() {
    for (index, entry) in [(0, 1), (1, 0), (2, 3), (3, 2), (4, 4), (11, 11)] {
        assert_eq!(
            resolved("", Color::Theme { index, tint: None }).expect("resolved"),
            plain(SCHEME_READ[entry]),
            "theme {index}"
        );
    }
    assert_eq!(
        resolved(
            "",
            Color::Theme {
                index: 4,
                tint: Some(-0.25)
            }
        )
        .expect("resolved"),
        Some(ResolvedColor {
            rgb: 0xFF4080C0,
            tint: Some(-0.25)
        })
    );
}

#[test]
fn the_strict_namespace_is_accepted() {
    let strict = theme_in(
        DRAWING_STRICT,
        &format!("<a:clrScheme name=\"S\">{SCHEME}</a:clrScheme>"),
    );
    assert_eq!(scheme_of(strict).expect("read"), Some(SCHEME_READ));
}

#[test]
fn no_theme_relationship_gives_none() {
    let mut package = with_theme("", None);
    assert_eq!(package.theme_colors().expect("read"), None);
    assert_eq!(
        package
            .resolve_color(&Color::Theme {
                index: 1,
                tint: None
            })
            .expect("resolved"),
        None
    );
}

#[test]
fn a_theme_relationship_naming_no_part_is_refused() {
    let mut parts = styled("", "");
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship("rId3", &format!("{ATTR_NS}/theme"), "theme/theme1.xml"),
        )),
    );
    match opened(parts).1.theme_colors() {
        Err(error) => assert_package(error, "no such part"),
        Ok(read) => panic!("read as {read:?}"),
    }
}

#[test]
fn a_val_of_five_digits_or_a_sys_clr_with_no_last_clr_is_refused() {
    theme_refused(theme(&SCHEME.replace("102030", "10203")));
    theme_refused(theme(&SCHEME.replace(" lastClr=\"FFFFFF\"", "")));
    theme_refused(theme(
        &SCHEME.replace("lastClr=\"FFFFFF\"", "lastClr=\"FFFFFG\""),
    ));
}

#[test]
fn two_clr_schemes_or_none_at_the_path_are_refused() {
    let one = format!("<a:clrScheme name=\"S\">{SCHEME}</a:clrScheme>");
    theme_refused(theme_in(DRAWING_NS, &format!("{one}{one}")));
    theme_refused(theme_in(DRAWING_NS, ""));
    // A root that is not a DrawingML theme.
    theme_refused(format!(
        "{DECLARATION}\n<a:theme xmlns:a=\"urn:other\"><a:themeElements>{one}\
         </a:themeElements></a:theme>"
    ));
}

#[test]
fn a_scheme_child_missing_twice_foreign_or_unknown_is_refused() {
    // Missing.
    theme_refused(theme(
        &SCHEME.replace("<a:hlink><a:srgbClr val=\"0000FE\"/></a:hlink>", ""),
    ));
    // Twice.
    theme_refused(theme(&format!(
        "{SCHEME}<a:hlink><a:srgbClr val=\"0000FE\"/></a:hlink>"
    )));
    // Outside the namespace.
    theme_refused(theme(&format!("{SCHEME}<o:x xmlns:o=\"urn:other\"/>")));
    // Unknown to the scheme.
    theme_refused(theme(&format!("{SCHEME}<a:accent7/>")));
}

#[test]
fn a_scheme_child_with_no_colour_another_form_or_a_transform_is_refused() {
    let lt2 = "<a:lt2><a:srgbClr val=\"E0E0D0\"/></a:lt2>";
    for replacement in [
        "<a:lt2/>",
        "<a:lt2></a:lt2>",
        "<a:lt2><a:prstClr val=\"black\"/></a:lt2>",
        "<a:lt2><a:srgbClr val=\"E0E0D0\"><a:lumMod val=\"75000\"/></a:srgbClr></a:lt2>",
        "<a:lt2><a:srgbClr val=\"E0E0D0\"/><a:srgbClr val=\"E0E0D0\"/></a:lt2>",
        "<a:lt2><o:srgbClr xmlns:o=\"urn:other\" val=\"E0E0D0\"/></a:lt2>",
    ] {
        theme_refused(theme(&SCHEME.replace(lt2, replacement)));
    }
}

// --- the resolve ---

#[test]
fn a_theme_index_past_11_or_a_tint_out_of_range_or_not_finite_is_refused() {
    for color in [
        Color::Theme {
            index: 12,
            tint: None,
        },
        Color::Theme {
            index: 0,
            tint: Some(1.5),
        },
        Color::Theme {
            index: 0,
            tint: Some(-1.5),
        },
        Color::Theme {
            index: 0,
            tint: Some(f64::NAN),
        },
        Color::Theme {
            index: 0,
            tint: Some(f64::INFINITY),
        },
    ] {
        match resolved("", color) {
            Err(error) => assert_package(error, "a colour cannot be resolved"),
            Ok(read) => panic!("{color:?} resolved as {read:?}"),
        }
    }
    // The bounds themselves resolve.
    for tint in [-1.0, 1.0] {
        assert!(
            resolved(
                "",
                Color::Theme {
                    index: 0,
                    tint: Some(tint)
                }
            )
            .is_ok()
        );
    }
}

#[test]
fn rgb_is_itself_and_auto_is_none() {
    assert_eq!(
        resolved("", Color::Rgb(0x80123456)).expect("rgb"),
        plain(0x80123456)
    );
    assert_eq!(resolved("", Color::Auto).expect("auto"), None);
}

#[test]
fn the_tint_formula_on_a_known_case_each_way_and_no_tint() {
    // ECMA-376 Part 1, 18.8.19 with HLSMAX 255: black lightened by 0.5 is
    // L 127.5, white darkened by 0.5 the same; 127.5 rounds to 128 (P2
    // ruling 4).
    let lighter = ResolvedColor {
        rgb: 0xFF000000,
        tint: Some(0.5),
    };
    assert_eq!(lighter.tinted(), 0xFF808080);
    let darker = ResolvedColor {
        rgb: 0xFFFFFFFF,
        tint: Some(-0.5),
    };
    assert_eq!(darker.tinted(), 0xFF808080);
    // No tint gives rgb; the alpha byte is kept through a tint.
    assert_eq!(
        ResolvedColor {
            rgb: 0x12345678,
            tint: None
        }
        .tinted(),
        0x12345678
    );
    assert_eq!(
        ResolvedColor {
            rgb: 0x00000000,
            tint: Some(0.5)
        }
        .tinted(),
        0x00808080
    );
    // A chromatic colour keeps its hue: pure red darkened by half.
    assert_eq!(
        ResolvedColor {
            rgb: 0xFFFF0000,
            tint: Some(-0.5)
        }
        .tinted(),
        0xFF800000
    );
}

// --- the typed lists (ruling 9) ---

#[test]
fn the_typed_lists_lend_the_part_s_elements_in_order_a_gradient_typed() {
    let body = "<fonts count=\"2\"><font><b/></font><font><color theme=\"1\"/></font></fonts>\
                <fills count=\"3\"><fill><patternFill patternType=\"none\"/></fill>\
                <fill><gradientFill><stop position=\"0\"><color rgb=\"FF000000\"/></stop>\
                </gradientFill></fill>\
                <fill><patternFill patternType=\"solid\"><fgColor indexed=\"2\"/></patternFill>\
                </fill></fills>\
                <borders count=\"2\"><border/><border><left style=\"thin\">\
                <color auto=\"1\"/></left></border></borders>\
                <cellXfs count=\"1\"><xf/></cellXfs>";
    let mut package = with_theme(body, None);
    let styles = package.styles().expect("read");
    assert_eq!(
        styles.typed_fonts(),
        [
            Font {
                bold: true,
                ..Font::default()
            },
            Font {
                color: Some(Color::Theme {
                    index: 1,
                    tint: None
                }),
                ..Font::default()
            },
        ]
    );
    assert_eq!(
        styles.typed_fills(),
        [
            Some(Fill {
                pattern: Pattern::None,
                foreground: None,
                background: None,
                gradient: None,
            }),
            Some(Fill {
                pattern: Pattern::None,
                foreground: None,
                background: None,
                gradient: Some(Gradient {
                    kind: GradientKind::Linear { degree: 0.0 },
                    stops: vec![Stop {
                        position: 0.0,
                        color: Color::Rgb(0xFF00_0000),
                    }],
                }),
            }),
            Some(Fill {
                pattern: Pattern::Solid,
                foreground: Some(Color::Indexed(2)),
                background: None,
                gradient: None,
            }),
        ]
    );
    assert_eq!(
        styles.typed_borders(),
        [
            Border::default(),
            Border {
                left: Some(Side {
                    style: BorderStyle::Thin,
                    color: Some(Color::Auto),
                }),
                ..Border::default()
            },
        ]
    );
    // The counts stand beside them.
    assert_eq!(
        (styles.fonts(), styles.fills(), styles.borders()),
        (Some(2), Some(3), Some(2))
    );
}
