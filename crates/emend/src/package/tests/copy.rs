//! Sheet copies on byte-built packages (`specs/0028-images-and-comments-
//! second-half.md`, "The copy" and "P3: the copy"): each test builds the
//! smallest package with a worksheet holding the body it names and, where it
//! says so, a rels part, comments and VML parts, records a copy, and compares
//! the copy's entry, its rels part, the parts it adds and the workbook with
//! what the design gives. The Cargo feature `copy`.

use super::*;

/// The content types of a comments part and of a VML part.
#[cfg(feature = "comments")]
const COMMENTS_PART_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml";
#[cfg(feature = "comments")]
const VML_TYPE: &str = "application/vnd.openxmlformats-officedocument.vmlDrawing";

/// The copy's entry in every test: the smallest package holds sheet1.
const COPY: &[u8] = b"xl/worksheets/sheet2.xml";
const COPY_RELS: &[u8] = b"xl/worksheets/_rels/sheet2.xml.rels";
const SHEET_RELS: &str = "xl/worksheets/_rels/sheet1.xml.rels";

/// The markup-compatibility namespace.
const MC_NS: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";

/// A worksheet part whose root binds `r` and `mc` as Excel's do.
fn sheet_r(body: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\" \
         xmlns:mc=\"{MC_NS}\">{body}</worksheet>"
    )
}

/// The smallest package with the worksheet `sheet` and its rels part
/// holding `rels_body` when one is given.
fn with_sheet(sheet: String, rels_body: Option<&str>) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet);
    if let Some(body) = rels_body {
        with_part(&mut parts, SHEET_RELS, &rels(body), None);
    }
    parts
}

/// An external hyperlink relationship and an internal one.
fn link_rel(id: &str, target: &str) -> String {
    format!(
        "<Relationship Id=\"{id}\" Type=\"{ATTR_NS}/hyperlink\" Target=\"{target}\" \
         TargetMode=\"External\"/>"
    )
}

fn internal_link_rel(id: &str, target: &str) -> String {
    relationship(id, &format!("{ATTR_NS}/hyperlink"), target)
}

/// An entry of the output as text, inflated through the package layer, for
/// the parts a write adds.
fn entry_text(output: &[u8], name: &[u8]) -> String {
    let mut package = Package::open(Cursor::new(output.to_vec())).expect("open the output");
    let index = package.part(name).expect("the entry");
    let mut bytes = Vec::new();
    package
        .read_part(index)
        .expect("read_part")
        .read_to_end(&mut bytes)
        .expect("read");
    String::from_utf8(bytes).expect("UTF-8")
}

/// The refusal a write of `edit` on `parts` gives, with no output.
#[track_caller]
fn refused_with(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(parts);
    edit(&mut package);
    refused(package)
}

/// Records the copy of `Sheet1` as `Copy`.
#[track_caller]
fn copy(package: &mut Package<Cursor<Vec<u8>>>) {
    assert_eq!(
        package.copy_sheet("Sheet1", "Copy").expect("copy_sheet"),
        COPY
    );
}

/// The workbook with the copy's `<sheet>` after the first, its relationship
/// `rId3`.
fn workbook_with_copy(parts: &Parts) -> String {
    part_text(parts, b"xl/workbook.xml").replace(
        "</sheets>",
        "<sheet name=\"Copy\" sheetId=\"2\" r:id=\"rId3\"/></sheets>",
    )
}

/// The workbook's relationships with the copy's.
fn workbook_rels_with_copy() -> String {
    rels(&format!(
        "{}{}{}",
        worksheet_rel(),
        styles_rel(),
        relationship(
            "rId3",
            &format!("{ATTR_NS}/worksheet"),
            "worksheets/sheet2.xml"
        )
    ))
}

/// The content types with the copy's `Override`, and those given after it.
fn types_with_copy(parts: &Parts, more: &str) -> String {
    part_text(parts, b"[Content_Types].xml").replace(
        "</Types>",
        &format!(
            "<Override PartName=\"/xl/worksheets/sheet2.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>{more}</Types>"
        ),
    )
}

