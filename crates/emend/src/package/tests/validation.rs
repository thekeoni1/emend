//! Data validations on byte-built worksheets
//! (`specs/0020-validation-and-conditional-formatting.md`, "P1", and the P1
//! rulings): each test builds the smallest package with its worksheet
//! holding a given body, records validation edits, and compares the written
//! worksheet with the bytes the design gives, every other part asserted
//! untouched.

use super::*;

/// An area from its `A1` form.
fn area(a1: &str) -> Area {
    Area::parse(a1.as_bytes()).expect("an A1 area")
}

/// The smallest package with its worksheet of `body`.
fn with_body(body: &str) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    parts
}

/// The worksheet after `edit` on a worksheet of `body`, as text; every other
/// part is asserted untouched.
#[track_caller]
fn edited(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let parts = with_body(body);
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[SHEET]);
    String::from_utf8(stored_data(&output, SHEET)).expect("UTF-8")
}

/// The refusal a write of `edit` on a worksheet of `body` gives, with no
/// output.
#[track_caller]
fn refused_on(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(with_body(body));
    edit(&mut package);
    refused(package)
}

/// The validations a worksheet of `body` reads back.
#[track_caller]
fn read(body: &str) -> Result<Vec<ValidationEntry>, Error> {
    let (_, mut package) = opened(with_body(body));
    package.validations(SHEET)
}

/// A whole-number validation between `1` and `10`, with no message.
fn whole() -> Validation<'static> {
    Validation {
        kind: ValidationKind::Whole,
        operator: Some(Operator::Between),
        formula1: "1",
        formula2: Some("10"),
        allow_blank: false,
        hide_drop_down: false,
        input: None,
        error: None,
    }
}

/// A list validation of a quoted list, with no message.
fn list() -> Validation<'static> {
    Validation {
        kind: ValidationKind::List,
        operator: None,
        formula1: "\"a,b\"",
        formula2: None,
        allow_blank: true,
        hide_drop_down: false,
        input: None,
        error: None,
    }
}

/// The element [`whole`] is written as on `sqref`.
fn whole_element(sqref: &str) -> String {
    format!(
        "<dataValidation type=\"whole\" operator=\"between\" allowBlank=\"0\" \
         showInputMessage=\"0\" showErrorMessage=\"0\" sqref=\"{sqref}\">\
         <formula1>1</formula1><formula2>10</formula2></dataValidation>"
    )
}

/// Three validations as a producer writes them: a list with an input
/// message, a whole `between` with two formulas and both messages, and an
/// input message alone with no `type`.
const THREE: &str = "<sheetData/><dataValidations count=\"3\" xWindow=\"5\">\
     <dataValidation type=\"list\" allowBlank=\"1\" showInputMessage=\"1\" \
     showErrorMessage=\"1\" promptTitle=\"Pick\" prompt=\"one &amp; only\" sqref=\"A1:A3 C1\">\
     <formula1>$E$1:$E$4</formula1></dataValidation>\
     <dataValidation type=\"whole\" errorStyle=\"warning\" operator=\"between\" \
     allowBlank=\"1\" showInputMessage=\"1\" showErrorMessage=\"1\" errorTitle=\"No\" \
     error=\"1 to 10\" promptTitle=\"Count\" prompt=\"a number\" sqref=\"B2\">\
     <formula1>1</formula1><formula2>10</formula2></dataValidation>\
     <dataValidation allowBlank=\"1\" showInputMessage=\"1\" showErrorMessage=\"1\" \
     prompt=\"read me\" sqref=\"D5:D9\"/></dataValidations><pageMargins left=\"0.7\"/>";

