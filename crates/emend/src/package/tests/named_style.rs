//! Named styles on byte-built styles parts
//! (`specs/0042-gradient-fills-and-named-styles.md`, "Types", "The styles
//! part", "Errors" and "P2: named styles"): each test builds the smallest
//! package with its styles part holding a given body, records named styles
//! and cell formats naming them, and compares the written styles part with
//! the bytes the design gives, every other part asserted untouched; the
//! refusals in the order "Planning rulings during P2" gives (rulings S, T
//! and U), and the written-then-read round trip under `read`.

use super::*;

/// One `cellStyleXfs` format and the `Normal` entry naming it, between the
/// collections and `<cellXfs>` with one plain format.
fn with_named(xfs: &str, styles: &str) -> String {
    format!(
        "{}<cellStyleXfs count=\"1\">{xfs}</cellStyleXfs><cellXfs count=\"1\"><xf/></cellXfs>\
         <cellStyles count=\"1\">{styles}</cellStyles>",
        with_collections("")
    )
}

/// The plain named format and the `Normal` entry, as Excel writes them.
const PLAIN_XF: &str = "<xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\"/>";
const NORMAL: &str = "<cellStyle name=\"Normal\" xfId=\"0\" builtinId=\"0\"/>";

/// [`with_named`] with the plain format and `Normal`.
fn named_body() -> String {
    with_named(PLAIN_XF, NORMAL)
}

/// A style with a bold font alone.
fn bold() -> Style {
    Style {
        font: Some(Font {
            bold: true,
            ..Font::default()
        }),
        ..Style::default()
    }
}

/// A style naming the named style `k` and nothing else.
fn naming(k: u32) -> Style {
    Style {
        named: Some(k),
        ..Style::default()
    }
}

/// Records one `add_named_style` and asserts the index it returns.
#[track_caller]
fn add_named(package: &mut Package<Cursor<Vec<u8>>>, name: &str, style: &Style, index: u32) {
    let got = package
        .add_named_style(name, style)
        .expect("the named style is recorded");
    assert_eq!(got, index, "the index the named style returns");
}

/// Asserts that no cell format, named style or element was recorded.
#[track_caller]
fn assert_none_recorded(package: &Package<Cursor<Vec<u8>>>) {
    assert_nothing_recorded(package);
    assert!(
        package.edits.named_additions.is_empty(),
        "a named style was recorded"
    );
}

/// Asserts `add_named_style(name, style)` refused with `message` and
/// recording nothing, on a package whose styles part holds `body`.
#[track_caller]
fn assert_named_refused(body: &str, name: &str, style: &Style, message: &str) {
    let (_, mut package) = opened(styled(body, ""));
    assert_package(
        package.add_named_style(name, style).expect_err("refused"),
        message,
    );
    assert_none_recorded(&package);
}

#[test]
fn a_named_style_writes_its_format_without_xf_id_and_its_entry_with_both_counts() {
    let body = named_body();
    let style = Style {
        number_format: Some(NumberFormat::BuiltIn(2)),
        ..bold()
    };
    let got = added_styles(&body, |p| add_named(p, "Mine", &style, 1));
    let expected = body
        .replace(
            "<font><sz val=\"11\"/></font></fonts>",
            "<font><sz val=\"11\"/></font><font><b/></font></fonts>",
        )
        .replace("<fonts count=\"1\">", "<fonts count=\"2\">")
        .replace(
            &format!("<cellStyleXfs count=\"1\">{PLAIN_XF}</cellStyleXfs>"),
            &format!(
                "<cellStyleXfs count=\"2\">{PLAIN_XF}<xf numFmtId=\"2\" fontId=\"1\" fillId=\"0\" \
                 borderId=\"0\" applyNumberFormat=\"1\" applyFont=\"1\"/></cellStyleXfs>"
            ),
        )
        .replace(
            &format!("<cellStyles count=\"1\">{NORMAL}</cellStyles>"),
            &format!(
                "<cellStyles count=\"2\">{NORMAL}<cellStyle name=\"Mine\" xfId=\"1\"/></cellStyles>"
            ),
        );
    assert_eq!(got, style_sheet(&expected));
}

