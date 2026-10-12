//! External links on byte-built packages (`specs/0033-read-breadth-survey.md`,
//! "P3: external links, the catalogue, the probe, the size record"): three
//! references in order, each kind read, a book with no sheet names and no
//! defined names, the cached values walked past, the strict namespace, the
//! refusals, and a workbook with no `<externalReferences>`. The Cargo
//! feature `read`.

use super::*;
use crate::package::{ExternalLink, ExternalLinkKind};

/// The external link part's content type.
const EXTERNAL_LINK_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml";

/// The message every refusal below carries.
const UNREADABLE: &str = "an external link cannot be read";

/// One link part of a fixture: its body and its rels part's body.
struct Link {
    body: String,
    rels: String,
}

/// The part name of link `n` (from 1).
fn link_part(n: usize) -> String {
    format!("xl/externalLinks/externalLink{n}.xml")
}

/// A `<Relationship>` with `TargetMode="External"`.
fn external(id: &str, kind: &str, target: &str) -> String {
    format!(
        "<Relationship Id=\"{id}\" Type=\"{kind}\" Target=\"{target}\" TargetMode=\"External\"/>"
    )
}

/// An `externalLink` part in `ns` (its `r:` prefix bound to `attr`) holding
/// `body`.
fn link_in(ns: &str, attr: &str, body: &str) -> String {
    format!("{DECLARATION}\n<externalLink xmlns=\"{ns}\" xmlns:r=\"{attr}\">{body}</externalLink>")
}
fn link_body(body: &str) -> String {
    link_in(MAIN_NS, ATTR_NS, body)
}

/// The link part's rels: `rId1` the external file `target`.
fn path_rels(target: &str) -> String {
    rels(&external(
        "rId1",
        &format!("{ATTR_NS}/externalLinkPath"),
        target,
    ))
}

/// A link to the book `target` holding `body`.
fn book(target: &str, body: &str) -> Link {
    Link {
        body: link_body(&format!(
            "<externalBook r:id=\"rId1\">{body}</externalBook>"
        )),
        rels: path_rels(target),
    }
}

/// The smallest package, its workbook in `ns` holding `references` after
/// `<sheets>` and relating each of `links` as `rIdE<n>`, every link part
/// with its rels part and override.
fn link_parts_in(
    ns: &str,
    attr: &str,
    references: &str,
    links: &[Link],
) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut parts = smallest();
    let mut overrides = String::new();
    let mut workbook_rels = String::new();
    for (at, link) in links.iter().enumerate() {
        let n = at + 1;
        overrides.push_str(&format!(
            "<Override PartName=\"/{}\" ContentType=\"{EXTERNAL_LINK_TYPE}\"/>",
            link_part(n)
        ));
        workbook_rels.push_str(&relationship(
            &format!("rIdE{n}"),
            &format!("{ATTR_NS}/externalLink"),
            &link_part(n)[3..],
        ));
        parts.push((link_part(n).into_bytes(), link.body.clone().into_bytes()));
        parts.push((
            format!("xl/externalLinks/_rels/externalLink{n}.xml.rels").into_bytes(),
            link.rels.clone().into_bytes(),
        ));
    }
    set(
        &mut parts,
        b"[Content_Types].xml",
        types(&format!(
            "<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
             <Default Extension=\"xml\" ContentType=\"application/xml\"/>\
             <Override PartName=\"/xl/workbook.xml\" ContentType=\"{WORKBOOK_TYPE}\"/>\
             <Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"{WORKSHEET_TYPE}\"/>{overrides}"
        )),
    );
    set(
        &mut parts,
        b"xl/workbook.xml",
        format!(
            "{DECLARATION}\n<workbook xmlns=\"{ns}\" xmlns:r=\"{attr}\">\
             <sheets><sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/></sheets>{references}</workbook>"
        ),
    );
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}{workbook_rels}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship("rId2", &format!("{ATTR_NS}/styles"), "styles.xml"),
        )),
    );
    parts
}

/// The references to every link of `links`, in order.
fn every(links: &[Link]) -> String {
    let references: String = (1..=links.len())
        .map(|n| format!("<externalReference r:id=\"rIdE{n}\"/>"))
        .collect();
    format!("<externalReferences>{references}</externalReferences>")
}

fn link_parts(links: &[Link]) -> Vec<(Vec<u8>, Vec<u8>)> {
    link_parts_in(MAIN_NS, ATTR_NS, &every(links), links)
}

/// The external links of a package built from `links`.
#[track_caller]
fn read(links: &[Link]) -> Vec<ExternalLink> {
    let mut package = package(link_parts(links));
    package
        .external_links()
        .expect("the external links read")
        .to_vec()
}

/// The refusal of a package built from `links`.
#[track_caller]
fn refused(links: &[Link]) -> Error {
    let mut package = package(link_parts(links));
    package
        .external_links()
        .expect_err("the external links are refused")
}