/// Every element "The copy" names, in schema order: the kept ones, with the
/// attributes and children the copy leaves out, and the root children it
/// drops whole.
const FULL_BODY: &str = "<sheetPr codeName=\"Sheet1\"><tabColor rgb=\"FFFF0000\"/></sheetPr>\
    <dimension ref=\"A1:B2\"/>\
    <sheetViews><sheetView tabSelected=\"1\" workbookViewId=\"0\">\
    <pane xSplit=\"1\" topLeftCell=\"B1\" activePane=\"topRight\" state=\"frozen\"/>\
    <selection pane=\"topRight\"/></sheetView></sheetViews>\
    <sheetFormatPr defaultRowHeight=\"15\"/>\
    <cols><col min=\"1\" max=\"1\" width=\"20\" customWidth=\"1\"/></cols>\
    <sheetData><row r=\"1\"><c r=\"A1\" s=\"1\" t=\"s\"><v>0</v></c><c r=\"B1\"><v>2</v></c></row>\
    <row r=\"2\"><c r=\"A2\" t=\"inlineStr\"><is><t>two</t></is></c></row></sheetData>\
    <sheetProtection sheet=\"1\" objects=\"1\"/>\
    <autoFilter ref=\"A1:B2\"/>\
    <mergeCells count=\"1\"><mergeCell ref=\"A2:B2\"/></mergeCells>\
    <conditionalFormatting sqref=\"B1\"><cfRule type=\"cellIs\" dxfId=\"0\" priority=\"1\" \
    operator=\"greaterThan\"><formula>1</formula></cfRule></conditionalFormatting>\
    <dataValidations count=\"1\"><dataValidation type=\"whole\" sqref=\"B1\">\
    <formula1>1</formula1></dataValidation></dataValidations>\
    <hyperlinks><hyperlink ref=\"A1\" r:id=\"rId1\"/><hyperlink ref=\"B1\" location=\"'Sheet1'!A1\"/>\
    <hyperlink ref=\"B2\" r:id=\"rId2\"/></hyperlinks>\
    <printOptions gridLines=\"1\"/>\
    <pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" header=\"0.3\" footer=\"0.3\"/>\
    <pageSetup orientation=\"landscape\" r:id=\"rId3\"/>\
    <headerFooter><oddHeader>&amp;CTop</oddHeader></headerFooter>\
    <drawing r:id=\"rId4\"/>\
    <legacyDrawing r:id=\"rId5\"/>\
    <tableParts count=\"1\"><tablePart r:id=\"rId6\"/></tableParts>\
    <extLst><ext uri=\"{78C0D931-6437-407d-A8EE-F0AAD7539E65}\" \
    xmlns:x14=\"http://schemas.microsoft.com/office/spreadsheetml/2009/9/main\">\
    <x14:conditionalFormattings/></ext></extLst>";

/// [`FULL_BODY`] as the copy writes it.
const FULL_COPIED: &str = "<sheetPr><tabColor rgb=\"FFFF0000\"/></sheetPr>\
    <dimension ref=\"A1:B2\"/>\
    <sheetViews><sheetView workbookViewId=\"0\">\
    <pane xSplit=\"1\" topLeftCell=\"B1\" activePane=\"topRight\" state=\"frozen\"/>\
    <selection pane=\"topRight\"/></sheetView></sheetViews>\
    <sheetFormatPr defaultRowHeight=\"15\"/>\
    <cols><col min=\"1\" max=\"1\" width=\"20\" customWidth=\"1\"/></cols>\
    <sheetData><row r=\"1\"><c r=\"A1\" s=\"1\" t=\"s\"><v>0</v></c><c r=\"B1\"><v>2</v></c></row>\
    <row r=\"2\"><c r=\"A2\" t=\"inlineStr\"><is><t>two</t></is></c></row></sheetData>\
    <sheetProtection sheet=\"1\" objects=\"1\"/>\
    <autoFilter ref=\"A1:B2\"/>\
    <mergeCells count=\"1\"><mergeCell ref=\"A2:B2\"/></mergeCells>\
    <conditionalFormatting sqref=\"B1\"><cfRule type=\"cellIs\" dxfId=\"0\" priority=\"1\" \
    operator=\"greaterThan\"><formula>1</formula></cfRule></conditionalFormatting>\
    <dataValidations count=\"1\"><dataValidation type=\"whole\" sqref=\"B1\">\
    <formula1>1</formula1></dataValidation></dataValidations>\
    <hyperlinks><hyperlink ref=\"A1\" r:id=\"rId1\"/><hyperlink ref=\"B1\" location=\"'Sheet1'!A1\"/>\
    <hyperlink ref=\"B2\" r:id=\"rId2\"/></hyperlinks>\
    <printOptions gridLines=\"1\"/>\
    <pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" header=\"0.3\" footer=\"0.3\"/>\
    <pageSetup orientation=\"landscape\"/>\
    <headerFooter><oddHeader>&amp;CTop</oddHeader></headerFooter>";

