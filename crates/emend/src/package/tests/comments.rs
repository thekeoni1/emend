//! Cell comments on byte-built worksheets (`specs/0022-comments-and-
//! images.md`, "P1", and the P1 rulings): each test builds the smallest
//! package with its worksheet holding a given body and, where it says so, a
//! rels part, a comments part and VML parts, records comment additions, and
//! compares the written parts it names with the bytes the design gives; the
//! tests that call `assert_untouched` also assert every other part
//! untouched.

use super::*;

/// The content types of a comments part and of a VML part.
const COMMENTS_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml";
const VML_TYPE: &str = "application/vnd.openxmlformats-officedocument.vmlDrawing";

/// The three VML namespaces.
const V_NS: &str = "urn:schemas-microsoft-com:vml";
const O_NS: &str = "urn:schemas-microsoft-com:office:office";
const X_NS: &str = "urn:schemas-microsoft-com:office:excel";

/// A `comments` relationship and a `vmlDrawing` one.
fn comments_rel(id: &str, target: &str) -> String {
    relationship(id, &format!("{ATTR_NS}/comments"), target)
}

fn vml_rel(id: &str, target: &str) -> String {
    relationship(id, &format!("{ATTR_NS}/vmlDrawing"), target)
}

/// A worksheet part whose root binds `r` to the relationships namespace.
fn sheet_r(body: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{body}</worksheet>"
    )
}

/// The smallest package with the worksheet `sheet`, its rels part holding
/// `rels_body` when one is given, and each named part with its `Override`.
fn with_parts(
    sheet: String,
    rels_body: Option<&str>,
    parts_given: &[(&str, String, &str)],
) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet);
    if let Some(body) = rels_body {
        with_part(
            &mut parts,
            "xl/worksheets/_rels/sheet1.xml.rels",
            &rels(body),
            None,
        );
    }
    for (name, bytes, content_type) in parts_given {
        with_part(&mut parts, name, bytes, Some(content_type));
    }
    parts
}

/// A comments part of the fixtures: the authors and the comment elements as
/// given, in the default namespace.
fn comments_xml(authors: &[&str], comments: &str) -> String {
    let authors: String = authors
        .iter()
        .map(|author| format!("<author>{author}</author>"))
        .collect();
    format!(
        "{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors>{authors}</authors>\
         <commentList>{comments}</commentList></comments>"
    )
}

/// A kept comment as Excel writes one: a run with properties.
fn kept_comment(cell: &str, author: u32, text: &str) -> String {
    format!(
        "<comment ref=\"{cell}\" authorId=\"{author}\"><text><r><rPr><b/><sz val=\"9\"/>\
         <rFont val=\"Tahoma\"/></rPr><t>{text}</t></r></text></comment>"
    )
}

/// A kept note shape in Excel's form, with the prefixes `v`, `o` and `x`.
fn kept_note(id: &str, row: u32, column: u32) -> String {
    format!(
        "<v:shape id=\"{id}\" type=\"#_x0000_t202\" style='position:absolute'\n  \
         fillcolor=\"#ffffe1\" o:insetmode=\"auto\"><v:fill color2=\"#ffffe1\"/>\n  \
         <x:ClientData ObjectType=\"Note\"><x:MoveWithCells/><x:SizeWithCells/>\n   \
         <x:Row>{row}</x:Row>\n   <x:Column>{column}</x:Column>\n  </x:ClientData>\n </v:shape>"
    )
}

/// A VML part in Excel's form: no declaration, an `idmap` of `data`, the
/// shapetype `_x0000_t202` and the shapes given.
fn excel_vml(data: &str, shapes: &str) -> String {
    format!(
        "<xml xmlns:v=\"{V_NS}\"\n xmlns:o=\"{O_NS}\"\n xmlns:x=\"{X_NS}\">\n \
         <o:shapelayout v:ext=\"edit\">\n  <o:idmap v:ext=\"edit\" data=\"{data}\"/>\n \
         </o:shapelayout><v:shapetype id=\"_x0000_t202\" coordsize=\"21600,21600\" o:spt=\"202\"\n  \
         path=\"m,l,21600r21600,l21600,xe\">\n  <v:stroke joinstyle=\"miter\"/>\n \
         </v:shapetype>{shapes}</xml>"
    )
}

/// The shapetype a note names, with the prefixes given (each with its
/// colon), as the design writes it.
fn shapetype(v: &str, o: &str) -> String {
    format!(
        "<{v}shapetype id=\"_x0000_t202\" coordsize=\"21600,21600\" {o}spt=\"202\" \
         path=\"m,l,21600r21600,l21600,xe\"><{v}stroke joinstyle=\"miter\"/>\
         <{v}path gradientshapeok=\"t\" {o}connecttype=\"rect\"/></{v}shapetype>"
    )
}

/// A note shape as the design writes it for the zero-based `row` and
/// `column`, with the id number `number`, the z-index `k` and the prefixes
/// given (each with its colon).
fn note(prefixes: [&str; 3], number: u64, k: u64, row: u64, column: u64) -> String {
    let [v, o, x] = prefixes;
    let top = if row > 0 { row - 1 } else { row };
    format!(
        "<{v}shape id=\"_x0000_s{number}\" type=\"#_x0000_t202\" style=\"position:absolute;\
         margin-left:59.25pt;margin-top:1.5pt;width:108pt;height:59.25pt;z-index:{k};\
         visibility:hidden\" fillcolor=\"#ffffe1\" {o}insetmode=\"auto\">\
         <{v}fill color2=\"#ffffe1\"/><{v}shadow on=\"t\" color=\"black\" obscured=\"t\"/>\
         <{v}path {o}connecttype=\"none\"/><{v}textbox style=\"mso-direction-alt:auto\">\
         <div style=\"text-align:left\"/></{v}textbox><{x}ClientData ObjectType=\"Note\">\
         <{x}MoveWithCells/><{x}SizeWithCells/><{x}Anchor>{}, 15, {top}, 2, {}, 15, {}, 2\
         </{x}Anchor><{x}AutoFill>False</{x}AutoFill><{x}Row>{row}</{x}Row>\
         <{x}Column>{column}</{x}Column></{x}ClientData></{v}shape>",
        column + 1,
        column + 3,
        top + 4
    )
}

