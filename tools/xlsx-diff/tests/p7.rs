//! Milestone 1 part 7 P1 tests (`specs/0014-xml-comparison.md`): the `xml`
//! rows and the four `archive` rows of the names-only XML comparison. Every
//! archive is written byte by byte from fictional XML in a temporary
//! directory; nothing is committed. Each expected row is written out by hand.

mod common;

use std::fs;
use std::path::PathBuf;

use common::*;

/// The namespace the fixtures bind, a fictional URN.
const NS: &str = "urn:example:fictional";

/// A stored entry, written byte by byte.
fn entry(name: &str, data: &str) -> RawEntry {
    raw_stored(name, data.as_bytes())
}

/// Writes the two archives, one entry list each, and returns their paths.
fn pair(dir: &TempDir, input: &[RawEntry], output: &[RawEntry]) -> (PathBuf, PathBuf) {
    let (a, b) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(&a, raw_zip(&[], input, b"")).expect("write");
    fs::write(&b, raw_zip(&[], output, b"")).expect("write");
    (a, b)
}

/// Compares one entry named `name` holding `input` on one side and
/// `output` on the other, and returns the run. Asserts exit status 1 and an
/// empty stderr, since every pair here differs and no archive is unreadable.
fn one(label: &str, name: &str, input: &str, output: &str) -> Outcome {
    let dir = TempDir::new("xml");
    let (a, b) = pair(&dir, &[entry(name, input)], &[entry(name, output)]);
    let result = run(&a, &b);
    assert_eq!(result.status, 1, "{label}: exit status");
    assert_eq!(result.stderr, "", "{label}: stderr");
    result
}

/// The `xml` rows, whole, in order.
fn xml_rows(stdout: &str) -> Vec<String> {
    rows(stdout, "xml").iter().map(|c| c.join("\t")).collect()
}

/// A hand-written `xml` row.
fn row(name: &str, kind: &str, columns: &[&str]) -> String {
    let mut parts = vec!["xml", name, kind];
    parts.extend_from_slice(columns);
    parts.join("\t")
}

/// The four `xml_*` archive values, in order.
fn xml_archive(stdout: &str) -> [String; 4] {
    [
        "xml_compared",
        "xml_parse_errors",
        "xml_names_equal",
        "xml_values_equal",
    ]
    .map(|k| archive_value(stdout, k))
}

/// The rows of a pair where both sides parse with no raw value, the names
/// are equal and the values are as given.
fn equal_names(name: &str, values: [&str; 4]) -> Vec<String> {
    vec![
        row(name, "parse", &["ok", "ok", "", "0", "0"]),
        row(name, "values", &values),
    ]
}