/// The source's relationships for [`FULL_BODY`]: an external and an
/// internal hyperlink, then what the copy does not carry.
fn full_rels() -> String {
    format!(
        "{}{}{}{}{}{}",
        link_rel("rId1", "https://example.com/a"),
        internal_link_rel("rId2", "../other.xml"),
        relationship(
            "rId3",
            &format!("{ATTR_NS}/printerSettings"),
            "../printerSettings/printerSettings1.bin"
        ),
        relationship(
            "rId4",
            &format!("{ATTR_NS}/drawing"),
            "../drawings/drawing1.xml"
        ),
        relationship(
            "rId5",
            &format!("{ATTR_NS}/vmlDrawing"),
            "../drawings/vmlDrawing1.vml"
        ),
        relationship("rId6", &format!("{ATTR_NS}/table"), "../tables/table1.xml"),
    )
}

#[test]
fn a_copy_keeps_the_sheet_and_leaves_out_what_it_does_not_carry() {
    let parts = with_sheet(sheet_r(FULL_BODY), Some(&full_rels()));
    let out = edited_output(parts.clone(), copy);
    // A created rels part is added before the part additions.
    let mut names = part_names(&parts);
    names.push(COPY_RELS);
    names.push(COPY);
    assert_names(&out, &names);
    assert_untouched(
        &out,
        &parts,
        &[
            b"[Content_Types].xml",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
        ],
    );
    assert_eq!(entry_text(&out, COPY), sheet_r(FULL_COPIED));
    // The hyperlink relationships alone, with their ids and target modes.
    assert_eq!(
        entry_text(&out, COPY_RELS),
        rels(&format!(
            "{}{}",
            link_rel("rId1", "https://example.com/a"),
            internal_link_rel("rId2", "../other.xml")
        ))
    );
    assert_part(&out, b"xl/workbook.xml", &workbook_with_copy(&parts));
    assert_part(
        &out,
        b"xl/_rels/workbook.xml.rels",
        &workbook_rels_with_copy(),
    );
    assert_part(&out, b"[Content_Types].xml", &types_with_copy(&parts, ""));
}

#[test]
fn a_copy_of_a_sheet_with_no_relationships_has_no_rels_part() {
    let body = "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row></sheetData>";
    let parts = with_sheet(sheet_r(body), None);
    let out = edited_output(parts.clone(), copy);
    let mut names = part_names(&parts);
    names.push(COPY);
    assert_names(&out, &names);
    assert_eq!(entry_text(&out, COPY), sheet_r(body));
}

#[test]
fn a_copy_keeps_an_attribute_in_the_xml_namespace() {
    // The `xml` prefix is bound without a declaration (Namespaces in XML
    // 1.0, section 3); an inline string's `xml:space`, which this layer's
    // own text cells write, is kept, not refused as an unbound prefix.
    let body = "<sheetData><row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is>\
        <t xml:space=\"preserve\"> a </t></is></c></row></sheetData>";
    let parts = with_sheet(sheet_r(body), None);
    let out = edited_output(parts.clone(), copy);
    assert_eq!(entry_text(&out, COPY), sheet_r(body));
}