/// The prefixes Excel's form uses.
const VOX: [&str; 3] = ["v:", "o:", "x:"];

/// A created VML part as the design writes it: `idmap` `data` and the notes.
fn created_vml(data: u64, notes: &str) -> String {
    format!(
        "<xml xmlns:v=\"{V_NS}\" xmlns:o=\"{O_NS}\" xmlns:x=\"{X_NS}\"><o:shapelayout v:ext=\"edit\">\
         <o:idmap v:ext=\"edit\" data=\"{data}\"/></o:shapelayout>{}{notes}</xml>",
        shapetype("v:", "o:")
    )
}

/// A comment element as the design writes one, with `prefix`.
fn added_comment(prefix: &str, cell: &str, author: u32, text: &str, space: bool) -> String {
    let space = if space { " xml:space=\"preserve\"" } else { "" };
    format!(
        "<{prefix}comment ref=\"{cell}\" authorId=\"{author}\"><{prefix}text>\
         <{prefix}t{space}>{text}</{prefix}t></{prefix}text></{prefix}comment>"
    )
}

/// A stored part of the output, as text.
fn text_of(output: &[u8], name: &[u8]) -> String {
    String::from_utf8(stored_data(output, name)).expect("UTF-8")
}

/// The refusal a write of `edit` on `parts` gives, with no output.
#[track_caller]
fn refused_with(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(parts);
    edit(&mut package);
    refused(package)
}

/// Records a comment by `author` with `text` on `cell` of the worksheet.
#[track_caller]
fn add(package: &mut Package<Cursor<Vec<u8>>>, cell: &str, author: &str, text: &str) {
    package
        .add_comment(SHEET, at(cell), Comment { author, text })
        .expect("recorded");
}

/// The entry `comments` reads for `cell`.
fn entry(cell: &str, author: &str, text: &str) -> CommentEntry {
    CommentEntry {
        cell: at(cell),
        author: author.to_string(),
        text: text.to_string(),
    }
}

/// A worksheet with both parts in Excel's form: two comments by `Ann` on
/// A1 and C3, their notes `_x0000_s1025` and `_x0000_s1026` in a VML part of
/// `idmap` 1, `rId1` the comments and `rId2` the VML part.
fn excel_both() -> Parts {
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
                comments_xml(
                    &["Ann"],
                    &format!(
                        "{}{}",
                        kept_comment("A1", 0, "one"),
                        kept_comment("C3", 0, "two")
                    ),
                ),
                COMMENTS_TYPE,
            ),
            (
                "xl/drawings/vmlDrawing1.vml",
                excel_vml(
                    "1",
                    &format!(
                        "{}{}",
                        kept_note("_x0000_s1025", 0, 0),
                        kept_note("_x0000_s1026", 2, 2)
                    ),
                ),
                VML_TYPE,
            ),
        ],
    )
}

#[test]
fn the_comments_of_a_part_with_two_authors_runs_and_a_plain_t_and_of_one_with_none_are_read() {
    let comments = comments_xml(
        &["Ann", "Bob &amp; Co"],
        &format!(
            "{}<comment ref=\"B2\" authorId=\"1\"><text><t>plain &lt;text&gt;</t></text></comment>\
             <comment ref=\"C3\" authorId=\"0\"><text><r><rPr><b/></rPr><t>first </t></r>\
             <r><t xml:space=\"preserve\">and second</t></r></text></comment>\
             <comment ref=\"D4\" authorId=\"7\"><text><t>past the list</t></text></comment>",
            kept_comment("A1", 0, "one")
        ),
    );
    let parts = with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&comments_rel("rId1", "/xl/comments1.xml")),
        &[("xl/comments1.xml", comments, COMMENTS_TYPE)],
    );
    let (_, mut package) = opened(parts);
    assert_eq!(
        package.comments(SHEET).expect("read"),
        [
            entry("A1", "Ann", "one"),
            entry("B2", "Bob & Co", "plain <text>"),
            entry("C3", "Ann", "first and second"),
            entry("D4", "", "past the list"),
        ]
    );
    let (_, mut package) = opened(smallest());
    assert_eq!(package.comments(SHEET).expect("read"), []);
}

#[test]
fn a_read_refuses_what_it_cannot_read() {
    let message = "a comments part or its drawing cannot be read";
    let with_comments = |comments: String| {
        with_parts(
            sheet_r("<sheetData/>"),
            Some(&comments_rel("rId1", "../comments1.xml")),
            &[("xl/comments1.xml", comments, COMMENTS_TYPE)],
        )
    };
    let cases = [
        // A comment without `ref`, one whose `ref` does not parse, one
        // without `authorId`.
        with_comments(comments_xml(
            &["Ann"],
            "<comment authorId=\"0\"><text><t>x</t></text></comment>",
        )),
        with_comments(comments_xml(
            &["Ann"],
            "<comment ref=\"A0\" authorId=\"0\"><text><t>x</t></text></comment>",
        )),
        with_comments(comments_xml(
            &["Ann"],
            "<comment ref=\"A1\"><text><t>x</t></text></comment>",
        )),
        // A root without `commentList`.
        with_comments(format!(
            "{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors/></comments>"
        )),
        // A relationship whose part is missing.
        with_parts(
            sheet_r("<sheetData/>"),
            Some(&comments_rel("rId1", "../comments9.xml")),
            &[],
        ),
    ];
    for parts in cases {
        let (_, mut package) = opened(parts);
        assert_package(package.comments(SHEET).expect_err("refused"), message);
    }
}

