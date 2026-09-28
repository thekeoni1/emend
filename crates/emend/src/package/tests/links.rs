//! Hyperlinks on byte-built worksheets
//! (`specs/0019-names-links-properties-protection.md`, "P2"): each test
//! builds the smallest package with its worksheet holding a given body (and,
//! where it says so, a rels part), records link edits, and compares the
//! written parts with the bytes the design gives, every other part asserted
//! untouched.

use super::*;

/// The relationships namespace's hyperlink type.
fn hyperlink_type() -> String {
    format!("{ATTR_NS}/hyperlink")
}

/// A worksheet part whose root binds `r` to the relationships namespace.
fn sheet_r(body: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{body}</worksheet>"
    )
}

/// An external hyperlink relationship as the write records one.
fn external(id: &str, target: &str) -> String {
    format!(
        "<Relationship Id=\"{id}\" Type=\"{}\" Target=\"{target}\" TargetMode=\"External\"/>",
        hyperlink_type()
    )
}

/// The worksheet's rels part as the fixtures write it: an external link as
/// `rId3` and an image as `rId7`.
fn sheet_rels_body() -> String {
    format!(
        "{}{}",
        external("rId3", "https://a.example/"),
        relationship("rId7", &format!("{ATTR_NS}/image"), "../media/image1.png")
    )
}

/// The smallest package with its worksheet `sheet` and, when `with_rels`,
/// the worksheet's rels part of [`sheet_rels_body`].
fn with_sheet(sheet: String, with_rels: bool) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet);
    if with_rels {
        with_part(
            &mut parts,
            "xl/worksheets/_rels/sheet1.xml.rels",
            &rels(&sheet_rels_body()),
            None,
        );
    }
    parts
}

/// The output of `edit` on `parts`, with every part but the worksheet and
/// its rels part asserted untouched.
#[track_caller]
fn linked(parts: &Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Vec<u8> {
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, parts, &[SHEET, SHEET_RELS]);
    output
}

/// A stored part of the output, as text.
fn text_of(output: &[u8], name: &[u8]) -> String {
    String::from_utf8(stored_data(output, name)).expect("UTF-8")
}

/// The refusal a write of `edit` on `parts` gives, with no output.
#[track_caller]
fn refused_links(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(parts);
    edit(&mut package);
    refused(package)
}

/// The fixture body with one external link on B2 before `pageMargins`.
const ONE_LINK: &str = "<sheetData/><hyperlinks><hyperlink ref=\"B2\" r:id=\"rId3\"/></hyperlinks>\
                        <pageMargins left=\"0.7\"/>";

#[test]
fn an_external_link_is_appended_with_its_relationship_one_past_the_largest_id() {
    let parts = with_sheet(sheet_r(ONE_LINK), true);
    let output = linked(&parts, |p| {
        p.set_hyperlink(
            SHEET,
            at("A1"),
            LinkTarget::External("https://example.com/?a=1&b=2"),
            None,
        )
        .expect("set")
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><hyperlinks><hyperlink ref=\"B2\" r:id=\"rId3\"/>\
             <hyperlink ref=\"A1\" r:id=\"rId8\"/></hyperlinks><pageMargins left=\"0.7\"/>"
        )
    );
    assert_eq!(
        text_of(&output, SHEET_RELS),
        rels(&format!(
            "{}{}",
            sheet_rels_body(),
            external("rId8", "https://example.com/?a=1&amp;b=2")
        ))
    );
}

#[test]
fn a_location_link_with_a_tooltip_is_written_escaped_and_the_rels_part_untouched() {
    let parts = with_sheet(sheet_r(ONE_LINK), true);
    let output = linked(&parts, |p| {
        p.set_hyperlink(
            SHEET,
            at("C3"),
            LinkTarget::Location("'My Sheet'!A1"),
            Some("a \"b\" <c> & d"),
        )
        .expect("set")
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><hyperlinks><hyperlink ref=\"B2\" r:id=\"rId3\"/>\
             <hyperlink ref=\"C3\" location=\"&apos;My Sheet&apos;!A1\" \
             tooltip=\"a &quot;b&quot; &lt;c&gt; &amp; d\"/></hyperlinks>\
             <pageMargins left=\"0.7\"/>"
        )
    );
    assert_eq!(text_of(&output, SHEET_RELS), rels(&sheet_rels_body()));
}

#[test]
fn a_container_is_created_before_page_margins() {
    let parts = with_sheet(sheet_r("<sheetData/><pageMargins left=\"0.7\"/>"), false);
    let output = linked(&parts, |p| {
        p.set_hyperlink(SHEET, at("A1"), LinkTarget::Location("S!A1"), None)
            .expect("set")
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><hyperlinks><hyperlink ref=\"A1\" location=\"S!A1\"/></hyperlinks>\
             <pageMargins left=\"0.7\"/>"
        )
    );
}

