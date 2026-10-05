//! Rich comments on byte-built worksheets (`specs/0024-rich-text.md`, "P2:
//! the comment runs", "Comments", and P1 verification ruling 4): each test
//! builds the smallest package or the comments tests' fixtures, records a
//! comment of runs or reads one, and compares the written parts with the
//! bytes the design gives or the read with the runs the part holds. The
//! Cargo features `comments` and `richtext` together.

use super::*;
use crate::cells::{RichText, Run};
use crate::styles::{Color, Font};

/// A run of `text` with `font`.
fn run(text: &str, font: Option<Font>) -> Run {
    Run {
        text: text.to_string(),
        font,
    }
}

/// A rich text of `runs`.
fn rich(runs: Vec<Run>) -> RichText {
    RichText { runs }
}

/// A bold font with nothing else.
fn bold() -> Font {
    Font {
        bold: true,
        ..Font::default()
    }
}

/// Records a comment by `author` of `text`'s runs on `cell` of the
/// worksheet.
#[track_caller]
fn add_rich(package: &mut Package<Cursor<Vec<u8>>>, cell: &str, author: &str, text: &RichText) {
    package
        .add_rich_comment(SHEET, at(cell), author, text)
        .expect("recorded");
}

/// A comment element of runs as the design writes one, with `prefix`: the
/// runs given as their bytes.
fn added_rich(prefix: &str, cell: &str, author: u32, runs: &str) -> String {
    format!(
        "<{prefix}comment ref=\"{cell}\" authorId=\"{author}\"><{prefix}text>{runs}\
         </{prefix}text></{prefix}comment>"
    )
}

/// The rich text the tool's sample writes: "emend:" bold and " rich
/// comment" with no font.
fn sample() -> RichText {
    rich(vec![
        run("emend:", Some(bold())),
        run(" rich comment", None),
    ])
}

/// The bytes of [`sample`]'s runs, with `prefix`.
fn sample_runs(prefix: &str) -> String {
    let p = prefix;
    format!(
        "<{p}r><{p}rPr><{p}b/></{p}rPr><{p}t>emend:</{p}t></{p}r>\
         <{p}r><{p}t xml:space=\"preserve\"> rich comment</{p}t></{p}r>"
    )
}

/// The comments of the worksheet of `output`, read back through the crate.
#[track_caller]
fn comments_back(output: Vec<u8>) -> Vec<CommentEntry> {
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    package.comments(SHEET).expect("the comments read")
}

/// A worksheet with both parts whose comments part binds the main namespace
/// to the prefix `x`: one comment by `Ann` on A1, its note in a VML part of
/// `idmap` 1.
fn prefixed_both() -> Parts {
    with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&format!(
            "{}{}",
            comments_rel("rId1", "../comments1.xml"),
            vml_rel("rId2", "../drawings/vmlDrawing1.vml")
        )),
        &[
            (
                "xl/comments1.xml",
                format!(
                    "{DECLARATION}\n<x:comments xmlns:x=\"{MAIN_NS}\"><x:authors>\
                     <x:author>Ann</x:author></x:authors><x:commentList>\
                     <x:comment ref=\"A1\" authorId=\"0\"><x:text><x:t>one</x:t></x:text>\
                     </x:comment></x:commentList></x:comments>"
                ),
                COMMENTS_TYPE,
            ),
            (
                "xl/drawings/vmlDrawing1.vml",
                excel_vml("1", &kept_note("_x0000_s1025", 0, 0)),
                VML_TYPE,
            ),
        ],
    )
}

