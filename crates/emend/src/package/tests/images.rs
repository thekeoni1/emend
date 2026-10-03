//! Images on byte-built worksheets (`specs/0022-comments-and-images.md`,
//! "P3", and the P3 rulings): each test builds the smallest package with its
//! worksheet holding a given body and, where it says so, a rels part, a
//! drawing part with its rels part and media entries, records image
//! additions or reads the images, and compares the written parts it names
//! with the bytes the design gives; the tests that call `assert_untouched`
//! also assert every other part untouched.

use super::*;

/// A drawing part's content type.
const DRAWING_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";

/// The spreadsheet-drawing, DrawingML and markup-compatibility namespaces.
const XDR_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing";
const A_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const MC_NS: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";

/// The drawing part of the fixtures and its rels part.
const DRAWING: &[u8] = b"xl/drawings/drawing1.xml";
const DRAWING_RELS: &[u8] = b"xl/drawings/_rels/drawing1.xml.rels";

/// The tool's constant (P3 ruling 2): a 2 by 2 RGB PNG of 82 bytes, one
/// stored deflate block, the pixels red, green, blue, white.
const PNG: [u8; 82] = [
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x08, 0x02, 0x00, 0x00, 0x00, 0xfd, 0xd4, 0x9a,
    0x73, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, 0x54, 0x78, 0x01, 0x01, 0x0e, 0x00, 0xf1, 0xff,
    0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x1f, 0xee,
    0x05, 0xfb, 0xde, 0xdd, 0xec, 0x2b, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42,
    0x60, 0x82,
];

/// A baseline JPEG's header, height 3 and width 5: the start, an APP0
/// segment, the frame and the scan's start, the scan itself left out.
const BASELINE_JPEG: [u8; 49] = [
    0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00, 0x01,
    0x00, 0x01, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x03, 0x00, 0x05, 0x03, 0x01, 0x22,
    0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01, 0xff, 0xda, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3f,
    0x00,
];

/// A progressive JPEG's header, height 7 and width 11, fill bytes before an
/// APP1 segment.
const PROGRESSIVE_JPEG: [u8; 33] = [
    0xff, 0xd8, 0xff, 0xff, 0xff, 0xe1, 0x00, 0x04, 0x00, 0x00, 0xff, 0xc2, 0x00, 0x11, 0x08, 0x00,
    0x07, 0x00, 0x0b, 0x03, 0x01, 0x22, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01, 0xff, 0xda, 0x00,
    0x02,
];

/// A `drawing` relationship and an `image` one.
fn drawing_rel(id: &str, target: &str) -> String {
    relationship(id, &format!("{ATTR_NS}/drawing"), target)
}

fn image_rel(id: &str, target: &str) -> String {
    relationship(id, &format!("{ATTR_NS}/image"), target)
}

/// A worksheet part whose root binds `r` to the relationships namespace.
fn sheet_r(body: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{body}</worksheet>"
    )
}

/// A drawing part binding `xdr` and `a` on its root, with `children`.
fn drawing_xml(children: &str) -> String {
    format!(
        "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\">{children}</xdr:wsDr>"
    )
}

/// A picture as Excel writes one, its `cNvPr` `id` and its `blip` naming
/// `embed`, binding `r` on the `blip`.
fn pic(id: u32, embed: &str) -> String {
    format!(
        "<xdr:pic><xdr:nvPicPr><xdr:cNvPr id=\"{id}\" name=\"Picture {id}\" descr=\"d\"/>\
         <xdr:cNvPicPr/></xdr:nvPicPr><xdr:blipFill><a:blip xmlns:r=\"{ATTR_NS}\" \
         r:embed=\"{embed}\"/><a:stretch><a:fillRect/></a:stretch></xdr:blipFill><xdr:spPr>\
         <a:xfrm><a:off x=\"0\" y=\"0\"/><a:ext cx=\"9525\" cy=\"9525\"/></a:xfrm>\
         <a:prstGeom prst=\"rect\"><a:avLst/></a:prstGeom></xdr:spPr></xdr:pic>"
    )
}

/// A chart's frame, which holds no picture.
fn chart(id: u32) -> String {
    format!(
        "<xdr:graphicFrame macro=\"\"><xdr:nvGraphicFramePr><xdr:cNvPr id=\"{id}\" \
         name=\"Chart {id}\"/><xdr:cNvGraphicFramePr/></xdr:nvGraphicFramePr><xdr:xfrm>\
         <a:off x=\"0\" y=\"0\"/><a:ext cx=\"0\" cy=\"0\"/></xdr:xfrm><a:graphic>\
         <a:graphicData uri=\"http://schemas.openxmlformats.org/drawingml/2006/chart\"/>\
         </a:graphic></xdr:graphicFrame>"
    )
}