#[test]
fn a_named_style_s_index_is_the_part_s_count_plus_those_recorded() {
    // Two named styles over a part with two formats, then a cell format: the
    // named styles' indexes follow `<cellStyleXfs>`, the cell format's
    // `<cellXfs>`.
    let body = with_named(&format!("{PLAIN_XF}{PLAIN_XF}"), NORMAL)
        .replace("<cellStyleXfs count=\"1\">", "<cellStyleXfs count=\"2\">");
    let (_, mut package) = opened(styled(&body, ""));
    add_named(&mut package, "First", &bold(), 2);
    add_named(&mut package, "Second", &Style::default(), 3);
    add_styled(&mut package, None, &naming(3), 1);
}

#[test]
fn a_named_style_s_name_is_escaped_as_an_attribute() {
    let body = named_body();
    let got = added_styles(&body, |p| {
        add_named(p, "A & \"B\"\t<C>", &Style::default(), 1);
    });
    assert!(
        got.contains(
            "<cellStyle name=\"A &amp; &quot;B&quot;&#9;&lt;C&gt;\" xfId=\"1\"/></cellStyles>"
        ),
        "{got}"
    );
}

#[test]
fn a_cell_format_named_over_no_base_takes_the_named_format_s_ids_and_children() {
    // The named format gives the ids the style leaves `None` and its
    // `alignment`; the style gives the number format and a protection, which
    // goes after the named format's alignment.
    let xf = "<xf numFmtId=\"3\" fontId=\"0\" fillId=\"1\" borderId=\"0\" applyAlignment=\"1\">\
              <alignment horizontal=\"left\"/></xf>";
    let body = with_named(xf, NORMAL);
    let style = Style {
        number_format: Some(NumberFormat::BuiltIn(4)),
        protection: Some(Protection {
            locked: Some(false),
            hidden: None,
        }),
        named: Some(0),
        ..Style::default()
    };
    let got = added_styles(&body, |p| add_styled(p, None, &style, 1));
    let expected = body.replace(
        "<cellXfs count=\"1\"><xf/></cellXfs>",
        "<cellXfs count=\"2\"><xf/><xf numFmtId=\"4\" fontId=\"0\" fillId=\"1\" borderId=\"0\" \
         xfId=\"0\" applyNumberFormat=\"1\" applyProtection=\"1\"><alignment horizontal=\"left\"/>\
         <protection locked=\"0\"/></xf></cellXfs>",
    );
    assert_eq!(got, style_sheet(&expected));
}

#[test]
fn a_cell_format_named_by_a_recorded_style_takes_its_recorded_ids() {
    let body = named_body();
    let got = added_styles(&body, |p| {
        add_named(p, "Mine", &bold(), 1);
        add_styled(p, None, &naming(1), 1);
    });
    assert!(
        got.contains(
            "<cellXfs count=\"2\"><xf/><xf numFmtId=\"0\" fontId=\"1\" fillId=\"0\" \
             borderId=\"0\" xfId=\"1\"/></cellXfs>"
        ),
        "{got}"
    );
}

#[test]
fn a_cell_format_named_over_a_base_has_its_xf_id_replaced_or_added() {
    // Over a base the base's ids are kept and `xfId` alone names the style
    // (ruling V): replaced in place, or added after the base's attributes
    // and the ids added, before the apply flags added (ruling U).
    let cell_xfs = "<cellXfs count=\"2\"><xf numFmtId=\"0\" xfId=\"0\" applyFont=\"0\"/>\
                    <xf numFmtId=\"2\" applyAlignment=\"1\"><alignment horizontal=\"left\"/></xf>\
                    </cellXfs>";
    let body = format!(
        "{}<cellStyleXfs count=\"2\">{PLAIN_XF}{PLAIN_XF}</cellStyleXfs>{cell_xfs}\
         <cellStyles count=\"1\">{NORMAL}</cellStyles>",
        with_collections("")
    );
    let with_font = Style {
        named: Some(1),
        ..bold()
    };
    let got = added_styles(&body, |p| {
        add_styled(p, Some(0), &naming(1), 2);
        add_styled(p, Some(1), &with_font, 3);
    });
    let expected = body
        .replace(
            "</xf>\
             </cellXfs>",
            "</xf><xf numFmtId=\"0\" xfId=\"1\" applyFont=\"0\"/>\
             <xf numFmtId=\"2\" applyAlignment=\"1\" fontId=\"1\" xfId=\"1\" applyFont=\"1\">\
             <alignment horizontal=\"left\"/></xf></cellXfs>",
        )
        .replace("<cellXfs count=\"2\">", "<cellXfs count=\"4\">")
        .replace(
            "<font><sz val=\"11\"/></font></fonts>",
            "<font><sz val=\"11\"/></font><font><b/></font></fonts>",
        )
        .replace("<fonts count=\"1\">", "<fonts count=\"2\">");
    assert_eq!(got, style_sheet(&expected));
}