#[test]
fn the_template_part_is_retagged_and_its_container_and_rels_part_created_at_the_end() {
    let mut package = new_package();
    package
        .set_hyperlink(
            SHEET,
            at("A1"),
            LinkTarget::External("https://example.com/"),
            None,
        )
        .expect("set external");
    package
        .set_hyperlink(SHEET, at("B1"), LinkTarget::Location("Sheet1!A1"), None)
        .expect("set location");
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
    println!("template worksheet after the links: {sheet}");
    let template = String::from_utf8(TEMPLATE[4].1.to_vec()).expect("UTF-8");
    let want = template
        .replace(
            &format!("<worksheet xmlns=\"{MAIN_NS}\">"),
            &format!("<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">"),
        )
        .replace(
            "</worksheet>",
            "<hyperlinks><hyperlink ref=\"A1\" r:id=\"rId1\"/>\
             <hyperlink ref=\"B1\" location=\"Sheet1!A1\"/></hyperlinks></worksheet>",
        );
    assert_ne!(want, template, "the template's root was found");
    assert_eq!(sheet, want);
    let rels_text = read(&mut reopened, SHEET_RELS);
    println!("template worksheet's created rels part: {rels_text}");
    assert!(
        rels_text.ends_with(&format!(
            "<Relationships xmlns=\"{RELS_NS}\">{}</Relationships>",
            external("rId1", "https://example.com/")
        )),
        "{rels_text}"
    );
}

#[test]
fn a_root_whose_r_prefix_is_taken_is_retagged_with_the_next_free_one() {
    let sheet = format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"urn:other\"><sheetData/></worksheet>"
    );
    let parts = with_sheet(sheet, false);
    let output = linked(&parts, |p| {
        p.set_hyperlink(
            SHEET,
            at("A1"),
            LinkTarget::External("https://x.example/"),
            None,
        )
        .expect("set")
    });
    assert_eq!(
        text_of(&output, SHEET),
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"urn:other\" \
             xmlns:r1=\"{ATTR_NS}\"><sheetData/><hyperlinks><hyperlink ref=\"A1\" r1:id=\"rId1\"/>\
             </hyperlinks></worksheet>"
        )
    );
}

#[test]
fn a_set_on_a_linked_cell_replaces_the_link_and_removes_its_relationship() {
    let parts = with_sheet(sheet_r(ONE_LINK), true);
    let output = linked(&parts, |p| {
        p.set_hyperlink(
            SHEET,
            at("B2"),
            LinkTarget::External("https://new.example/"),
            None,
        )
        .expect("set")
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><hyperlinks><hyperlink ref=\"B2\" r:id=\"rId8\"/></hyperlinks>\
             <pageMargins left=\"0.7\"/>"
        )
    );
    assert_eq!(
        text_of(&output, SHEET_RELS),
        rels(&format!(
            "{}{}",
            relationship("rId7", &format!("{ATTR_NS}/image"), "../media/image1.png"),
            external("rId8", "https://new.example/")
        ))
    );
}