#[test]
fn the_created_parts_bytes_and_a_text_with_leading_whitespace() {
    let parts = smallest();
    let output = edited_output(parts.clone(), |p| {
        add(p, "A1", "Ann & Co", "one <two>");
        add(p, "B5", "Ann & Co", " spaced");
    });
    assert_untouched(&output, &parts, &[SHEET, b"[Content_Types].xml".as_slice()]);
    let comments = format!(
        "{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors><author>Ann &amp; Co</author>\
         </authors><commentList>{}{}</commentList></comments>",
        added_comment("", "A1", 0, "one &lt;two&gt;", false),
        added_comment("", "B5", 0, " spaced", true)
    );
    assert_added_part(&output, b"xl/comments1.xml", comments.as_bytes());
    let vml = created_vml(
        1,
        &format!("{}{}", note(VOX, 1025, 1, 0, 0), note(VOX, 1026, 2, 4, 1)),
    );
    assert_added_part(&output, b"xl/drawings/vmlDrawing1.vml", vml.as_bytes());
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>")
    );
    assert_added_part(
        &output,
        SHEET_RELS,
        rels(&format!(
            "{}{}",
            comments_rel("rId1", "../comments1.xml"),
            vml_rel("rId2", "../drawings/vmlDrawing1.vml")
        ))
        .as_bytes(),
    );
    let types = part_text(&parts, b"[Content_Types].xml").replace(
        "</Types>",
        &format!(
            "<Override PartName=\"/xl/comments1.xml\" ContentType=\"{COMMENTS_TYPE}\"/>\
             <Override PartName=\"/xl/drawings/vmlDrawing1.vml\" ContentType=\"{VML_TYPE}\"/></Types>"
        ),
    );
    assert_eq!(text_of(&output, b"[Content_Types].xml"), types);
}

#[test]
fn an_addition_to_a_sheet_with_both_parts_appends_to_each_and_reuses_an_equal_author() {
    // Kept: two comments by Ann and the notes 1025 and 1026 in `idmap` 1.
    // Added: Ann on B5 (her index 0) and Bob on D2 (a new author, index 1),
    // the notes 1027 and 1028; the worksheet, its rels part and the content
    // types untouched.
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| {
        add(p, "B5", "Ann", "three");
        add(p, "D2", "Bob", "four");
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
                added_comment("", "B5", 0, "three", false),
                added_comment("", "D2", 1, "four", false)
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
    let (_, mut package) = opened(parts);
    add(&mut package, "B5", "Ann", "three");
    let reopened = written(package);
    let mut package = Package::open(Cursor::new(reopened)).expect("open the output");
    assert_eq!(
        package.comments(SHEET).expect("read back"),
        [
            entry("A1", "Ann", "one"),
            entry("C3", "Ann", "two"),
            entry("B5", "Ann", "three"),
        ]
    );
}

#[test]
fn libreoffice_s_form_takes_an_allocated_idmap_and_the_count_after_the_kept_shapes() {
    // The part has a declaration, `shape_<n>` ids and no `idmap`; another
    // VML part of the workbook uses `idmap` 1, so D is 2, and the kept
    // count 2 gives K 3: the note is `_x0000_s2051`.
    let libre = format!(
        "{DECLARATION}\n<xml xmlns:v=\"{V_NS}\" xmlns:o=\"{O_NS}\" xmlns:x=\"{X_NS}\">\
         {}{}{}</xml>",
        shapetype("v:", "o:"),
        kept_note("shape_0", 0, 0),
        kept_note("shape_1", 1, 1)
    );
    let parts = with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&format!(
            "{}{}",
            comments_rel("rId1", "../comments1.xml"),
            vml_rel("rId2", "../drawings/vmlDrawing1.vml")
        )),
        &[
            (
                "xl/comments1.xml",
                comments_xml(
                    &["Ann"],
                    &format!(
                        "{}{}",
                        kept_comment("A1", 0, "one"),
                        kept_comment("B2", 0, "two")
                    ),
                ),
                COMMENTS_TYPE,
            ),
            ("xl/drawings/vmlDrawing1.vml", libre, VML_TYPE),
            ("xl/drawings/vmlDrawing9.vml", excel_vml("1", ""), VML_TYPE),
        ],
    );
    let output = edited_output(parts.clone(), |p| add(p, "C3", "Ann", "three"));
    let vml = part_text(&parts, b"xl/drawings/vmlDrawing1.vml")
        .replace("</xml>", &format!("{}</xml>", note(VOX, 2051, 3, 2, 2)));
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), vml);
}

