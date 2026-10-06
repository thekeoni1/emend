//! Image anchors on byte-built worksheets (`specs/0028-images-and-comments-
//! second-half.md`, "Anchors" and "P2: the anchors and the removal"): each
//! test records an image on a given anchor in the smallest package and
//! compares the created drawing part with the bytes the design gives, or
//! records a refused anchor and asserts nothing was recorded. The Cargo
//! feature `images`.

use super::*;

/// The created drawing part holding `anchors`.
fn created(anchors: &str) -> String {
    format!(
        "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\">{anchors}</xdr:wsDr>"
    )
}

/// The picture of an added anchor, its `cNvPr` `id` 1 and its `blip`
/// naming `rId1`, then the `clientData`.
const ADDED_PIC: &str = "<xdr:pic><xdr:nvPicPr><xdr:cNvPr id=\"1\" name=\"Image 1\"/>\
    <xdr:cNvPicPr/></xdr:nvPicPr><xdr:blipFill><a:blip \
    xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\" \
    r:embed=\"rId1\"/><a:stretch><a:fillRect/></a:stretch></xdr:blipFill><xdr:spPr>\
    <a:prstGeom prst=\"rect\"><a:avLst/></a:prstGeom></xdr:spPr></xdr:pic><xdr:clientData/>";

/// The created drawing part of a write of `image` on `anchor` on the
/// smallest package's worksheet.
fn drawing_of(anchor: ImageAnchor, image: Image<'_>) -> String {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r("<sheetData/>"));
    let output = edited_output(parts, |p| {
        p.add_image_at(SHEET, anchor, image).expect("recorded");
    });
    text_of(&output, DRAWING)
}

/// The PNG at its own size, 2 by 2 pixels.
fn png() -> Image<'static> {
    Image {
        bytes: &PNG,
        size: None,
    }
}

#[test]
fn a_one_cell_anchor_writes_its_offsets_in_emu() {
    assert_eq!(
        drawing_of(
            ImageAnchor::OneCell {
                from: at("B3"),
                offset: (10, 20),
            },
            png(),
        ),
        created(&format!(
            "<xdr:oneCellAnchor><xdr:from><xdr:col>1</xdr:col><xdr:colOff>95250</xdr:colOff>\
             <xdr:row>2</xdr:row><xdr:rowOff>190500</xdr:rowOff></xdr:from>\
             <xdr:ext cx=\"19050\" cy=\"19050\"/>{ADDED_PIC}</xdr:oneCellAnchor>"
        ))
    );
}

#[test]
fn add_image_is_a_one_cell_anchor_at_the_cell_with_zero_offsets() {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r("<sheetData/>"));
    let by_cell = edited_output(parts.clone(), |p| add(p, "C5"));
    let by_anchor = edited_output(parts, |p| {
        p.add_image_at(
            SHEET,
            ImageAnchor::OneCell {
                from: at("C5"),
                offset: (0, 0),
            },
            png(),
        )
        .expect("recorded");
    });
    assert_eq!(by_cell, by_anchor);
    assert_eq!(
        text_of(&by_cell, DRAWING),
        created(&added_anchor(2, 4, 1, "rId1"))
    );
}

#[test]
fn a_two_cell_anchor_writes_edit_as_from_and_to_for_each_value() {
    for (edit_as, word) in [
        (EditAs::TwoCell, "twoCell"),
        (EditAs::OneCell, "oneCell"),
        (EditAs::Absolute, "absolute"),
    ] {
        assert_eq!(
            drawing_of(
                ImageAnchor::TwoCell {
                    from: at("B3"),
                    to: at("D6"),
                    from_offset: (1, 2),
                    to_offset: (3, 4),
                    edit_as,
                },
                png(),
            ),
            created(&format!(
                "<xdr:twoCellAnchor editAs=\"{word}\"><xdr:from><xdr:col>1</xdr:col>\
                 <xdr:colOff>9525</xdr:colOff><xdr:row>2</xdr:row><xdr:rowOff>19050</xdr:rowOff>\
                 </xdr:from><xdr:to><xdr:col>3</xdr:col><xdr:colOff>28575</xdr:colOff>\
                 <xdr:row>5</xdr:row><xdr:rowOff>38100</xdr:rowOff></xdr:to>{ADDED_PIC}\
                 </xdr:twoCellAnchor>"
            ))
        );
    }
}