/// The `from` of an anchor at zero-based `column` and `row`.
fn from(column: u32, row: u32) -> String {
    format!(
        "<xdr:from><xdr:col>{column}</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>{row}</xdr:row>\
         <xdr:rowOff>0</xdr:rowOff></xdr:from>"
    )
}

fn two_cell(column: u32, row: u32, content: &str) -> String {
    format!(
        "<xdr:twoCellAnchor>{}<xdr:to><xdr:col>9</xdr:col><xdr:colOff>0</xdr:colOff>\
         <xdr:row>9</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:to>{content}<xdr:clientData/>\
         </xdr:twoCellAnchor>",
        from(column, row)
    )
}

fn one_cell(column: u32, row: u32, content: &str) -> String {
    format!(
        "<xdr:oneCellAnchor>{}<xdr:ext cx=\"9525\" cy=\"9525\"/>{content}<xdr:clientData/>\
         </xdr:oneCellAnchor>",
        from(column, row)
    )
}

fn absolute(content: &str) -> String {
    format!(
        "<xdr:absoluteAnchor><xdr:pos x=\"0\" y=\"0\"/><xdr:ext cx=\"9525\" cy=\"9525\"/>\
         {content}<xdr:clientData/></xdr:absoluteAnchor>"
    )
}

/// An `AlternateContent` with `choice` and `fallback`.
fn alternate(choice: &str, fallback: &str) -> String {
    format!(
        "<mc:AlternateContent xmlns:mc=\"{MC_NS}\"><mc:Choice Requires=\"a14\">{choice}\
         </mc:Choice><mc:Fallback>{fallback}</mc:Fallback></mc:AlternateContent>"
    )
}

/// The smallest package with its worksheet's body `body` naming the drawing
/// part as `rId1`, the drawing part `drawing` with its `Override`, its rels
/// part holding `drawing_rels` when given, and `media` PNG entries named
/// `xl/media/<name>`.
fn with_drawing(body: &str, drawing: String, drawing_rels: Option<&str>, media: &[&str]) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r(body));
    with_part(
        &mut parts,
        "xl/worksheets/_rels/sheet1.xml.rels",
        &rels(&drawing_rel("rId1", "../drawings/drawing1.xml")),
        None,
    );
    with_part(
        &mut parts,
        "xl/drawings/drawing1.xml",
        &drawing,
        Some(DRAWING_TYPE),
    );
    if let Some(body) = drawing_rels {
        with_part(
            &mut parts,
            "xl/drawings/_rels/drawing1.xml.rels",
            &rels(body),
            None,
        );
    }
    for name in media {
        parts.push((format!("xl/media/{name}").into_bytes(), PNG.to_vec()));
    }
    parts
}

/// The worksheet body the drawing fixtures use.
const BODY: &str = "<sheetData/><drawing r:id=\"rId1\"/>";

/// The image relationships `rId1` to `rId<count>` naming `image<k>.png`.
fn image_rels(count: u32) -> String {
    (1..=count)
        .map(|k| image_rel(&format!("rId{k}"), &format!("../media/image{k}.png")))
        .collect()
}

/// A part of the output, read back through a reopened package: a created
/// entry is deflated.
fn part_of(output: &[u8], name: &[u8]) -> Vec<u8> {
    let mut package = Package::open(Cursor::new(output.to_vec())).expect("open the output");
    let index = package.part(name).expect("the part");
    let mut bytes = Vec::new();
    package
        .read_part(index)
        .expect("read_part")
        .read_to_end(&mut bytes)
        .expect("read");
    bytes
}

/// A part of the output, as text.
fn text_of(output: &[u8], name: &[u8]) -> String {
    String::from_utf8(part_of(output, name)).expect("UTF-8")
}

/// The refusal a write of `edit` on `parts` gives, with no output.
#[track_caller]
fn refused_with(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(parts);
    edit(&mut package);
    refused(package)
}

/// Records the PNG at `cell` of the worksheet at its own size.
#[track_caller]
fn add(package: &mut Package<Cursor<Vec<u8>>>, cell: &str) {
    add_bytes(package, cell, &PNG);
}

#[track_caller]
fn add_bytes(package: &mut Package<Cursor<Vec<u8>>>, cell: &str, bytes: &[u8]) {
    package
        .add_image(SHEET, at(cell), Image { bytes, size: None })
        .expect("recorded");
}