#[test]
fn a_kept_number_above_the_kept_count_wins_and_one_outside_the_range_does_not() {
    // Two kept notes, 1030 and 3000 in `idmap` 1: 3000 lies outside 1025 to
    // 2048, so the largest in the range gives K 7 and the note 1031.
    let parts = with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&format!(
            "{}{}",
            comments_rel("rId1", "../comments1.xml"),
            vml_rel("rId2", "../drawings/vmlDrawing1.vml")
        )),
        &[
            (
                "xl/comments1.xml",
                comments_xml(
                    &["Ann"],
                    &format!(
                        "{}{}",
                        kept_comment("A1", 0, "one"),
                        kept_comment("B2", 0, "two")
                    ),
                ),
                COMMENTS_TYPE,
            ),
            (
                "xl/drawings/vmlDrawing1.vml",
                excel_vml(
                    "1",
                    &format!(
                        "{}{}",
                        kept_note("_x0000_s1030", 0, 0),
                        kept_note("_x0000_s3000", 1, 1)
                    ),
                ),
                VML_TYPE,
            ),
        ],
    );
    let output = edited_output(parts.clone(), |p| add(p, "C3", "Ann", "three"));
    let vml = part_text(&parts, b"xl/drawings/vmlDrawing1.vml")
        .replace("</xml>", &format!("{}</xml>", note(VOX, 1031, 7, 2, 2)));
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), vml);
}

#[test]
fn a_sheet_of_form_controls_gains_a_comments_part_and_a_note_past_a_control_s_spid() {
    // The VML part holds two controls and no `_x0000_t202`: a scroll bar
    // whose number is on `o:spid` (1030) and a spinner `_x0000_s1025`; the
    // comments part is created, the shapetype and the note 1031 appended,
    // `idmap` 1 read, the worksheet untouched.
    let controls = format!(
        "{}{}",
        "<v:shape id=\"Scroll Bar 1\" o:spid=\"_x0000_s1030\" type=\"#_x0000_t201\">\
         <x:ClientData ObjectType=\"Scroll\"><x:Row>4</x:Row><x:Column>4</x:Column>\
         </x:ClientData></v:shape>",
        "<v:shape id=\"_x0000_s1025\" type=\"#_x0000_t201\"><x:ClientData ObjectType=\"Spin\">\
         </x:ClientData></v:shape>"
    );
    let vml = format!(
        "<xml xmlns:v=\"{V_NS}\" xmlns:o=\"{O_NS}\" xmlns:x=\"{X_NS}\"><o:shapelayout v:ext=\"edit\">\
         <o:idmap v:ext=\"edit\" data=\"1\"/></o:shapelayout>{controls}</xml>"
    );
    let parts = with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId1\"/>"),
        Some(&vml_rel("rId1", "../drawings/vmlDrawing1.vml")),
        &[("xl/drawings/vmlDrawing1.vml", vml, VML_TYPE)],
    );
    let output = edited_output(parts.clone(), |p| add(p, "E5", "Ann", "note"));
    assert_untouched(
        &output,
        &parts,
        &[
            SHEET_RELS,
            b"xl/drawings/vmlDrawing1.vml".as_slice(),
            b"[Content_Types].xml".as_slice(),
        ],
    );
    let comments = format!(
        "{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors><author>Ann</author></authors>\
         <commentList>{}</commentList></comments>",
        added_comment("", "E5", 0, "note", false)
    );
    assert_added_part(&output, b"xl/comments1.xml", comments.as_bytes());
    let want = part_text(&parts, b"xl/drawings/vmlDrawing1.vml").replace(
        "</xml>",
        &format!(
            "{}{}</xml>",
            shapetype("v:", "o:"),
            note(VOX, 1031, 7, 4, 4)
        ),
    );
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), want);
    assert_eq!(
        text_of(&output, SHEET_RELS),
        rels(&format!(
            "{}{}",
            vml_rel("rId1", "../drawings/vmlDrawing1.vml"),
            comments_rel("rId2", "../comments1.xml")
        ))
    );
}

#[test]
fn a_vml_part_with_no_declaration_and_prefixes_of_its_own_takes_them() {
    let own = format!(
        "<xml xmlns:vv=\"{V_NS}\" xmlns:oo=\"{O_NS}\" xmlns:xx=\"{X_NS}\"><oo:shapelayout vv:ext=\"edit\">\
         <oo:idmap vv:ext=\"edit\" data=\"3\"/></oo:shapelayout>\
         <vv:shape id=\"_x0000_s3073\"><xx:ClientData ObjectType=\"Note\"><xx:Row>0</xx:Row>\
         <xx:Column>0</xx:Column></xx:ClientData></vv:shape></xml>"
    );
    let parts = with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&format!(
            "{}{}",
            comments_rel("rId1", "../comments1.xml"),
            vml_rel("rId2", "../drawings/vmlDrawing1.vml")
        )),
        &[
            (
                "xl/comments1.xml",
                comments_xml(&["Ann"], &kept_comment("A1", 0, "one")),
                COMMENTS_TYPE,
            ),
            ("xl/drawings/vmlDrawing1.vml", own, VML_TYPE),
        ],
    );
    let output = edited_output(parts.clone(), |p| add(p, "B2", "Ann", "two"));
    let want = part_text(&parts, b"xl/drawings/vmlDrawing1.vml").replace(
        "</xml>",
        &format!(
            "{}{}</xml>",
            shapetype("vv:", "oo:"),
            note(["vv:", "oo:", "xx:"], 3074, 2, 1, 1)
        ),
    );
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), want);
}