#[test]
fn the_validations_of_a_part_with_three_and_of_one_with_none_are_read() {
    let entries = read(THREE).expect("read");
    assert_eq!(
        entries,
        vec![
            ValidationEntry {
                areas: vec![area("A1:A3"), area("C1")],
                kind: Some(ValidationKind::List),
                operator: None,
                formula1: Some("$E$1:$E$4".to_string()),
                formula2: None,
                allow_blank: true,
                hide_drop_down: false,
                input: Some(("Pick".to_string(), "one & only".to_string())),
                error: Some((ErrorStyle::Stop, String::new(), String::new())),
            },
            ValidationEntry {
                areas: vec![area("B2")],
                kind: Some(ValidationKind::Whole),
                operator: Some(Operator::Between),
                formula1: Some("1".to_string()),
                formula2: Some("10".to_string()),
                allow_blank: true,
                hide_drop_down: false,
                input: Some(("Count".to_string(), "a number".to_string())),
                error: Some((ErrorStyle::Warning, "No".to_string(), "1 to 10".to_string())),
            },
            ValidationEntry {
                areas: vec![area("D5:D9")],
                kind: None,
                operator: None,
                formula1: None,
                formula2: None,
                allow_blank: true,
                hide_drop_down: false,
                input: Some((String::new(), "read me".to_string())),
                error: Some((ErrorStyle::Stop, String::new(), String::new())),
            },
        ]
    );
    assert_eq!(read("<sheetData/>").expect("read"), Vec::new());
    // An absent `showInputMessage` or `showErrorMessage` is the schema's
    // false (P1 ruling 3), and `none` is no kind.
    let entries = read(
        "<sheetData/><dataValidations count=\"1\"><dataValidation type=\"none\" \
         showDropDown=\"true\" sqref=\"A1\"/></dataValidations>",
    )
    .expect("read");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].kind, None);
    assert_eq!(entries[0].input, None);
    assert_eq!(entries[0].error, None);
    assert!(entries[0].hide_drop_down);
    assert!(!entries[0].allow_blank);
}

#[test]
fn a_read_refuses_an_unparsed_sqref_or_an_enumeration_value_it_does_not_name() {
    for element in [
        "<dataValidation sqref=\"A1:\"/>",
        "<dataValidation/>",
        "<dataValidation sqref=\"\"/>",
        "<dataValidation type=\"wholeNumber\" sqref=\"A1\"/>",
        "<dataValidation operator=\"between2\" sqref=\"A1\"/>",
        "<dataValidation errorStyle=\"halt\" sqref=\"A1\"/>",
    ] {
        let body = format!("<sheetData/><dataValidations>{element}</dataValidations>");
        assert_package(
            read(&body).expect_err("refused"),
            "a validation overlaps one the worksheet has, or a dataValidation sqref does not parse",
        );
    }
    assert_package(
        read("<sheetData/><dataValidations><dataValidation q:type=\"list\" sqref=\"A1\"/></dataValidations>")
            .expect_err("refused"),
        "an element or attribute uses a prefix its document does not bind",
    );
}