#[test]
fn a_rich_comment_is_created_in_a_part_with_none_one_r_per_run() {
    let tahoma = Font {
        name: Some("Tahoma".to_string()),
        size: Some(9.0),
        bold: true,
        color: Some(Color::Indexed(81)),
        ..Font::default()
    };
    let text = rich(vec![
        run("one <two>", Some(tahoma)),
        run(" three", None),
        run("x", Some(Font::default())),
        run("tail ", None),
    ]);
    let parts = smallest();
    let output = edited_output(parts.clone(), |p| add_rich(p, "A1", "Ann & Co", &text));
    assert_untouched(&output, &parts, &[SHEET, b"[Content_Types].xml".as_slice()]);
    let comments = format!(
        "{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors><author>Ann &amp; Co</author>\
         </authors><commentList>{}</commentList></comments>",
        added_rich(
            "",
            "A1",
            0,
            "<r><rPr><b/><sz val=\"9\"/><color indexed=\"81\"/><rFont val=\"Tahoma\"/></rPr>\
             <t>one &lt;two&gt;</t></r><r><t xml:space=\"preserve\"> three</t></r>\
             <r><rPr/><t>x</t></r><r><t xml:space=\"preserve\">tail </t></r>"
        )
    );
    assert_added_part(&output, b"xl/comments1.xml", comments.as_bytes());
    let vml = created_vml(1, &note(VOX, 1025, 1, 0, 0));
    assert_added_part(&output, b"xl/drawings/vmlDrawing1.vml", vml.as_bytes());
    let back = comments_back(output);
    assert_eq!(back.len(), 1);
    assert_eq!(back[0].text, "one <two> threextail ", "the flattened text");
    assert_eq!(back[0].runs, Some(text), "the runs read back equal");
}

#[test]
fn a_rich_comment_is_appended_beside_kept_ones_which_stay_byte_identical() {
    // Kept: two comments by Ann and their notes in `idmap` 1. Added: a rich
    // comment by Bob on B5 and a plain one by Ann on D2, in record order.
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| {
        add_rich(p, "B5", "Bob", &sample());
        add(p, "D2", "Ann", "plain");
    });
    assert_untouched(
        &output,
        &parts,
        &[
            b"xl/comments1.xml".as_slice(),
            b"xl/drawings/vmlDrawing1.vml".as_slice(),
        ],
    );
    let comments = part_text(&parts, b"xl/comments1.xml")
        .replace("</authors>", "<author>Bob</author></authors>")
        .replace(
            "</commentList>",
            &format!(
                "{}{}</commentList>",
                added_rich("", "B5", 1, &sample_runs("")),
                added_comment("", "D2", 0, "plain", false)
            ),
        );
    assert_eq!(text_of(&output, b"xl/comments1.xml"), comments);
    let vml = part_text(&parts, b"xl/drawings/vmlDrawing1.vml").replace(
        "</xml>",
        &format!(
            "{}{}</xml>",
            note(VOX, 1027, 3, 4, 1),
            note(VOX, 1028, 4, 1, 3)
        ),
    );
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), vml);
    let back = comments_back(output);
    assert_eq!(
        back,
        [
            kept_entry("A1", "Ann", "one"),
            kept_entry("C3", "Ann", "two"),
            CommentEntry {
                runs: Some(sample()),
                ..added_entry("B5", "Bob", "emend: rich comment")
            },
            added_entry("D2", "Ann", "plain"),
        ]
    );
}

#[test]
fn a_prefixed_comments_part_gets_its_runs_in_its_prefix() {
    let parts = prefixed_both();
    let output = edited_output(parts.clone(), |p| add_rich(p, "B2", "Ann", &sample()));
    let comments = part_text(&parts, b"xl/comments1.xml").replace(
        "</x:commentList>",
        &format!(
            "{}</x:commentList>",
            added_rich("x:", "B2", 0, &sample_runs("x:"))
        ),
    );
    assert_eq!(text_of(&output, b"xl/comments1.xml"), comments);
    assert_eq!(comments_back(output)[1].runs, Some(sample()));
}

#[test]
fn the_read_gives_runs_for_a_comment_with_runs_and_none_for_a_plain_t() {
    // The corpus's shape: every run with an `<rPr>`, here with the children
    // Excel writes, `charset` among them, which the typed view drops.
    let corpus_run = |text: &str, bold: bool| {
        let b = if bold { "<b/>" } else { "" };
        format!(
            "<r><rPr>{b}<sz val=\"9\"/><color indexed=\"81\"/><rFont val=\"Tahoma\"/>\
             <family val=\"2\"/><charset val=\"1\"/></rPr><t xml:space=\"preserve\">{text}</t></r>"
        )
    };
    let font = |bold: bool| Font {
        name: Some("Tahoma".to_string()),
        size: Some(9.0),
        bold,
        color: Some(Color::Indexed(81)),
        family: Some(2),
        ..Font::default()
    };
    let body = format!(
        "<comment ref=\"A1\" authorId=\"0\"><text>{}{}</text></comment>\
         <comment ref=\"B2\" authorId=\"0\"><text><t>plain</t></text></comment>\
         <comment ref=\"C3\" authorId=\"0\"/>",
        corpus_run("Ann:", true),
        corpus_run("\nnote", false)
    );
    let parts = with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&comments_rel("rId1", "../comments1.xml")),
        &[(
            "xl/comments1.xml",
            comments_xml(&["Ann"], &body),
            COMMENTS_TYPE,
        )],
    );
    let (_, mut package) = opened(parts);
    assert_eq!(
        package.comments(SHEET).expect("read"),
        [
            CommentEntry {
                runs: Some(rich(vec![
                    run("Ann:", Some(font(true))),
                    run("\nnote", Some(font(false))),
                ])),
                ..entry("A1", "Ann", "Ann:\nnote")
            },
            entry("B2", "Ann", "plain"),
            entry("C3", "Ann", ""),
        ]
    );
}