/// The images `images` reads from the worksheet of `parts`.
#[track_caller]
fn read(parts: Parts) -> Result<Vec<ImageEntry>, Error> {
    let (_, mut package) = opened(parts);
    package.images(SHEET)
}

fn entry(from: (u32, u32), anchor: Anchor, k: u32) -> ImageEntry {
    ImageEntry {
        from,
        anchor,
        part: format!("xl/media/image{k}.png").into_bytes(),
        bytes: 82,
    }
}

/// An added anchor in the created part's form: `xdr` and `a` from the root,
/// `r` bound on the `blip`, at zero-based `column` and `row`, 2 by 2 pixels.
fn added_anchor(column: u32, row: u32, id: u32, rel: &str) -> String {
    format!(
        "<xdr:oneCellAnchor>{}<xdr:ext cx=\"19050\" cy=\"19050\"/><xdr:pic><xdr:nvPicPr>\
         <xdr:cNvPr id=\"{id}\" name=\"Image {id}\"/><xdr:cNvPicPr/></xdr:nvPicPr><xdr:blipFill>\
         <a:blip xmlns:r=\"{ATTR_NS}\" r:embed=\"{rel}\"/><a:stretch><a:fillRect/></a:stretch>\
         </xdr:blipFill><xdr:spPr><a:prstGeom prst=\"rect\"><a:avLst/></a:prstGeom></xdr:spPr>\
         </xdr:pic><xdr:clientData/></xdr:oneCellAnchor>",
        from(column, row)
    )
}

#[test]
fn the_images_of_each_anchor_kind_are_read() {
    let children = [
        two_cell(1, 2, &pic(2, "rId1")),
        one_cell(3, 4, &pic(3, "rId2")),
        two_cell(
            0,
            0,
            &format!(
                "<xdr:grpSp><xdr:nvGrpSpPr><xdr:cNvPr id=\"4\" name=\"Group 4\"/>\
                 <xdr:cNvGrpSpPr/></xdr:nvGrpSpPr><xdr:grpSpPr/>{}</xdr:grpSp>",
                pic(5, "rId3")
            ),
        ),
        two_cell(5, 6, &alternate(&pic(6, "rId4"), &pic(7, "rId5"))),
        two_cell(7, 8, &chart(8)),
        absolute(&pic(9, "rId6")),
    ]
    .concat();
    let media = [
        "image1.png",
        "image2.png",
        "image3.png",
        "image4.png",
        "image5.png",
        "image6.png",
    ];
    let parts = with_drawing(BODY, drawing_xml(&children), Some(&image_rels(6)), &media);
    assert_eq!(
        read(parts).expect("read"),
        vec![
            entry((1, 2), Anchor::TwoCell, 1),
            entry((3, 4), Anchor::OneCell, 2),
            entry((0, 0), Anchor::TwoCell, 3),
            entry((5, 6), Anchor::TwoCell, 4),
            entry((0, 0), Anchor::Absolute, 6),
        ]
    );
}

#[test]
fn an_anchor_in_a_root_choice_is_read() {
    // P3 ruling 1: the first anchor in a root `AlternateContent`'s first
    // `Choice` is read as a root anchor; the corpus's form, a chart's anchor
    // with an empty `Fallback`, gives no entry.
    let children = [
        alternate(&two_cell(2, 3, &pic(2, "rId1")), ""),
        alternate(&two_cell(4, 4, &chart(3)), ""),
    ]
    .concat();
    let parts = with_drawing(
        BODY,
        drawing_xml(&children),
        Some(&image_rels(1)),
        &["image1.png"],
    );
    assert_eq!(
        read(parts).expect("read"),
        vec![entry((2, 3), Anchor::TwoCell, 1)]
    );
}

#[test]
fn a_sheet_with_no_drawing_has_no_images() {
    assert_eq!(read(smallest()).expect("read"), Vec::new());
    let (_, mut package) = opened(smallest());
    assert_package(
        package
            .images(b"xl/worksheets/sheet9.xml")
            .expect_err("refused"),
        "no such part",
    );
}