#[test]
fn every_kind_and_operator_is_written_as_its_literal_bytes() {
    let kinds = [
        (ValidationKind::Whole, "whole"),
        (ValidationKind::Decimal, "decimal"),
        (ValidationKind::Date, "date"),
        (ValidationKind::Time, "time"),
        (ValidationKind::TextLength, "textLength"),
    ];
    let operators = [
        (Operator::Between, "between", true),
        (Operator::NotBetween, "notBetween", true),
        (Operator::Equal, "equal", false),
        (Operator::NotEqual, "notEqual", false),
        (Operator::GreaterThan, "greaterThan", false),
        (Operator::LessThan, "lessThan", false),
        (Operator::GreaterThanOrEqual, "greaterThanOrEqual", false),
        (Operator::LessThanOrEqual, "lessThanOrEqual", false),
    ];
    for (kind, kind_name) in kinds {
        for (operator, operator_name, two) in operators {
            let validation = Validation {
                kind,
                operator: Some(operator),
                formula2: two.then_some("B1<2"),
                ..whole()
            };
            let sheet = edited("<sheetData/>", |p| {
                p.add_validation(SHEET, &[area("C3")], validation)
                    .expect("add")
            });
            let second = if two {
                "<formula2>B1&lt;2</formula2>"
            } else {
                ""
            };
            assert_eq!(
                sheet,
                worksheet(&format!(
                    "<sheetData/><dataValidations count=\"1\"><dataValidation \
                     type=\"{kind_name}\" operator=\"{operator_name}\" allowBlank=\"0\" \
                     showInputMessage=\"0\" showErrorMessage=\"0\" sqref=\"C3\">\
                     <formula1>1</formula1>{second}</dataValidation></dataValidations>"
                ))
            );
        }
        // No operator writes none, which reads as `between` and takes two
        // formulas (P1 ruling 2).
        let sheet = edited("<sheetData/>", |p| {
            p.add_validation(
                SHEET,
                &[area("C3")],
                Validation {
                    kind,
                    operator: None,
                    ..whole()
                },
            )
            .expect("add")
        });
        assert_eq!(
            sheet,
            worksheet(&format!(
                "<sheetData/><dataValidations count=\"1\"><dataValidation \
                 type=\"{kind_name}\" allowBlank=\"0\" showInputMessage=\"0\" \
                 showErrorMessage=\"0\" sqref=\"C3\"><formula1>1</formula1>\
                 <formula2>10</formula2></dataValidation></dataValidations>"
            ))
        );
    }
    // A list with a hidden arrow, both messages escaped and each error
    // style; a custom formula.
    for (style, style_name) in [
        (ErrorStyle::Stop, "stop"),
        (ErrorStyle::Warning, "warning"),
        (ErrorStyle::Information, "information"),
    ] {
        let sheet = edited("<sheetData/>", |p| {
            p.add_validation(
                SHEET,
                &[area("B2:A1"), area("D4")],
                Validation {
                    hide_drop_down: true,
                    input: Some(("In \"t\"", "a\tb")),
                    error: Some((style, "E & t", "x<y")),
                    ..list()
                },
            )
            .expect("add")
        });
        assert_eq!(
            sheet,
            worksheet(&format!(
                "<sheetData/><dataValidations count=\"1\"><dataValidation type=\"list\" \
                 allowBlank=\"1\" showDropDown=\"1\" showInputMessage=\"1\" \
                 showErrorMessage=\"1\" errorStyle=\"{style_name}\" errorTitle=\"E &amp; t\" \
                 error=\"x&lt;y\" promptTitle=\"In &quot;t&quot;\" prompt=\"a&#9;b\" \
                 sqref=\"A1:B2 D4\"><formula1>\"a,b\"</formula1></dataValidation>\
                 </dataValidations>"
            ))
        );
    }
    let sheet = edited("<sheetData/>", |p| {
        p.add_validation(
            SHEET,
            &[area("A1")],
            Validation {
                kind: ValidationKind::Custom,
                formula1: "AND(A1>0,A1<>\"x\")",
                ..list()
            },
        )
        .expect("add")
    });
    assert_eq!(
        sheet,
        worksheet(
            "<sheetData/><dataValidations count=\"1\"><dataValidation type=\"custom\" \
             allowBlank=\"1\" showInputMessage=\"0\" showErrorMessage=\"0\" sqref=\"A1\">\
             <formula1>AND(A1&gt;0,A1&lt;&gt;\"x\")</formula1></dataValidation>\
             </dataValidations>"
        )
    );
}