#[test]
fn an_absolute_anchor_writes_pos_and_the_extent() {
    assert_eq!(
        drawing_of(
            ImageAnchor::Absolute { position: (96, 96) },
            Image {
                bytes: &PNG,
                size: Some((4, 4)),
            },
        ),
        created(&format!(
            "<xdr:absoluteAnchor><xdr:pos x=\"914400\" y=\"914400\"/>\
             <xdr:ext cx=\"38100\" cy=\"38100\"/>{ADDED_PIC}</xdr:absoluteAnchor>"
        ))
    );
}

#[test]
fn each_form_reads_back_with_its_kind_and_from() {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r("<sheetData/>"));
    let output = edited_output(parts, |p| {
        for anchor in [
            ImageAnchor::OneCell {
                from: at("B3"),
                offset: (5, 5),
            },
            ImageAnchor::TwoCell {
                from: at("C4"),
                to: at("C4"),
                from_offset: (0, 0),
                to_offset: (8, 8),
                edit_as: EditAs::OneCell,
            },
            ImageAnchor::Absolute { position: (1, 2) },
        ] {
            p.add_image_at(SHEET, anchor, png()).expect("recorded");
        }
    });
    let mut reopened = Package::open(Cursor::new(output)).expect("open the output");
    let read: Vec<((u32, u32), Anchor)> = reopened
        .images(SHEET)
        .expect("read back")
        .into_iter()
        .map(|entry| (entry.from, entry.anchor))
        .collect();
    assert_eq!(
        read,
        [
            ((1, 2), Anchor::OneCell),
            ((2, 3), Anchor::TwoCell),
            ((0, 0), Anchor::Absolute),
        ]
    );
}

#[test]
fn the_call_refuses_each_anchor_rule_and_records_nothing() {
    let range = "an image anchor is out of range";
    let index = "a row or column index or count is out of range";
    // 225457 pixels is the most whose EMU stays within i32::MAX.
    let over = 225_458;
    let two = |from: &str, to: &str, from_offset, to_offset| ImageAnchor::TwoCell {
        from: at(from),
        to: at(to),
        from_offset,
        to_offset,
        edit_as: EditAs::TwoCell,
    };
    let off_grid = CellRef {
        row: 1_048_577,
        col: 1,
    };
    // Each anchor, the image's size, and the message.
    type Case<'m> = (ImageAnchor, Option<(u32, u32)>, &'m str);
    let cases: Vec<Case<'_>> = vec![
        // `to` before `from` in row, then in column.
        (two("B3", "B2", (0, 0), (0, 0)), None, range),
        (two("B3", "A3", (0, 0), (0, 0)), None, range),
        // An offset or a position past i32::MAX EMU.
        (two("B3", "C4", (over, 0), (0, 0)), None, range),
        (two("B3", "C4", (0, 0), (0, over)), None, range),
        (
            ImageAnchor::OneCell {
                from: at("A1"),
                offset: (0, over),
            },
            None,
            range,
        ),
        (
            ImageAnchor::Absolute {
                position: (over, 0),
            },
            None,
            range,
        ),
        // A size on a two-cell anchor.
        (
            two("B3", "C4", (0, 0), (0, 0)),
            Some((4, 4)),
            "an image size does not apply to a two-cell anchor",
        ),
        // A cell off the grid.
        (
            ImageAnchor::TwoCell {
                from: at("A1"),
                to: off_grid,
                from_offset: (0, 0),
                to_offset: (0, 0),
                edit_as: EditAs::TwoCell,
            },
            None,
            index,
        ),
        (
            ImageAnchor::OneCell {
                from: off_grid,
                offset: (0, 0),
            },
            None,
            index,
        ),
    ];
    for (anchor, size, message) in cases {
        let (_, mut package) = opened(smallest());
        assert_package(
            package
                .add_image_at(SHEET, anchor, Image { bytes: &PNG, size })
                .expect_err("refused"),
            message,
        );
        let out = written(package);
        assert_untouched(&out, &smallest(), &[]);
    }
    // The limits themselves are taken: a two-cell anchor on one cell, and
    // the largest offset.
    let (_, mut package) = opened(smallest());
    for anchor in [
        two("B3", "B3", (0, 0), (0, 0)),
        two("B3", "C4", (over - 1, 0), (0, over - 1)),
        ImageAnchor::Absolute {
            position: (over - 1, over - 1),
        },
    ] {
        package
            .add_image_at(SHEET, anchor, png())
            .expect("recorded");
    }
}