#[test]
fn a_root_alternate_content_holding_only_dropped_elements_goes_whole() {
    // As two corpus worksheets wrap their `controls`; a choice holding
    // anything else is kept, and here refused for the relationship it names.
    let controls = "<mc:AlternateContent><mc:Choice Requires=\"x14\"><controls>\
        <mc:AlternateContent><mc:Choice Requires=\"x14\"><control shapeId=\"1025\" r:id=\"rId1\" \
        name=\"Box\"/></mc:Choice></mc:AlternateContent></controls></mc:Choice></mc:AlternateContent>";
    let body = format!("<sheetData/>{controls}<tableParts count=\"0\"/>");
    let ctrl = relationship(
        "rId1",
        &format!("{ATTR_NS}/ctrlProp"),
        "../ctrlProps/ctrlProp1.xml",
    );
    let parts = with_sheet(sheet_r(&body), Some(&ctrl));
    let out = edited_output(parts, copy);
    assert_eq!(entry_text(&out, COPY), sheet_r("<sheetData/>"));

    let kept = "<sheetData/><mc:AlternateContent><mc:Choice Requires=\"x14\">\
        <oleObjects/><picture r:id=\"rId1\"/></mc:Choice></mc:AlternateContent>";
    let mixed = "<sheetData/><mc:AlternateContent><mc:Choice Requires=\"x14\">\
        <controls/><x:extra xmlns:x=\"urn:x\" r:id=\"rId1\"/></mc:Choice></mc:AlternateContent>";
    // Every element of the branch dropped: gone, though one names a
    // relationship.
    let out = edited_output(with_sheet(sheet_r(kept), Some(&ctrl)), copy);
    assert_eq!(entry_text(&out, COPY), sheet_r("<sheetData/>"));
    assert_package(
        refused_with(with_sheet(sheet_r(mixed), Some(&ctrl)), copy),
        "a sheet copy meets a relationship it cannot carry",
    );
    // And one with nothing in a branch is kept as it is.
    let empty =
        "<sheetData/><mc:AlternateContent><mc:Choice Requires=\"x14\"/></mc:AlternateContent>";
    let out = edited_output(with_sheet(sheet_r(empty), None), copy);
    assert_eq!(entry_text(&out, COPY), sheet_r(empty));
}

#[test]
fn a_kept_relationship_outside_a_hyperlink_is_refused() {
    // A `customSheetView`'s `pageSetup` (none in the corpus), and a hyperlink
    // naming a relationship that is not a hyperlink or is not there.
    let printer = relationship(
        "rId1",
        &format!("{ATTR_NS}/printerSettings"),
        "../printerSettings/printerSettings1.bin",
    );
    for (body, rels_body) in [
        (
            "<sheetData/><customSheetViews><customSheetView guid=\"{00000000-0000-0000-0000-000000000000}\">\
             <pageSetup r:id=\"rId1\"/></customSheetView></customSheetViews>",
            printer.clone(),
        ),
        (
            "<sheetData/><hyperlinks><hyperlink ref=\"A1\" r:id=\"rId1\"/></hyperlinks>",
            printer.clone(),
        ),
        (
            "<sheetData/><hyperlinks><hyperlink ref=\"A1\" r:id=\"rId2\"/></hyperlinks>",
            link_rel("rId1", "https://example.com/a"),
        ),
        (
            "<sheetData r:id=\"rId1\"/>",
            link_rel("rId1", "https://example.com/a"),
        ),
    ] {
        assert_package(
            refused_with(with_sheet(sheet_r(body), Some(&rels_body)), copy),
            "a sheet copy meets a relationship it cannot carry",
        );
    }
}

#[test]
fn a_relationship_on_a_sheet_view_or_the_sheet_properties_is_refused() {
    // `specs/0029-engineering-close.md` item 6: the two start tags the copy
    // edits before writing are checked as every other kept tag is (none the
    // schema allows, none in the corpus), the stripped attribute beside it.
    let rels_body = link_rel("rId1", "https://example.com/a");
    for body in [
        "<sheetViews><sheetView tabSelected=\"1\" workbookViewId=\"0\" r:id=\"rId1\"/>\
         </sheetViews><sheetData/>",
        "<sheetPr codeName=\"Sheet1\" r:id=\"rId1\"/><sheetData/>",
    ] {
        assert_package(
            refused_with(with_sheet(sheet_r(body), Some(&rels_body)), copy),
            "a sheet copy meets a relationship it cannot carry",
        );
    }
}