#[test]
fn an_addition_is_appended_to_a_container_its_count_updated_and_the_rest_kept() {
    let sheet = edited(THREE, |p| {
        p.add_validation(SHEET, &[area("F1")], whole())
            .expect("add");
        p.add_validation(SHEET, &[area("G1:G2")], list())
            .expect("add");
    });
    let want = THREE.replace("count=\"3\"", "count=\"5\"").replace(
        "</dataValidations>",
        &format!(
            "{}<dataValidation type=\"list\" allowBlank=\"1\" showInputMessage=\"0\" \
                 showErrorMessage=\"0\" sqref=\"G1:G2\"><formula1>\"a,b\"</formula1>\
                 </dataValidation></dataValidations>",
            whole_element("F1")
        ),
    );
    assert_eq!(sheet, worksheet(&want));
    // A container without `count` gains one; an empty one is opened up; a
    // prefixed one gives its prefix to the addition.
    let sheet = edited(
        "<sheetData/><dataValidations><dataValidation sqref=\"A1\"/></dataValidations>",
        |p| {
            p.add_validation(SHEET, &[area("F1")], whole())
                .expect("add")
        },
    );
    assert_eq!(
        sheet,
        worksheet(&format!(
            "<sheetData/><dataValidations count=\"2\"><dataValidation sqref=\"A1\"/>{}\
             </dataValidations>",
            whole_element("F1")
        ))
    );
    let sheet = edited("<sheetData/><dataValidations count=\"0\"/>", |p| {
        p.add_validation(SHEET, &[area("F1")], whole())
            .expect("add")
    });
    assert_eq!(
        sheet,
        worksheet(&format!(
            "<sheetData/><dataValidations count=\"1\">{}</dataValidations>",
            whole_element("F1")
        ))
    );
    let prefixed = format!(
        "<sheetData/><v:dataValidations xmlns:v=\"{MAIN_NS}\" count=\"1\">\
         <v:dataValidation sqref=\"A1\"/></v:dataValidations>"
    );
    let sheet = edited(&prefixed, |p| {
        p.add_validation(SHEET, &[area("F1")], whole())
            .expect("add")
    });
    assert_eq!(
        sheet,
        worksheet(&prefixed.replace("count=\"1\"", "count=\"2\"").replace(
            "</v:dataValidations>",
            &format!(
                        "{}</v:dataValidations>",
                        whole_element("F1")
                            .replace("<data", "<v:data")
                            .replace("</data", "</v:data")
                            .replace("<formula", "<v:formula")
                            .replace("</formula", "</v:formula")
                    )
        ))
    );
}

#[test]
fn a_container_is_created_before_hyperlinks_page_margins_ext_lst_or_the_root_end() {
    let element = whole_element("A1");
    let created = format!("<dataValidations count=\"1\">{element}</dataValidations>");
    for (body, before) in [
        (
            "<sheetData/><hyperlinks><hyperlink ref=\"B2\" location=\"S!A1\"/></hyperlinks>\
             <pageMargins left=\"0.7\"/>",
            "<hyperlinks>",
        ),
        (
            "<sheetData/><conditionalFormatting sqref=\"A1\"/><pageMargins left=\"0.7\"/>",
            "<pageMargins",
        ),
        ("<sheetData/><extLst><ext uri=\"x\"/></extLst>", "<extLst>"),
    ] {
        let sheet = edited(body, |p| {
            p.add_validation(SHEET, &[area("A1")], whole())
                .expect("add")
        });
        println!("created before {before}: {sheet}");
        assert_eq!(
            sheet,
            worksheet(&body.replacen(before, &format!("{created}{before}"), 1))
        );
    }
    // The template's worksheet: at the root's end.
    let mut package = new_package();
    package
        .add_validation(SHEET, &[area("A1")], whole())
        .expect("add");
    let out = {
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = Vec::new();
        package.write_to(&mut out).expect("write_to");
        out
    };
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    let index = reopened.part(SHEET).expect("the part");
    let mut sheet = String::new();
    reopened
        .read_part(index)
        .expect("read_part")
        .read_to_string(&mut sheet)
        .expect("UTF-8");
    println!("template worksheet after the validation: {sheet}");
    let template = String::from_utf8(TEMPLATE[4].1.to_vec()).expect("UTF-8");
    let want = template.replace("</worksheet>", &format!("{created}</worksheet>"));
    assert_ne!(want, template, "the template's root end was found");
    assert_eq!(sheet, want);
    assert_eq!(
        reopened.validations(SHEET).expect("read").len(),
        1,
        "the created validation reads back"
    );
}

