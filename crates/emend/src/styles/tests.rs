//! Byte-built styles parts for the styles reader.
//!
//! `specs/0012-styles.md`, "P1": every fixture is a real zip built byte by
//! byte through [`crate::testing`], and every case goes through
//! [`Package::styles`] or [`Package::workbook`] rather than [`Styles::read`]
//! alone, so the relationship, the entry and the parse are all covered by
//! the test that names the behaviour.
//!
//! The package scaffolding is this module's own, as `cells::tests` keeps its
//! own: the helpers of `package::tests` are private to that module. What is
//! shared is the zip builder.

use std::io::Cursor;

use super::*;
use crate::package::Package;
use crate::testing::{Build, Spec};

const TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const MAIN_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const MAIN_STRICT: &str = "http://purl.oclc.org/ooxml/spreadsheetml/main";
const ATTR_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const DECLARATION: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>";

/// A package whose workbook part carries `workbook_pr` before its
/// `<sheets>`, and whose styles part is `styles` behind a relationship to
/// `target`.
///
/// `styles` of `None` leaves the entry out; `target` of `None` leaves the
/// relationship out. They are separate because the two absences are
/// different cases: no relationship at all, and one naming no entry.
fn build(workbook_pr: &str, styles: Option<&str>, target: Option<&str>) -> Vec<u8> {
    let content_types = format!(
        "{DECLARATION}\n<Types xmlns=\"{TYPES_NS}\">\
         <Default Extension=\"rels\" \
         ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
         <Default Extension=\"xml\" ContentType=\"application/xml\"/></Types>"
    );
    let package_rels = format!(
        "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/officeDocument\" \
         Target=\"xl/workbook.xml\"/></Relationships>"
    );
    let workbook = format!(
        "{DECLARATION}\n<workbook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{workbook_pr}\
         <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets></workbook>"
    );
    let styles_rel = match target {
        Some(target) => {
            format!("<Relationship Id=\"rId2\" Type=\"{ATTR_NS}/styles\" Target=\"{target}\"/>")
        }
        None => String::new(),
    };
    let workbook_rels = format!(
        "{DECLARATION}\n<Relationships xmlns=\"{RELS_NS}\">\
         <Relationship Id=\"rId1\" Type=\"{ATTR_NS}/worksheet\" \
         Target=\"worksheets/sheet1.xml\"/>{styles_rel}</Relationships>"
    );
    let sheet = format!("{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"><sheetData/></worksheet>");
    let mut specs = vec![
        Spec::new(b"[Content_Types].xml", content_types.as_bytes()),
        Spec::new(b"_rels/.rels", package_rels.as_bytes()),
        Spec::new(b"xl/workbook.xml", workbook.as_bytes()),
        Spec::new(b"xl/_rels/workbook.xml.rels", workbook_rels.as_bytes()),
        Spec::new(b"xl/worksheets/sheet1.xml", sheet.as_bytes()),
    ];
    if let Some(body) = styles {
        specs.push(Spec::new(b"xl/styles.xml", body.as_bytes()).deflated());
    }
    Build::new(specs).bytes()
}

fn opened(
    workbook_pr: &str,
    styles: Option<&str>,
    target: Option<&str>,
) -> Package<Cursor<Vec<u8>>> {
    Package::open(Cursor::new(build(workbook_pr, styles, target))).expect("the fixture opens")
}

/// A styles part in the main namespace with `body` between its root tags.
fn part(body: &str) -> String {
    format!("{DECLARATION}\n<styleSheet xmlns=\"{MAIN_NS}\">{body}</styleSheet>")
}

/// What the package whose styles part holds `body` under a main-namespace
/// root reads as.
#[track_caller]
fn read(body: &str) -> Styles {
    read_part(&part(body))
}

/// What the package with the whole styles part `bytes` reads as.
#[track_caller]
fn read_part(bytes: &str) -> Styles {
    opened("", Some(bytes), Some("styles.xml"))
        .styles()
        .expect("the styles part reads")
        .clone()
}

/// What the package with the whole styles part `body` is refused with.
#[track_caller]
fn refused(body: &str) -> Error {
    match opened("", Some(body), Some("styles.xml")).styles() {
        Ok(styles) => panic!("not refused; read {styles:?}"),
        Err(error) => error,
    }
}

#[track_caller]
fn assert_package(error: Error, message: &str) {
    match error {
        Error::Package(what) => assert_eq!(what, message, "the message"),
        other => panic!("expected Error::Package({message:?}), got {other:?}"),
    }
}

/// A `cellXfs` holding `xfs`, with no `count`: the reader ignores it.
fn cell_xfs(xfs: &str) -> String {
    format!("<cellXfs>{xfs}</cellXfs>")
}

// ------------------------------------------------------- what is read

#[test]
fn the_five_ids_are_read_present_and_absent() {
    let styles = read(&cell_xfs(
        "<xf numFmtId=\"164\" fontId=\"1\" fillId=\"2\" borderId=\"3\" xfId=\"4\"/><xf/>",
    ));
    let ids = |xf: &Xf| {
        (
            xf.num_fmt_id,
            xf.font_id,
            xf.fill_id,
            xf.border_id,
            xf.xf_id,
        )
    };
    assert_eq!(styles.cell_xfs.len(), 2);
    assert_eq!(
        ids(&styles.cell_xfs[0]),
        (Some(164), Some(1), Some(2), Some(3), Some(4))
    );
    assert_eq!(ids(&styles.cell_xfs[1]), (None, None, None, None, None));
}

#[test]
fn num_fmts_are_read_in_order_with_their_codes_unescaped() {
    // An escaped double quote and an escaped `<`, the two escapes the corpus
    // writes in a code (`specs/0012-styles.md`, "Facts").
    let styles = read(
        "<numFmts count=\"2\"><numFmt numFmtId=\"164\" formatCode=\"0.0&quot;x&quot;\"/>\
         <numFmt numFmtId=\"165\" formatCode=\"[&lt;1]0;0\"/></numFmts>",
    );
    assert_eq!(
        styles.num_fmts,
        vec![
            NumFmt {
                id: 164,
                code: "0.0\"x\"".to_string()
            },
            NumFmt {
                id: 165,
                code: "[<1]0;0".to_string()
            },
        ]
    );
}

#[test]
fn a_code_reference_is_kept_and_a_literal_tab_becomes_a_space() {
    // P1 ruling 3: `formatCode` is normalised as an attribute value, so a
    // character reference stands for its character and a literal tab is a
    // space, as XML 1.0 section 3.3.3 says.
    let styles = read(
        "<numFmts><numFmt numFmtId=\"164\" formatCode=\"a&#9;b\"/>\
         <numFmt numFmtId=\"165\" formatCode=\"a\tb\"/></numFmts>",
    );
    assert_eq!(
        styles.num_fmts[0].code, "a\tb",
        "a reference is its character"
    );
    assert_eq!(
        styles.num_fmts[1].code, "a b",
        "a literal tab is normalised"
    );
}

#[test]
fn a_part_without_num_fmts_has_none() {
    let styles = read(&cell_xfs("<xf numFmtId=\"0\"/>"));
    assert!(styles.num_fmts.is_empty());
    assert_eq!(styles.cell_xfs.len(), 1);
}

#[test]
fn a_part_without_cell_xfs_has_none() {
    let styles = read("<numFmts><numFmt numFmtId=\"164\" formatCode=\"0\"/></numFmts>");
    assert_eq!(styles.num_fmts.len(), 1);
    assert!(styles.cell_xfs.is_empty());
}

#[test]
fn an_xf_keeps_its_own_bytes_with_children_and_empty() {
    // The bytes an addition copies (`specs/0012-styles.md`, "Module
    // `styles`: what is read"): start tag through end tag, children and the
    // whitespace between them included, or the one empty-element tag.
    let with_children = "<xf numFmtId=\"0\" applyAlignment=\"1\">\n  \
                         <alignment horizontal=\"center\"/><protection locked=\"0\"></protection></xf>";
    let empty = "<xf numFmtId=\"14\" fontId=\"0\" />";
    let styles = read(&cell_xfs(&format!("{with_children}{empty}")));
    assert_eq!(styles.cell_xfs[0].raw(), with_children.as_bytes());
    assert_eq!(styles.cell_xfs[1].raw(), empty.as_bytes());
}

#[test]
fn only_the_two_containers_children_are_read() {
    // A `numFmt` inside a `dxf` and an `xf` inside `cellStyleXfs` are other
    // tables' and not read; neither is an element of the same local name in
    // another namespace, nor anything inside an `extLst`.
    let styles = read(
        "<numFmts><numFmt numFmtId=\"164\" formatCode=\"0\"/></numFmts>\
         <cellStyleXfs count=\"1\"><xf numFmtId=\"1\"/></cellStyleXfs>\
         <cellXfs><xf numFmtId=\"2\"/><o:xf xmlns:o=\"urn:other\" numFmtId=\"3\"/></cellXfs>\
         <dxfs count=\"1\"><dxf><numFmt numFmtId=\"165\" formatCode=\"0.0\"/></dxf></dxfs>\
         <extLst><ext uri=\"urn:x\"><cellXfs><xf numFmtId=\"4\"/></cellXfs></ext></extLst>",
    );
    assert_eq!(styles.num_fmts.len(), 1);
    assert_eq!(styles.num_fmts[0].id, 164);
    assert_eq!(styles.cell_xfs.len(), 1);
    assert_eq!(styles.cell_xfs[0].num_fmt_id, Some(2));
}

#[test]
fn a_prefixed_root_and_the_strict_namespace_are_read() {
    let prefixed = format!(
        "{DECLARATION}\n<x:styleSheet xmlns:x=\"{MAIN_NS}\"><x:numFmts>\
         <x:numFmt numFmtId=\"164\" formatCode=\"0\"/></x:numFmts>\
         <x:cellXfs><x:xf numFmtId=\"164\"/></x:cellXfs></x:styleSheet>"
    );
    let styles = read_part(&prefixed);
    assert_eq!((styles.num_fmts.len(), styles.cell_xfs.len()), (1, 1));
    assert_eq!(styles.cell_xfs[0].raw(), b"<x:xf numFmtId=\"164\"/>");

    let strict = format!(
        "{DECLARATION}\n<styleSheet xmlns=\"{MAIN_STRICT}\">\
         <cellXfs><xf numFmtId=\"0\"/><xf numFmtId=\"14\"/></cellXfs></styleSheet>"
    );
    assert_eq!(read_part(&strict).cell_xfs.len(), 2);
}

#[test]
fn the_part_is_read_once_and_kept() {
    let mut package = opened("", Some(&part(&cell_xfs("<xf/>"))), Some("styles.xml"));
    let first = package.styles().expect("the first read").clone();
    let second = package.styles().expect("the second read").clone();
    assert_eq!(first, second);
}

// ---------------------------------------------------------- number_format

#[test]
fn number_format_answers_by_the_first_num_fmt_then_the_built_in() {
    let styles = read(&format!(
        "<numFmts><numFmt numFmtId=\"164\" formatCode=\"0.000\"/>\
         <numFmt numFmtId=\"14\" formatCode=\"d/m/yyyy\"/>\
         <numFmt numFmtId=\"164\" formatCode=\"a later duplicate\"/></numFmts>{}",
        cell_xfs("<xf numFmtId=\"164\"/><xf numFmtId=\"14\"/><xf numFmtId=\"22\"/><xf/>")
    ));
    // A custom id, the first numFmt with it.
    assert_eq!(
        styles.number_format(0),
        Some(NumberFormat::Code("0.000".to_string()))
    );
    // A built-in id a producer spelled out.
    assert_eq!(
        styles.number_format(1),
        Some(NumberFormat::Code("d/m/yyyy".to_string()))
    );
    // A built-in id with no numFmt.
    assert_eq!(styles.number_format(2), Some(NumberFormat::BuiltIn(22)));
    // An xf with no numFmtId is General.
    assert_eq!(styles.number_format(3), Some(NumberFormat::BuiltIn(0)));
    // Out of range.
    assert_eq!(styles.number_format(4), None);
    assert_eq!(styles.number_format(u32::MAX), None);
}

// --------------------------------------------------------------- date1904

#[test]
fn date1904_reads_one_and_true_and_nothing_else() {
    let date1904 = |workbook_pr: &str| {
        opened(workbook_pr, None, None)
            .workbook()
            .expect("the workbook reads")
            .date1904
    };
    assert!(date1904("<workbookPr date1904=\"1\"/>"));
    assert!(date1904(
        "<workbookPr date1904=\"true\" defaultThemeVersion=\"1\"/>"
    ));
    assert!(!date1904("<workbookPr date1904=\"0\"/>"));
    assert!(!date1904("<workbookPr date1904=\"false\"/>"));
    assert!(!date1904("<workbookPr defaultThemeVersion=\"1\"/>"));
    assert!(!date1904(""));
}

// --------------------------------------------------------------- refusals

#[test]
fn a_workbook_with_no_styles_relationship_is_refused() {
    let error = opened("", Some(&part("")), None).styles().unwrap_err();
    assert_package(error, "the workbook has no styles part");
}

#[test]
fn a_styles_relationship_naming_no_entry_is_refused() {
    let error = opened("", None, Some("styles.xml")).styles().unwrap_err();
    assert_package(error, "no such part");
}

#[test]
fn a_root_of_the_wrong_name_is_refused() {
    let body = format!("{DECLARATION}\n<styles xmlns=\"{MAIN_NS}\"/>");
    assert_package(refused(&body), "the styles part's root is not styleSheet");
}

#[test]
fn a_root_in_another_namespace_is_refused() {
    // Which is also what the part 2 fixtures' unnamespaced `<styleSheet/>`
    // is: a root this layer does not recognise.
    assert_package(
        refused("<styleSheet/>"),
        "the styles part's root is not styleSheet",
    );
}

#[test]
fn a_part_with_no_root_at_all_is_refused() {
    assert_package(
        refused(DECLARATION),
        "the styles part's root is not styleSheet",
    );
}

#[test]
fn a_num_fmt_without_an_id_is_refused() {
    let body = part("<numFmts><numFmt formatCode=\"0\"/></numFmts>");
    assert_package(refused(&body), "a numFmt lacks numFmtId or formatCode");
}

#[test]
fn a_num_fmt_without_a_code_is_refused() {
    let body = part("<numFmts><numFmt numFmtId=\"164\"/></numFmts>");
    assert_package(refused(&body), "a numFmt lacks numFmtId or formatCode");
}

#[test]
fn a_num_fmt_id_that_is_not_a_number_is_refused() {
    // `decimal`'s rule: digits alone, no sign, no leading zero.
    for id in ["x", "", "+164", "0164", "-1", "4294967296"] {
        let body = part(&format!(
            "<numFmts><numFmt numFmtId=\"{id}\" formatCode=\"0\"/></numFmts>"
        ));
        assert_package(refused(&body), "a style attribute is not a number");
    }
}

#[test]
fn an_xf_id_that_is_not_a_number_is_refused() {
    for attribute in ["numFmtId", "fontId", "fillId", "borderId", "xfId"] {
        let body = part(&cell_xfs(&format!("<xf {attribute}=\"one\"/>")));
        assert_package(refused(&body), "a style attribute is not a number");
    }
}

#[test]
fn a_second_num_fmts_is_refused() {
    // P1 ruling 4.
    let body = part(
        "<numFmts><numFmt numFmtId=\"164\" formatCode=\"0\"/></numFmts>\
         <numFmts/><cellXfs><xf/></cellXfs>",
    );
    assert_package(
        refused(&body),
        "the styles part has more than one numFmts, fonts, fills, borders or cellXfs",
    );
}

#[test]
fn a_second_cell_xfs_is_refused() {
    // P1 ruling 4: a second cellXfs would make every `s` ambiguous.
    let body = part("<cellXfs><xf/></cellXfs><cellStyles/><cellXfs><xf/></cellXfs>");
    assert_package(
        refused(&body),
        "the styles part has more than one numFmts, fonts, fills, borders or cellXfs",
    );
}

// ------------------------------------------ the counts (specs/0017, P1)

#[test]
fn the_template_part_counts_one_font_two_fills_and_one_border() {
    let mut package = Package::new().expect("the template opens");
    let styles = package.styles().expect("the template's styles part reads");
    assert_eq!(
        (styles.fonts(), styles.fills(), styles.borders()),
        (Some(1), Some(2), Some(1))
    );
}

#[test]
fn a_part_without_a_collection_counts_none_and_an_empty_one_zero() {
    let styles = read("<fills/><cellXfs><xf/></cellXfs>");
    assert_eq!(
        (styles.fonts(), styles.fills(), styles.borders()),
        (None, Some(0), None)
    );
}

// Without the Cargo feature `read`: with it, each counted child is read
// typed, and the `font` nested in a `font` here is refused
// (`specs/0027-read-back.md`, "Cell formats"; the read side is in
// `package/tests/readback.rs`).
#[cfg(not(feature = "read"))]
#[test]
fn the_counts_are_the_children_not_the_count_attribute() {
    // Only a main-namespace child of the container's own name counts, at
    // depth 3: not an element of another namespace, not a `font` inside a
    // `dxf` or an `extLst`, not a `font` nested in a `font`. Nothing inside a
    // counted child is read, so a child the reader would refuse elsewhere
    // passes.
    let styles = read(
        "<fonts count=\"9\"><font><b/><font/></font><o:font xmlns:o=\"urn:other\"/>\
         <font/></fonts>\
         <fills count=\"0\"><fill><patternFill patternType=\"none\"/></fill></fills>\
         <borders><border><left/></border><border/><border numFmtId=\"x\"/></borders>\
         <cellXfs><xf/></cellXfs>\
         <dxfs><dxf><font><b/></font><fill/><border/></dxf></dxfs>\
         <extLst><ext uri=\"urn:x\"><fonts><font/></fonts></ext></extLst>",
    );
    assert_eq!(
        (styles.fonts(), styles.fills(), styles.borders()),
        (Some(2), Some(1), Some(3))
    );
}

#[test]
fn the_counts_are_read_under_a_prefix_and_in_the_strict_namespace() {
    let prefixed = read_part(&format!(
        "{DECLARATION}\n<x:styleSheet xmlns:x=\"{MAIN_NS}\"><x:fonts><x:font/></x:fonts>\
         <x:cellXfs><x:xf/></x:cellXfs></x:styleSheet>"
    ));
    assert_eq!(prefixed.fonts(), Some(1));
    let strict = read_part(&format!(
        "{DECLARATION}\n<styleSheet xmlns=\"{MAIN_STRICT}\"><borders><border/><border/>\
         </borders><cellXfs><xf/></cellXfs></styleSheet>"
    ));
    assert_eq!(strict.borders(), Some(2));
}

#[test]
fn a_second_fonts_fills_or_borders_is_refused() {
    // `specs/0017-fuller-styles.md`, "Module `styles`: the counts".
    for twice in [
        "<fonts/><fonts/>",
        "<fills/><fills><fill/></fills>",
        "<borders/><borders/>",
    ] {
        assert_package(
            refused(&part(&format!("{twice}<cellXfs><xf/></cellXfs>"))),
            "the styles part has more than one numFmts, fonts, fills, borders or cellXfs",
        );
    }
}

#[test]
fn an_xf_child_with_an_unbound_prefix_is_refused() {
    // specs/0017-fuller-styles.md, P2 verification ruling 5: the reader
    // resolves each direct child of an xf, so an unbound prefix there refuses
    // the part; the same child under a bound prefix reads.
    assert_package(
        refused(&part("<cellXfs><xf><y:alignment/></xf></cellXfs>")),
        "an element or attribute uses a prefix its document does not bind",
    );
    let styles = read("<cellXfs><xf xmlns:y=\"urn:other\"><y:alignment/></xf></cellXfs>");
    assert_eq!(styles.cell_xfs.len(), 1);
}
