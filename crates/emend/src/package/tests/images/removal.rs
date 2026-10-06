//! Image removal on byte-built worksheets (`specs/0028-images-and-comments-
//! second-half.md`, "Image removal" and "P2: the anchors and the removal"):
//! each test builds a drawing part with its rels part and media entries,
//! records a removal by the index `images` lists, and compares the drawing,
//! its rels part, the media entries and the content types with what the
//! design gives. The Cargo feature `images`.

use super::*;

/// `parts` with an `Override` of `image/png` for each media entry named.
fn with_media_overrides(mut parts: Parts, names: &[&str]) -> Parts {
    let overrides: String = names
        .iter()
        .map(|name| format!("<Override PartName=\"/xl/media/{name}\" ContentType=\"image/png\"/>"))
        .collect();
    let types = part_text(&parts, b"[Content_Types].xml")
        .replace("</Types>", &format!("{overrides}</Types>"));
    set(&mut parts, b"[Content_Types].xml", types);
    parts
}

/// The content types of `parts` less the `Override` of each part named.
fn types_without(parts: &Parts, names: &[&str]) -> String {
    let mut types = part_text(parts, b"[Content_Types].xml");
    for name in names {
        let start = types
            .find(&format!("<Override PartName=\"/{name}\""))
            .expect("the override");
        let end = start + types[start..].find("/>").expect("its end") + 2;
        types.replace_range(start..end, "");
    }
    types
}

/// Whether the output holds an entry `name`.
fn has(output: &[u8], name: &[u8]) -> bool {
    let package = Package::open(Cursor::new(output.to_vec())).expect("open the output");
    package.part(name).is_some()
}

/// Records the removal of the image at `index`.
#[track_caller]
fn remove(package: &mut Package<Cursor<Vec<u8>>>, index: usize) {
    package.remove_image(SHEET, index).expect("recorded");
}

/// Three anchors, a picture each, on `image1.png` to `image3.png`, the media
/// typed by an `Override`.
fn three() -> (Parts, [String; 3]) {
    let anchors = [
        two_cell(0, 0, &pic(1, "rId1")),
        one_cell(2, 2, &pic(2, "rId2")),
        two_cell(4, 4, &pic(3, "rId3")),
    ];
    let media = ["image1.png", "image2.png", "image3.png"];
    let parts = with_media_overrides(
        with_drawing(
            BODY,
            drawing_xml(&anchors.concat()),
            Some(&image_rels(3)),
            &media,
        ),
        &media,
    );
    (parts, anchors)
}

#[test]
fn a_removal_of_the_middle_of_three_leaves_the_rest_byte_identical() {
    let (parts, anchors) = three();
    let output = edited_output(parts.clone(), |p| remove(p, 1));
    assert_untouched(
        &output,
        &parts,
        &[
            DRAWING,
            DRAWING_RELS,
            b"[Content_Types].xml".as_slice(),
            b"xl/media/image2.png",
        ],
    );
    assert_eq!(
        text_of(&output, DRAWING),
        drawing_xml(&format!("{}{}", anchors[0], anchors[2]))
    );
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&format!(
            "{}{}",
            image_rel("rId1", "../media/image1.png"),
            image_rel("rId3", "../media/image3.png")
        ))
    );
    assert!(!has(&output, b"xl/media/image2.png"));
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        types_without(&parts, &["xl/media/image2.png"])
    );
    let mut reopened = Package::open(Cursor::new(output)).expect("open the output");
    assert_eq!(
        reopened.images(SHEET).expect("read back"),
        [
            entry((0, 0), Anchor::TwoCell, 1),
            entry((4, 4), Anchor::TwoCell, 3)
        ]
    );
}

#[test]
fn a_relationship_a_kept_picture_names_stays_with_its_media() {
    let anchors = [
        two_cell(0, 0, &pic(1, "rId1")),
        two_cell(2, 2, &pic(2, "rId1")),
    ];
    let parts = with_drawing(
        BODY,
        drawing_xml(&anchors.concat()),
        Some(&image_rels(1)),
        &["image1.png"],
    );
    let output = edited_output(parts.clone(), |p| remove(p, 0));
    assert_untouched(&output, &parts, &[DRAWING]);
    assert_eq!(text_of(&output, DRAWING), drawing_xml(&anchors[1]));
}