#[test]
fn an_overlap_with_a_kept_validation_or_an_earlier_addition_is_refused() {
    let message =
        "a validation overlaps one the worksheet has, or a dataValidation sqref does not parse";
    // C1 is the kept list's second area.
    assert_package(
        refused_on(THREE, |p| {
            p.add_validation(SHEET, &[area("F9"), area("B1:C1")], whole())
                .expect("recorded")
        }),
        message,
    );
    assert_package(
        refused_on("<sheetData/>", |p| {
            p.add_validation(SHEET, &[area("A1:B2")], whole())
                .expect("recorded");
            p.add_validation(SHEET, &[area("B2:C3")], list())
                .expect("recorded");
        }),
        message,
    );
    // A removal and an addition of one list replace the validation.
    let sheet = edited(THREE, |p| {
        p.remove_validation(SHEET, &[area("B2")]).expect("recorded");
        p.add_validation(SHEET, &[area("B2")], whole())
            .expect("add");
    });
    let kept_second = THREE
        .find("<dataValidation type=\"whole\"")
        .expect("the second element");
    let third = THREE
        .find("<dataValidation allowBlank")
        .expect("the third element");
    let want = format!("{}{}", &THREE[..kept_second], &THREE[third..]).replace(
        "</dataValidations>",
        &format!("{}</dataValidations>", whole_element("B2")),
    );
    assert_eq!(sheet, worksheet(&want));
}

#[test]
fn a_removal_drops_its_list_in_order_and_the_last_drops_the_container() {
    let sheet = edited(THREE, |p| {
        p.remove_validation(SHEET, &[area("A1:A3"), area("C1")])
            .expect("recorded")
    });
    let second = THREE
        .find("<dataValidation type=\"whole\"")
        .expect("the second element");
    let first = THREE
        .find("<dataValidation type=\"list\"")
        .expect("the first element");
    let want =
        format!("{}{}", &THREE[..first], &THREE[second..]).replace("count=\"3\"", "count=\"2\"");
    assert_eq!(sheet, worksheet(&want));
    // Every file element of the list goes; the last drops the container.
    let sheet = edited(
        "<sheetData/><dataValidations count=\"2\"><dataValidation sqref=\"A1\"/>\
         <dataValidation sqref=\"A1\"><formula1>1</formula1></dataValidation>\
         </dataValidations><pageMargins left=\"0.7\"/>",
        |p| p.remove_validation(SHEET, &[area("A1")]).expect("recorded"),
    );
    assert_eq!(sheet, worksheet("<sheetData/><pageMargins left=\"0.7\"/>"));
    for (body, areas) in [
        (THREE, vec![area("C1"), area("A1:A3")]),
        (THREE, vec![area("A1:A3")]),
        (THREE, vec![area("Z9")]),
        ("<sheetData/>", vec![area("A1")]),
    ] {
        assert_package(
            refused_on(body, |p| {
                p.remove_validation(SHEET, &areas).expect("recorded")
            }),
            "no validation on those areas",
        );
    }
    // A removal matches the file's elements only, never an addition of the
    // same write (P1 ruling 5).
    assert_package(
        refused_on("<sheetData/>", |p| {
            p.add_validation(SHEET, &[area("A1")], whole())
                .expect("add");
            p.remove_validation(SHEET, &[area("A1")]).expect("recorded");
        }),
        "no validation on those areas",
    );
}