#[test]
fn a_copy_of_a_sheet_with_a_formula_drops_the_chain() {
    let (parts, mut package) = opened(with_chain(2));
    package.copy_sheet("S2", "Copy").expect("copy_sheet");
    let out = written(package);
    let copy_part = b"xl/worksheets/sheet3.xml";
    assert_eq!(
        entry_text(&out, copy_part),
        part_text(&parts, b"xl/worksheets/sheet2.xml")
    );
    assert!(
        parse_independently(&out)
            .entries
            .iter()
            .all(|entry| entry.name != b"xl/calcChain.xml"),
        "the chain goes"
    );
    let workbook = String::from_utf8(stored_data(&out, b"xl/workbook.xml")).expect("UTF-8");
    assert!(
        workbook.contains("<calcPr calcId=\"191029\" fullCalcOnLoad=\"1\"/>"),
        "a full calculation on load: {workbook}"
    );
    // A copy of a sheet with no formula leaves the chain alone.
    let (parts, mut package) = opened(with_chain(2));
    package.copy_sheet("S1", "Copy").expect("copy_sheet");
    let out = written(package);
    assert_untouched(
        &out,
        &parts,
        &[
            b"[Content_Types].xml",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
        ],
    );
}

#[test]
fn the_call_refuses_an_absent_source_a_chartsheet_and_the_name_rules() {
    let mut parts = smallest();
    let rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        "</Relationships>",
        &format!(
            "{}</Relationships>",
            relationship(
                "rId3",
                &format!("{ATTR_NS}/chartsheet"),
                "chartsheets/sheet1.xml"
            )
        ),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", rels);
    let workbook = part_text(&parts, b"xl/workbook.xml").replace(
        "</sheets>",
        "<sheet name=\"Chart\" sheetId=\"2\" r:id=\"rId3\"/></sheets>",
    );
    set(&mut parts, b"xl/workbook.xml", workbook);
    with_part(
        &mut parts,
        "xl/chartsheets/sheet1.xml",
        "<chartsheet/>",
        Some(CHARTSHEET_TYPE),
    );
    let (parts, mut package) = opened(parts);
    let cases: [(&str, &str, &str); 5] = [
        ("Nothing", "Copy", "no sheet of that name"),
        ("Chart", "Copy", "a sheet copy needs a worksheet"),
        ("Sheet1", "", "a sheet name is empty"),
        ("Sheet1", "chart", "a sheet name is already in use"),
        (
            "Sheet1",
            "a/b",
            "a sheet name holds a character Excel refuses",
        ),
    ];
    for (name, to, message) in cases {
        assert_package(package.copy_sheet(name, to).expect_err("refused"), message);
    }
    // Nothing was recorded: the write changes nothing.
    let out = written(package);
    assert_names(&out, &part_names(&parts));
    assert_untouched(&out, &parts, &[]);
}

#[test]
fn an_edit_on_the_source_or_the_copy_or_a_source_added_is_refused_at_the_write() {
    let parts = || with_rows("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>");
    assert_package(
        refused_with(parts(), |package| {
            package.set_cell(SHEET, at("B1"), Value::Number(2.0));
            copy(package);
        }),
        "a sheet copy of a part this write edits",
    );
    assert_package(
        refused_with(parts(), |package| {
            copy(package);
            package
                .set_column_width(SHEET, 1, Some(12.0))
                .expect("recorded");
        }),
        "a sheet copy of a part this write edits",
    );
    assert_package(
        refused_with(parts(), |package| {
            copy(package);
            package.replace_part(
                SHEET,
                replacement_from_reader(Cursor::new(worksheet("<sheetData/>").into_bytes())),
            );
        }),
        "a sheet copy of a part this write edits",
    );
    assert_package(
        refused_with(parts(), |package| {
            copy(package);
            package.set_cell(COPY, at("A1"), Value::Number(2.0));
        }),
        "no such part",
    );
    assert_package(
        refused_with(parts(), |package| {
            package.add_sheet("Added").expect("add_sheet");
            package.copy_sheet("Added", "Copy").expect("copy_sheet");
        }),
        "no such part",
    );
    // A copy of a copy is a copy of a sheet this write adds.
    assert_package(
        refused_with(parts(), |package| {
            copy(package);
            package.copy_sheet("Copy", "Again").expect("copy_sheet");
        }),
        "no such part",
    );
}