#[test]
fn a_read_refuses_what_it_cannot_read() {
    let message = "a drawing part or its relationship cannot be read";
    let no_embed = pic(2, "rId1").replace(" r:embed=\"rId1\"", "");
    let no_column = two_cell(1, 2, &pic(2, "rId1")).replace("<xdr:col>1</xdr:col>", "");
    let cases = [
        // A `blip` without `r:embed`.
        with_drawing(
            BODY,
            drawing_xml(&two_cell(1, 2, &no_embed)),
            Some(&image_rels(1)),
            &["image1.png"],
        ),
        // An `r:embed` naming no relationship.
        with_drawing(
            BODY,
            drawing_xml(&two_cell(1, 2, &pic(2, "rId9"))),
            Some(&image_rels(1)),
            &["image1.png"],
        ),
        // A relationship whose entry is missing.
        with_drawing(
            BODY,
            drawing_xml(&two_cell(1, 2, &pic(2, "rId1"))),
            Some(&image_rels(1)),
            &[],
        ),
        // A `from` without its `col`.
        with_drawing(
            BODY,
            drawing_xml(&no_column),
            Some(&image_rels(1)),
            &["image1.png"],
        ),
        // A root other than `wsDr`.
        with_drawing(
            BODY,
            format!(
                "{DECLARATION}\n<c:userShapes xmlns:c=\"http://schemas.openxmlformats.org/drawingml/2006/chart\"/>"
            ),
            None,
            &[],
        ),
    ];
    for parts in cases {
        assert_package(read(parts).expect_err("refused"), message);
    }
    // A relationship whose part is missing.
    let mut parts = with_drawing(BODY, drawing_xml(""), None, &[]);
    drop_part(&mut parts, DRAWING);
    assert_package(read(parts).expect_err("refused"), message);
}

#[test]
fn the_header_reads_take_a_png_and_both_jpeg_frames() {
    assert_eq!(
        crate::package::image_header(&PNG).expect("read"),
        (true, 2, 2)
    );
    assert_eq!(
        crate::package::image_header(&BASELINE_JPEG).expect("read"),
        (false, 5, 3)
    );
    assert_eq!(
        crate::package::image_header(&PROGRESSIVE_JPEG).expect("read"),
        (false, 11, 7)
    );
}

/// A call-time case: the cell, the bytes, the size and the message.
type Refusal<'a> = (CellRef, Vec<u8>, Option<(u32, u32)>, &'a str);

#[test]
fn the_call_refuses_each_rule_and_records_nothing() {
    let not_image = "an image is not a PNG or JPEG";
    let size = "an image size is out of range";
    let mut idat_first = PNG.to_vec();
    idat_first[12..16].copy_from_slice(b"IDAT");
    let no_frame: Vec<u8> = vec![
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x00, 0x00, 0xff, 0xda, 0x00, 0x02,
    ];
    let gif = b"GIF89a\x02\x00\x02\x00\x00\x00\x00".to_vec();
    let cases: [Refusal<'_>; 7] = [
        (at("A1"), idat_first, None, not_image),
        (at("A1"), no_frame, None, not_image),
        (at("A1"), PNG[..23].to_vec(), None, not_image),
        (at("A1"), gif, None, not_image),
        (at("A1"), PNG.to_vec(), Some((0, 2)), size),
        (at("A1"), PNG.to_vec(), Some((225_458, 1)), size),
        (
            CellRef {
                row: 1_048_577,
                col: 1,
            },
            PNG.to_vec(),
            None,
            "a row or column index or count is out of range",
        ),
    ];
    for (cell, bytes, size, message) in cases {
        let (parts, mut package) = opened(smallest());
        assert_package(
            package
                .add_image(
                    SHEET,
                    cell,
                    Image {
                        bytes: &bytes,
                        size,
                    },
                )
                .expect_err("refused"),
            message,
        );
        let mut out = Vec::new();
        {
            let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            package.write_to(&mut out).expect("write_to");
        }
        assert_untouched(&out, &parts, &[]);
    }
}

#[test]
fn the_size_override_sets_the_extent() {
    let output = edited_output(smallest(), |p| {
        p.add_image(
            SHEET,
            at("A1"),
            Image {
                bytes: &PNG,
                size: Some((225_457, 3)),
            },
        )
        .expect("recorded");
    });
    assert!(
        text_of(&output, DRAWING).contains("<xdr:ext cx=\"2147477925\" cy=\"28575\"/>"),
        "the extent is the override's"
    );
}

#[test]
fn the_created_parts_bytes() {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r("<sheetData/>"));
    let output = edited_output(parts.clone(), |p| add(p, "B3"));
    assert_untouched(&output, &parts, &[SHEET, b"[Content_Types].xml".as_slice()]);
    assert_eq!(
        text_of(&output, DRAWING),
        format!(
            "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\">{}</xdr:wsDr>",
            added_anchor(1, 2, 1, "rId1")
        )
    );
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&image_rel("rId1", "../media/image1.png"))
    );
    assert_eq!(part_of(&output, b"xl/media/image1.png"), PNG.to_vec());
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r("<sheetData/><drawing r:id=\"rId1\"/>")
    );
    assert_eq!(
        text_of(&output, SHEET_RELS),
        rels(&drawing_rel("rId1", "../drawings/drawing1.xml"))
    );
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        part_text(&parts, b"[Content_Types].xml").replace(
            "</Types>",
            &format!(
                "<Override PartName=\"/xl/media/image1.png\" ContentType=\"image/png\"/>\
                 <Override PartName=\"/xl/drawings/drawing1.xml\" ContentType=\"{DRAWING_TYPE}\"/>\
                 </Types>"
            )
        )
    );
}