#[test]
fn two_containers_or_a_kept_sqref_that_does_not_parse_are_refused() {
    assert_package(
        refused_on(
            "<sheetData/><dataValidations count=\"1\"><dataValidation sqref=\"A1\"/>\
             </dataValidations><dataValidations/>",
            |p| {
                p.add_validation(SHEET, &[area("F1")], whole())
                    .expect("recorded")
            },
        ),
        "a worksheet has two dataValidations elements",
    );
    // A container created before a root child out of schema order, then met.
    assert_package(
        refused_on(
            "<sheetData/><pageMargins left=\"0.7\"/><dataValidations/>",
            |p| {
                p.add_validation(SHEET, &[area("F1")], whole())
                    .expect("recorded")
            },
        ),
        "a worksheet has two dataValidations elements",
    );
    assert_package(
        refused_on(
            "<sheetData/><dataValidations count=\"1\"><dataValidation sqref=\"A1 B\"/>\
             </dataValidations>",
            |p| {
                p.add_validation(SHEET, &[area("F1")], whole())
                    .expect("recorded")
            },
        ),
        "a validation overlaps one the worksheet has, or a dataValidation sqref does not parse",
    );
}

#[test]
fn the_call_refuses_each_rule_and_records_nothing() {
    let refusals: [(&[Area], Validation<'static>, &str); 17] = [
        (
            &[],
            whole(),
            "a validation or a conditional format has no area",
        ),
        (
            &[Area {
                first: CellRef { row: 1, col: 1 },
                last: CellRef {
                    row: 1_048_577,
                    col: 1,
                },
            }],
            whole(),
            "a row or column index or count is out of range",
        ),
        (
            &[Area {
                first: CellRef { row: 0, col: 1 },
                last: CellRef { row: 1, col: 1 },
            }],
            whole(),
            "a row or column index or count is out of range",
        ),
        (
            &[],
            Validation {
                formula1: "",
                ..whole()
            },
            "a validation or a conditional format has no area",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                formula1: "",
                ..whole()
            },
            "a validation or a rule formula is empty or missing",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                operator: Some(Operator::Equal),
                ..list()
            },
            "a validation field does not apply to its kind",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                operator: Some(Operator::Equal),
                ..whole()
            },
            "a validation or a rule formula is empty or missing",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                hide_drop_down: true,
                ..whole()
            },
            "a validation field does not apply to its kind",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                input: Some(("123456789012345678901234567890123", "t")),
                ..whole()
            },
            "a validation message is too long",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                error: Some((ErrorStyle::Stop, "t", "a\u{1}b")),
                ..whole()
            },
            "a text holds a character XML cannot carry",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                formula1: "=A1",
                ..whole()
            },
            "a formula text starts with an equals sign",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                formula2: Some("\u{b}"),
                ..whole()
            },
            "a text holds a character XML cannot carry",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                formula2: Some("2"),
                ..list()
            },
            "a validation or a rule formula is empty or missing",
        ),
        // P1 verification ruling 5: a missing or empty second formula for
        // the two-value operators, and an operator on a custom rule.
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                formula2: None,
                ..whole()
            },
            "a validation or a rule formula is empty or missing",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                operator: Some(Operator::NotBetween),
                formula2: None,
                ..whole()
            },
            "a validation or a rule formula is empty or missing",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                formula2: Some(""),
                ..whole()
            },
            "a validation or a rule formula is empty or missing",
        ),
        (
            &[Area::parse(b"A1").expect("A1")],
            Validation {
                kind: ValidationKind::Custom,
                operator: Some(Operator::Equal),
                ..list()
            },
            "a validation field does not apply to its kind",
        ),
    ];
    let parts = with_body("<sheetData/>");
    let (_, mut package) = opened(parts.clone());
    for (areas, validation, message) in refusals {
        assert_package(
            package
                .add_validation(SHEET, areas, validation)
                .expect_err("refused"),
            message,
        );
    }
    let long_text = "x".repeat(256);
    assert_package(
        package
            .add_validation(
                SHEET,
                &[area("A1")],
                Validation {
                    error: Some((ErrorStyle::Stop, "t", &long_text)),
                    ..whole()
                },
            )
            .expect_err("refused"),
        "a validation message is too long",
    );
    assert_package(
        package.remove_validation(SHEET, &[]).expect_err("refused"),
        "a validation or a conditional format has no area",
    );
    // Nothing recorded: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
    // The limits themselves are taken: a title of 32 characters, a text of
    // 255, counted in characters (P1 ruling 5), and a missing operator on a
    // kind that takes one reads as `between`.
    let (_, mut package) = opened(with_body("<sheetData/>"));
    let title = "\u{e9}".repeat(32);
    let text = "\u{e9}".repeat(255);
    package
        .add_validation(
            SHEET,
            &[area("A1")],
            Validation {
                operator: None,
                input: Some((&title, &text)),
                ..whole()
            },
        )
        .expect("the limits are taken");
}