#[test]
fn a_named_index_past_the_count_and_those_recorded_is_refused_and_records_nothing() {
    let (_, mut package) = opened(styled(&named_body(), ""));
    for past in [1, 2, u32::MAX] {
        assert_package(
            package.add_style(None, &naming(past)).expect_err("refused"),
            "a style index is out of range",
        );
        assert_none_recorded(&package);
    }
    // One recorded makes 1 a style and leaves 2 past.
    add_named(&mut package, "Mine", &Style::default(), 1);
    assert_package(
        package.add_style(None, &naming(2)).expect_err("refused"),
        "a style index is out of range",
    );
    add_styled(&mut package, None, &naming(1), 1);
}

#[test]
fn an_empty_too_long_or_unwritable_name_is_refused_and_records_nothing() {
    let body = named_body();
    let message = "a style name is empty, too long or taken";
    for name in [
        String::new(),
        "x".repeat(256),
        "\u{e9}".repeat(256),
        "a\u{1}b".to_string(),
        "a\u{FFFE}".to_string(),
    ] {
        assert_named_refused(&body, &name, &Style::default(), message);
    }
    // 255 characters, multi-byte ones among them, is the limit.
    let (_, mut package) = opened(styled(&body, ""));
    add_named(&mut package, &"\u{e9}".repeat(255), &Style::default(), 1);
    add_named(&mut package, &"x".repeat(255), &Style::default(), 2);
}

#[test]
fn a_name_equal_ignoring_case_to_one_held_or_recorded_is_refused() {
    let body = named_body();
    let message = "a style name is empty, too long or taken";
    // The part's own name, in any ASCII case.
    for name in ["Normal", "NORMAL", "normal"] {
        assert_named_refused(&body, name, &Style::default(), message);
    }
    // One recorded earlier by this package; the folding is ASCII's alone.
    let (_, mut package) = opened(styled(&body, ""));
    add_named(&mut package, "Caf\u{e9}", &Style::default(), 1);
    assert_package(
        package
            .add_named_style("CAF\u{e9}", &Style::default())
            .expect_err("refused"),
        message,
    );
    add_named(&mut package, "CAF\u{c9}", &Style::default(), 2);
    // The part's names as read, unescaped.
    let escaped = with_named(PLAIN_XF, "<cellStyle name=\"A &amp; B\" xfId=\"0\"/>");
    assert_named_refused(&escaped, "a & b", &Style::default(), message);
}

#[test]
fn a_part_without_cell_style_xfs_or_cell_styles_is_refused_with_the_collection_message() {
    // Ruling T: the name first, then the containers.
    let no_styles = format!(
        "{}<cellStyleXfs count=\"1\">{PLAIN_XF}</cellStyleXfs><cellXfs count=\"1\"><xf/></cellXfs>",
        with_collections("")
    );
    // No entry, as an entry naming a format of a missing `<cellStyleXfs>`
    // is one the typed read refuses.
    let no_xfs = format!(
        "{}<cellXfs count=\"1\"><xf/></cellXfs><cellStyles count=\"0\"/>",
        with_collections("")
    );
    for body in [&no_styles, &no_xfs] {
        assert_named_refused(
            body,
            "Mine",
            &bold(),
            "a style addition needs a collection the styles part lacks",
        );
        assert_named_refused(
            body,
            "",
            &bold(),
            "a style name is empty, too long or taken",
        );
    }
}

#[test]
fn a_named_style_given_named_is_refused_as_out_of_range() {
    assert_named_refused(
        &named_body(),
        "Mine",
        &naming(0),
        "a style field is out of range",
    );
}

#[test]
fn the_named_index_is_checked_after_protection_and_before_the_gradient() {
    let (_, mut package) = opened(styled(&named_body(), ""));
    // An alignment out of range is checked before the index.
    let rotated = Style {
        alignment: Some(Alignment {
            text_rotation: Some(200),
            ..Alignment::default()
        }),
        named: Some(9),
        ..Style::default()
    };
    assert_package(
        package.add_style(None, &rotated).expect_err("refused"),
        "a style field is out of range",
    );
    // The index is checked before the gradient's degree.
    let gradient = Style {
        fill: Some(Fill {
            pattern: Pattern::None,
            foreground: None,
            background: None,
            gradient: Some(crate::styles::Gradient {
                kind: crate::styles::GradientKind::Linear { degree: 400.0 },
                stops: Vec::new(),
            }),
        }),
        named: Some(9),
        ..Style::default()
    };
    assert_package(
        package.add_style(None, &gradient).expect_err("refused"),
        "a style index is out of range",
    );
    assert_none_recorded(&package);
}