#[test]
fn an_addition_to_a_drawing_of_two_anchors_appends_with_the_next_id() {
    let kept = [
        two_cell(0, 0, &pic(1, "rId1")),
        two_cell(2, 2, &pic(2, "rId2")),
    ]
    .concat();
    let media = ["image1.png", "image2.png"];
    let parts = with_drawing(BODY, drawing_xml(&kept), Some(&image_rels(2)), &media);
    let output = edited_output(parts.clone(), |p| add(p, "C5"));
    assert_untouched(
        &output,
        &parts,
        &[DRAWING, DRAWING_RELS, b"[Content_Types].xml".as_slice()],
    );
    assert_eq!(
        text_of(&output, DRAWING),
        drawing_xml(&format!("{kept}{}", added_anchor(2, 4, 3, "rId3")))
    );
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&format!(
            "{}{}",
            image_rels(2),
            image_rel("rId3", "../media/image3.png")
        ))
    );
    assert_eq!(part_of(&output, b"xl/media/image3.png"), PNG.to_vec());
}

#[test]
fn the_namespaces_are_bound_on_the_added_elements_or_taken_from_the_root() {
    // A root binding neither `a` nor `r`: both bound on the added elements.
    let kept = two_cell(0, 0, &chart(1)).replace(
        "<xdr:graphicFrame ",
        &format!("<xdr:graphicFrame xmlns:a=\"{A_NS}\" "),
    );
    let drawing = format!("{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\">{kept}</xdr:wsDr>");
    let parts = with_drawing(BODY, drawing, None, &[]);
    let output = edited_output(parts, |p| add(p, "A1"));
    let added = added_anchor(0, 0, 2, "rId1").replacen(
        "<xdr:oneCellAnchor>",
        &format!("<xdr:oneCellAnchor xmlns:a=\"{A_NS}\">"),
        1,
    );
    assert_eq!(
        text_of(&output, DRAWING),
        format!("{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\">{kept}{added}</xdr:wsDr>")
    );
    // The drawing had no rels part: one is created.
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&image_rel("rId1", "../media/image1.png"))
    );
    // A root binding the drawing namespace as its default and the other two
    // to prefixes of its own: those are used.
    let drawing = format!(
        "{DECLARATION}\n<wsDr xmlns=\"{XDR_NS}\" xmlns:d=\"{A_NS}\" xmlns:q=\"{ATTR_NS}\"></wsDr>"
    );
    let parts = with_drawing(BODY, drawing, None, &[]);
    let output = edited_output(parts, |p| add(p, "A1"));
    let added = "<oneCellAnchor><from><col>0</col><colOff>0</colOff><row>0</row><rowOff>0</rowOff></from>\
         <ext cx=\"19050\" cy=\"19050\"/><pic><nvPicPr><cNvPr id=\"1\" name=\"Image 1\"/><cNvPicPr/>\
         </nvPicPr><blipFill><d:blip q:embed=\"rId1\"/><d:stretch><d:fillRect/></d:stretch>\
         </blipFill><spPr><d:prstGeom prst=\"rect\"><d:avLst/></d:prstGeom></spPr></pic>\
         <clientData/></oneCellAnchor>";
    assert_eq!(
        text_of(&output, DRAWING),
        format!(
            "{DECLARATION}\n<wsDr xmlns=\"{XDR_NS}\" xmlns:d=\"{A_NS}\" xmlns:q=\"{ATTR_NS}\">{added}</wsDr>"
        )
    );
}

#[test]
fn a_drawing_whose_root_is_user_shapes_is_refused() {
    let drawing = format!(
        "{DECLARATION}\n<c:userShapes xmlns:c=\"http://schemas.openxmlformats.org/drawingml/2006/chart\"/>"
    );
    assert_package(
        refused_with(with_drawing(BODY, drawing, None, &[]), |p| add(p, "A1")),
        "a drawing part or its relationship cannot be read",
    );
}