#[test]
fn a_copy_moved_in_the_same_write_takes_its_place() {
    let (parts, mut package) = opened(with_rows("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>"));
    copy(&mut package);
    package.move_sheet("Copy", 0).expect("move_sheet");
    let out = written(package);
    assert_part(
        &out,
        b"xl/workbook.xml",
        &part_text(&parts, b"xl/workbook.xml").replace(
            "<sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets>",
            "<sheets><sheet name=\"Copy\" sheetId=\"2\" r:id=\"rId3\"/>\
             <sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets>",
        ),
    );
    assert_eq!(entry_text(&out, COPY), part_text(&parts, SHEET));
}

#[test]
fn a_copy_whose_source_is_removed_in_the_same_write_stays() {
    let mut parts = sheeted(2, "", "");
    set(
        &mut parts,
        b"xl/worksheets/sheet2.xml",
        worksheet("<sheetData><row r=\"1\"><c r=\"A1\"><v>7</v></c></row></sheetData>"),
    );
    let (parts, mut package) = opened(parts);
    let copy_part = package.copy_sheet("S2", "Copy").expect("copy_sheet");
    package.remove_sheet("S2").expect("remove_sheet");
    let out = written(package);
    assert_eq!(
        entry_text(&out, &copy_part),
        part_text(&parts, b"xl/worksheets/sheet2.xml")
    );
    assert!(
        parse_independently(&out)
            .entries
            .iter()
            .all(|entry| entry.name != b"xl/worksheets/sheet2.xml"),
        "the source goes"
    );
    // A copy removed again before the write leaves nothing behind.
    let (parts, mut package) = opened(with_rows(""));
    copy(&mut package);
    package.remove_sheet("Copy").expect("remove_sheet");
    let out = written(package);
    assert_names(&out, &part_names(&parts));
    assert_untouched(&out, &parts, &[]);
}

#[cfg(not(feature = "comments"))]
#[test]
fn a_source_with_comments_needs_the_comments_feature() {
    let comments = relationship("rId1", &format!("{ATTR_NS}/comments"), "../comments1.xml");
    assert_package(
        refused_with(with_sheet(sheet_r("<sheetData/>"), Some(&comments)), copy),
        "a sheet copy needs the comments feature",
    );
}

/// A comments part of two comments, on B2 and A5, and the VML part Excel
/// would write for them: the first with a 200 by 100 shown box in pixels,
/// the second with no size.
#[cfg(feature = "comments")]
fn with_comments(body: &str) -> Parts {
    let comments = format!(
        "{DECLARATION}\n<comments xmlns=\"{MAIN_NS}\"><authors><author>a</author></authors>\
         <commentList><comment ref=\"B2\" authorId=\"0\"><text><t>one</t></text></comment>\
         <comment ref=\"A5\" authorId=\"0\"><text><t>two</t></text></comment></commentList></comments>"
    );
    let vml = "<xml xmlns:v=\"urn:schemas-microsoft-com:vml\" \
        xmlns:o=\"urn:schemas-microsoft-com:office:office\" \
        xmlns:x=\"urn:schemas-microsoft-com:office:excel\">\
        <o:shapelayout v:ext=\"edit\"><o:idmap v:ext=\"edit\" data=\"1\"/></o:shapelayout>\
        <v:shape id=\"_x0000_s1025\" type=\"#_x0000_t202\" \
        style=\"position:absolute;width:150pt;height:75pt;visibility:visible\">\
        <x:ClientData ObjectType=\"Note\"><x:Row>1</x:Row><x:Column>1</x:Column></x:ClientData></v:shape>\
        <v:shape id=\"_x0000_s1026\" type=\"#_x0000_t202\" style=\"position:absolute\">\
        <x:ClientData ObjectType=\"Note\"><x:Row>4</x:Row><x:Column>0</x:Column></x:ClientData></v:shape>\
        </xml>";
    let rels_body = format!(
        "{}{}{}",
        link_rel("rId1", "https://example.com/a"),
        relationship("rId2", &format!("{ATTR_NS}/comments"), "../comments1.xml"),
        relationship(
            "rId3",
            &format!("{ATTR_NS}/vmlDrawing"),
            "../drawings/vmlDrawing1.vml"
        ),
    );
    let mut parts = with_sheet(sheet_r(body), Some(&rels_body));
    with_part(
        &mut parts,
        "xl/comments1.xml",
        &comments,
        Some(COMMENTS_PART_TYPE),
    );
    with_part(
        &mut parts,
        "xl/drawings/vmlDrawing1.vml",
        vml,
        Some(VML_TYPE),
    );
    parts
}