#[test]
fn the_template_part_is_retagged_and_its_parts_and_rels_part_created() {
    let mut package = new_package();
    package
        .add_comment(
            SHEET,
            at("A1"),
            Comment {
                author: "emend",
                text: "a note",
            },
        )
        .expect("recorded");
    let out = {
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = Vec::new();
        package.write_to(&mut out).expect("write_to");
        out
    };
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    let read = |package: &mut Package<Cursor<Vec<u8>>>, name: &[u8]| {
        let index = package.part(name).expect("the part");
        let mut text = String::new();
        package
            .read_part(index)
            .expect("read_part")
            .read_to_string(&mut text)
            .expect("UTF-8");
        text
    };
    let sheet = read(&mut reopened, SHEET);
    println!("template worksheet after the comment: {sheet}");
    let template = String::from_utf8(TEMPLATE[4].1.to_vec()).expect("UTF-8");
    let want = template
        .replace(
            &format!("<worksheet xmlns=\"{MAIN_NS}\">"),
            &format!("<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">"),
        )
        .replace("</worksheet>", "<legacyDrawing r:id=\"rId2\"/></worksheet>");
    assert_ne!(want, template, "the template's root was found");
    assert_eq!(sheet, want);
    let rels_text = read(&mut reopened, SHEET_RELS);
    println!("template worksheet's created rels part: {rels_text}");
    assert!(
        rels_text.ends_with(&format!(
            "<Relationships xmlns=\"{RELS_NS}\">{}{}</Relationships>",
            comments_rel("rId1", "../comments1.xml"),
            vml_rel("rId2", "../drawings/vmlDrawing1.vml")
        )),
        "{rels_text}"
    );
    assert_eq!(
        read(&mut reopened, b"xl/drawings/vmlDrawing1.vml"),
        created_vml(1, &note(VOX, 1025, 1, 0, 0))
    );
    assert_eq!(
        reopened.comments(SHEET).expect("read back"),
        [entry("A1", "emend", "a note")]
    );
}

#[test]
fn the_element_is_created_before_table_parts_and_ext_lst() {
    let parts = with_parts(
        sheet_r(
            "<sheetData/><tableParts count=\"1\"><tablePart r:id=\"rId1\"/></tableParts>\
             <extLst><ext uri=\"x\"/></extLst>",
        ),
        Some(&relationship(
            "rId1",
            &format!("{ATTR_NS}/table"),
            "../tables/table1.xml",
        )),
        &[],
    );
    let output = edited_output(parts, |p| add(p, "A1", "Ann", "one"));
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><legacyDrawing r:id=\"rId3\"/><tableParts count=\"1\">\
             <tablePart r:id=\"rId1\"/></tableParts><extLst><ext uri=\"x\"/></extLst>"
        )
    );
    let parts = with_parts(
        sheet_r("<sheetData/><extLst><ext uri=\"x\"/></extLst>"),
        None,
        &[],
    );
    let output = edited_output(parts, |p| add(p, "A1", "Ann", "one"));
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/><extLst><ext uri=\"x\"/></extLst>")
    );
}

#[test]
fn a_comment_on_a_kept_comment_s_cell_replaces_its_element_and_its_shape() {
    // A1's comment and note go; the addition goes at the end, numbered past
    // the kept note 1026: K 3, the note 1027.
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| add(p, "A1", "Ann", "new"));
    let comments = part_text(&parts, b"xl/comments1.xml")
        .replace(&kept_comment("A1", 0, "one"), "")
        .replace(
            "</commentList>",
            &format!("{}</commentList>", added_comment("", "A1", 0, "new", false)),
        );
    assert_eq!(text_of(&output, b"xl/comments1.xml"), comments);
    let vml = part_text(&parts, b"xl/drawings/vmlDrawing1.vml")
        .replace(&kept_note("_x0000_s1025", 0, 0), "")
        .replace("</xml>", &format!("{}</xml>", note(VOX, 1027, 3, 0, 0)));
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), vml);
}

#[test]
fn of_two_additions_on_one_cell_the_last_wins_in_its_place() {
    let output = edited_output(smallest(), |p| {
        add(p, "A1", "Ann", "first");
        add(p, "B1", "Bob", "other");
        add(p, "A1", "Ann", "second");
    });
    let comments = format!(
        "{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors><author>Bob</author>\
         <author>Ann</author></authors><commentList>{}{}</commentList></comments>",
        added_comment("", "B1", 0, "other", false),
        added_comment("", "A1", 1, "second", false)
    );
    assert_added_part(&output, b"xl/comments1.xml", comments.as_bytes());
    let vml = created_vml(
        1,
        &format!("{}{}", note(VOX, 1025, 1, 0, 1), note(VOX, 1026, 2, 0, 0)),
    );
    assert_added_part(&output, b"xl/drawings/vmlDrawing1.vml", vml.as_bytes());
}

#[test]
fn the_names_and_the_idmap_skip_what_is_taken() {
    // `xl/comments1.xml` (unreferenced) and `xl/drawings/VMLDRAWING1.vml`,
    // `idmap` 1, are taken: the created parts are the 2s and D is 2.
    let mut parts = smallest();
    with_part(&mut parts, "xl/comments1.xml", "<x/>", None);
    with_part(
        &mut parts,
        "xl/drawings/VMLDRAWING1.vml",
        &excel_vml("1", ""),
        None,
    );
    let output = edited_output(parts, |p| add(p, "A1", "Ann", "one"));
    assert_added_part(
        &output,
        b"xl/drawings/vmlDrawing2.vml",
        created_vml(2, &note(VOX, 2049, 1, 0, 0)).as_bytes(),
    );
    assert_added_part(
        &output,
        b"xl/comments2.xml",
        format!(
            "{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors><author>Ann</author></authors>\
             <commentList>{}</commentList></comments>",
            added_comment("", "A1", 0, "one", false)
        )
        .as_bytes(),
    );
}

#[test]
fn a_sheet_with_threaded_comments_is_refused() {
    let parts = with_parts(
        sheet_r("<sheetData/>"),
        Some(&relationship(
            "rId1",
            "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
            "../threadedComments/threadedComment1.xml",
        )),
        &[],
    );
    assert_package(
        refused_with(parts, |p| add(p, "A1", "Ann", "one")),
        "a worksheet has threaded comments",
    );
}

