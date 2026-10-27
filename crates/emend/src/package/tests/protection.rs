//! Worksheet and workbook protection on byte-built packages
//! (`specs/0019-names-links-properties-protection.md`, "P3"): the legacy
//! hash pinned on passwords of known hash, the `<sheetProtection>` and
//! `<workbookProtection>` elements the design gives, and the refusals.

use super::*;

/// The worksheet part after `edit` on the smallest package whose worksheet
/// holds `body`; every other part is asserted untouched.
#[track_caller]
fn protected(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[SHEET]);
    String::from_utf8(stored_data(&output, SHEET)).expect("UTF-8")
}

/// The workbook part after `edit` on a one-sheet workbook with `before`
/// before `<sheets>` and `after` after it; every other part untouched.
#[track_caller]
fn protected_book(
    before: &str,
    after: &str,
    edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>),
) -> String {
    let parts = sheeted(1, before, after);
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[WORKBOOK]);
    String::from_utf8(stored_data(&output, WORKBOOK)).expect("UTF-8")
}

/// The workbook part `sheeted(1, before, after)` has.
fn book(before: &str, after: &str) -> String {
    part_text(&sheeted(1, before, after), WORKBOOK)
}

#[test]
fn the_legacy_hash_is_pinned_on_passwords_of_known_hash() {
    // The published values of the legacy algorithm, printed for the log.
    for (password, want) in [("password", 0x83AF), ("test", 0xCBEB), ("secret", 0xDAA7)] {
        let got = legacy_password_hash(password);
        println!("legacy_password_hash({password:?}) = {got:04X}");
        assert_eq!(got, want, "{password}");
    }
}

#[test]
fn a_password_outside_printable_ascii_or_past_255_characters_is_refused() {
    let (parts, mut package) = opened(smallest());
    let long = "a".repeat(256);
    for bad in ["", long.as_str(), "caf\u{e9}", "tab\there"] {
        assert_package(
            package
                .protect_sheet(SHEET, Some(bad), SheetProtection::default())
                .expect_err("refused"),
            "a password is not valid",
        );
        assert_package(
            package
                .protect_workbook(Some(bad), true, false)
                .expect_err("refused"),
            "a password is not valid",
        );
    }
    // Nothing recorded: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
    let (_, mut package) = opened(smallest());
    package
        .protect_sheet(SHEET, Some(&"~".repeat(255)), SheetProtection::default())
        .expect("255 printable characters");
}

#[test]
fn the_default_options_write_sheet_alone_with_and_without_a_password() {
    let hash = format!("{:04X}", legacy_password_hash("emend"));
    assert_eq!(
        protected("<sheetData/>", |p| {
            p.protect_sheet(SHEET, None, SheetProtection::default())
                .expect("protect")
        }),
        worksheet("<sheetData/><sheetProtection sheet=\"1\"/>")
    );
    assert_eq!(
        protected("<sheetData/>", |p| {
            p.protect_sheet(SHEET, Some("emend"), SheetProtection::default())
                .expect("protect")
        }),
        worksheet(&format!(
            "<sheetData/><sheetProtection password=\"{hash}\" sheet=\"1\"/>"
        ))
    );
}

