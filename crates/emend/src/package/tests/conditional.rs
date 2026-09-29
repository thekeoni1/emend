//! Conditional formats on byte-built worksheets
//! (`specs/0020-validation-and-conditional-formatting.md`, "Conditional
//! formatting", "P2", and the P2 rulings): each test builds the smallest
//! package with its worksheet holding a given body and its styles part two
//! differential styles, records conditional format edits, and compares the
//! written worksheet with the bytes the design gives, every other part
//! asserted untouched.

use super::*;

/// An area from its `A1` form.
fn area(a1: &str) -> Area {
    Area::parse(a1.as_bytes()).expect("an A1 area")
}

/// The smallest package with its worksheet of `body` and a styles part of
/// two `dxf` children.
fn with_body(body: &str) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    set(
        &mut parts,
        STYLES_PART,
        style_sheet("<cellXfs count=\"1\"><xf/></cellXfs><dxfs count=\"2\"><dxf/><dxf/></dxfs>"),
    );
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

/// The rules a worksheet of `body` reads back.
#[track_caller]
fn read(body: &str) -> Result<Vec<RuleEntry>, Error> {
    let (_, mut package) = opened(with_body(body));
    package.conditional_formats(SHEET)
}

/// Records `rule` on `areas` with style 0, no priority and no stop.
#[track_caller]
fn add(package: &mut Package<Cursor<Vec<u8>>>, areas: &[Area], rule: Rule<'_>) {
    package
        .add_conditional_format(SHEET, areas, rule, Some(0), None, false)
        .expect("recorded");
}

/// A `cellIs` greater-than rule of `5`.
fn greater() -> Rule<'static> {
    Rule::CellIs {
        operator: Operator::GreaterThan,
        formula1: "5",
        formula2: None,
    }
}

/// Two conditional formats as a producer writes them, the second with two
/// rules, before `dataValidations` and `pageMargins`.
const TWO: &str = "<sheetData/><conditionalFormatting sqref=\"A1:A5\">\
     <cfRule type=\"cellIs\" dxfId=\"1\" priority=\"3\" operator=\"lessThan\">\
     <formula>0</formula></cfRule></conditionalFormatting>\
     <conditionalFormatting sqref=\"C1 E1:E2\"><cfRule type=\"expression\" dxfId=\"0\" \
     priority=\"1\" stopIfTrue=\"1\"><formula>$C1&gt;2</formula></cfRule>\
     <cfRule type=\"containsText\" dxfId=\"0\" priority=\"7\" operator=\"containsText\" \
     text=\"x\"><formula>NOT(ISERROR(SEARCH(\"x\",C1)))</formula></cfRule>\
     </conditionalFormatting><dataValidations count=\"1\"><dataValidation sqref=\"Z1\"/>\
     </dataValidations><pageMargins left=\"0.7\"/>";

/// The byte offset in [`TWO`] just past the last conditional format.
fn after_two() -> usize {
    TWO.find("<dataValidations").expect("the validations")
}