#[test]
fn the_template_part_is_retagged_and_its_drawing_and_rels_parts_created() {
    let mut package = new_package();
    add(&mut package, "A1");
    let out = {
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = Vec::new();
        package.write_to(&mut out).expect("write_to");
        out
    };
    let sheet = text_of(&out, SHEET);
    let sheet_rels = text_of(&out, SHEET_RELS);
    let drawing_rels = text_of(&out, DRAWING_RELS);
    println!("template worksheet after the image: {sheet}");
    println!("template worksheet's created rels part: {sheet_rels}");
    println!("template drawing's created rels part: {drawing_rels}");
    let template = String::from_utf8(TEMPLATE[4].1.to_vec()).expect("UTF-8");
    let want = template
        .replace(
            &format!("<worksheet xmlns=\"{MAIN_NS}\">"),
            &format!("<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">"),
        )
        .replace("</worksheet>", "<drawing r:id=\"rId1\"/></worksheet>");
    assert_ne!(want, template, "the template's root was found");
    assert_eq!(sheet, want);
    assert!(sheet_rels.ends_with(&format!(
        "<Relationships xmlns=\"{RELS_NS}\">{}</Relationships>",
        drawing_rel("rId1", "../drawings/drawing1.xml")
    )));
    assert!(drawing_rels.ends_with(&format!(
        "<Relationships xmlns=\"{RELS_NS}\">{}</Relationships>",
        image_rel("rId1", "../media/image1.png")
    )));
    assert_eq!(part_of(&out, b"xl/media/image1.png"), PNG.to_vec());
}

#[test]
fn a_drawing_is_created_before_legacy_drawing() {
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId1\"/>"),
    );
    with_part(
        &mut parts,
        "xl/worksheets/_rels/sheet1.xml.rels",
        &rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/vmlDrawing"),
            "../drawings/vmlDrawing1.vml",
        )),
        None,
    );
    let output = edited_output(parts, |p| add(p, "A1"));
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r("<sheetData/><drawing r:id=\"rId2\"/><legacyDrawing r:id=\"rId1\"/>")
    );
}

#[test]
fn the_media_name_skips_names_in_use_over_both_extensions() {
    let mut parts = smallest();
    parts.push((b"xl/media/image1.png".to_vec(), PNG.to_vec()));
    parts.push((b"xl/media/image2.jpeg".to_vec(), BASELINE_JPEG.to_vec()));
    let output = edited_output(parts, |p| add(p, "A1"));
    assert_eq!(part_of(&output, b"xl/media/image3.png"), PNG.to_vec());
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&image_rel("rId1", "../media/image3.png"))
    );
}

#[test]
fn two_images_in_one_write() {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r("<sheetData/>"));
    let output = edited_output(parts.clone(), |p| {
        add(p, "A1");
        add_bytes(p, "B2", &BASELINE_JPEG);
    });
    let jpeg_anchor = added_anchor(1, 1, 2, "rId2")
        .replace("cx=\"19050\" cy=\"19050\"", "cx=\"47625\" cy=\"28575\"");
    assert_eq!(
        text_of(&output, DRAWING),
        format!(
            "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\">{}{jpeg_anchor}</xdr:wsDr>",
            added_anchor(0, 0, 1, "rId1")
        )
    );
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&format!(
            "{}{}",
            image_rel("rId1", "../media/image1.png"),
            image_rel("rId2", "../media/image2.jpeg")
        ))
    );
    assert_eq!(
        part_of(&output, b"xl/media/image2.jpeg"),
        BASELINE_JPEG.to_vec()
    );
    assert!(
        text_of(&output, b"[Content_Types].xml").contains(
            "<Override PartName=\"/xl/media/image1.png\" ContentType=\"image/png\"/>\
             <Override PartName=\"/xl/media/image2.jpeg\" ContentType=\"image/jpeg\"/>"
        ),
        "the two media overrides, in record order"
    );
}

#[test]
fn two_drawing_elements_or_one_out_of_schema_order_are_refused() {
    let message = "a worksheet has two drawing elements";
    for body in [
        "<sheetData/><drawing r:id=\"rId1\"/><drawing r:id=\"rId1\"/>",
        "<sheetData/><legacyDrawing r:id=\"rId2\"/><drawing r:id=\"rId1\"/>",
    ] {
        let parts = with_drawing(body, drawing_xml(""), None, &[]);
        assert_package(refused_with(parts, |p| add(p, "A1")), message);
    }
}

