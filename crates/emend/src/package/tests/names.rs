//! Defined names on byte-built workbook parts
//! (`specs/0019-names-links-properties-protection.md`, "P1"): each test
//! builds a package whose workbook part holds given children after
//! `<sheets>`, records name edits, and compares the written workbook part
//! with the bytes the design gives, every other part asserted untouched.

use super::*;

/// The `calcPr` the formula-change rule creates where the file has none.
const NEW_CALC: &str = "<calcPr fullCalcOnLoad=\"1\"/>";

/// A package of `n` sheets (`S1` to `Sn`) whose workbook part holds `after`
/// after `<sheets>`.
fn book(n: usize, after: &str) -> Parts {
    sheeted(n, "", after)
}

/// The workbook part `book(n, after)` would have.
fn book_part(n: usize, after: &str) -> String {
    part_text(&book(n, after), WORKBOOK)
}

/// The workbook part after `edit` on `book(n, after)`, as text; every other
/// part is asserted untouched.
#[track_caller]
fn edited_book(n: usize, after: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let parts = book(n, after);
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[WORKBOOK]);
    String::from_utf8(stored_data(&output, WORKBOOK)).expect("the part is UTF-8")
}

/// The refusal a write of `edit` on `book(n, after)` gives, with no output.
#[track_caller]
fn refused_book(n: usize, after: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(book(n, after));
    edit(&mut package);
    refused(package)
}

/// The names a package reads.
fn names_of(package: &mut Package<Cursor<Vec<u8>>>) -> Vec<DefinedName> {
    package
        .workbook()
        .expect("the workbook")
        .defined_names()
        .to_vec()
}

fn name(name: &str, text: &str, sheet: Option<u32>, hidden: bool) -> DefinedName {
    DefinedName {
        name: name.to_string(),
        text: text.to_string(),
        sheet,
        hidden,
    }
}

#[test]
fn names_are_read_with_their_scope_hidden_flag_and_text() {
    let (_, mut package) = opened(book(
        2,
        "<definedNames><definedName name=\"Local\" localSheetId=\"1\" hidden=\"1\">\
         &apos;S 2&apos;!$A$1</definedName><definedName name=\"G&amp;x\">1&lt;2\r\n3\
         </definedName><definedName name=\"e\" hidden=\"true\"/></definedNames>",
    ));
    assert_eq!(
        names_of(&mut package),
        vec![
            name("Local", "'S 2'!$A$1", Some(1), true),
            name("G&x", "1<2\n3", None, false),
            name("e", "", None, true),
        ]
    );
    let (_, mut package) = opened(smallest());
    assert_eq!(names_of(&mut package), Vec::new());
    // A `localSheetId` that is not a decimal `u32` is refused at the read,
    // with part 1's message (P3 ruling 10 of specs/0016).
    let (_, mut package) = opened(book(
        1,
        "<definedNames><definedName name=\"a\" localSheetId=\"x\">1</definedName></definedNames>",
    ));
    assert_package(
        package.workbook().expect_err("refused"),
        "a sheet edit no longer applies at the write",
    );
}

#[test]
fn a_local_sheet_id_that_is_not_a_number_is_refused_at_the_read() {
    // The workbook of part 1's
    // `a_local_sheet_id_that_is_not_a_number_refuses_a_move_at_the_write`,
    // which that test refuses at the write without the feature: with it the
    // names are read with the sheets, so the move's own read refuses it and
    // nothing is recorded.
    let names =
        "<definedNames><definedName name=\"a\" localSheetId=\"x\">A1</definedName></definedNames>";
    let (_, mut package) = opened(sheeted(2, "", names));
    assert_package(
        package.move_sheet("S2", 0).expect_err("refused"),
        "a sheet edit no longer applies at the write",
    );
}

#[test]
fn a_defined_name_attribute_with_an_unbound_prefix_is_refused_at_the_read() {
    // P1 verification ruling 6 (G3): with the feature the names are read
    // with the sheets and their attributes resolved, so a prefix nothing
    // binds is refused at the read, for every caller, with part 2's message.
    let (_, mut package) = opened(book(
        1,
        "<definedNames><definedName name=\"a\" q:x=\"1\">1</definedName></definedNames>",
    ));
    assert_package(
        package.workbook().expect_err("refused"),
        "an element or attribute uses a prefix its document does not bind",
    );
    // The control: the same attribute with its prefix bound is read.
    let (_, mut package) = opened(book(
        1,
        "<definedNames><definedName name=\"a\" xmlns:q=\"urn:q\" q:x=\"1\">1\
         </definedName></definedNames>",
    ));
    assert_eq!(names_of(&mut package), vec![name("a", "1", None, false)]);
}