#[test]
fn a_removal_whose_media_another_drawing_targets_keeps_the_media() {
    // The sheet's rels part names a second drawing, whose rels part targets
    // the same media entry: the relationship goes, the media stays.
    let anchors = [
        two_cell(0, 0, &pic(1, "rId1")),
        two_cell(2, 2, &pic(2, "rId2")),
    ];
    let mut parts = with_media_overrides(
        with_drawing(
            BODY,
            drawing_xml(&anchors.concat()),
            Some(&image_rels(2)),
            &["image1.png", "image2.png"],
        ),
        &["image1.png", "image2.png"],
    );
    set(
        &mut parts,
        SHEET_RELS,
        rels(&format!(
            "{}{}",
            drawing_rel("rId1", "../drawings/drawing1.xml"),
            drawing_rel("rId2", "../drawings/drawing2.xml")
        )),
    );
    with_part(
        &mut parts,
        "xl/drawings/drawing2.xml",
        &drawing_xml(&two_cell(0, 0, &pic(1, "rId1"))),
        Some(DRAWING_TYPE),
    );
    with_part(
        &mut parts,
        "xl/drawings/_rels/drawing2.xml.rels",
        &rels(&image_rel("rId1", "../media/image2.png")),
        None,
    );
    let output = edited_output(parts.clone(), |p| remove(p, 1));
    assert_untouched(&output, &parts, &[DRAWING, DRAWING_RELS]);
    assert_eq!(text_of(&output, DRAWING), drawing_xml(&anchors[0]));
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&image_rel("rId1", "../media/image1.png"))
    );
}

#[test]
fn a_removal_of_a_picture_with_an_svg_blip_drops_both_media_entries() {
    let svg = format!(
        "<xdr:pic><xdr:nvPicPr><xdr:cNvPr id=\"2\" name=\"Picture 2\"/><xdr:cNvPicPr/>\
         </xdr:nvPicPr><xdr:blipFill><a:blip xmlns:r=\"{ATTR_NS}\" r:embed=\"rId2\"><a:extLst>\
         <a:ext uri=\"{{96DAC541-7B7A-43D3-8B79-37D633B846F1}}\"><asvg:svgBlip \
         xmlns:asvg=\"http://schemas.microsoft.com/office/drawing/2016/SVG/main\" \
         r:embed=\"rId3\"/></a:ext></a:extLst></a:blip><a:stretch><a:fillRect/></a:stretch>\
         </xdr:blipFill><xdr:spPr/></xdr:pic>"
    );
    let anchors = [two_cell(0, 0, &pic(1, "rId1")), two_cell(2, 2, &svg)];
    let media = ["image1.png", "image2.png", "image3.png"];
    let parts = with_media_overrides(
        with_drawing(
            BODY,
            drawing_xml(&anchors.concat()),
            Some(&image_rels(3)),
            &media,
        ),
        &media,
    );
    let output = edited_output(parts.clone(), |p| remove(p, 1));
    assert_eq!(text_of(&output, DRAWING), drawing_xml(&anchors[0]));
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&image_rel("rId1", "../media/image1.png"))
    );
    assert!(!has(&output, b"xl/media/image2.png"));
    assert!(!has(&output, b"xl/media/image3.png"));
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        types_without(&parts, &["xl/media/image2.png", "xl/media/image3.png"])
    );
}

#[test]
fn a_removal_leaving_a_chart_keeps_the_drawing() {
    let anchors = [two_cell(0, 0, &pic(1, "rId1")), two_cell(2, 2, &chart(2))];
    let parts = with_drawing(
        BODY,
        drawing_xml(&anchors.concat()),
        Some(&image_rels(1)),
        &["image1.png"],
    );
    let output = edited_output(parts.clone(), |p| remove(p, 0));
    assert_eq!(text_of(&output, DRAWING), drawing_xml(&anchors[1]));
    assert_eq!(text_of(&output, DRAWING_RELS), rels(""));
    assert!(!has(&output, b"xl/media/image1.png"));
    assert_eq!(text_of(&output, SHEET), sheet_r(BODY));
}