#[test]
fn each_option_flipped_writes_its_one_attribute() {
    // The sixteen elements, printed for the log: each option the default's
    // opposite, alone.
    type Flip = (&'static str, fn(&mut SheetProtection));
    let flips: [Flip; 16] = [
        ("sheet=\"0\"", |o| o.sheet = false),
        ("objects=\"1\"", |o| o.objects = true),
        ("scenarios=\"1\"", |o| o.scenarios = true),
        ("formatCells=\"0\"", |o| o.format_cells = false),
        ("formatColumns=\"0\"", |o| o.format_columns = false),
        ("formatRows=\"0\"", |o| o.format_rows = false),
        ("insertColumns=\"0\"", |o| o.insert_columns = false),
        ("insertRows=\"0\"", |o| o.insert_rows = false),
        ("insertHyperlinks=\"0\"", |o| o.insert_hyperlinks = false),
        ("deleteColumns=\"0\"", |o| o.delete_columns = false),
        ("deleteRows=\"0\"", |o| o.delete_rows = false),
        ("selectLockedCells=\"1\"", |o| o.select_locked_cells = true),
        ("sort=\"0\"", |o| o.sort = false),
        ("autoFilter=\"0\"", |o| o.auto_filter = false),
        ("pivotTables=\"0\"", |o| o.pivot_tables = false),
        ("selectUnlockedCells=\"1\"", |o| {
            o.select_unlocked_cells = true
        }),
    ];
    for (attribute, flip) in flips {
        let mut options = SheetProtection::default();
        flip(&mut options);
        let got = protected("<sheetData/>", |p| {
            p.protect_sheet(SHEET, None, options).expect("protect")
        });
        // `sheet` unlocked is the schema's default, so no attribute stands.
        let element = if attribute.starts_with("sheet=") {
            "<sheetProtection/>".to_string()
        } else {
            format!("<sheetProtection sheet=\"1\" {attribute}/>")
        };
        println!("{element}");
        assert_eq!(
            got,
            worksheet(&format!("<sheetData/>{element}")),
            "{attribute}"
        );
    }
}

#[test]
fn an_existing_element_is_replaced_and_an_absent_one_created_in_schema_position() {
    let options = SheetProtection {
        objects: true,
        scenarios: true,
        ..SheetProtection::default()
    };
    let element = "<sheetProtection sheet=\"1\" objects=\"1\" scenarios=\"1\"/>";
    // Replaced whole, content and all.
    assert_eq!(
        protected(
            "<sheetData/><sheetProtection password=\"ABCD\" sort=\"0\"><extLst/></sheetProtection>\
             <pageMargins left=\"0.7\"/>",
            |p| p.protect_sheet(SHEET, None, options).expect("protect")
        ),
        worksheet(&format!("<sheetData/>{element}<pageMargins left=\"0.7\"/>"))
    );
    // Created before protectedRanges, mergeCells and pageMargins.
    for next in [
        "<protectedRanges/>",
        "<mergeCells count=\"1\"><mergeCell ref=\"A1:B1\"/></mergeCells>",
        "<pageMargins left=\"0.7\"/>",
    ] {
        assert_eq!(
            protected(&format!("<sheetData/>{next}"), |p| {
                p.protect_sheet(SHEET, None, options).expect("protect")
            }),
            worksheet(&format!("<sheetData/>{element}{next}")),
            "{next}"
        );
    }
    // Created before a merge the same write creates.
    assert_eq!(
        protected("<sheetData/>", |p| {
            p.protect_sheet(SHEET, None, options).expect("protect");
            p.merge_cells(SHEET, Area::parse(b"A1:B1").expect("area"))
                .expect("merge");
        }),
        worksheet(&format!(
            "<sheetData/>{element}<mergeCells count=\"1\"><mergeCell ref=\"A1:B1\"/></mergeCells>"
        ))
    );
}

#[test]
fn an_unprotect_drops_the_element_and_leaves_a_part_without_one() {
    assert_eq!(
        protected(
            "<sheetData/><sheetProtection sheet=\"1\"/><pageMargins left=\"0.7\"/>",
            |p| p.unprotect_sheet(SHEET).expect("unprotect")
        ),
        worksheet("<sheetData/><pageMargins left=\"0.7\"/>")
    );
    assert_eq!(
        protected("<sheetData/><pageMargins left=\"0.7\"/>", |p| {
            p.unprotect_sheet(SHEET).expect("unprotect")
        }),
        worksheet("<sheetData/><pageMargins left=\"0.7\"/>")
    );
    // The last record for a part wins.
    assert_eq!(
        protected("<sheetData/>", |p| {
            p.unprotect_sheet(SHEET).expect("unprotect");
            p.protect_sheet(SHEET, None, SheetProtection::default())
                .expect("protect");
        }),
        worksheet("<sheetData/><sheetProtection sheet=\"1\"/>")
    );
}

#[test]
fn a_second_sheet_protection_is_refused() {
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        worksheet("<sheetData/><sheetProtection sheet=\"1\"/><sheetProtection sheet=\"1\"/>"),
    );
    let (_, mut package) = opened(parts);
    package.unprotect_sheet(SHEET).expect("recorded");
    assert_package(
        refused(package),
        "a worksheet has two hyperlinks or sheetProtection elements",
    );
}

#[cfg(feature = "rowcol")]
#[test]
fn a_protection_on_a_part_with_a_pending_shift_is_refused() {
    let (_, mut package) = opened(smallest());
    package.insert_rows(SHEET, 1, 1).expect("recorded");
    package
        .protect_sheet(SHEET, None, SheetProtection::default())
        .expect("recorded");
    assert_package(
        refused(package),
        "a worksheet with a row or column insertion or deletion takes no other edit in the same write",
    );
}

