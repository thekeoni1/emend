//! A sheet a pivot cache sources, refused at `remove_sheet`
//! (`specs/0048-the-two-refusals.md`, "The pivot cache source", decision
//! 1 A): each test builds a two-sheet workbook with one pivot cache whose
//! worksheet source names a sheet, a table or a defined name, and removes a
//! sheet.

use super::*;

const SOURCED: &str = "a pivot cache sources the sheet";
const UNREADABLE: &str = "a pivot cache cannot be read";
const CACHE: &str = "xl/pivotCache/pivotCacheDefinition1.xml";
const CACHE_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition";

/// A cache definition part whose `<cacheSource>` is `source`.
fn cache(source: &str) -> String {
    format!(
        "{DECLARATION}\n<pivotCacheDefinition xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\" \
         r:id=\"rId1\">{source}<cacheFields count=\"0\"/></pivotCacheDefinition>"
    )
}

/// A worksheet source with the attributes given.
fn worksheet_source(attributes: &str) -> String {
    format!(
        "<cacheSource type=\"worksheet\"><worksheetSource ref=\"A1:B2\" {attributes}/></cacheSource>"
    )
}

/// The two-sheet workbook with `defined` (a `<definedNames>` body, or
/// nothing), `<pivotCaches>` naming the cache when `listed`, the workbook's
/// relationship to the cache part, and the cache part `cache_bytes`.
fn with_cache(defined: &str, listed: bool, cache_bytes: &str) -> Parts {
    let defined = if defined.is_empty() {
        String::new()
    } else {
        format!("<definedNames>{defined}</definedNames>")
    };
    let caches = if listed {
        "<pivotCaches><pivotCache cacheId=\"1\" r:id=\"rId9\"/></pivotCaches>"
    } else {
        ""
    };
    let mut parts = sheeted(2, "", &format!("{defined}{caches}"));
    let rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        "</Relationships>",
        &format!(
            "{}</Relationships>",
            relationship("rId9", CACHE_REL, "pivotCache/pivotCacheDefinition1.xml")
        ),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", rels);
    with_part(&mut parts, CACHE, cache_bytes, None);
    parts
}

/// A table part on sheet 2, named `name` and displayed as `display`.
fn with_table(parts: &mut Parts, name: &str, display: &str) {
    with_sheet_rels(
        parts,
        2,
        &relationship("rId1", &format!("{ATTR_NS}/table"), "../tables/table1.xml"),
    );
    with_part(
        parts,
        "xl/tables/table1.xml",
        &format!(
            "{DECLARATION}\n<table xmlns=\"{MAIN_NS}\" id=\"1\" name=\"{name}\" \
             displayName=\"{display}\" ref=\"A1:B2\"/>"
        ),
        None,
    );
}

/// The refusal `remove_sheet("S2")` gives on `parts`, the list unchanged.
#[track_caller]
fn refused_removal(parts: Parts) -> Error {
    let (_, mut package) = opened(parts);
    let error = package.remove_sheet("S2").expect_err("refused");
    assert_eq!(sheet_names(&mut package), ["S1", "S2"], "nothing recorded");
    error
}

/// Whether `remove_sheet("S2")` is allowed on `parts` and the write keeps
/// the cache part.
#[track_caller]
fn allowed_removal(parts: Parts) {
    let (_, mut package) = opened(parts);
    package.remove_sheet("S2").expect("allowed");
    let out = written(package);
    assert!(
        names(&parse_independently(&out))
            .iter()
            .any(|name| name.as_slice() == CACHE.as_bytes()),
        "the cache part is kept"
    );
}

#[test]
fn a_cache_naming_the_sheet_by_its_sheet_attribute_refuses_and_the_next_call_works() {
    let parts = with_cache("", true, &cache(&worksheet_source("sheet=\"S2\"")));
    let (_, mut package) = opened(parts.clone());
    assert_package(package.remove_sheet("S2").expect_err("refused"), SOURCED);
    assert_eq!(sheet_names(&mut package), ["S1", "S2"], "nothing recorded");
    // The next call on the package works: S1 is not the source.
    package.remove_sheet("S1").expect("the next call");
    assert_eq!(sheet_names(&mut package), ["S2"]);
    let _ = written(package);
}