#[test]
fn a_removal_emptying_the_drawing_drops_it_with_its_rels_relationship_and_element() {
    let parts = with_media_overrides(
        with_drawing(
            "<sheetData/><drawing r:id=\"rId1\"/><tableParts count=\"0\"/>",
            drawing_xml(&two_cell(0, 0, &pic(1, "rId1"))),
            Some(&image_rels(1)),
            &["image1.png"],
        ),
        &["image1.png"],
    );
    let output = edited_output(parts.clone(), |p| remove(p, 0));
    for gone in [DRAWING, DRAWING_RELS, b"xl/media/image1.png".as_slice()] {
        assert!(!has(&output, gone), "{}", String::from_utf8_lossy(gone));
    }
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r("<sheetData/><tableParts count=\"0\"/>")
    );
    assert_eq!(text_of(&output, SHEET_RELS), rels(""));
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        types_without(&parts, &["xl/drawings/drawing1.xml", "xl/media/image1.png"])
    );
    let mut reopened = Package::open(Cursor::new(output)).expect("open the output");
    assert_eq!(reopened.images(SHEET).expect("read back"), []);
}

#[test]
fn an_index_past_the_list_named_twice_or_with_an_addition_is_refused_at_the_write() {
    let message = "no image at that position";
    let (parts, _) = three();
    assert_package(refused_with(parts.clone(), |p| remove(p, 3)), message);
    assert_package(
        refused_with(parts.clone(), |p| {
            remove(p, 1);
            remove(p, 1);
        }),
        message,
    );
    assert_package(
        refused_with(parts.clone(), |p| {
            remove(p, 0);
            add(p, "A1");
        }),
        message,
    );
    assert_package(
        refused_with(parts, |p| {
            add(p, "A1");
            remove(p, 0);
        }),
        message,
    );
    // A sheet with no drawing has no image at any position.
    assert_package(refused_with(smallest(), |p| remove(p, 0)), message);
    // A chart is not an image: the list is empty.
    let charted = with_drawing(BODY, drawing_xml(&two_cell(0, 0, &chart(1))), None, &[]);
    assert_package(refused_with(charted, |p| remove(p, 0)), message);
}

#[test]
fn a_removal_on_a_removed_replaced_or_shifted_part_is_refused() {
    let (parts, _) = three();
    assert_package(
        refused_with(parts.clone(), |p| {
            p.remove_part(SHEET);
            remove(p, 0);
        }),
        "an edit to a removed part",
    );
    assert_package(
        refused_with(parts.clone(), |p| {
            p.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
            remove(p, 0);
        }),
        "a cell edit to a part this write replaces",
    );
    assert_package(
        refused_with(parts.clone(), |p| {
            p.remove_part(DRAWING);
            remove(p, 0);
        }),
        "an edit to a removed part",
    );
    #[cfg(feature = "rowcol")]
    assert_package(
        refused_with(parts, |p| {
            p.insert_rows(SHEET, 20, 1).expect("recorded");
            remove(p, 0);
        }),
        "a worksheet with a row or column insertion or deletion takes no other edit in the same write",
    );
}

#[test]
fn a_removal_reading_a_missing_media_entry_is_refused() {
    let (mut parts, _) = three();
    parts.retain(|(name, _)| name.as_slice() != b"xl/media/image3.png");
    assert_package(
        refused_with(parts, |p| remove(p, 0)),
        "a drawing part or its relationship cannot be read",
    );
}

#[test]
fn a_picture_in_a_root_choice_goes_with_its_alternate_content_whole() {
    // The picture `images` reads from a root `AlternateContent`'s first
    // `Choice`: its `Fallback` describes the same object, so the root child
    // goes whole, and a relationship the `Fallback` alone named goes too.
    let children = [
        alternate(
            &two_cell(2, 3, &pic(2, "rId1")),
            &two_cell(2, 3, &pic(2, "rId2")),
        ),
        two_cell(0, 0, &pic(3, "rId3")),
    ];
    let media = ["image1.png", "image2.png", "image3.png"];
    let parts = with_drawing(
        BODY,
        drawing_xml(&children.concat()),
        Some(&image_rels(3)),
        &media,
    );
    let output = edited_output(parts, |p| remove(p, 0));
    assert_eq!(text_of(&output, DRAWING), drawing_xml(&children[1]));
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&image_rel("rId3", "../media/image3.png"))
    );
    assert!(!has(&output, b"xl/media/image1.png"));
    assert!(!has(&output, b"xl/media/image2.png"));
}