#[test]
fn two_legacy_drawings_or_one_out_of_schema_order_are_refused() {
    let message = "a worksheet has two legacyDrawing elements";
    let rels_body = vml_rel("rId1", "../drawings/vmlDrawing1.vml");
    for body in [
        "<sheetData/><legacyDrawing r:id=\"rId1\"/><legacyDrawing r:id=\"rId1\"/>",
        "<sheetData/><tableParts count=\"0\"/><legacyDrawing r:id=\"rId1\"/>",
    ] {
        let parts = with_parts(
            sheet_r(body),
            Some(&rels_body),
            &[("xl/drawings/vmlDrawing1.vml", excel_vml("1", ""), VML_TYPE)],
        );
        assert_package(refused_with(parts, |p| add(p, "A1", "Ann", "one")), message);
    }
}

#[test]
fn the_write_refuses_parts_it_cannot_read() {
    let message = "a comments part or its drawing cannot be read";
    let both = |comments: String, vml: String| {
        with_parts(
            sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
            Some(&format!(
                "{}{}",
                comments_rel("rId1", "../comments1.xml"),
                vml_rel("rId2", "../drawings/vmlDrawing1.vml")
            )),
            &[
                ("xl/comments1.xml", comments, COMMENTS_TYPE),
                ("xl/drawings/vmlDrawing1.vml", vml, VML_TYPE),
            ],
        )
    };
    let one = || comments_xml(&["Ann"], &kept_comment("A1", 0, "one"));
    let cases = [
        // A comments part on a sheet with no `legacyDrawing`.
        with_parts(
            sheet_r("<sheetData/>"),
            Some(&comments_rel("rId1", "../comments1.xml")),
            &[("xl/comments1.xml", one(), COMMENTS_TYPE)],
        ),
        // A comments relationship whose part is missing.
        with_parts(
            sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
            Some(&format!(
                "{}{}",
                comments_rel("rId1", "../comments9.xml"),
                vml_rel("rId2", "../drawings/vmlDrawing1.vml")
            )),
            &[("xl/drawings/vmlDrawing1.vml", excel_vml("1", ""), VML_TYPE)],
        ),
        // A `legacyDrawing` whose relationship is missing, and one whose part
        // is.
        with_parts(
            sheet_r("<sheetData/><legacyDrawing r:id=\"rId9\"/>"),
            None,
            &[],
        ),
        with_parts(
            sheet_r("<sheetData/><legacyDrawing r:id=\"rId1\"/>"),
            Some(&vml_rel("rId1", "../drawings/vmlDrawing9.vml")),
            &[],
        ),
        // A root without `commentList`, a kept comment without `ref`.
        both(
            format!("{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors/></comments>"),
            excel_vml("1", &kept_note("_x0000_s1025", 0, 0)),
        ),
        both(
            comments_xml(&["Ann"], "<comment authorId=\"0\"/>"),
            excel_vml("1", &kept_note("_x0000_s1025", 0, 0)),
        ),
        // A VML part with no shape where the comments part holds a comment.
        both(one(), excel_vml("1", "")),
        // A VML root binding no prefix to the excel namespace, and one whose
        // end tag is not there.
        both(
            one(),
            format!(
                "<xml xmlns:v=\"{V_NS}\" xmlns:o=\"{O_NS}\">{}</xml>",
                "<v:shape id=\"_x0000_s1025\"/>"
            ),
        ),
        both(
            one(),
            format!("<xml xmlns:v=\"{V_NS}\" xmlns:o=\"{O_NS}\" xmlns:x=\"{X_NS}\"><v:shape>"),
        ),
    ];
    for parts in cases {
        assert_package(refused_with(parts, |p| add(p, "B2", "Ann", "two")), message);
    }
}

#[test]
fn the_call_refuses_each_rule_and_records_nothing() {
    let long_author = "a".repeat(256);
    let long_text = "t".repeat(32768);
    let cases: [(CellRef, &str, &str, &str); 6] = [
        (at("A1"), "", "x", "a comment author is not valid"),
        (at("A1"), &long_author, "x", "a comment author is not valid"),
        (at("A1"), "Ann", "", "a comment text is not valid"),
        (at("A1"), "Ann", &long_text, "a comment text is not valid"),
        (
            at("A1"),
            "Ann",
            "a\u{1}b",
            "a text holds a character XML cannot carry",
        ),
        (
            CellRef {
                row: 1_048_577,
                col: 1,
            },
            "Ann",
            "x",
            "a row or column index or count is out of range",
        ),
    ];
    for (cell, author, text, message) in cases {
        let (_, mut package) = opened(smallest());
        assert_package(
            package
                .add_comment(SHEET, cell, Comment { author, text })
                .expect_err("refused"),
            message,
        );
        let out = written(package);
        assert_untouched(&out, &smallest(), &[]);
    }
    // The limits themselves are taken.
    let (_, mut package) = opened(smallest());
    package
        .add_comment(
            SHEET,
            at("A1"),
            Comment {
                author: &"a".repeat(255),
                text: &"t".repeat(32767),
            },
        )
        .expect("recorded");
}

