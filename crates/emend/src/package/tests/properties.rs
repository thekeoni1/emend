//! Document properties on byte-built packages
//! (`specs/0019-names-links-properties-protection.md`, "P3"): each test
//! builds the smallest package with a `docProps/core.xml` of a given body
//! and its package relationship, records property edits, and compares the
//! written core part with the bytes the design gives, every other part
//! asserted untouched.

use super::*;

const CORE: &[u8] = b"docProps/core.xml";
const CP: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
const DC: &str = "http://purl.org/dc/elements/1.1/";
const DCTERMS: &str = "http://purl.org/dc/terms/";
const DCMITYPE: &str = "http://purl.org/dc/dcmitype/";
const XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";
const CORE_TYPE: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";

/// The root start tag Excel writes, binding the five prefixes.
fn root_start() -> String {
    format!(
        "<cp:coreProperties xmlns:cp=\"{CP}\" xmlns:dc=\"{DC}\" xmlns:dcterms=\"{DCTERMS}\" \
         xmlns:dcmitype=\"{DCMITYPE}\" xmlns:xsi=\"{XSI}\">"
    )
}

/// A core part with `body` in Excel's root.
fn core(body: &str) -> String {
    format!("{DECLARATION}\n{}{body}</cp:coreProperties>", root_start())
}

/// The smallest package with a core part of `part` and the package
/// relationship to it.
fn with_core(part: String) -> Parts {
    let mut parts = smallest();
    let package_rels = rels(&format!(
        "{}{}",
        relationship(
            "rId1",
            &format!("{ATTR_NS}/officeDocument"),
            "xl/workbook.xml"
        ),
        relationship("rId2", CORE_TYPE, "docProps/core.xml")
    ));
    set(&mut parts, b"_rels/.rels", package_rels);
    parts.push((CORE.to_vec(), part.into_bytes()));
    parts
}

/// The core part after `edit` on `with_core(part)`, as text; every other
/// part is asserted untouched.
#[track_caller]
fn edited_core(part: String, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let parts = with_core(part);
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[CORE]);
    String::from_utf8(stored_data(&output, CORE)).expect("UTF-8")
}

/// Every one of the thirteen elements, each with its property's name as its
/// text but for the dates.
const ALL: &str = "<dc:title>Title</dc:title><dc:subject>Subject</dc:subject>\
                   <dc:creator>Creator</dc:creator><cp:keywords>Keywords</cp:keywords>\
                   <dc:description>Description</dc:description>\
                   <cp:lastModifiedBy>LastModifiedBy</cp:lastModifiedBy>\
                   <cp:revision>Revision</cp:revision>\
                   <dcterms:created xsi:type=\"dcterms:W3CDTF\">2020-01-02T03:04:05Z</dcterms:created>\
                   <dcterms:modified xsi:type=\"dcterms:W3CDTF\">2021-06-07T08:09:10Z</dcterms:modified>\
                   <cp:category>Category</cp:category><cp:contentStatus>ContentStatus</cp:contentStatus>\
                   <dc:language>Language</dc:language><cp:version>Version</cp:version>\
                   <cp:other>not read</cp:other>";

#[test]
fn the_thirteen_properties_are_read_and_the_template_has_none() {
    let (_, mut package) = opened(with_core(core(ALL)));
    let read = package.properties().expect("properties");
    for (property, want) in [
        (Property::Title, "Title"),
        (Property::Subject, "Subject"),
        (Property::Creator, "Creator"),
        (Property::Description, "Description"),
        (Property::Language, "Language"),
        (Property::Keywords, "Keywords"),
        (Property::LastModifiedBy, "LastModifiedBy"),
        (Property::Revision, "Revision"),
        (Property::Category, "Category"),
        (Property::ContentStatus, "ContentStatus"),
        (Property::Version, "Version"),
        (Property::Created, "2020-01-02T03:04:05Z"),
        (Property::Modified, "2021-06-07T08:09:10Z"),
    ] {
        assert_eq!(read.get(property), Some(want), "{property:?}");
    }
    let mut package = new_package();
    assert_eq!(
        package.properties().expect("properties"),
        Properties::default()
    );
    // A relationship whose part is missing.
    let mut parts = with_core(core(""));
    drop_part(&mut parts, CORE);
    let (_, mut package) = opened(parts);
    assert_package(package.properties().expect_err("refused"), "no such part");
}