#[test]
fn with_no_named_style_both_containers_are_rewritten_byte_for_byte() {
    // Counts that disagree with the children, single quotes and a prefix the
    // rewrite would otherwise normalise: none is touched when only a cell
    // format is added.
    let body = format!(
        "{}<cellStyleXfs count='7'>{PLAIN_XF}</cellStyleXfs><cellXfs count=\"1\"><xf/></cellXfs>\
         <cellStyles count='9' ><cellStyle name='Normal' xfId='0' builtinId='0'/></cellStyles>",
        with_collections("")
    );
    let got = added_styles(&body, |p| add_styled(p, None, &bold(), 1));
    assert!(
        got.contains(&format!(
            "<cellStyleXfs count='7'>{PLAIN_XF}</cellStyleXfs>"
        )),
        "{got}"
    );
    assert!(
        got.contains(
            "<cellStyles count='9' ><cellStyle name='Normal' xfId='0' builtinId='0'/></cellStyles>"
        ),
        "{got}"
    );
}

#[test]
fn self_closing_cell_style_xfs_and_cell_styles_are_opened_for_a_named_style() {
    // Ruling U, pinned by rulings AL and AU: both containers self-closing, so
    // the named style's index is 0 and each container is opened up, its
    // `count` rewritten where the tag has one and none added where it has
    // none, as the other grown containers' are.
    for count in ["", " count=\"0\""] {
        let body = format!(
            "{}<cellStyleXfs{count}/><cellXfs count=\"1\"><xf/></cellXfs><cellStyles{count}/>",
            with_collections("")
        );
        let got = added_styles(&body, |p| {
            add_named(p, "Mine", &Style::default(), 0);
            add_styled(p, None, &naming(0), 1);
        });
        let opened = count.replace('0', "1");
        let expected = body
            .replace(
                &format!("<cellStyleXfs{count}/>"),
                &format!("<cellStyleXfs{opened}>{PLAIN_XF}</cellStyleXfs>"),
            )
            .replace(
                "<cellXfs count=\"1\"><xf/></cellXfs>",
                "<cellXfs count=\"2\"><xf/><xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" \
                 borderId=\"0\" xfId=\"0\"/></cellXfs>",
            )
            .replace(
                &format!("<cellStyles{count}/>"),
                &format!("<cellStyles{opened}><cellStyle name=\"Mine\" xfId=\"0\"/></cellStyles>"),
            );
        assert_eq!(got, style_sheet(&expected), "count form {count:?}");
    }
}

#[cfg(feature = "rules")]
#[test]
fn a_differential_style_given_named_is_refused() {
    let (_, mut package) = opened(styled(&named_body(), ""));
    let style = Style {
        named: Some(0),
        ..bold()
    };
    assert_package(
        package.add_differential_style(&style).expect_err("refused"),
        "a differential style holds a field this part does not write",
    );
}

#[cfg(feature = "read")]
#[test]
fn a_written_named_style_reads_back_equal_and_its_cell_formats_name_it() {
    let style = Style {
        fill: Some(Fill {
            pattern: Pattern::Solid,
            foreground: Some(Color::Rgb(0xFFDD_EEFF)),
            background: None,
            gradient: None,
        }),
        ..bold()
    };
    let output = edited_output(styled(&named_body(), ""), |p| {
        add_named(p, "Mine", &style, 1);
        add_styled(p, None, &naming(1), 1);
        add_styled(p, Some(0), &naming(1), 2);
    });
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    let read = package.named_style(1).expect("read");
    assert_eq!(Some(&read.font), style.font.as_ref(), "the font");
    assert_eq!(read.fill.as_ref(), style.fill.as_ref(), "the fill");
    let styles = package.styles().expect("styles");
    assert_eq!(styles.cell_xfs[1].xf_id, Some(1));
    assert_eq!(styles.cell_xfs[2].xf_id, Some(1));
    let entry = styles.cell_styles.last().expect("an entry");
    assert_eq!((entry.name.as_str(), entry.xf_id), ("Mine", 1));
    assert_eq!(entry.builtin_id, None);
}