#[test]
fn the_workbook_element_in_its_four_lock_forms_with_and_without_a_password() {
    let hash = format!("{:04X}", legacy_password_hash("emend"));
    for password in [None, Some("emend")] {
        for (structure, windows) in [(false, false), (true, false), (false, true), (true, true)] {
            let mut want = String::from("<workbookProtection");
            if password.is_some() {
                want.push_str(&format!(" workbookPassword=\"{hash}\""));
            }
            if structure {
                want.push_str(" lockStructure=\"1\"");
            }
            if windows {
                want.push_str(" lockWindows=\"1\"");
            }
            want.push_str("/>");
            println!("{want}");
            assert_eq!(
                protected_book("<workbookPr/><bookViews/>", "", |p| {
                    p.protect_workbook(password, structure, windows)
                        .expect("protect")
                }),
                book(&format!("<workbookPr/>{want}<bookViews/>"), ""),
                "{want}"
            );
        }
    }
}

#[test]
fn the_workbook_element_replaces_an_empty_one_and_is_dropped_by_an_unprotect() {
    assert_eq!(
        protected_book("<workbookPr/><workbookProtection/><bookViews/>", "", |p| {
            p.protect_workbook(None, true, false).expect("protect")
        }),
        book(
            "<workbookPr/><workbookProtection lockStructure=\"1\"/><bookViews/>",
            ""
        )
    );
    assert_eq!(
        protected_book("<workbookPr/><workbookProtection/><bookViews/>", "", |p| {
            p.unprotect_workbook()
        }),
        book("<workbookPr/><bookViews/>", "")
    );
    // An unprotect of a workbook with none writes the input's bytes.
    let parts = sheeted(1, "<workbookPr/>", "");
    let output = edited_output(parts.clone(), |p| p.unprotect_workbook());
    assert_untouched(&output, &parts, &[]);
    // Created with no workbookPr, before the sheets; and neither a formula
    // change nor a calcPr.
    assert_eq!(
        protected_book("", "", |p| p
            .protect_workbook(None, false, false)
            .expect("protect")),
        book("<workbookProtection/>", "")
    );
}

/// Fix 2 of `specs/0045-publish.md` (ruling E): the created element goes
/// immediately before the next child of the main namespace in schema order,
/// so an element of another namespace Excel writes after `workbookPr` stays
/// before it.
#[test]
fn the_workbook_element_is_created_before_the_next_main_child_after_a_planted_one() {
    let revision = "<xr:revisionPtr xmlns:xr=\"http://schemas.microsoft.com/office/spreadsheetml/\
                    2014/revision\" revIDLastSave=\"0\"/>";
    let alternate = "<mc:AlternateContent xmlns:mc=\"http://schemas.openxmlformats.org/\
                     markup-compatibility/2006\"><mc:Choice Requires=\"x15\"/>\
                     </mc:AlternateContent>";
    for planted in [
        revision.to_owned(),
        alternate.to_owned(),
        format!("{alternate}{revision}"),
    ] {
        let output = protected_book(&format!("<workbookPr/>{planted}<bookViews/>"), "", |p| {
            p.protect_workbook(None, true, false).expect("protect")
        });
        assert_eq!(
            output,
            book(
                &format!(
                    "<workbookPr/>{planted}<workbookProtection lockStructure=\"1\"/><bookViews/>"
                ),
                ""
            )
        );
        // Read back: the element after the planted one and before bookViews.
        let created = output.find("<workbookProtection").expect("created");
        assert!(output.find(&planted).expect("kept") < created);
        assert!(created < output.find("<bookViews/>").expect("kept"));
    }
    // With no main-namespace child after its place but the sheets, the
    // planted element still goes first.
    assert_eq!(
        protected_book(&format!("<workbookPr/>{revision}"), "", |p| {
            p.protect_workbook(None, false, false).expect("protect")
        }),
        book(&format!("<workbookPr/>{revision}<workbookProtection/>"), "")
    );
}

#[test]
fn a_second_workbook_protection_is_refused() {
    let parts = sheeted(1, "<workbookProtection/><workbookProtection/>", "");
    let (_, mut package) = opened(parts);
    package.unprotect_workbook();
    assert_package(
        refused(package),
        "a workbook has two definedNames or workbookProtection elements",
    );
}