#[test]
fn a_name_added_to_a_container_is_appended_and_the_file_children_kept() {
    let file = "<definedNames>\n  <definedName name=\"a\" localSheetId=\"0\">1</definedName>\n\
                </definedNames><calcPr calcId=\"1\"/>";
    let got = edited_book(2, file, |p| {
        p.add_defined_name("b", "S1!$A$1", None).expect("add b");
        p.add_defined_name("cn", "'S 2'!$B$2&\"x\"", Some("S2"))
            .expect("add cn");
    });
    assert_eq!(
        got,
        book_part(
            2,
            "<definedNames>\n  <definedName name=\"a\" localSheetId=\"0\">1</definedName>\n\
             <definedName name=\"b\">S1!$A$1</definedName>\
             <definedName name=\"cn\" localSheetId=\"1\">&apos;S 2&apos;!$B$2&amp;&quot;x&quot;\
             </definedName></definedNames><calcPr calcId=\"1\" fullCalcOnLoad=\"1\"/>"
        )
    );
    // An empty container is opened up.
    let got = edited_book(1, "<definedNames/>", |p| {
        p.add_defined_name("b", "1", None).expect("add b");
    });
    assert_eq!(
        got,
        book_part(
            1,
            &format!(
                "<definedNames><definedName name=\"b\">1</definedName></definedNames>{NEW_CALC}"
            )
        )
    );
}

#[test]
fn a_name_added_to_the_template_creates_the_container_before_the_new_calc_pr() {
    let mut package = new_package();
    package
        .add_defined_name("emend_global", "Sheet1!$A$1", None)
        .expect("add");
    let out = {
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = Vec::new();
        package.write_to(&mut out).expect("write_to");
        out
    };
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    let index = reopened.part(WORKBOOK).expect("the workbook part");
    let mut text = String::new();
    reopened
        .read_part(index)
        .expect("read_part")
        .read_to_string(&mut text)
        .expect("UTF-8");
    // The log prints the output (specs/0019, "P1").
    println!("template workbook part after the name: {text}");
    let template = String::from_utf8(TEMPLATE[2].1.to_vec()).expect("UTF-8");
    let want = template.replace(
        "</workbook>",
        &format!(
            "<definedNames><definedName name=\"emend_global\">Sheet1!$A$1</definedName>\
             </definedNames>{NEW_CALC}</workbook>"
        ),
    );
    assert_ne!(want, template, "the template's root end was found");
    assert_eq!(text, want);
}

#[test]
fn a_created_container_goes_in_schema_position() {
    let created =
        "<definedNames><definedName name=\"n\">1</definedName></definedNames>".to_string();
    let add = |p: &mut Package<Cursor<Vec<u8>>>| p.add_defined_name("n", "1", None).expect("add");
    // Before `extLst`, the new `calcPr` after it, the whitespace held.
    assert_eq!(
        edited_book(1, "\n<extLst><ext uri=\"{x}\"/></extLst>", add),
        book_part(
            1,
            &format!("{created}{NEW_CALC}\n<extLst><ext uri=\"{{x}}\"/></extLst>")
        )
    );
    // Before a `calcPr` the file has, after `externalReferences`.
    assert_eq!(
        edited_book(
            1,
            "<externalReferences/>\n<calcPr calcId=\"1\"/><pivotCaches/>",
            add
        ),
        book_part(
            1,
            &format!(
                "<externalReferences/>{created}\n<calcPr calcId=\"1\" fullCalcOnLoad=\"1\"/>\
                 <pivotCaches/>"
            )
        )
    );
    // Past another namespace's child and a comment between the table's names.
    assert_eq!(
        edited_book(
            1,
            "<!-- c --><q:x xmlns:q=\"urn:q\"/><oleSize ref=\"A1\"/>",
            add
        ),
        book_part(
            1,
            &format!("{created}{NEW_CALC}<!-- c --><q:x xmlns:q=\"urn:q\"/><oleSize ref=\"A1\"/>")
        )
    );
}