#[test]
fn each_property_is_replaced_in_place_removed_or_appended_with_its_prefix() {
    let body = "<dc:title>Old &amp; gone</dc:title>\n<cp:keywords/>\
                <dcterms:modified xsi:type=\"dcterms:W3CDTF\">2000-01-01T00:00:00Z</dcterms:modified>\
                <dc:subject>Kept</dc:subject><dc:creator>Dropped</dc:creator>";
    let got = edited_core(core(body), |p| {
        p.set_property(Property::Title, Some("New <title> & \"more\""))
            .expect("title");
        p.set_property(Property::Keywords, Some("a b"))
            .expect("keywords");
        p.set_property(Property::Modified, Some("2026-01-01T00:00:00Z"))
            .expect("modified");
        p.set_property(Property::Creator, None).expect("creator");
        p.set_property(Property::Category, Some("cat"))
            .expect("category");
        p.set_property(Property::Created, Some("2025-12-31T23:59:59Z"))
            .expect("created");
        p.set_property(Property::Language, Some("en"))
            .expect("language");
        p.set_property(Property::Revision, Some("1"))
            .expect("first");
        p.set_property(Property::Revision, Some("2"))
            .expect("last wins");
    });
    assert_eq!(
        got,
        core(
            "<dc:title>New &lt;title&gt; &amp; &quot;more&quot;</dc:title>\n<cp:keywords>a b</cp:keywords>\
             <dcterms:modified xsi:type=\"dcterms:W3CDTF\">2026-01-01T00:00:00Z</dcterms:modified>\
             <dc:subject>Kept</dc:subject>\
             <cp:category>cat</cp:category>\
             <dcterms:created xsi:type=\"dcterms:W3CDTF\">2025-12-31T23:59:59Z</dcterms:created>\
             <dc:language>en</dc:language><cp:revision>2</cp:revision>"
        )
    );
}

#[test]
fn a_root_binding_no_dcterms_prefix_refuses_an_append_and_allows_a_replace() {
    let bare = |body: &str| {
        format!(
            "{DECLARATION}\n<cp:coreProperties xmlns:cp=\"{CP}\" xmlns:dc=\"{DC}\" \
             xmlns:xsi=\"{XSI}\">{body}</cp:coreProperties>"
        )
    };
    let (_, mut package) = opened(with_core(bare("<dc:title>t</dc:title>")));
    package
        .set_property(Property::Modified, Some("2026-01-01T00:00:00Z"))
        .expect("recorded");
    assert_package(
        refused(package),
        "a core properties part binds no prefix for a namespace the edit needs",
    );
    // A replace keeps the element's own start tag, prefix and all.
    let body =
        "<q:modified xmlns:q=\"http://purl.org/dc/terms/\">2000-01-01T00:00:00Z</q:modified>";
    let got = edited_core(bare(body), |p| {
        p.set_property(Property::Modified, Some("2026-01-01T00:00:00Z"))
            .expect("modified")
    });
    assert_eq!(
        got,
        bare("<q:modified xmlns:q=\"http://purl.org/dc/terms/\">2026-01-01T00:00:00Z</q:modified>")
    );
}

#[test]
fn a_second_element_of_one_property_is_refused_at_the_read_and_the_write() {
    let two = core("<dc:title>a</dc:title><dc:title>b</dc:title>");
    let (_, mut package) = opened(with_core(two.clone()));
    assert_package(
        package.properties().expect_err("refused"),
        "a core properties part has two elements of one property",
    );
    let (_, mut package) = opened(with_core(two));
    package
        .set_property(Property::Keywords, Some("k"))
        .expect("recorded");
    assert_package(
        refused(package),
        "a core properties part has two elements of one property",
    );
}