#[test]
fn a_cache_naming_a_table_on_the_sheet_by_its_display_name_or_name_refuses() {
    for by in ["Sales", "Table1"] {
        let mut parts = with_cache(
            "",
            true,
            &cache(&worksheet_source(&format!("name=\"{by}\""))),
        );
        with_table(&mut parts, "Table1", "Sales");
        assert_package(refused_removal(parts), SOURCED);
    }
}

#[test]
fn a_cache_naming_a_defined_name_qualified_with_the_sheet_quoted_or_bare_refuses() {
    for text in [
        "'S2'!$A$1:$B$2",
        "S2!$A$1:$B$2",
        "=S2!$A$1:$B$2",
        "='S2'!A1:B2",
    ] {
        let parts = with_cache(
            &format!("<definedName name=\"Src\">{text}</definedName>"),
            true,
            &cache(&worksheet_source("name=\"Src\"")),
        );
        assert_package(refused_removal(parts), SOURCED);
    }
}

#[test]
fn a_cache_naming_a_defined_name_qualified_with_another_sheet_is_allowed() {
    for text in ["S1!$A$1:$B$2", "'S1'!A1", "S22!A1"] {
        allowed_removal(with_cache(
            &format!("<definedName name=\"Src\">{text}</definedName>"),
            true,
            &cache(&worksheet_source("name=\"Src\"")),
        ));
    }
}

#[test]
fn a_cache_of_an_external_workbook_or_of_another_type_or_not_listed_is_allowed() {
    // An `r:id` on the worksheet source: an external workbook.
    allowed_removal(with_cache(
        "<definedName name=\"Src\">S2!A1</definedName>",
        true,
        &cache(&worksheet_source("name=\"Src\" r:id=\"rId1\"")),
    ));
    // A cache of type external.
    allowed_removal(with_cache(
        "",
        true,
        &cache("<cacheSource type=\"external\" connectionId=\"1\"/>"),
    ));
    // A workbook with no `<pivotCaches>`.
    allowed_removal(with_cache(
        "",
        false,
        &cache(&worksheet_source("sheet=\"S2\"")),
    ));
    // A cache whose part is not there.
    let mut missing = with_cache("", true, &cache(&worksheet_source("sheet=\"S2\"")));
    missing.retain(|(name, _)| name.as_slice() != CACHE.as_bytes());
    let (_, mut package) = opened(missing);
    package.remove_sheet("S2").expect("allowed");
    let _ = written(package);
}

#[test]
fn the_pivot_table_s_own_sheet_with_the_source_elsewhere_is_allowed_and_the_cache_kept() {
    let mut parts = with_cache("", true, &cache(&worksheet_source("sheet=\"S1\"")));
    with_sheet_rels(
        &mut parts,
        2,
        &relationship(
            "rId1",
            &format!("{ATTR_NS}/pivotTable"),
            "../pivotTables/pivotTable1.xml",
        ),
    );
    with_part(
        &mut parts,
        "xl/pivotTables/pivotTable1.xml",
        &format!(
            "{DECLARATION}\n<pivotTableDefinition xmlns=\"{MAIN_NS}\" name=\"P\" cacheId=\"1\"/>"
        ),
        None,
    );
    with_part(
        &mut parts,
        "xl/pivotTables/_rels/pivotTable1.xml.rels",
        &rels(&relationship(
            "rId1",
            CACHE_REL,
            "../pivotCache/pivotCacheDefinition1.xml",
        )),
        None,
    );
    allowed_removal(parts);
}

#[test]
fn an_unreadable_cache_part_refuses_with_the_read_message() {
    for bytes in [
        format!("{DECLARATION}\n<other xmlns=\"{MAIN_NS}\"/>"),
        format!("{DECLARATION}\n<pivotCacheDefinition xmlns=\"urn:other\"/>"),
        format!(
            "{DECLARATION}\n<pivotCacheDefinition xmlns=\"{MAIN_NS}\"><cacheSource></pivotCacheDefinition>"
        ),
    ] {
        assert_package(refused_removal(with_cache("", true, &bytes)), UNREADABLE);
    }
}