#[test]
fn the_write_refuses_a_drawing_it_cannot_read() {
    let message = "a drawing part or its relationship cannot be read";
    // A `<drawing>` without `r:id`, one naming no relationship, and one whose
    // relationship's part is missing.
    let parts = with_drawing("<sheetData/><drawing/>", drawing_xml(""), None, &[]);
    assert_package(refused_with(parts, |p| add(p, "A1")), message);
    let parts = with_drawing(
        "<sheetData/><drawing r:id=\"rId7\"/>",
        drawing_xml(""),
        None,
        &[],
    );
    assert_package(refused_with(parts, |p| add(p, "A1")), message);
    let mut parts = with_drawing(BODY, drawing_xml(""), None, &[]);
    drop_part(&mut parts, DRAWING);
    assert_package(refused_with(parts, |p| add(p, "A1")), message);
    // A `cNvPr` `id` that is not a number.
    let kept = two_cell(0, 0, &pic(1, "rId1")).replace("id=\"1\"", "id=\"x\"");
    let parts = with_drawing(
        BODY,
        drawing_xml(&kept),
        Some(&image_rels(1)),
        &["image1.png"],
    );
    assert_package(refused_with(parts, |p| add(p, "A1")), message);
}

#[test]
fn an_image_on_a_removed_replaced_or_added_part_is_refused() {
    let (_, mut package) = opened(smallest());
    add(&mut package, "A1");
    package.remove_part(SHEET);
    assert_package(refused(package), "an edit to a removed part");
    let (_, mut package) = opened(smallest());
    add(&mut package, "A1");
    package.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
    assert_package(
        refused(package),
        "a cell edit to a part this write replaces",
    );
    let (_, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add_sheet");
    package
        .add_image(
            &part,
            at("A1"),
            Image {
                bytes: &PNG,
                size: None,
            },
        )
        .expect("recorded");
    assert_package(refused(package), "no such part");
    // The sheet's drawing part removed or replaced through the part calls.
    let parts = with_drawing(BODY, drawing_xml(""), None, &[]);
    assert_package(
        refused_with(parts.clone(), |p| {
            p.remove_part(DRAWING);
            add(p, "A1");
        }),
        "an edit to a removed part",
    );
    assert_package(
        refused_with(parts, |p| {
            p.replace_part(DRAWING, replacement_from_reader(Cursor::new(b"x".to_vec())));
            add(p, "A1");
        }),
        "a cell edit to a part this write replaces",
    );
}

#[cfg(feature = "rowcol")]
#[test]
fn an_image_with_a_row_insertion_is_refused() {
    assert_package(
        refused_with(smallest(), |p| {
            p.insert_rows(SHEET, 20, 1).expect("recorded");
            add(p, "A1");
        }),
        "a worksheet with a row or column insertion or deletion takes no other edit in the same write",
    );
}

#[test]
fn the_read_and_the_write_follow_the_drawing_element_past_another_drawing_relationship() {
    // P3 verification ruling 1 (2): the sheet's first drawing relationship
    // names a part its `<drawing>` element does not; both calls follow the
    // element's `r:id`, rId2.
    let mut parts = with_drawing(
        "<sheetData/><drawing r:id=\"rId2\"/>",
        drawing_xml(&two_cell(1, 1, &pic(1, "rId1"))),
        Some(&image_rels(1)),
        &["image1.png", "image2.png"],
    );
    set(
        &mut parts,
        b"xl/worksheets/_rels/sheet1.xml.rels",
        rels(&format!(
            "{}{}",
            drawing_rel("rId1", "../drawings/drawing1.xml"),
            drawing_rel("rId2", "../drawings/drawing2.xml")
        )),
    );
    with_part(
        &mut parts,
        "xl/drawings/drawing2.xml",
        &drawing_xml(&two_cell(4, 5, &pic(7, "rId1"))),
        Some(DRAWING_TYPE),
    );
    with_part(
        &mut parts,
        "xl/drawings/_rels/drawing2.xml.rels",
        &rels(&image_rel("rId1", "../media/image2.png")),
        None,
    );
    assert_eq!(
        read(parts.clone()).expect("read"),
        vec![entry((4, 5), Anchor::TwoCell, 2)]
    );
    let output = edited_output(parts.clone(), |p| add(p, "A1"));
    assert_untouched(
        &output,
        &parts,
        &[
            b"xl/drawings/drawing2.xml".as_slice(),
            b"xl/drawings/_rels/drawing2.xml.rels",
            b"[Content_Types].xml",
        ],
    );
    assert_eq!(
        text_of(&output, b"xl/drawings/drawing2.xml"),
        drawing_xml(&format!(
            "{}{}",
            two_cell(4, 5, &pic(7, "rId1")),
            added_anchor(0, 0, 8, "rId2")
        ))
    );
    // A sheet whose drawing relationship no `<drawing>` names has no images.
    let mut parts = with_drawing(
        "<sheetData/>",
        drawing_xml(&two_cell(1, 1, &pic(1, "rId1"))),
        Some(&image_rels(1)),
        &["image1.png"],
    );
    set(&mut parts, SHEET, sheet_r("<sheetData/>"));
    assert_eq!(read(parts).expect("read"), Vec::new());
}

#[test]
fn a_strict_sheet_s_created_drawing_takes_the_transitional_namespaces_and_the_strict_types() {
    // P3 verification ruling 1 (3).
    const STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
    let mut parts = smallest();
    let workbook_rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        &format!("{ATTR_NS}/worksheet"),
        &format!("{STRICT}/worksheet"),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", workbook_rels);
    let output = edited_output(parts, |p| add(p, "B3"));
    assert_eq!(
        text_of(&output, DRAWING),
        format!(
            "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\">{}</xdr:wsDr>",
            added_anchor(1, 2, 1, "rId1")
        )
    );
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&relationship(
            "rId1",
            &format!("{STRICT}/image"),
            "../media/image1.png"
        ))
    );
    assert_eq!(
        text_of(&output, SHEET),
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{STRICT}\"><sheetData/>\
             <drawing r:id=\"rId1\"/></worksheet>"
        )
    );
    assert!(
        text_of(&output, SHEET_RELS).ends_with(&format!(
            "<Relationships xmlns=\"{RELS_NS}\">{}</Relationships>",
            relationship(
                "rId1",
                &format!("{STRICT}/drawing"),
                "../drawings/drawing1.xml"
            )
        )),
        "the sheet's drawing relationship takes the strict type"
    );
}