#[test]
fn the_template_gets_the_part_its_relationship_and_its_override() {
    let mut package = new_package();
    package
        .set_property(Property::Title, Some("emend"))
        .expect("title");
    package
        .set_property(Property::Created, Some("2026-01-01T00:00:00Z"))
        .expect("created");
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
    let part = read(&mut reopened, CORE);
    let package_rels = read(&mut reopened, b"_rels/.rels");
    let types = read(&mut reopened, b"[Content_Types].xml");
    println!("created core part: {part}");
    println!("package rels part: {package_rels}");
    println!("content types part: {types}");
    assert_eq!(
        part,
        core(
            "<dc:title>emend</dc:title>\
             <dcterms:created xsi:type=\"dcterms:W3CDTF\">2026-01-01T00:00:00Z</dcterms:created>"
        )
    );
    let template_rels = String::from_utf8(TEMPLATE[1].1.to_vec()).expect("UTF-8");
    assert_eq!(
        package_rels,
        template_rels.replace(
            "</Relationships>",
            &format!(
                "<Relationship Id=\"rId2\" Type=\"{CORE_TYPE}\" Target=\"docProps/core.xml\"/>\
                 </Relationships>"
            )
        )
    );
    let template_types = String::from_utf8(TEMPLATE[0].1.to_vec()).expect("UTF-8");
    assert_eq!(
        types,
        template_types.replace(
            "</Types>",
            "<Override PartName=\"/docProps/core.xml\" \
             ContentType=\"application/vnd.openxmlformats-package.core-properties+xml\"/></Types>"
        )
    );
    assert_eq!(
        reopened
            .properties()
            .expect("properties")
            .get(Property::Title),
        Some("emend")
    );
    // A removal alone on the template records nothing: the write is the
    // template.
    let mut package = new_package();
    package
        .set_property(Property::Title, None)
        .expect("removal");
    let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    package.write_to(&mut out).expect("write_to");
    drop(_guard);
    assert_eq!(out, new_bytes());
}

#[test]
fn a_date_value_must_be_the_one_form_and_every_value_xml_can_carry() {
    let (parts, mut package) = opened(with_core(core("")));
    let good = "2026-01-01T00:00:00Z";
    for at in 0..good.len() {
        let mut bad = good.as_bytes().to_vec();
        bad[at] = if bad[at].is_ascii_digit() { b'x' } else { b'0' };
        let bad = String::from_utf8(bad).expect("ASCII");
        for property in [Property::Created, Property::Modified] {
            assert_package(
                package
                    .set_property(property, Some(&bad))
                    .expect_err("refused"),
                "a property value is not valid",
            );
        }
    }
    for bad in [
        "2026-01-01T00:00:00",
        "2026-01-01T00:00:00Z ",
        "",
        "2026-01-01",
    ] {
        assert_package(
            package
                .set_property(Property::Created, Some(bad))
                .expect_err("refused"),
            "a property value is not valid",
        );
    }
    assert_package(
        package
            .set_property(Property::Title, Some("a\u{1}"))
            .expect_err("refused"),
        "a text holds a character XML cannot carry",
    );
    // Nothing recorded: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
    // Any text is taken for a property that is not a date.
    let (_, mut package) = opened(with_core(core("")));
    package
        .set_property(Property::Title, Some("2026"))
        .expect("title");
}

#[test]
fn a_property_edit_on_a_missing_core_part_is_refused_at_the_write() {
    // P3 ruling 1: the relationship names a part the archive lacks; a set
    // and a removal, each alone, are refused as the read refuses them.
    for value in [Some("t"), None] {
        let mut parts = with_core(core(""));
        drop_part(&mut parts, CORE);
        let (_, mut package) = opened(parts);
        package
            .set_property(Property::Title, value)
            .expect("recorded");
        assert_package(refused(package), "no such part");
    }
}