#[test]
fn a_local_name_resolves_against_the_projected_list() {
    // An add and a move before the call, a rename after it: the scope is
    // the sheet, and its index is the output's.
    let (_, mut package) = opened(book(
        2,
        "<definedNames><definedName name=\"old\" localSheetId=\"1\">1</definedName></definedNames>",
    ));
    package.add_sheet("New").expect("add_sheet");
    package.move_sheet("New", 0).expect("move_sheet");
    package
        .add_defined_name("on_s2", "2", Some("S2"))
        .expect("add on S2");
    package
        .add_defined_name("on_new", "3", Some("New"))
        .expect("add on New");
    package
        .add_defined_name("on_s1", "4", Some("S1"))
        .expect("add on S1");
    package.rename_sheet("S1", "T").expect("rename");
    let out = written(package);
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    assert_eq!(
        names_of(&mut reopened),
        vec![
            name("old", "1", Some(2), false),
            name("on_s2", "2", Some(2), false),
            name("on_new", "3", Some(0), false),
            name("on_s1", "4", Some(1), false),
        ]
    );
    // A scope whose sheet a later edit removed is refused at the write.
    let (_, mut package) = opened(book(2, ""));
    package
        .add_defined_name("gone", "1", Some("S2"))
        .expect("add");
    package.remove_sheet("S2").expect("remove_sheet");
    assert_package(refused(package), "no sheet of that name");
    // A scope the list does not hold is refused at the call.
    let (_, mut package) = opened(book(1, ""));
    assert_package(
        package
            .add_defined_name("x", "1", Some("Nowhere"))
            .expect_err("refused"),
        "no sheet of that name",
    );
}

#[test]
fn a_removal_drops_a_name_by_scope_and_refuses_an_absent_one() {
    let file = "<definedNames><definedName name=\"Keep\">1</definedName>\
                <definedName name=\"Dup\">2</definedName>\
                <definedName name=\"dup\" localSheetId=\"0\"><![CDATA[3]]></definedName>\
                </definedNames>";
    // Case-insensitively, the global alone.
    assert_eq!(
        edited_book(1, file, |p| p
            .remove_defined_name("DUP", None)
            .expect("remove")),
        book_part(
            1,
            &format!(
                "<definedNames><definedName name=\"Keep\">1</definedName>\
                 <definedName name=\"dup\" localSheetId=\"0\"><![CDATA[3]]></definedName>\
                 </definedNames>{NEW_CALC}"
            )
        )
    );
    // The local by its scope, the global of the same name kept.
    assert_eq!(
        edited_book(1, file, |p| p
            .remove_defined_name("Dup", Some("S1"))
            .expect("remove")),
        book_part(
            1,
            &format!(
                "<definedNames><definedName name=\"Keep\">1</definedName>\
                 <definedName name=\"Dup\">2</definedName></definedNames>{NEW_CALC}"
            )
        )
    );
    // An absent one: no such name, and no such scope for a present name.
    assert_package(
        refused_book(1, file, |p| {
            p.remove_defined_name("none", None).expect("recorded")
        }),
        "no defined name of that name and scope",
    );
    assert_package(
        refused_book(1, file, |p| {
            p.remove_defined_name("Keep", Some("S1")).expect("recorded")
        }),
        "no defined name of that name and scope",
    );
    // The last name drops the container, whitespace inside it too.
    assert_eq!(
        edited_book(
            1,
            "<definedNames>\n<definedName name=\"a\">1</definedName>\n</definedNames>",
            |p| p.remove_defined_name("a", None).expect("remove")
        ),
        book_part(1, NEW_CALC)
    );
    // And a removal with an addition keeps it.
    assert_eq!(
        edited_book(
            1,
            "<definedNames><definedName name=\"a\">1</definedName></definedNames>",
            |p| {
                p.remove_defined_name("a", None).expect("remove");
                p.add_defined_name("b", "2", None).expect("add");
            }
        ),
        book_part(
            1,
            &format!(
                "<definedNames><definedName name=\"b\">2</definedName></definedNames>{NEW_CALC}"
            )
        )
    );
}

#[test]
fn a_duplicate_against_the_file_or_a_pending_edit_is_refused() {
    let file = "<definedNames><definedName name=\"a\">1</definedName></definedNames>";
    // Against the file, ignoring case.
    assert_package(
        refused_book(1, file, |p| {
            p.add_defined_name("A", "2", None).expect("recorded")
        }),
        "a defined name of that name and scope already exists",
    );
    // Another scope is another name.
    edited_book(1, file, |p| {
        p.add_defined_name("a", "2", Some("S1")).expect("add")
    });
    // Against a pending addition.
    assert_package(
        refused_book(1, "", |p| {
            p.add_defined_name("b", "1", None).expect("recorded");
            p.add_defined_name("B", "2", None).expect("recorded");
        }),
        "a defined name of that name and scope already exists",
    );
    // An addition and a removal of one name and scope, in either order.
    assert_package(
        refused_book(1, file, |p| {
            p.remove_defined_name("a", None).expect("recorded");
            p.add_defined_name("a", "2", None).expect("recorded");
        }),
        "a defined name of that name and scope already exists",
    );
    assert_package(
        refused_book(1, "", |p| {
            p.add_defined_name("cn", "2", None).expect("recorded");
            p.remove_defined_name("cn", None).expect("recorded");
        }),
        "a defined name of that name and scope already exists",
    );
}