#[test]
fn every_dxf_rule_kind_is_written_as_its_literal_bytes() {
    let cases: [(Rule<'static>, &str); 11] = [
        (
            greater(),
            "<cfRule type=\"cellIs\" dxfId=\"0\" priority=\"1\" operator=\"greaterThan\">\
             <formula>5</formula></cfRule>",
        ),
        (
            Rule::CellIs {
                operator: Operator::NotBetween,
                formula1: "1",
                formula2: Some("A1<9"),
            },
            "<cfRule type=\"cellIs\" dxfId=\"0\" priority=\"1\" operator=\"notBetween\">\
             <formula>1</formula><formula>A1&lt;9</formula></cfRule>",
        ),
        (
            Rule::Expression("AND($B2>0,$B2<>\"x\")"),
            "<cfRule type=\"expression\" dxfId=\"0\" priority=\"1\">\
             <formula>AND($B2&gt;0,$B2&lt;&gt;\"x\")</formula></cfRule>",
        ),
        (
            Rule::ContainsBlanks,
            "<cfRule type=\"containsBlanks\" dxfId=\"0\" priority=\"1\">\
             <formula>LEN(TRIM(B2))=0</formula></cfRule>",
        ),
        (
            Rule::NotContainsBlanks,
            "<cfRule type=\"notContainsBlanks\" dxfId=\"0\" priority=\"1\">\
             <formula>LEN(TRIM(B2))&gt;0</formula></cfRule>",
        ),
        (
            Rule::DuplicateValues,
            "<cfRule type=\"duplicateValues\" dxfId=\"0\" priority=\"1\"/>",
        ),
        (
            Rule::UniqueValues,
            "<cfRule type=\"uniqueValues\" dxfId=\"0\" priority=\"1\"/>",
        ),
        (
            Rule::Top {
                rank: 10,
                percent: false,
                bottom: false,
            },
            "<cfRule type=\"top10\" dxfId=\"0\" priority=\"1\" rank=\"10\"/>",
        ),
        (
            Rule::Top {
                rank: 1000,
                percent: true,
                bottom: true,
            },
            "<cfRule type=\"top10\" dxfId=\"0\" priority=\"1\" rank=\"1000\" percent=\"1\" \
             bottom=\"1\"/>",
        ),
        (
            Rule::AboveAverage {
                above: true,
                equal: false,
            },
            "<cfRule type=\"aboveAverage\" dxfId=\"0\" priority=\"1\"/>",
        ),
        (
            Rule::AboveAverage {
                above: false,
                equal: true,
            },
            "<cfRule type=\"aboveAverage\" dxfId=\"0\" priority=\"1\" aboveAverage=\"0\" \
             equalAverage=\"1\"/>",
        ),
    ];
    for (rule, element) in cases {
        // The blank rules' formula names the FIRST area's top-left cell.
        let sheet = edited("<sheetData/>", |p| {
            add(p, &[area("C3:B2"), area("A1")], rule)
        });
        assert_eq!(
            sheet,
            worksheet(&format!(
                "<sheetData/><conditionalFormatting sqref=\"B2:C3 A1\">{element}\
                 </conditionalFormatting>"
            ))
        );
    }
    // A stop, a given priority and the second style.
    let sheet = edited("<sheetData/>", |p| {
        p.add_conditional_format(SHEET, &[area("A1")], greater(), Some(1), Some(9), true)
            .expect("recorded")
    });
    assert_eq!(
        sheet,
        worksheet(
            "<sheetData/><conditionalFormatting sqref=\"A1\"><cfRule type=\"cellIs\" \
             dxfId=\"1\" priority=\"9\" stopIfTrue=\"1\" operator=\"greaterThan\">\
             <formula>5</formula></cfRule></conditionalFormatting>"
        )
    );
}

#[test]
fn priorities_follow_the_highest_kept_one_and_a_given_one_is_written_as_given() {
    // The kept rules' highest priority is 7; two additions without one take
    // 8 and 9, and one with one keeps it, in record order.
    let sheet = edited(TWO, |p| {
        add(p, &[area("G1")], greater());
        p.add_conditional_format(SHEET, &[area("H1")], greater(), Some(0), Some(2), false)
            .expect("recorded");
        add(p, &[area("I1")], Rule::DuplicateValues);
    });
    let at = after_two();
    let want = format!(
        "{}<conditionalFormatting sqref=\"G1\"><cfRule type=\"cellIs\" dxfId=\"0\" \
         priority=\"8\" operator=\"greaterThan\"><formula>5</formula></cfRule>\
         </conditionalFormatting><conditionalFormatting sqref=\"H1\"><cfRule \
         type=\"cellIs\" dxfId=\"0\" priority=\"2\" operator=\"greaterThan\">\
         <formula>5</formula></cfRule></conditionalFormatting>\
         <conditionalFormatting sqref=\"I1\"><cfRule type=\"duplicateValues\" dxfId=\"0\" \
         priority=\"9\"/></conditionalFormatting>{}",
        &TWO[..at],
        &TWO[at..]
    );
    assert_eq!(sheet, worksheet(&want));
    // A removed element's priorities are not counted: without the second
    // element the highest kept is 3.
    let sheet = edited(TWO, |p| {
        p.remove_conditional_format(SHEET, &[area("C1"), area("E1:E2")])
            .expect("recorded");
        add(p, &[area("G1")], Rule::UniqueValues);
    });
    let second = TWO
        .find("<conditionalFormatting sqref=\"C1")
        .expect("the second element");
    let want = format!(
        "{}<conditionalFormatting sqref=\"G1\"><cfRule type=\"uniqueValues\" dxfId=\"0\" \
         priority=\"4\"/></conditionalFormatting>{}",
        &TWO[..second],
        &TWO[at..]
    );
    assert_eq!(sheet, worksheet(&want));
    // A kept priority that does not parse, or is absent, refuses the write.
    for bad in ["priority=\"x\"", "priority=\"01\"", ""] {
        let body = format!(
            "<sheetData/><conditionalFormatting sqref=\"A1\"><cfRule type=\"expression\" \
             dxfId=\"0\" {bad}><formula>1</formula></cfRule></conditionalFormatting>"
        );
        assert_package(
            refused_on(&body, |p| add(p, &[area("G1")], greater())),
            "a conditionalFormatting sqref or a cfRule attribute does not parse",
        );
    }
}

#[test]
fn the_additions_are_created_before_the_next_child_in_schema_order_or_the_root_end() {
    let element = "<conditionalFormatting sqref=\"A1\"><cfRule type=\"duplicateValues\" \
                   dxfId=\"0\" priority=\"1\"/></conditionalFormatting>";
    for (body, before) in [
        (
            "<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"A1:B1\"/></mergeCells>\
             <dataValidations count=\"1\"><dataValidation sqref=\"Z1\"/></dataValidations>",
            "<dataValidations",
        ),
        (
            "<sheetData/><hyperlinks><hyperlink ref=\"B2\" location=\"S!A1\"/></hyperlinks>",
            "<hyperlinks",
        ),
        ("<sheetData/><phoneticPr fontId=\"1\"/>", "</worksheet>"),
    ] {
        let sheet = edited(body, |p| add(p, &[area("A1")], Rule::DuplicateValues));
        println!("created before {before}: {sheet}");
        assert_eq!(
            sheet,
            worksheet(body).replacen(before, &format!("{element}{before}"), 1)
        );
    }
    // With a validation created on the same part, the conditional format
    // goes first, as the schema orders them.
    let sheet = edited("<sheetData/><pageMargins left=\"0.7\"/>", |p| {
        add(p, &[area("A1")], Rule::DuplicateValues);
        p.add_validation(
            SHEET,
            &[area("B1")],
            Validation {
                kind: ValidationKind::Custom,
                operator: None,
                formula1: "TRUE",
                formula2: None,
                allow_blank: false,
                hide_drop_down: false,
                input: None,
                error: None,
            },
        )
        .expect("recorded");
    });
    assert_eq!(
        sheet,
        worksheet(&format!(
            "<sheetData/>{element}<dataValidations count=\"1\"><dataValidation \
             type=\"custom\" allowBlank=\"0\" showInputMessage=\"0\" showErrorMessage=\"0\" \
             sqref=\"B1\"><formula1>TRUE</formula1></dataValidation></dataValidations>\
             <pageMargins left=\"0.7\"/>"
        ))
    );
}

#[test]
fn a_removal_drops_its_list_and_an_absent_or_extended_one_is_refused() {
    let sheet = edited(TWO, |p| {
        p.remove_conditional_format(SHEET, &[area("A1:A5")])
            .expect("recorded")
    });
    let second = TWO
        .find("<conditionalFormatting sqref=\"C1")
        .expect("the second element");
    let first = TWO
        .find("<conditionalFormatting")
        .expect("the first element");
    assert_eq!(
        sheet,
        worksheet(&format!("{}{}", &TWO[..first], &TWO[second..]))
    );
    for areas in [
        vec![area("E1:E2"), area("C1")],
        vec![area("C1")],
        vec![area("Z9")],
    ] {
        assert_package(
            refused_on(TWO, |p| {
                p.remove_conditional_format(SHEET, &areas)
                    .expect("recorded")
            }),
            "no conditional format on those areas",
        );
    }
    let extended = "<sheetData/><conditionalFormatting sqref=\"A1:A9\"><cfRule \
                    type=\"dataBar\" priority=\"1\"><dataBar><cfvo type=\"min\"/>\
                    <cfvo type=\"max\"/><color rgb=\"FF638EC6\"/></dataBar><extLst><ext \
                    uri=\"x\"/></extLst></cfRule></conditionalFormatting>";
    assert_package(
        refused_on(extended, |p| {
            p.remove_conditional_format(SHEET, &[area("A1:A9")])
                .expect("recorded")
        }),
        "a conditional format has an extension this part does not remove",
    );
    // A kept sqref that does not parse refuses the write.
    assert_package(
        refused_on(
            "<sheetData/><conditionalFormatting sqref=\"A1:\"><cfRule type=\"expression\" \
             dxfId=\"0\" priority=\"1\"/></conditionalFormatting>",
            |p| add(p, &[area("G1")], greater()),
        ),
        "a conditionalFormatting sqref or a cfRule attribute does not parse",
    );
}

#[test]
fn the_rules_are_read_back_a_kind_this_part_does_not_write_and_an_extension_included() {
    let entries = read(TWO).expect("read");
    assert_eq!(
        entries[..2],
        [
            RuleEntry {
                areas: vec![area("A1:A5")],
                kind: "cellIs".to_string(),
                operator: Some(Operator::LessThan),
                formulas: vec!["0".to_string()],
                dxf: Some(1),
                priority: 3,
                stop_if_true: false,
                extended: false,
            },
            RuleEntry {
                areas: vec![area("C1"), area("E1:E2")],
                kind: "expression".to_string(),
                operator: None,
                formulas: vec!["$C1>2".to_string()],
                dxf: Some(0),
                priority: 1,
                stop_if_true: true,
                extended: false,
            },
        ]
    );
    // A kind this part does not write is listed by its `type` text.
    assert_eq!(
        entries[2..],
        [RuleEntry {
            areas: vec![area("C1"), area("E1:E2")],
            kind: "containsText".to_string(),
            operator: None,
            formulas: vec!["NOT(ISERROR(SEARCH(\"x\",C1)))".to_string()],
            dxf: Some(0),
            priority: 7,
            stop_if_true: false,
            extended: false,
        }]
    );
    let extended = read(
        "<sheetData/><conditionalFormatting sqref=\"A1:A9\"><cfRule type=\"dataBar\" \
         priority=\"2\"><dataBar><cfvo type=\"min\"/><cfvo type=\"max\"/><color \
         rgb=\"FF638EC6\"/></dataBar><extLst><ext uri=\"x\"/></extLst></cfRule>\
         </conditionalFormatting>",
    )
    .expect("read");
    assert_eq!(
        extended,
        vec![RuleEntry {
            areas: vec![area("A1:A9")],
            kind: "dataBar".to_string(),
            operator: None,
            formulas: Vec::new(),
            dxf: None,
            priority: 2,
            stop_if_true: false,
            extended: true,
        }]
    );
    assert_eq!(read("<sheetData/>").expect("read"), Vec::new());
    // P2 ruling 6: each text operator reads as none.
    for operator in ["containsText", "notContains", "beginsWith", "endsWith"] {
        let body = format!(
            "<sheetData/><conditionalFormatting sqref=\"A1\"><cfRule type=\"x\" \
             priority=\"1\" operator=\"{operator}\"/></conditionalFormatting>"
        );
        assert_eq!(read(&body).expect("read")[0].operator, None, "{operator}");
    }
}

#[test]
fn a_read_refuses_an_attribute_that_does_not_parse() {
    for (sqref, rule) in [
        ("A1:", "<cfRule type=\"x\" priority=\"1\"/>"),
        ("A1", "<cfRule type=\"x\" priority=\"y\"/>"),
        ("A1", "<cfRule type=\"x\"/>"),
        ("A1", "<cfRule type=\"x\" priority=\"1\" dxfId=\"-1\"/>"),
        (
            "A1",
            "<cfRule type=\"x\" priority=\"1\" operator=\"above\"/>",
        ),
        (
            "A1",
            "<cfRule type=\"x\" priority=\"1\" operator=\"contains\"/>",
        ),
    ] {
        let body = format!(
            "<sheetData/><conditionalFormatting sqref=\"{sqref}\">{rule}</conditionalFormatting>"
        );
        assert_package(
            read(&body).expect_err("refused"),
            "a conditionalFormatting sqref or a cfRule attribute does not parse",
        );
    }
}

#[test]
fn the_call_refuses_each_rule_and_records_nothing() {
    let off_grid = Area {
        first: CellRef { row: 1, col: 1 },
        last: CellRef {
            row: 1,
            col: 16_385,
        },
    };
    type Call = (
        Vec<Area>,
        Rule<'static>,
        Option<u32>,
        Option<u32>,
        &'static str,
    );
    let refusals: Vec<Call> = vec![
        (
            Vec::new(),
            greater(),
            Some(0),
            None,
            "a validation or a conditional format has no area",
        ),
        (
            vec![off_grid],
            greater(),
            Some(0),
            None,
            "a row or column index or count is out of range",
        ),
        (
            vec![area("A1")],
            greater(),
            None,
            None,
            "a conditional format rule and its differential style do not agree",
        ),
        (
            vec![area("A1")],
            greater(),
            Some(0),
            Some(0),
            "a row or column index or count is out of range",
        ),
        (
            vec![area("A1")],
            Rule::Top {
                rank: 0,
                percent: false,
                bottom: false,
            },
            Some(0),
            None,
            "a row or column index or count is out of range",
        ),
        (
            vec![area("A1")],
            Rule::Top {
                rank: 1001,
                percent: true,
                bottom: false,
            },
            Some(0),
            None,
            "a row or column index or count is out of range",
        ),
        (
            vec![area("A1")],
            Rule::CellIs {
                operator: Operator::Equal,
                formula1: "",
                formula2: None,
            },
            Some(0),
            None,
            "a validation or a rule formula is empty or missing",
        ),
        (
            vec![area("A1")],
            Rule::CellIs {
                operator: Operator::Between,
                formula1: "1",
                formula2: None,
            },
            Some(0),
            None,
            "a validation or a rule formula is empty or missing",
        ),
        (
            vec![area("A1")],
            Rule::CellIs {
                operator: Operator::Equal,
                formula1: "1",
                formula2: Some("2"),
            },
            Some(0),
            None,
            "a validation or a rule formula is empty or missing",
        ),
        (
            vec![area("A1")],
            Rule::Expression(""),
            Some(0),
            None,
            "a validation or a rule formula is empty or missing",
        ),
        (
            vec![area("A1")],
            Rule::Expression("=A1"),
            Some(0),
            None,
            "a formula text starts with an equals sign",
        ),
        (
            vec![area("A1")],
            Rule::CellIs {
                operator: Operator::Between,
                formula1: "1",
                formula2: Some("\u{1}"),
            },
            Some(0),
            None,
            "a text holds a character XML cannot carry",
        ),
    ];
    let parts = with_body("<sheetData/>");
    let (_, mut package) = opened(parts.clone());
    for (areas, rule, dxf, priority, message) in refusals {
        assert_package(
            package
                .add_conditional_format(SHEET, &areas, rule, dxf, priority, false)
                .expect_err("refused"),
            message,
        );
    }
    assert_package(
        package
            .remove_conditional_format(SHEET, &[])
            .expect_err("refused"),
        "a validation or a conditional format has no area",
    );
    // Nothing recorded: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
}

#[test]
fn a_style_index_at_the_count_plus_additions_is_refused_and_below_it_taken() {
    // The styles part has two `dxf` children; one more added makes 3.
    assert_package(
        refused_on("<sheetData/>", |p| {
            p.add_conditional_format(SHEET, &[area("A1")], greater(), Some(2), None, false)
                .expect("recorded")
        }),
        "a differential style index is out of range",
    );
    let parts = with_body("<sheetData/>");
    let (_, mut package) = opened(parts);
    let index = package
        .add_differential_style(&Style {
            font: Some(crate::styles::Font {
                bold: true,
                ..crate::styles::Font::default()
            }),
            ..Style::default()
        })
        .expect("recorded");
    assert_eq!(index, 2);
    package
        .add_conditional_format(SHEET, &[area("A1")], greater(), Some(2), None, false)
        .expect("recorded");
    let out = written(package);
    let sheet = String::from_utf8(stored_data(&out, SHEET)).expect("UTF-8");
    assert!(sheet.contains("dxfId=\"2\""), "{sheet}");
    let package = {
        let (_, mut package) = opened(with_body("<sheetData/>"));
        package
            .add_differential_style(&Style {
                font: Some(crate::styles::Font::default()),
                ..Style::default()
            })
            .expect("recorded");
        package
            .add_conditional_format(SHEET, &[area("A1")], greater(), Some(3), None, false)
            .expect("recorded");
        package
    };
    assert_package(
        refused(package),
        "a differential style index is out of range",
    );
}

#[cfg(feature = "rowcol")]
#[test]
fn a_conditional_format_edit_with_a_row_insertion_is_refused() {
    let message = "a worksheet with a row or column insertion or deletion takes no other edit in the same write";
    assert_package(
        refused_on(TWO, |p| {
            p.insert_rows(SHEET, 1, 1).expect("recorded");
            add(p, &[area("G1")], greater());
        }),
        message,
    );
    assert_package(
        refused_on(TWO, |p| {
            p.insert_rows(SHEET, 1, 1).expect("recorded");
            p.remove_conditional_format(SHEET, &[area("A1:A5")])
                .expect("recorded");
        }),
        message,
    );
}

#[test]
fn a_conditional_format_edit_on_a_removed_or_replaced_part_is_refused() {
    type Edit = fn(&mut Package<Cursor<Vec<u8>>>);
    let edits: [Edit; 2] = [
        |p| add(p, &[area("A1")], greater()),
        |p| {
            p.remove_conditional_format(SHEET, &[area("A1")])
                .expect("recorded")
        },
    ];
    for edit in edits {
        let (_, mut package) = opened(with_body("<sheetData/>"));
        edit(&mut package);
        package.remove_part(SHEET);
        assert_package(
            refusal(package).expect("refused"),
            "an edit to a removed part",
        );
        let (_, mut package) = opened(with_body("<sheetData/>"));
        edit(&mut package);
        package.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
        assert_package(
            refusal(package).expect("refused"),
            "a cell edit to a part this write replaces",
        );
    }
}