#[test]
fn an_empty_run_in_a_comment_is_dropped_and_a_phonetic_run_is_text_alone() {
    // P1 verification ruling 4 on a comment: an `<r/>` and a run whose
    // `<t>` texts total empty are dropped, and a `text` whose every run is
    // empty reads with `runs` `None`. An `<rPh>`'s `<t>` stays in the text,
    // as it always has, and is not a run, as in a cell.
    let body = "<comment ref=\"A1\" authorId=\"0\"><text><r/><r><rPr><b/></rPr><t>a</t></r>\
                <r><rPr><i/></rPr><t></t></r><r><t>b</t></r></text></comment>\
                <comment ref=\"B2\" authorId=\"0\"><text><r/><r><rPr><b/></rPr><t/></r></text>\
                </comment>\
                <comment ref=\"C3\" authorId=\"0\"><text><r><t>ab</t></r>\
                <rPh sb=\"0\" eb=\"1\"><t>x</t></rPh><phoneticPr fontId=\"0\"/></text></comment>";
    let parts = with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&comments_rel("rId1", "../comments1.xml")),
        &[(
            "xl/comments1.xml",
            comments_xml(&["Ann"], body),
            COMMENTS_TYPE,
        )],
    );
    let (_, mut package) = opened(parts);
    assert_eq!(
        package.comments(SHEET).expect("read"),
        [
            CommentEntry {
                runs: Some(rich(vec![run("a", Some(bold())), run("b", None)])),
                ..entry("A1", "Ann", "ab")
            },
            entry("B2", "Ann", ""),
            CommentEntry {
                runs: Some(rich(vec![run("ab", None)])),
                ..entry("C3", "Ann", "abx")
            },
        ]
    );
}

#[test]
fn a_comment_s_run_properties_that_cannot_be_read_are_refused() {
    for rpr in ["<b val=\"yes\"/>", "<unknown/>", "<sz val=\"0\"/>"] {
        let body = format!(
            "<comment ref=\"A1\" authorId=\"0\"><text><r><rPr>{rpr}</rPr><t>a</t></r></text>\
             </comment>"
        );
        let parts = with_parts(
            sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
            Some(&comments_rel("rId1", "../comments1.xml")),
            &[(
                "xl/comments1.xml",
                comments_xml(&["Ann"], &body),
                COMMENTS_TYPE,
            )],
        );
        let (_, mut package) = opened(parts);
        assert_package(
            package.comments(SHEET).expect_err("refused"),
            "a run's properties cannot be read",
        );
    }
}

#[test]
fn an_unbound_prefix_on_a_comment_s_run_property_is_refused_by_the_prefix_rule() {
    // specs/0024 P3 verification ruling 4, through a comment: part 2's
    // prefix rule refuses the child before the run reader sees it; the
    // control binds the prefix and reaches the run reader's own refusal.
    for (rpr, message) in [
        (
            "<o:b/>",
            "an element or attribute uses a prefix its document does not bind",
        ),
        (
            "<o:b xmlns:o=\"urn:other\"/>",
            "a run's properties cannot be read",
        ),
    ] {
        let body = format!(
            "<comment ref=\"A1\" authorId=\"0\"><text><r><rPr>{rpr}</rPr><t>a</t></r></text>\
             </comment>"
        );
        let parts = with_parts(
            sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
            Some(&comments_rel("rId1", "../comments1.xml")),
            &[(
                "xl/comments1.xml",
                comments_xml(&["Ann"], &body),
                COMMENTS_TYPE,
            )],
        );
        let (_, mut package) = opened(parts);
        assert_package(package.comments(SHEET).expect_err("refused"), message);
    }
}