/// A DDE link and an OLE link to `target`.
fn dde() -> Link {
    Link {
        body: link_body(
            "<ddeLink ddeService=\"Service\" ddeTopic=\"Topic\"><ddeItems><ddeItem name=\"I\"/></ddeItems></ddeLink>",
        ),
        rels: rels(""),
    }
}
fn ole(target: &str) -> Link {
    Link {
        body: link_body(
            "<oleLink r:id=\"rId1\" progId=\"Package\"><oleItems><oleItem name=\"O\"/></oleItems></oleLink>",
        ),
        rels: rels(&external("rId1", &format!("{ATTR_NS}/oleObject"), target)),
    }
}

#[test]
fn a_workbook_with_no_external_references_has_none() {
    let mut package = package(smallest());
    assert!(package.external_links().expect("read").is_empty());
}

#[test]
fn three_references_are_read_in_their_order_each_kind_read() {
    let links = [
        book(
            "other%20book.xlsx",
            "<sheetNames><sheetName val=\"First\"/><sheetName val=\"Second\"/></sheetNames>\
             <definedNames><definedName name=\"Rate\" refersTo=\"=First!$A$1\"/></definedNames>\
             <sheetDataSet><sheetData sheetId=\"0\"><row r=\"1\"><cell r=\"A1\"><v>1</v></cell></row></sheetData></sheetDataSet>",
        ),
        dde(),
        ole("file:///C:/objects/thing.bin"),
    ];
    // The references name the parts in another order than their numbers.
    let references = "<externalReferences><externalReference r:id=\"rIdE3\"/>\
                      <externalReference r:id=\"rIdE1\"/><externalReference r:id=\"rIdE2\"/></externalReferences>";
    let mut package = package(link_parts_in(MAIN_NS, ATTR_NS, references, &links));
    let read = package.external_links().expect("read").to_vec();
    assert_eq!(
        read,
        vec![
            ExternalLink {
                part: link_part(3).into_bytes(),
                kind: ExternalLinkKind::Ole {
                    prog_id: "Package".to_owned(),
                    target: "file:///C:/objects/thing.bin".to_owned(),
                },
            },
            ExternalLink {
                part: link_part(1).into_bytes(),
                kind: ExternalLinkKind::Book {
                    target: "other%20book.xlsx".to_owned(),
                    sheet_names: vec!["First".to_owned(), "Second".to_owned()],
                    defined_names: 1,
                },
            },
            ExternalLink {
                part: link_part(2).into_bytes(),
                kind: ExternalLinkKind::Dde {
                    service: "Service".to_owned(),
                    topic: "Topic".to_owned(),
                },
            },
        ]
    );
}

#[test]
fn a_book_with_no_sheet_names_and_no_defined_names_reads_empty_lists() {
    for body in ["", "<sheetNames/><definedNames/>"] {
        let read = read(&[book("b.xlsx", body)]);
        assert_eq!(
            read[0].kind,
            ExternalLinkKind::Book {
                target: "b.xlsx".to_owned(),
                sheet_names: Vec::new(),
                defined_names: 0,
            }
        );
    }
    let empty = Link {
        body: link_body("<externalBook r:id=\"rId1\"/>"),
        rels: path_rels("c.xlsx"),
    };
    assert_eq!(
        read(&[empty])[0].kind,
        ExternalLinkKind::Book {
            target: "c.xlsx".to_owned(),
            sheet_names: Vec::new(),
            defined_names: 0,
        }
    );
}

#[test]
fn the_cached_values_and_unmodelled_children_are_walked_past() {
    // A sheetDataSet whose cells hold elements named as the read's own, and
    // an extLst beside the book: neither is read.
    let link = Link {
        body: link_body(
            "<externalBook r:id=\"rId1\"><sheetNames><sheetName val=\"S\"/></sheetNames>\
             <sheetDataSet><sheetData sheetId=\"0\"><row r=\"1\"><cell r=\"A1\" t=\"s\"><v>x</v></cell></row>\
             <sheetNames><sheetName/></sheetNames><definedNames><definedName name=\"N\"/></definedNames></sheetData>\
             </sheetDataSet></externalBook><extLst><ext uri=\"u\"><oleLink/></ext></extLst>",
        ),
        rels: path_rels("d.xlsx"),
    };
    assert_eq!(
        read(&[link])[0].kind,
        ExternalLinkKind::Book {
            target: "d.xlsx".to_owned(),
            sheet_names: vec!["S".to_owned()],
            defined_names: 0,
        }
    );
}