#[cfg(feature = "rowcol")]
#[test]
fn a_validation_edit_with_a_row_insertion_is_refused() {
    let message = "a worksheet with a row or column insertion or deletion takes no other edit in the same write";
    assert_package(
        refused_on(THREE, |p| {
            p.insert_rows(SHEET, 1, 1).expect("recorded");
            p.add_validation(SHEET, &[area("F1")], whole())
                .expect("recorded");
        }),
        message,
    );
    assert_package(
        refused_on(THREE, |p| {
            p.insert_rows(SHEET, 1, 1).expect("recorded");
            p.remove_validation(SHEET, &[area("B2")]).expect("recorded");
        }),
        message,
    );
}

#[test]
fn a_validation_edit_on_a_removed_or_replaced_part_is_refused() {
    type Edit = fn(&mut Package<Cursor<Vec<u8>>>);
    let edits: [Edit; 2] = [
        |p| {
            p.add_validation(SHEET, &[area("A1")], whole())
                .expect("recorded")
        },
        |p| p.remove_validation(SHEET, &[area("A1")]).expect("recorded"),
    ];
    for edit in edits {
        let (_, mut package) = opened(smallest());
        edit(&mut package);
        package.remove_part(SHEET);
        assert_package(
            refusal(package).expect("refused"),
            "an edit to a removed part",
        );
        let (_, mut package) = opened(smallest());
        edit(&mut package);
        package.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
        assert_package(
            refusal(package).expect("refused"),
            "a cell edit to a part this write replaces",
        );
    }
}

#[test]
fn a_validation_and_a_cell_edit_on_one_part_compose() {
    let sheet = edited(THREE, |p| {
        p.set_cell(SHEET, at("A1"), Value::Number(3.0));
        p.remove_validation(SHEET, &[area("D5:D9")])
            .expect("recorded");
    });
    let third = THREE
        .find("<dataValidation allowBlank")
        .expect("the third element");
    let end = THREE.find("</dataValidations>").expect("the end tag");
    let want = format!("{}{}", &THREE[..third], &THREE[end..])
        .replace("count=\"3\"", "count=\"2\"")
        .replacen(
            "<sheetData/>",
            "<sheetData><row r=\"1\"><c r=\"A1\"><v>3</v></c></row></sheetData>",
            1,
        );
    assert_eq!(sheet, worksheet(&want));
}

#[test]
fn a_validation_edit_on_a_sheet_added_and_removed_in_one_write_goes_with_it() {
    // P1 verification ruling 5 and P2 ruling 1: the case is a PENDING sheet,
    // added and removed again in one write, whose edits are dropped with it,
    // so the write is accepted and is the input. A validation edit on a sheet
    // the file has, whose part the write removes, is refused instead
    // (`a_validation_edit_on_a_removed_or_replaced_part_is_refused`).
    let parts = smallest();
    let bytes = Build::new(
        parts
            .iter()
            .map(|(name, data)| Spec::new(name, data))
            .collect(),
    )
    .bytes();
    let mut package = open_fixture(bytes.clone());
    let part = package.add_sheet("Two").expect("add");
    package
        .add_validation(&part, &[area("A1")], whole())
        .expect("recorded");
    package
        .remove_validation(&part, &[area("B2")])
        .expect("recorded");
    package.remove_sheet("Two").expect("remove");
    let out = written(package);
    assert_eq!(out, bytes, "the output is the input");
}