#[cfg(feature = "comments")]
#[test]
fn a_copy_carries_the_comments_with_their_boxes() {
    let body = "<sheetData/><hyperlinks><hyperlink ref=\"A1\" r:id=\"rId1\"/></hyperlinks>\
        <pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" header=\"0.3\" footer=\"0.3\"/>\
        <legacyDrawing r:id=\"rId3\"/>";
    let parts = with_comments(body);
    let out = edited_output(parts.clone(), copy);
    let mut names = part_names(&parts);
    names.extend([
        COPY_RELS,
        COPY,
        b"xl/comments2.xml".as_slice(),
        b"xl/drawings/vmlDrawing2.vml".as_slice(),
    ]);
    assert_names(&out, &names);
    assert_untouched(
        &out,
        &parts,
        &[
            b"[Content_Types].xml",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
        ],
    );
    // The legacyDrawing goes where the source's stood, naming the new VML
    // relationship, after the carried hyperlink's and the comments'.
    assert_eq!(
        entry_text(&out, COPY),
        sheet_r(
            "<sheetData/><hyperlinks><hyperlink ref=\"A1\" r:id=\"rId1\"/></hyperlinks>\
             <pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" header=\"0.3\" footer=\"0.3\"/>\
             <legacyDrawing r:id=\"rId3\"/>"
        )
    );
    assert_eq!(
        entry_text(&out, COPY_RELS),
        rels(&format!(
            "{}{}{}",
            link_rel("rId1", "https://example.com/a"),
            relationship("rId2", &format!("{ATTR_NS}/comments"), "../comments2.xml"),
            relationship(
                "rId3",
                &format!("{ATTR_NS}/vmlDrawing"),
                "../drawings/vmlDrawing2.vml"
            ),
        ))
    );
    assert_eq!(
        entry_text(&out, b"xl/comments2.xml"),
        part_text(&parts, b"xl/comments1.xml")
    );
    assert_eq!(
        entry_text(&out, b"xl/drawings/vmlDrawing2.vml").into_bytes(),
        vml_part_bytes(
            2,
            &[
                (
                    at("B2"),
                    CommentBox {
                        width: 200,
                        height: 100,
                        visible: true,
                    },
                ),
                (at("A5"), DEFAULT_BOX),
            ],
        )
    );
    assert_part(
        &out,
        b"[Content_Types].xml",
        &types_with_copy(
            &parts,
            &format!(
                "<Override PartName=\"/xl/comments2.xml\" ContentType=\"{COMMENTS_PART_TYPE}\"/>\
                 <Override PartName=\"/xl/drawings/vmlDrawing2.vml\" ContentType=\"{VML_TYPE}\"/>"
            ),
        ),
    );
    // The copy reads back the source's comments and boxes.
    let mut copied = Package::open(Cursor::new(out)).expect("open");
    let mut source = Package::open(Cursor::new(parts_bytes(&parts))).expect("open");
    assert_eq!(
        copied.comments(COPY).expect("comments"),
        source
            .comments(SHEET)
            .expect("comments")
            .into_iter()
            .map(|entry| CommentEntry {
                shape: Some(entry.shape.unwrap_or(DEFAULT_BOX)),
                ..entry
            })
            .collect::<Vec<_>>()
    );
}

/// The parts built into an archive's bytes.
#[cfg(feature = "comments")]
fn parts_bytes(parts: &Parts) -> Vec<u8> {
    let specs = parts
        .iter()
        .map(|(name, data)| Spec::new(name, data))
        .collect();
    Build::new(specs).bytes()
}