#[test]
fn the_strict_namespace_is_read() {
    let link = Link {
        body: link_in(
            MAIN_STRICT,
            ATTR_STRICT,
            "<externalBook r:id=\"rId1\"><sheetNames><sheetName val=\"S\"/></sheetNames></externalBook>",
        ),
        rels: path_rels("e.xlsx"),
    };
    let references = "<externalReferences><externalReference r:id=\"rIdE1\"/></externalReferences>";
    let mut package = package(link_parts_in(MAIN_STRICT, ATTR_STRICT, references, &[link]));
    let read = package.external_links().expect("read").to_vec();
    assert_eq!(
        read[0].kind,
        ExternalLinkKind::Book {
            target: "e.xlsx".to_owned(),
            sheet_names: vec!["S".to_owned()],
            defined_names: 0,
        }
    );
}

#[test]
fn a_reference_with_no_id_or_naming_no_link_relationship_is_refused() {
    let links = [book("b.xlsx", "")];
    for references in [
        // No r:id.
        "<externalReferences><externalReference/></externalReferences>",
        // An r:id naming no relationship.
        "<externalReferences><externalReference r:id=\"rIdX\"/></externalReferences>",
        // An r:id of another type: the worksheet's.
        "<externalReferences><externalReference r:id=\"rId1\"/></externalReferences>",
    ] {
        let mut package = package(link_parts_in(MAIN_NS, ATTR_NS, references, &links));
        assert_package(package.external_links().expect_err("refused"), UNREADABLE);
    }
}

#[test]
fn a_root_outside_the_main_namespace_or_of_another_name_is_refused() {
    for body in [
        link_in("urn:other", ATTR_NS, "<externalBook r:id=\"rId1\"/>"),
        format!(
            "{DECLARATION}\n<externalBook xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\" r:id=\"rId1\"/>"
        ),
    ] {
        let link = Link {
            body,
            rels: path_rels("b.xlsx"),
        };
        assert_package(refused(&[link]), UNREADABLE);
    }
}

#[test]
fn the_spec_s_refusals_are_refused_with_the_message() {
    let cases = [
        // None of the three children.
        Link {
            body: link_body("<extLst/>"),
            rels: path_rels("b.xlsx"),
        },
        // Two children.
        Link {
            body: link_body(
                "<externalBook r:id=\"rId1\"/><ddeLink ddeService=\"S\" ddeTopic=\"T\"/>",
            ),
            rels: path_rels("b.xlsx"),
        },
        // An externalBook with no r:id.
        Link {
            body: link_body("<externalBook/>"),
            rels: path_rels("b.xlsx"),
        },
        // An externalBook whose r:id names no relationship of the part.
        Link {
            body: link_body("<externalBook r:id=\"rId9\"/>"),
            rels: path_rels("b.xlsx"),
        },
        // An externalBook whose relationship is internal.
        Link {
            body: link_body("<externalBook r:id=\"rId1\"/>"),
            rels: rels(&relationship(
                "rId1",
                &format!("{ATTR_NS}/externalLinkPath"),
                "b.xlsx",
            )),
        },
        // A sheetName with no val.
        book(
            "b.xlsx",
            "<sheetNames><sheetName val=\"S\"/><sheetName/></sheetNames>",
        ),
        // A ddeLink with no topic, and one with no service.
        Link {
            body: link_body("<ddeLink ddeService=\"S\"/>"),
            rels: rels(""),
        },
        Link {
            body: link_body("<ddeLink ddeTopic=\"T\"/>"),
            rels: rels(""),
        },
        // An oleLink with no progId.
        Link {
            body: link_body("<oleLink r:id=\"rId1\"/>"),
            rels: path_rels("o.bin"),
        },
        // An oleLink with no r:id, and one whose relationship is internal.
        Link {
            body: link_body("<oleLink progId=\"P\"/>"),
            rels: path_rels("o.bin"),
        },
        Link {
            body: link_body("<oleLink r:id=\"rId1\" progId=\"P\"/>"),
            rels: rels(&relationship(
                "rId1",
                &format!("{ATTR_NS}/oleObject"),
                "o.bin",
            )),
        },
    ];
    for (at, link) in cases.into_iter().enumerate() {
        let error = refused(&[link]);
        assert!(
            matches!(error, Error::Package(message) if message == UNREADABLE),
            "case {at}: {error:?}"
        );
    }
}

#[test]
fn a_link_relationship_naming_no_entry_is_no_such_part() {
    let mut parts = link_parts(&[book("b.xlsx", "")]);
    drop_part(&mut parts, link_part(1).as_bytes());
    let mut package = package(parts);
    assert_package(
        package.external_links().expect_err("refused"),
        "no such part",
    );
}

#[test]
fn the_list_is_read_once_and_lent() {
    let mut package = package(link_parts(&[dde()]));
    let first = package.external_links().expect("read").to_vec();
    let again = package.external_links().expect("read again").to_vec();
    assert_eq!(first, again, "the same list");
    assert_eq!(first.len(), 1);
}