#[cfg(feature = "comments")]
#[test]
fn a_drawing_and_a_legacy_drawing_created_in_one_write_come_in_schema_order() {
    // P3 verification ruling 1 (5): the comment's parts are recorded first
    // (rId1 and rId2), the drawing's after (rId3); the elements come out
    // drawing then legacyDrawing.
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        sheet_r("<sheetData/><tableParts count=\"0\"/>"),
    );
    let output = edited_output(parts, |p| {
        p.add_comment(
            SHEET,
            at("A1"),
            Comment {
                author: "Ann",
                text: "one",
            },
        )
        .expect("recorded");
        add(p, "A1");
    });
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r(
            "<sheetData/><drawing r:id=\"rId3\"/><legacyDrawing r:id=\"rId2\"/>\
             <tableParts count=\"0\"/>"
        )
    );
}

#[cfg(all(feature = "sheetprops", feature = "metadata"))]
#[test]
fn the_four_page_elements_land_between_a_created_hyperlinks_and_a_created_drawing() {
    // specs/0025 P2 verification ruling 2 (C2): with a `<hyperlinks>` and a
    // `<drawing>` created in the same write, the settings pass's four
    // elements go between them in schema order, at the root's end and
    // before a later root child alike.
    let page = "<printOptions horizontalCentered=\"1\"/><pageMargins left=\"0.5\" right=\"0.7\" \
                top=\"0.75\" bottom=\"0.75\" header=\"0.3\" footer=\"0.3\"/><pageSetup \
                orientation=\"landscape\"/><headerFooter><oddHeader>&amp;Ca</oddHeader></headerFooter>";
    let link = "<hyperlinks><hyperlink ref=\"A1\" location=\"S!A1\"/></hyperlinks>";
    for after in ["", "<webPublishItems count=\"0\"/>"] {
        let mut parts = smallest();
        set(&mut parts, SHEET, sheet_r(&format!("<sheetData/>{after}")));
        let output = edited_output(parts, |p| {
            p.set_header_footer(
                SHEET,
                &HeaderFooter {
                    odd_header: Some("&Ca".to_string()),
                    ..HeaderFooter::default()
                },
            )
            .expect("recorded");
            p.set_page_setup(
                SHEET,
                &PageSetup {
                    orientation: Some(Orientation::Landscape),
                    ..PageSetup::default()
                },
            )
            .expect("recorded");
            p.set_page_margins(
                SHEET,
                &PageMargins {
                    left: Some(0.5),
                    ..PageMargins::default()
                },
            )
            .expect("recorded");
            p.set_print_options(
                SHEET,
                &PrintOptions {
                    horizontal_centered: Some(true),
                    ..PrintOptions::default()
                },
            )
            .expect("recorded");
            add(p, "B3");
            p.set_hyperlink(SHEET, at("A1"), LinkTarget::Location("S!A1"), None)
                .expect("set");
        });
        assert_eq!(
            text_of(&output, SHEET),
            sheet_r(&format!(
                "<sheetData/>{link}{page}<drawing r:id=\"rId1\"/>{after}"
            )),
            "{after}"
        );
    }
}