#[test]
fn the_call_refuses_each_rule_and_records_nothing() {
    let half = "t".repeat(16384);
    let cases: Vec<(CellRef, String, RichText, &str)> = vec![
        (
            at("A1"),
            "Ann".to_string(),
            rich(vec![]),
            "a comment text is not valid",
        ),
        (
            at("A1"),
            "Ann".to_string(),
            rich(vec![run("a", None), run("", Some(bold()))]),
            "a comment text is not valid",
        ),
        (
            at("A1"),
            "Ann".to_string(),
            rich(vec![run(&half, None), run(&half, Some(bold()))]),
            "a comment text is not valid",
        ),
        (
            at("A1"),
            "Ann".to_string(),
            rich(vec![run("a", None), run("b\u{1}", None)]),
            "a text holds a character XML cannot carry",
        ),
        (
            at("A1"),
            "Ann".to_string(),
            rich(vec![run(
                "a",
                Some(Font {
                    name: Some(String::new()),
                    ..Font::default()
                }),
            )]),
            "a font name is empty or holds a character XML cannot carry",
        ),
        (
            at("A1"),
            "Ann".to_string(),
            rich(vec![run(
                "a",
                Some(Font {
                    size: Some(0.0),
                    ..Font::default()
                }),
            )]),
            "a style field is out of range",
        ),
        (
            at("A1"),
            "Ann".to_string(),
            rich(vec![run(
                "a",
                Some(Font {
                    color: Some(Color::Theme {
                        index: 1,
                        tint: Some(1.5),
                    }),
                    ..Font::default()
                }),
            )]),
            "a style field is out of range",
        ),
        (
            at("A1"),
            String::new(),
            sample(),
            "a comment author is not valid",
        ),
        (
            at("A1"),
            "a".repeat(256),
            sample(),
            "a comment author is not valid",
        ),
        (
            CellRef {
                row: 1_048_577,
                col: 1,
            },
            "Ann".to_string(),
            sample(),
            "a row or column index or count is out of range",
        ),
    ];
    for (cell, author, text, message) in cases {
        let (_, mut package) = opened(smallest());
        assert_package(
            package
                .add_rich_comment(SHEET, cell, &author, &text)
                .expect_err("refused"),
            message,
        );
        let out = written(package);
        assert_untouched(&out, &smallest(), &[]);
    }
    // The limit itself is taken, counted over the runs together.
    let (_, mut package) = opened(smallest());
    package
        .add_rich_comment(
            SHEET,
            at("A1"),
            &"a".repeat(255),
            &rich(vec![run(&half, None), run(&"t".repeat(16383), None)]),
        )
        .expect("recorded");
}

#[test]
fn a_rich_comment_on_a_kept_comment_s_cell_replaces_its_element_and_its_shape() {
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| add_rich(p, "A1", "Ann", &sample()));
    let comments = part_text(&parts, b"xl/comments1.xml")
        .replace(&kept_comment("A1", 0, "one"), "")
        .replace(
            "</commentList>",
            &format!(
                "{}</commentList>",
                added_rich("", "A1", 0, &sample_runs(""))
            ),
        );
    assert_eq!(text_of(&output, b"xl/comments1.xml"), comments);
    let vml = part_text(&parts, b"xl/drawings/vmlDrawing1.vml")
        .replace(&kept_note("_x0000_s1025", 0, 0), "")
        .replace("</xml>", &format!("{}</xml>", note(VOX, 1027, 3, 0, 0)));
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), vml);
}

#[test]
fn a_rich_comment_is_removed_as_any_comment_is() {
    // Written beside the kept two, then removed from the reopened output:
    // the comments part and the VML part come back to the input's bytes.
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| add_rich(p, "B5", "Ann", &sample()));
    // Opened as a fixture, so a keep records the written output as the
    // removal's input.
    let mut package = open_fixture(output);
    package.remove_comment(SHEET, at("B5")).expect("recorded");
    let output = written(package);
    for name in [
        b"xl/comments1.xml".as_slice(),
        b"xl/drawings/vmlDrawing1.vml",
    ] {
        assert_eq!(text_of(&output, name), part_text(&parts, name));
    }
    assert_eq!(
        comments_back(output),
        [
            kept_entry("A1", "Ann", "one"),
            kept_entry("C3", "Ann", "two")
        ]
    );
}