#[test]
fn a_core_part_removed_or_replaced_refuses_a_property_edit() {
    // P3 ruling 5.
    let (_, mut package) = opened(with_core(core("")));
    package.remove_part(CORE);
    package
        .set_property(Property::Title, Some("t"))
        .expect("recorded");
    assert_package(refused(package), "an edit to a removed part");
    let (_, mut package) = opened(with_core(core("")));
    package.replace_part(CORE, replacement_from_reader(b"<x/>".as_slice()));
    package
        .set_property(Property::Title, Some("t"))
        .expect("recorded");
    assert_package(refused(package), "an edit to a removed part");
}

#[test]
fn xsi_bound_only_as_the_default_namespace_refuses_an_appended_date() {
    // P3 ruling 6: an attribute cannot take the default namespace.
    let part = format!(
        "{DECLARATION}\n<cp:coreProperties xmlns:cp=\"{CP}\" xmlns:dc=\"{DC}\" \
         xmlns:dcterms=\"{DCTERMS}\" xmlns=\"{XSI}\"></cp:coreProperties>"
    );
    let (_, mut package) = opened(with_core(part.clone()));
    package
        .set_property(Property::Created, Some("2026-01-01T00:00:00Z"))
        .expect("recorded");
    assert_package(
        refused(package),
        "a core properties part binds no prefix for a namespace the edit needs",
    );
    // The control: an append that needs no xsi prefix is written.
    let got = edited_core(part.clone(), |p| {
        p.set_property(Property::Title, Some("t")).expect("title")
    });
    assert_eq!(
        got,
        part.replace(
            "</cp:coreProperties>",
            "<dc:title>t</dc:title></cp:coreProperties>"
        )
    );
}

#[test]
fn all_thirteen_properties_are_written_in_each_of_replace_remove_and_append() {
    // P3 ruling 8: every property, each mode, against expected bytes. The
    // properties in the order ALL writes them, with their elements' names.
    let order: [(Property, &str); 13] = [
        (Property::Title, "dc:title"),
        (Property::Subject, "dc:subject"),
        (Property::Creator, "dc:creator"),
        (Property::Keywords, "cp:keywords"),
        (Property::Description, "dc:description"),
        (Property::LastModifiedBy, "cp:lastModifiedBy"),
        (Property::Revision, "cp:revision"),
        (Property::Created, "dcterms:created"),
        (Property::Modified, "dcterms:modified"),
        (Property::Category, "cp:category"),
        (Property::ContentStatus, "cp:contentStatus"),
        (Property::Language, "dc:language"),
        (Property::Version, "cp:version"),
    ];
    let value = |property: Property| match property {
        Property::Created => "2030-01-01T00:00:00Z".to_string(),
        Property::Modified => "2031-02-02T01:02:03Z".to_string(),
        other => format!("new {other:?} & <x>"),
    };
    let escaped = |text: &str| {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    };
    let element = |property: Property, name: &str| {
        let open = if matches!(property, Property::Created | Property::Modified) {
            format!("<{name} xsi:type=\"dcterms:W3CDTF\">")
        } else {
            format!("<{name}>")
        };
        format!("{open}{}</{name}>", escaped(&value(property)))
    };
    let set_all = |p: &mut Package<Cursor<Vec<u8>>>| {
        for (property, _) in order {
            p.set_property(property, Some(&value(property)))
                .expect("set");
        }
    };
    let every: String = order
        .iter()
        .map(|(property, name)| element(*property, name))
        .collect();
    // Replace: each in place, its start tag kept, the unknown child kept.
    assert_eq!(
        edited_core(core(ALL), set_all),
        core(&format!("{every}<cp:other>not read</cp:other>"))
    );
    // Remove: each dropped whole, the unknown child kept.
    assert_eq!(
        edited_core(core(ALL), |p| {
            for (property, _) in order {
                p.set_property(property, None).expect("remove");
            }
        }),
        core("<cp:other>not read</cp:other>")
    );
    // Append: each after what the part holds, in record order.
    assert_eq!(
        edited_core(core("<cp:other>kept</cp:other>"), set_all),
        core(&format!("<cp:other>kept</cp:other>{every}"))
    );
}