#[cfg(feature = "comments")]
#[test]
fn a_copy_with_comments_and_no_legacy_drawing_creates_one_in_schema_position() {
    // The source's comments relationship with no `<legacyDrawing>` in the
    // part: the copy creates one before `tableParts`, which it drops.
    let body = "<sheetData/><pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" \
        header=\"0.3\" footer=\"0.3\"/><tableParts count=\"0\"/>";
    let out = edited_output(with_comments(body), copy);
    assert_eq!(
        entry_text(&out, COPY),
        sheet_r(
            "<sheetData/><pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" \
             header=\"0.3\" footer=\"0.3\"/><legacyDrawing r:id=\"rId3\"/>"
        )
    );
    // And at the root's end when nothing after it is kept; a root binding no
    // relationships prefix gains one.
    let mut parts = with_comments("");
    set(&mut parts, SHEET, worksheet("<sheetData/>"));
    let out = edited_output(parts, copy);
    assert_eq!(
        entry_text(&out, COPY),
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheetData/>\
             <legacyDrawing r:id=\"rId3\"/></worksheet>"
        )
    );
}

#[cfg(feature = "comments")]
#[test]
fn a_copy_s_vml_block_is_not_taken_again_by_a_comment_in_the_same_write() {
    // The source's VML holds block 1; the copy takes 2, and a comment added
    // on a sheet with no VML part in the same write takes 3.
    let mut parts = with_comments("<sheetData/><legacyDrawing r:id=\"rId3\"/>");
    let rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        "</Relationships>",
        &format!(
            "{}</Relationships>",
            relationship(
                "rId9",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/other.xml"
            )
        ),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", rels);
    let workbook = part_text(&parts, b"xl/workbook.xml").replace(
        "</sheets>",
        "<sheet name=\"Other\" sheetId=\"5\" r:id=\"rId9\"/></sheets>",
    );
    set(&mut parts, b"xl/workbook.xml", workbook);
    with_part(
        &mut parts,
        "xl/worksheets/other.xml",
        &worksheet("<sheetData/>"),
        Some(WORKSHEET_TYPE),
    );
    let out = edited_output(parts, |package| {
        package.copy_sheet("Sheet1", "Copy").expect("copy_sheet");
        package
            .add_comment(
                b"xl/worksheets/other.xml",
                at("A1"),
                Comment {
                    author: "a",
                    text: "t",
                },
            )
            .expect("add_comment");
    });
    assert!(entry_text(&out, b"xl/drawings/vmlDrawing2.vml").contains("data=\"2\""));
    assert!(entry_text(&out, b"xl/drawings/vmlDrawing3.vml").contains("data=\"3\""));
}

#[cfg(feature = "comments")]
#[test]
fn a_copy_of_a_sheet_with_threaded_comments_or_a_comment_edit_is_refused() {
    let mut parts = with_comments("<sheetData/><legacyDrawing r:id=\"rId3\"/>");
    let rels = part_text(&parts, SHEET_RELS.as_bytes()).replace(
        "</Relationships>",
        &format!(
            "{}</Relationships>",
            relationship(
                "rId4",
                "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
                "../threadedComments/threadedComment1.xml"
            )
        ),
    );
    set(&mut parts, SHEET_RELS.as_bytes(), rels);
    assert_package(
        refused_with(parts, copy),
        "a worksheet has threaded comments",
    );
    assert_package(
        refused_with(
            with_comments("<sheetData/><legacyDrawing r:id=\"rId3\"/>"),
            |package| {
                copy(package);
                package
                    .add_comment(
                        COPY,
                        at("A1"),
                        Comment {
                            author: "a",
                            text: "t",
                        },
                    )
                    .expect("recorded");
            },
        ),
        "no such part",
    );
    assert_package(
        refused_with(
            with_comments("<sheetData/><legacyDrawing r:id=\"rId3\"/>"),
            |package| {
                package.remove_comment(SHEET, at("B2")).expect("recorded");
                copy(package);
            },
        ),
        "a sheet copy of a part this write edits",
    );
}