#[test]
fn a_comment_on_a_removed_replaced_or_added_part_is_refused() {
    let (_, mut package) = opened(smallest());
    add(&mut package, "A1", "Ann", "one");
    package.remove_part(SHEET);
    assert_package(refused(package), "an edit to a removed part");
    let (_, mut package) = opened(smallest());
    add(&mut package, "A1", "Ann", "one");
    package.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
    assert_package(
        refused(package),
        "a cell edit to a part this write replaces",
    );
    let (_, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add_sheet");
    package
        .add_comment(
            &part,
            at("A1"),
            Comment {
                author: "Ann",
                text: "one",
            },
        )
        .expect("recorded");
    assert_package(refused(package), "no such part");
}

#[cfg(feature = "rowcol")]
#[test]
fn a_comment_with_a_row_insertion_is_refused() {
    assert_package(
        refused_with(smallest(), |p| {
            p.insert_rows(SHEET, 20, 1).expect("recorded");
            add(p, "A1", "Ann", "one");
        }),
        "a worksheet with a row or column insertion or deletion takes no other edit in the same write",
    );
}

/// The refusal of a comment on B5 of `parts` after `edit`, which removes,
/// replaces or adds the comments part or the VML part (P1 verification
/// ruling 1 (2)).
#[track_caller]
fn refused_after(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    refused_with(parts, |p| {
        edit(p);
        add(p, "B5", "Ann", "three");
    })
}

/// [`excel_both`] with the comments part, or the VML part, left out, so the
/// relationship names an entry the archive lacks.
fn excel_both_without(name: &[u8]) -> Parts {
    let mut parts = excel_both();
    drop_part(&mut parts, name);
    parts
}

#[test]
fn a_comment_on_a_sheet_whose_comments_part_this_write_removes_is_refused() {
    assert_package(
        refused_after(excel_both(), |p| p.remove_part(b"xl/comments1.xml")),
        "an edit to a removed part",
    );
}

#[test]
fn a_comment_on_a_sheet_whose_comments_part_this_write_replaces_is_refused() {
    assert_package(
        refused_after(excel_both(), |p| {
            p.replace_part(
                b"xl/comments1.xml",
                replacement_from_reader(Cursor::new(b"x".to_vec())),
            )
        }),
        "a cell edit to a part this write replaces",
    );
}

#[test]
fn a_comment_on_a_sheet_whose_comments_part_this_write_adds_is_refused() {
    assert_package(
        refused_after(excel_both_without(b"xl/comments1.xml"), |p| {
            p.add_part(
                b"xl/comments1.xml",
                replacement_from_reader(Cursor::new(b"x".to_vec())),
            )
        }),
        "a cell edit to a part this write replaces",
    );
}

#[test]
fn a_comment_on_a_sheet_whose_vml_part_this_write_removes_is_refused() {
    assert_package(
        refused_after(excel_both(), |p| {
            p.remove_part(b"xl/drawings/vmlDrawing1.vml")
        }),
        "an edit to a removed part",
    );
}

#[test]
fn a_comment_on_a_sheet_whose_vml_part_this_write_replaces_is_refused() {
    assert_package(
        refused_after(excel_both(), |p| {
            p.replace_part(
                b"xl/drawings/vmlDrawing1.vml",
                replacement_from_reader(Cursor::new(b"x".to_vec())),
            )
        }),
        "a cell edit to a part this write replaces",
    );
}

#[test]
fn a_comment_on_a_sheet_whose_vml_part_this_write_adds_is_refused() {
    assert_package(
        refused_after(excel_both_without(b"xl/drawings/vmlDrawing1.vml"), |p| {
            p.add_part(
                b"xl/drawings/vmlDrawing1.vml",
                replacement_from_reader(Cursor::new(b"x".to_vec())),
            )
        }),
        "a cell edit to a part this write replaces",
    );
}

/// Records the removal of the comment on `cell` of the worksheet.
#[track_caller]
fn remove(package: &mut Package<Cursor<Vec<u8>>>, cell: &str) {
    package.remove_comment(SHEET, at(cell)).expect("recorded");
}

/// A worksheet with one comment by `Ann` on E5 and a VML part holding its
/// note, beside `extra` shapes, `rId1` the comments part and `rId2` the VML
/// part.
fn one_comment(extra: &str) -> Parts {
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
                comments_xml(&["Ann"], &kept_comment("E5", 0, "one")),
                COMMENTS_TYPE,
            ),
            (
                "xl/drawings/vmlDrawing1.vml",
                excel_vml("1", &format!("{extra}{}", kept_note("_x0000_s1026", 4, 4))),
                VML_TYPE,
            ),
        ],
    )
}

/// A form control's shape, a scroll bar.
const CONTROL: &str = "<v:shape id=\"_x0000_s1025\" type=\"#_x0000_t201\"><x:ClientData \
                       ObjectType=\"Scroll\"></x:ClientData></v:shape>";

#[test]
fn a_removal_of_one_of_two_comments_drops_its_element_and_its_note_and_keeps_the_rest() {
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| remove(p, "A1"));
    assert_untouched(
        &output,
        &parts,
        &[
            b"xl/comments1.xml".as_slice(),
            b"xl/drawings/vmlDrawing1.vml".as_slice(),
        ],
    );
    assert_eq!(
        text_of(&output, b"xl/comments1.xml"),
        part_text(&parts, b"xl/comments1.xml").replace(&kept_comment("A1", 0, "one"), "")
    );
    assert_eq!(
        text_of(&output, b"xl/drawings/vmlDrawing1.vml"),
        part_text(&parts, b"xl/drawings/vmlDrawing1.vml")
            .replace(&kept_note("_x0000_s1025", 0, 0), "")
    );
}