#[test]
fn a_removal_drops_a_link_with_or_without_r_id_and_the_last_drops_the_container() {
    let two = "<sheetData/><hyperlinks><hyperlink ref=\"B2\" r:id=\"rId3\"/>\
               <hyperlink ref=\"C3\" location=\"S!A1\" display=\"x\"></hyperlink></hyperlinks>\
               <pageMargins left=\"0.7\"/>";
    // With r:id: the element and its relationship go.
    let output = linked(&with_sheet(sheet_r(two), true), |p| {
        p.remove_hyperlink(SHEET, at("B2")).expect("remove")
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><hyperlinks><hyperlink ref=\"C3\" location=\"S!A1\" display=\"x\">\
             </hyperlink></hyperlinks><pageMargins left=\"0.7\"/>"
        )
    );
    assert_eq!(
        text_of(&output, SHEET_RELS),
        rels(&relationship(
            "rId7",
            &format!("{ATTR_NS}/image"),
            "../media/image1.png"
        ))
    );
    // Without r:id: the element alone goes, content and all.
    let output = linked(&with_sheet(sheet_r(two), true), |p| {
        p.remove_hyperlink(SHEET, at("C3")).expect("remove")
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><hyperlinks><hyperlink ref=\"B2\" r:id=\"rId3\"/></hyperlinks>\
             <pageMargins left=\"0.7\"/>"
        )
    );
    assert_eq!(text_of(&output, SHEET_RELS), rels(&sheet_rels_body()));
    // Both: the container goes too.
    let output = linked(&with_sheet(sheet_r(two), true), |p| {
        p.remove_hyperlink(SHEET, at("B2")).expect("remove");
        p.remove_hyperlink(SHEET, at("C3")).expect("remove");
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r("<sheetData/><pageMargins left=\"0.7\"/>")
    );
}

#[test]
fn a_removal_of_an_absent_link_or_of_one_whose_ref_is_a_range_is_refused() {
    assert_package(
        refused_links(with_sheet(sheet_r(ONE_LINK), true), |p| {
            p.remove_hyperlink(SHEET, at("A1")).expect("recorded")
        }),
        "no hyperlink on that cell",
    );
    let ranged = "<sheetData/><hyperlinks><hyperlink ref=\"A1:B2\" location=\"S!A1\"/>\
                  </hyperlinks>";
    assert_package(
        refused_links(with_sheet(sheet_r(ranged), false), |p| {
            p.remove_hyperlink(SHEET, at("A1")).expect("recorded")
        }),
        "no hyperlink on that cell",
    );
    // A set on a cell inside the range leaves the range link as it is.
    let output = linked(&with_sheet(sheet_r(ranged), false), |p| {
        p.set_hyperlink(SHEET, at("A1"), LinkTarget::Location("S!B1"), None)
            .expect("set")
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><hyperlinks><hyperlink ref=\"A1:B2\" location=\"S!A1\"/>\
             <hyperlink ref=\"A1\" location=\"S!B1\"/></hyperlinks>"
        )
    );
}

#[test]
fn a_second_hyperlinks_is_refused() {
    for body in [
        "<sheetData/><hyperlinks/><hyperlinks/>",
        "<sheetData/><hyperlinks><hyperlink ref=\"B2\" location=\"S!A1\"/></hyperlinks><hyperlinks/>",
    ] {
        assert_package(
            refused_links(with_sheet(sheet_r(body), false), |p| {
                p.set_hyperlink(SHEET, at("A1"), LinkTarget::Location("S!A1"), None)
                    .expect("recorded")
            }),
            "a worksheet has two hyperlinks or sheetProtection elements",
        );
    }
}

#[test]
fn a_link_and_a_cell_set_on_one_cell_compose_and_the_last_record_wins() {
    let parts = with_sheet(sheet_r("<sheetData/>"), false);
    let output = linked(&parts, |p| {
        p.set_cell(SHEET, at("A1"), Value::Text("go".to_string()));
        p.set_hyperlink(SHEET, at("A1"), LinkTarget::Location("S!Z9"), None)
            .expect("set");
        p.set_hyperlink(SHEET, at("A1"), LinkTarget::Location("S!B1"), Some("t"))
            .expect("set again");
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData><row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is><t xml:space=\"preserve\">\
             go</t></is></c></row></sheetData><hyperlinks><hyperlink ref=\"A1\" \
             location=\"S!B1\" tooltip=\"t\"/></hyperlinks>"
        )
    );
}

#[test]
fn the_calls_refuse_an_empty_target_a_forbidden_character_or_a_cell_off_the_grid() {
    let (parts, mut package) = opened(with_sheet(sheet_r(ONE_LINK), true));
    for target in [LinkTarget::External(""), LinkTarget::Location("")] {
        assert_package(
            package
                .set_hyperlink(SHEET, at("A1"), target, None)
                .expect_err("refused"),
            "a hyperlink target or location is empty",
        );
    }
    assert_package(
        package
            .set_hyperlink(SHEET, at("A1"), LinkTarget::Location("a\u{1}"), None)
            .expect_err("refused"),
        "a text holds a character XML cannot carry",
    );
    assert_package(
        package
            .set_hyperlink(SHEET, at("A1"), LinkTarget::Location("a"), Some("\u{1}"))
            .expect_err("refused"),
        "a text holds a character XML cannot carry",
    );
    let off = CellRef {
        row: 1_048_577,
        col: 1,
    };
    assert_package(
        package
            .set_hyperlink(SHEET, off, LinkTarget::Location("a"), None)
            .expect_err("refused"),
        "a row or column index or count is out of range",
    );
    assert_package(
        package.remove_hyperlink(SHEET, off).expect_err("refused"),
        "a row or column index or count is out of range",
    );
    // Nothing recorded: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
}

#[cfg(feature = "rowcol")]
#[test]
fn a_link_on_a_part_with_a_row_insertion_is_refused() {
    assert_package(
        refused_links(with_sheet(sheet_r(ONE_LINK), true), |p| {
            p.insert_rows(SHEET, 1, 1).expect("recorded");
            p.set_hyperlink(SHEET, at("A1"), LinkTarget::Location("S!A1"), None)
                .expect("recorded");
        }),
        "a worksheet with a row or column insertion or deletion takes no other edit in the same write",
    );
}