#[test]
fn the_name_rule_refuses_each_clause_and_records_nothing() {
    let long = "a".repeat(256);
    for bad in [
        "",
        "A1",
        "a1",
        "xfd1048576",
        "R1C1",
        "rc",
        "R2C",
        "rC3",
        "_xlnm.Print_Area",
        "_XLNM.x",
        "1abc",
        ".a",
        "?a",
        "a b",
        "a-b",
        "R",
        "C",
        "r",
        "c",
        long.as_str(),
    ] {
        let (parts, mut package) = opened(book(1, ""));
        assert_package(
            package
                .add_defined_name(bad, "1", None)
                .expect_err("refused"),
            "a defined name is not valid",
        );
        // Nothing recorded: the write is the input.
        assert_untouched(&written(package), &parts, &[]);
    }
    let (_, mut package) = opened(book(1, ""));
    assert_package(
        package
            .add_defined_name("a", "", None)
            .expect_err("refused"),
        "a defined name is not valid",
    );
    assert_package(
        package
            .add_defined_name("a", "x\u{1}", None)
            .expect_err("refused"),
        "a text holds a character XML cannot carry",
    );
    let long = "b".repeat(255);
    for good in [
        "_a",
        "\\a",
        "a.b?c_1",
        "\u{e9}t\u{e9}",
        "RC1x",
        "A01",
        "Table1",
        "xyz1",
        long.as_str(),
    ] {
        package
            .add_defined_name(good, "1", None)
            .unwrap_or_else(|e| panic!("{good:?} refused: {e}"));
    }
}

#[test]
fn a_name_edit_drops_the_chain_and_sets_the_flag() {
    // Decision 2 A: added or removed, a name is a formula change.
    for remove in [false, true] {
        let mut parts = with_chain(2);
        let workbook = part_text(&parts, WORKBOOK).replace(
            "<calcPr calcId=\"191029\"/>",
            "<definedNames><definedName name=\"a\">1</definedName></definedNames>\
             <calcPr calcId=\"191029\"/>",
        );
        set(&mut parts, WORKBOOK, workbook);
        let (parts, mut package) = opened(parts);
        if remove {
            package.remove_defined_name("a", None).expect("remove");
        } else {
            package.add_defined_name("b", "1", None).expect("add");
        }
        let out = written(package);
        assert_names(&out, &names_without(&parts, &["xl/calcChain.xml"]));
        let names = if remove {
            String::new()
        } else {
            "<definedNames><definedName name=\"a\">1</definedName>\
             <definedName name=\"b\">1</definedName></definedNames>"
                .to_string()
        };
        assert_part(
            &out,
            WORKBOOK,
            &part_text(&parts, WORKBOOK).replace(
                "<definedNames><definedName name=\"a\">1</definedName></definedNames>\
                 <calcPr calcId=\"191029\"/>",
                &format!("{names}<calcPr calcId=\"191029\" fullCalcOnLoad=\"1\"/>"),
            ),
        );
        assert_part(
            &out,
            b"[Content_Types].xml",
            &types_without(&parts, &["xl/calcChain.xml"]),
        );
    }
}

#[test]
fn a_second_defined_names_is_refused() {
    let two = "<definedNames><definedName name=\"a\">1</definedName></definedNames>\
               <definedNames/>";
    // Reading does not refuse a second container; the first's name is read
    // (the second is empty).
    let (_, mut package) = opened(book(1, two));
    assert_eq!(names_of(&mut package), vec![name("a", "1", None, false)]);
    for remove in [false, true] {
        assert_package(
            refused_book(1, two, |p| {
                if remove {
                    p.remove_defined_name("a", None).expect("recorded");
                } else {
                    p.add_defined_name("b", "1", None).expect("recorded");
                }
            }),
            "a workbook has two definedNames or workbookProtection elements",
        );
    }
}

#[test]
fn a_name_edit_on_a_replaced_workbook_part_is_refused() {
    let (_, mut package) = opened(book(1, ""));
    package.add_defined_name("a", "1", None).expect("add");
    package.replace_part(WORKBOOK, replacement_from_reader(b"<workbook/>".as_slice()));
    assert_package(
        refused(package),
        "the workbook part cannot be edited in a package that replaces or removes it",
    );
}