#[test]
fn an_entry_equal_after_recompression_has_no_xml_row() {
    let data = format!(r#"<a xmlns="{NS}"><b c="1">x</b></a>"#);
    let dir = TempDir::new("recompressed");
    let deflated_entry = raw_deflated(
        "a.xml",
        data.as_bytes(),
        &stored_blocks(data.as_bytes(), &[]),
    );
    let (a, b) = pair(&dir, &[entry("a.xml", &data)], &[deflated_entry]);
    let result = run(&a, &b);
    assert_eq!(result.status, 1);
    assert_eq!(
        summary(&result.stdout),
        [("a.xml".to_string(), "recompressed".to_string())]
    );
    assert_eq!(xml_rows(&result.stdout), Vec::<String>::new());
    assert_eq!(xml_archive(&result.stdout), ["0", "0", "0", "0"]);
}

#[test]
fn elements_and_attributes_added_removed_and_changed() {
    let base = format!(r#"<a xmlns="{NS}"><b c="1">x</b><d/></a>"#);
    let e = format!("{{{NS}}}");
    let cases: Vec<(&str, String, Vec<String>)> = vec![
        (
            "one element added",
            format!(r#"<a xmlns="{NS}"><b c="1">x</b><d/><d/></a>"#),
            vec![
                row("a.xml", "parse", &["ok", "ok", "", "0", "0"]),
                row("a.xml", "element", &[&format!("{e}d"), "1", "2"]),
                row("a.xml", "values", &["2", "2", "0", "0"]),
            ],
        ),
        (
            "one element removed",
            format!(r#"<a xmlns="{NS}"><b c="1">x</b></a>"#),
            vec![
                row("a.xml", "parse", &["ok", "ok", "", "0", "0"]),
                row("a.xml", "element", &[&format!("{e}d"), "1", "0"]),
                row("a.xml", "values", &["2", "2", "0", "0"]),
            ],
        ),
        (
            "one attribute added",
            format!(r#"<a xmlns="{NS}"><b c="1">x</b><d f="2"/></a>"#),
            vec![
                row("a.xml", "parse", &["ok", "ok", "", "0", "0"]),
                row("a.xml", "attribute", &[&format!("{e}d@{{}}f"), "0", "1"]),
                row("a.xml", "values", &["2", "3", "0", "1"]),
            ],
        ),
        (
            "one attribute's value changed",
            format!(r#"<a xmlns="{NS}"><b c="9">x</b><d/></a>"#),
            equal_names("a.xml", ["2", "2", "1", "1"]),
        ),
        (
            "an element added and one of the same name removed",
            format!(r#"<a xmlns="{NS}"><d/><b c="1">x</b></a>"#),
            equal_names("a.xml", ["2", "2", "0", "0"]),
        ),
        (
            "all four at once",
            format!(r#"<a xmlns="{NS}"><b c="9" g="3">x</b><h/></a>"#),
            vec![
                row("a.xml", "parse", &["ok", "ok", "", "0", "0"]),
                row("a.xml", "element", &[&format!("{e}d"), "1", "0"]),
                row("a.xml", "element", &[&format!("{e}h"), "0", "1"]),
                row("a.xml", "attribute", &[&format!("{e}b@{{}}g"), "0", "1"]),
                row("a.xml", "values", &["2", "3", "1", "2"]),
            ],
        ),
    ];
    for (label, output, expected) in cases {
        let result = one(label, "a.xml", &base, &output);
        assert_eq!(xml_rows(&result.stdout), expected, "{label}");
    }
}

#[test]
fn spellings_that_mean_the_same_compare_equal() {
    let cases: Vec<(&str, String, String)> = vec![
        (
            "prefix choice",
            format!(r#"<p:a xmlns:p="{NS}"><p:b p:c="1">x</p:b></p:a>"#),
            format!(r#"<q:a xmlns:q="{NS}"><q:b q:c="1">x</q:b></q:a>"#),
        ),
        (
            "default namespace against a prefix",
            format!(r#"<a xmlns="{NS}"><b c="1">x</b></a>"#),
            format!(r#"<p:a xmlns:p="{NS}"><p:b c="1">x</p:b></p:a>"#),
        ),
        (
            "attribute order",
            r#"<a><b c="1" d="2" e="3"/></a>"#.to_string(),
            r#"<a><b e="3" c="1" d="2"/></a>"#.to_string(),
        ),
        (
            "whitespace-only text, a comment and a processing instruction added",
            r#"<a><b>x</b><c/></a>"#.to_string(),
            "<a>\n  <!-- fictional -->\n  <b>x</b>\t<?pi fictional?>\r\n  <c/>\n</a>".to_string(),
        ),
        (
            "an entity against a literal",
            r#"<a b="x&amp;y&#65;">p&lt;q&#x42;&apos;</a>"#.to_string(),
            r#"<a b="x&amp;yA">p&lt;qB'</a>"#.to_string(),
        ),
        (
            "a CDATA section against escaped text",
            r#"<a>x&lt;y&amp;z</a>"#.to_string(),
            r#"<a><![CDATA[x<y&z]]></a>"#.to_string(),
        ),
        (
            "a comment inside text is transparent",
            r#"<a>x<!--c-->y</a>"#.to_string(),
            r#"<a>xy</a>"#.to_string(),
        ),
        (
            "a raw CR LF against a raw LF",
            "<a>x\r\ny</a>".to_string(),
            "<a>x\ny</a>".to_string(),
        ),
        (
            "an empty-element tag against a start and end tag",
            r#"<a><b c="1"/></a>"#.to_string(),
            r#"<a><b c="1"></b></a>"#.to_string(),
        ),
    ];
    for (label, input, output) in cases {
        let result = one(label, "a.xml", &input, &output);
        let values = xml_rows(&result.stdout)
            .last()
            .cloned()
            .expect("a values row");
        let count = values
            .split('\t')
            .nth(3)
            .expect("values on the input")
            .to_string();
        assert_eq!(
            xml_rows(&result.stdout),
            equal_names("a.xml", [&count, &count, "0", "0"]),
            "{label}"
        );
        assert_ne!(count, "0", "{label}: precondition, the entry holds values");
        assert_eq!(xml_archive(&result.stdout), ["1", "0", "1", "1"], "{label}");
    }
}

#[test]
fn a_raw_cr_against_a_cr_reference_differs() {
    // A raw CR is a line end and reads as LF; `&#13;` is a CR character.
    let result = one("CR", "a.xml", "<a>x\ry</a>", "<a>x&#13;y</a>");
    assert_eq!(
        xml_rows(&result.stdout),
        equal_names("a.xml", ["1", "1", "1", "1"])
    );
    assert_eq!(xml_archive(&result.stdout), ["1", "0", "1", "0"]);
}

#[test]
fn an_unbound_name_against_a_bound_one() {
    let result = one(
        "unbound",
        "a.xml",
        r#"<a><b/></a>"#,
        &format!(r#"<a><b xmlns="{NS}"/></a>"#),
    );
    assert_eq!(
        xml_rows(&result.stdout),
        [
            row("a.xml", "parse", &["ok", "ok", "", "0", "0"]),
            row("a.xml", "element", &[&format!("{{{NS}}}b"), "0", "1"]),
            row("a.xml", "element", &["{}b", "1", "0"]),
            row("a.xml", "values", &["0", "0", "0", "0"]),
        ]
    );
    assert_eq!(xml_archive(&result.stdout), ["1", "0", "0", "1"]);
}

/// The `entry` row of the pair with each entry renamed to `.bin`, so that
/// no XML comparison runs, with the name column put back.
fn entry_row_without_xml(name: &str, input: &str, output: &str) -> Vec<String> {
    let bin = format!("{name}.bin");
    let dir = TempDir::new("bin");
    let (a, b) = pair(&dir, &[entry(&bin, input)], &[entry(&bin, output)]);
    let result = run(&a, &b);
    assert_eq!(xml_rows(&result.stdout), Vec::<String>::new());
    rows(&result.stdout, "entry")
        .iter()
        .map(|c| c.join("\t").replacen(&bin, name, 1))
        .collect()
}

#[test]
fn parse_errors_end_the_entry_at_its_parse_row() {
    let good = r#"<a><b c="1">x</b></a>"#;
    // An unknown prefix is at the buffer position after its tag: `<a><p:b/>`
    // ends at byte 9. An element still open at the end of input is at the
    // end: 7 bytes. An unclosed tag at the end of input is a syntax error at
    // the `<` that opens it (`error_position()`'s documentation): byte 3.
    let cases: Vec<(&str, &str, &str, [&str; 5])> = vec![
        (
            "unknown prefix on the input",
            "<a><p:b/></a>",
            good,
            ["error", "ok", "9", "0", "0"],
        ),
        (
            "unknown prefix on the output",
            good,
            "<a><p:b/></a>",
            ["ok", "error", "9", "0", "0"],
        ),
        (
            "unknown attribute prefix",
            good,
            r#"<a><b p:c="1"/></a>"#,
            ["ok", "error", "15", "0", "0"],
        ),
        (
            "an element left open",
            good,
            "<a><b>x",
            ["ok", "error", "7", "0", "0"],
        ),
        (
            "malformed on the output",
            good,
            "<a><b",
            ["ok", "error", "3", "0", "0"],
        ),
        (
            "both sides, the input's offset",
            "<a><b>x",
            "<a><b",
            ["error", "error", "7", "0", "0"],
        ),
        (
            "raw values counted up to the failure",
            "<a>&fictional;<b/>",
            good,
            ["error", "ok", "18", "1", "0"],
        ),
    ];
    for (label, input, output, parse) in cases {
        let result = one(label, "a.xml", input, output);
        assert_eq!(
            xml_rows(&result.stdout),
            [row("a.xml", "parse", &parse)],
            "{label}"
        );
        assert_eq!(xml_archive(&result.stdout), ["1", "1", "0", "0"], "{label}");
        let entry_rows: Vec<String> = rows(&result.stdout, "entry")
            .iter()
            .map(|c| c.join("\t"))
            .collect();
        assert_eq!(
            entry_rows,
            entry_row_without_xml("a.xml", input, output),
            "{label}: the entry row is unchanged"
        );
    }
}

#[test]
fn a_reference_the_tool_does_not_know_is_hashed_raw() {
    let result = one(
        "unknown entity",
        "a.xml",
        r#"<a b="&fictional;">x&fictional;y</a>"#,
        r#"<a b="&fictional;">x&fictional;z&#0;</a>"#,
    );
    assert_eq!(
        xml_rows(&result.stdout),
        [
            row("a.xml", "parse", &["ok", "ok", "", "2", "2"]),
            row("a.xml", "values", &["2", "2", "1", "1"]),
        ]
    );
    assert_eq!(xml_archive(&result.stdout), ["1", "0", "1", "0"]);
}

#[test]
fn which_names_are_compared() {
    let (x, y) = ("<a>x</a>", "<a>y</a>");
    let dir = TempDir::new("names");
    let names = ["a.vml", "b.rels", "c.bin", "D.XML", "e.xml.txt", "f.Rels"];
    let input: Vec<RawEntry> = names.iter().map(|n| entry(n, x)).collect();
    let output: Vec<RawEntry> = names.iter().map(|n| entry(n, y)).collect();
    let (a, b) = pair(&dir, &input, &output);
    let result = run(&a, &b);
    assert_eq!(result.status, 1);
    let mut expected = Vec::new();
    for name in ["a.vml", "b.rels", "D.XML", "f.Rels"] {
        expected.extend(equal_names(name, ["1", "1", "1", "1"]));
    }
    assert_eq!(xml_rows(&result.stdout), expected);
    assert_eq!(xml_archive(&result.stdout), ["4", "0", "4", "0"]);
}

#[test]
fn dropped_and_added_xml_entries_have_no_xml_row() {
    let dir = TempDir::new("dropped-added");
    let (a, b) = pair(
        &dir,
        &[entry("dropped.xml", "<a>x</a>")],
        &[entry("added.xml", "<a>y</a>")],
    );
    let result = run(&a, &b);
    assert_eq!(result.status, 1);
    assert_eq!(
        summary(&result.stdout),
        [
            ("dropped.xml".to_string(), "dropped".to_string()),
            ("added.xml".to_string(), "added".to_string())
        ]
    );
    assert_eq!(xml_rows(&result.stdout), Vec::<String>::new());
    assert_eq!(xml_archive(&result.stdout), ["0", "0", "0", "0"]);
}

#[test]
fn one_hundred_thousand_values_differing_in_one() {
    // 50000 elements, each with one attribute and one text node.
    let document = |last: &str| {
        let mut data = String::from("<a>");
        for i in 0..50_000 {
            let text = if i == 49_999 {
                last.to_string()
            } else {
                i.to_string()
            };
            data.push_str(&format!(r#"<b c="{i}">{text}</b>"#));
        }
        data.push_str("</a>");
        data
    };
    let result = one("large", "a.xml", &document("x"), &document("y"));
    assert_eq!(
        xml_rows(&result.stdout),
        equal_names("a.xml", ["100000", "100000", "1", "1"])
    );
}

#[test]
fn archive_rows_with_two_compared_entries_one_failing() {
    let dir = TempDir::new("archive");
    let (a, b) = pair(
        &dir,
        &[
            entry("a.xml", "<a>x</a>"),
            entry("b.xml", "<a>x</a>"),
            entry("c.txt", "x"),
        ],
        &[
            entry("a.xml", "<a>y</a>"),
            entry("b.xml", "<a>x</b>"),
            entry("c.txt", "y"),
        ],
    );
    let result = run(&a, &b);
    assert_eq!(result.status, 1);
    assert_eq!(result.stderr, "");
    let mut expected = equal_names("a.xml", ["1", "1", "1", "1"]);
    // The end tag's error is reported at its `<`: byte 4.
    expected.push(row("b.xml", "parse", &["ok", "error", "4", "0", "0"]));
    assert_eq!(xml_rows(&result.stdout), expected);
    assert_eq!(xml_archive(&result.stdout), ["2", "1", "1", "0"]);
    // The xml rows stand after every entry row and before the archive rows.
    let kinds: Vec<&str> = result
        .stdout
        .lines()
        .map(|l| l.split('\t').next().expect("a kind"))
        .collect();
    let first_xml = kinds.iter().position(|k| *k == "xml").expect("an xml row");
    assert!(kinds[..first_xml].iter().all(|k| *k == "entry"));
    assert!(kinds[first_xml..first_xml + 3].iter().all(|k| *k == "xml"));
    assert!(kinds[first_xml + 3..].iter().all(|k| *k == "archive"));
}

#[test]
fn an_unreadable_archive_is_still_exit_2() {
    let dir = TempDir::new("unreadable");
    let (a, b) = (dir.path("in.zip"), dir.path("out.zip"));
    fs::write(&a, raw_zip(&[], &[entry("a.xml", "<a>x</a>")], b"")).expect("write");
    fs::write(&b, b"not a zip archive, fictional").expect("write");
    let result = run(&a, &b);
    assert_eq!(result.status, 2);
    assert_eq!(result.stdout, "");
    assert_eq!(
        result.stderr,
        "error: the output file is not a readable zip archive\n"
    );
}

/// The two sentinel strings, one per side, built from pieces so that the
/// whole strings do not appear in the source.
fn sentinels() -> [String; 2] {
    [
        ["zq", "-sent", "inel-", "m1p7-in"].concat(),
        ["zq", "-sent", "inel-", "m1p7-out"].concat(),
    ]
}

/// Asserts that no form of either sentinel appears in stdout or stderr: the
/// string itself, its hex digits, and its escaped-name form.
fn assert_no_value(label: &str, stdout: &[u8], stderr: &[u8]) {
    let hex = |s: &str| s.bytes().map(|b| format!("{b:02x}")).collect::<String>();
    for sentinel in sentinels() {
        let piece = &sentinel[..12];
        for (stream, bytes) in [("stdout", stdout), ("stderr", stderr)] {
            for needle in [sentinel.as_str(), piece, &hex(&sentinel)] {
                assert!(
                    !bytes.windows(needle.len()).any(|w| w == needle.as_bytes()),
                    "{label}: a sentinel appears in {stream}"
                );
            }
        }
    }
}

#[test]
fn no_value_in_any_output() {
    let [s_in, s_out] = sentinels();
    let doc = |s: &str| {
        format!(
            r#"<a xmlns="{NS}" b="{s}"><c d="{s}">{s}</c><e><![CDATA[{s}]]></e>&fictional;{s}</a>"#
        )
    };
    let cases: Vec<(&str, String, String, u8)> = vec![
        ("values differ", doc(&s_in), doc(&s_out), 1),
        ("output malformed", doc(&s_in), format!("<a>{s_out}<b"), 1),
        (
            "input unknown prefix",
            format!("<p:a>{s_in}</p:a>"),
            doc(&s_out),
            1,
        ),
        (
            "values the same",
            doc(&s_in),
            doc(&s_in).replace("<c ", "<c  "),
            1,
        ),
    ];
    let dir = TempDir::new("no-value");
    for (label, input, output, status) in cases {
        let (a, b) = pair(&dir, &[entry("a.xml", &input)], &[entry("a.xml", &output)]);
        let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
        let got = xlsx_diff::run(&a, &b, &mut stdout, &mut stderr);
        assert_eq!(got, status, "{label}");
        let text = String::from_utf8(stdout.clone()).expect("UTF-8");
        assert!(
            !xml_rows(&text).is_empty(),
            "{label}: precondition, xml rows"
        );
        assert_no_value(label, &stdout, &stderr);
    }
    // Control: the check sees a sentinel when one is there.
    let planted = format!("xml\ta.xml\t{s_in}\n");
    let caught = std::panic::catch_unwind(|| assert_no_value("control", planted.as_bytes(), b""));
    assert!(caught.is_err(), "control: a planted sentinel is seen");
}