#[test]
fn the_last_comment_beside_a_control_drops_the_comments_part_and_keeps_the_vml_part() {
    // The comments part, its relationship and its Override go; the VML part
    // keeps the control, its note left out, and the worksheet its
    // legacyDrawing.
    let parts = one_comment(CONTROL);
    let output = edited_output(parts.clone(), |p| remove(p, "E5"));
    assert_untouched(
        &output,
        &parts,
        &[
            b"xl/comments1.xml".as_slice(),
            b"xl/drawings/vmlDrawing1.vml".as_slice(),
            SHEET_RELS,
            b"[Content_Types].xml".as_slice(),
        ],
    );
    assert!(
        !parse_independently(&output)
            .entries
            .iter()
            .any(|entry| entry.name == b"xl/comments1.xml"),
        "the comments part is gone"
    );
    assert_eq!(
        text_of(&output, b"xl/drawings/vmlDrawing1.vml"),
        excel_vml("1", CONTROL)
    );
    assert_eq!(
        text_of(&output, SHEET_RELS),
        rels(&vml_rel("rId2", "../drawings/vmlDrawing1.vml"))
    );
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        part_text(&parts, b"[Content_Types].xml").replace(
            &format!("<Override PartName=\"/xl/comments1.xml\" ContentType=\"{COMMENTS_TYPE}\"/>"),
            ""
        )
    );
}

#[test]
fn the_last_comment_with_notes_alone_drops_both_parts_and_the_legacy_drawing() {
    let parts = one_comment("");
    let output = edited_output(parts.clone(), |p| remove(p, "E5"));
    assert_untouched(
        &output,
        &parts,
        &[
            b"xl/comments1.xml".as_slice(),
            b"xl/drawings/vmlDrawing1.vml".as_slice(),
            SHEET,
            SHEET_RELS,
            b"[Content_Types].xml".as_slice(),
        ],
    );
    let names = names(&parse_independently(&output));
    for gone in [
        b"xl/comments1.xml".as_slice(),
        b"xl/drawings/vmlDrawing1.vml",
    ] {
        assert!(!names.iter().any(|name| name == gone), "a part is left");
    }
    assert_eq!(text_of(&output, SHEET), sheet_r("<sheetData/>"));
    assert_eq!(text_of(&output, SHEET_RELS), rels(""));
    let types = part_text(&parts, b"[Content_Types].xml")
        .replace(
            &format!("<Override PartName=\"/xl/comments1.xml\" ContentType=\"{COMMENTS_TYPE}\"/>"),
            "",
        )
        .replace(
            &format!(
                "<Override PartName=\"/xl/drawings/vmlDrawing1.vml\" ContentType=\"{VML_TYPE}\"/>"
            ),
            "",
        );
    assert_eq!(text_of(&output, b"[Content_Types].xml"), types);
    let mut package = Package::open(Cursor::new(output)).expect("open the output");
    assert_eq!(package.comments(SHEET).expect("read back"), []);
}

#[test]
fn a_removal_of_an_absent_cell_a_cell_this_write_adds_or_one_named_twice_is_refused() {
    let message = "no comment on that cell";
    assert_package(refused_with(excel_both(), |p| remove(p, "B5")), message);
    assert_package(refused_with(smallest(), |p| remove(p, "A1")), message);
    assert_package(
        refused_with(excel_both(), |p| {
            add(p, "A1", "Ann", "new");
            remove(p, "A1");
        }),
        message,
    );
    assert_package(
        refused_with(excel_both(), |p| {
            remove(p, "A1");
            remove(p, "A1");
        }),
        message,
    );
}

#[test]
fn a_removal_and_an_addition_on_different_cells_in_one_write() {
    // A1's comment and note go; B5's are appended, the note numbered past
    // the kept 1026: K 3, the note 1027.
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| {
        remove(p, "A1");
        add(p, "B5", "Bob", "new");
    });
    let comments = part_text(&parts, b"xl/comments1.xml")
        .replace(&kept_comment("A1", 0, "one"), "")
        .replace("</authors>", "<author>Bob</author></authors>")
        .replace(
            "</commentList>",
            &format!("{}</commentList>", added_comment("", "B5", 1, "new", false)),
        );
    assert_eq!(text_of(&output, b"xl/comments1.xml"), comments);
    let vml = part_text(&parts, b"xl/drawings/vmlDrawing1.vml")
        .replace(&kept_note("_x0000_s1025", 0, 0), "")
        .replace("</xml>", &format!("{}</xml>", note(VOX, 1027, 3, 4, 1)));
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), vml);
}

#[test]
fn a_removal_then_an_addition_of_one_cell_replaces_and_needs_a_kept_comment() {
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| {
        remove(p, "A1");
        add(p, "A1", "Ann", "new");
    });
    let comments = part_text(&parts, b"xl/comments1.xml")
        .replace(&kept_comment("A1", 0, "one"), "")
        .replace(
            "</commentList>",
            &format!("{}</commentList>", added_comment("", "A1", 0, "new", false)),
        );
    assert_eq!(text_of(&output, b"xl/comments1.xml"), comments);
    assert_package(
        refused_with(excel_both(), |p| {
            remove(p, "B5");
            add(p, "B5", "Ann", "new");
        }),
        "no comment on that cell",
    );
}

#[test]
fn a_removal_on_a_sheet_whose_comments_part_is_missing_is_refused_at_the_write() {
    let parts = with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&format!(
            "{}{}",
            comments_rel("rId1", "../comments9.xml"),
            vml_rel("rId2", "../drawings/vmlDrawing1.vml")
        )),
        &[(
            "xl/drawings/vmlDrawing1.vml",
            excel_vml("1", &kept_note("_x0000_s1025", 0, 0)),
            VML_TYPE,
        )],
    );
    assert_package(
        refused_with(parts, |p| remove(p, "A1")),
        "a comments part or its drawing cannot be read",
    );
}
