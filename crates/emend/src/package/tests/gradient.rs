//! Gradient fills on byte-built styles parts
//! (`specs/0042-gradient-fills-and-named-styles.md`, "Types", "The styles
//! part" and "P1: gradient fills"): each test builds the smallest package
//! with its styles part holding a given body, adds a cell format with a
//! gradient fill, and compares the written styles part with the bytes the
//! design gives, every other part asserted untouched; the typed read of
//! each form and its refusals, and a differential style's round trip.

use super::*;
use crate::styles::{Gradient, GradientKind, Stop};

/// The two stops the tests write: opaque black at 0, opaque white at 1.
fn two_stops() -> Vec<Stop> {
    vec![
        Stop {
            position: 0.0,
            color: Color::Rgb(0xFF00_0000),
        },
        Stop {
            position: 1.0,
            color: Color::Rgb(0xFFFF_FFFF),
        },
    ]
}

/// A fill holding `kind` over [`two_stops`] and nothing else.
fn gradient(kind: GradientKind) -> Fill {
    Fill {
        pattern: Pattern::None,
        foreground: None,
        background: None,
        gradient: Some(Gradient {
            kind,
            stops: two_stops(),
        }),
    }
}

/// A linear gradient at `degree`.
fn linear(degree: f64) -> Fill {
    gradient(GradientKind::Linear { degree })
}

/// A path gradient from the rectangle the four give.
fn path(left: f64, right: f64, top: f64, bottom: f64) -> Fill {
    gradient(GradientKind::Path {
        left,
        right,
        top,
        bottom,
    })
}

/// A style with `fill` alone.
fn filled(fill: Fill) -> Style {
    Style {
        fill: Some(fill),
        ..Style::default()
    }
}

/// [`two_stops`] as written, after a `gradientFill` start tag.
const STOPS: &str = "<stop position=\"0\"><color rgb=\"FF000000\"/></stop>\
                     <stop position=\"1\"><color rgb=\"FFFFFFFF\"/></stop></gradientFill></fill>";

/// The styles part after one `add_style` of `fill` over no base, against
/// [`with_collections`] with one plain cell format: `fills` grows by the
/// element `start` and [`STOPS`] give, and `cellXfs` by the new format.
#[track_caller]
fn assert_written(fill: Fill, start: &str) {
    let body = with_collections("<cellXfs count=\"1\"><xf/></cellXfs>");
    let got = added_styles(&body, |p| add_styled(p, None, &filled(fill), 1));
    let expected = body
        .replace(
            "<fill><patternFill patternType=\"gray125\"/></fill></fills>",
            &format!(
                "<fill><patternFill patternType=\"gray125\"/></fill><fill>{start}{STOPS}</fills>"
            ),
        )
        .replace("<fills count=\"2\">", "<fills count=\"3\">")
        .replace(
            "<cellXfs count=\"1\"><xf/></cellXfs>",
            "<cellXfs count=\"2\"><xf/><xf numFmtId=\"0\" fontId=\"0\" fillId=\"2\" \
             borderId=\"0\" xfId=\"0\" applyFill=\"1\"/></cellXfs>",
        );
    assert_eq!(got, style_sheet(&expected));
}

#[test]
fn a_linear_gradient_is_written_with_its_degree_and_its_stops() {
    assert_written(linear(90.0), "<gradientFill degree=\"90\">");
    assert_written(linear(22.5), "<gradientFill degree=\"22.5\">");
    assert_written(linear(360.0), "<gradientFill degree=\"360\">");
}

#[test]
fn a_linear_gradient_of_degree_zero_writes_no_degree() {
    // 0 is the schema's default, and Excel's form writes none.
    assert_written(linear(0.0), "<gradientFill>");
}

#[test]
fn a_path_gradient_writes_its_type_and_all_four_sides() {
    assert_written(
        path(0.5, 0.5, 0.5, 0.5),
        "<gradientFill type=\"path\" left=\"0.5\" right=\"0.5\" top=\"0.5\" bottom=\"0.5\">",
    );
    // Each of the four is written, a 0 among them.
    assert_written(
        path(0.0, 1.0, 0.25, 0.0),
        "<gradientFill type=\"path\" left=\"0\" right=\"1\" top=\"0.25\" bottom=\"0\">",
    );
}

#[test]
fn a_stop_takes_each_colour_form_and_equal_gradients_share_one_fill() {
    let theme = Fill {
        gradient: Some(Gradient {
            kind: GradientKind::Linear { degree: 45.0 },
            stops: vec![
                Stop {
                    position: 0.0,
                    color: Color::Theme {
                        index: 4,
                        tint: Some(-0.25),
                    },
                },
                Stop {
                    position: 0.5,
                    color: Color::Indexed(10),
                },
                Stop {
                    position: 0.5,
                    color: Color::Auto,
                },
            ],
        }),
        ..linear(0.0)
    };
    let body = with_collections("<cellXfs count=\"1\"><xf/></cellXfs>");
    let got = added_styles(&body, |p| {
        add_styled(p, None, &filled(theme.clone()), 1);
        add_styled(p, None, &filled(theme.clone()), 2);
        add_styled(p, None, &filled(linear(90.0)), 3);
    });
    assert!(
        got.contains(
            "<fills count=\"4\"><fill><patternFill patternType=\"none\"/></fill>\
             <fill><patternFill patternType=\"gray125\"/></fill>\
             <fill><gradientFill degree=\"45\"><stop position=\"0\">\
             <color theme=\"4\" tint=\"-0.25\"/></stop><stop position=\"0.5\">\
             <color indexed=\"10\"/></stop><stop position=\"0.5\"><color auto=\"1\"/></stop>\
             </gradientFill></fill><fill><gradientFill degree=\"90\">"
        ),
        "fills: {got}"
    );
    // The first two formats name the one fill, the third the next.
    for (fill, formats) in [(2, 2), (3, 1)] {
        assert_eq!(
            got.matches(&format!(
                "fillId=\"{fill}\" borderId=\"0\" xfId=\"0\" applyFill=\"1\"/>"
            ))
            .count(),
            formats,
            "the formats naming fill {fill}"
        );
    }
}

#[test]
fn the_gradient_s_elements_take_the_fills_prefix() {
    let body = format!(
        "<x:fills xmlns:x=\"{MAIN_NS}\" count=\"1\"><x:fill><x:patternFill patternType=\"none\"/>\
         </x:fill></x:fills><cellXfs count=\"1\"><xf/></cellXfs>"
    );
    let got = added_styles(&body, |p| add_styled(p, None, &filled(linear(90.0)), 1));
    assert!(
        got.contains(
            "<x:fill><x:gradientFill degree=\"90\"><x:stop position=\"0\">\
             <x:color rgb=\"FF000000\"/></x:stop><x:stop position=\"1\">\
             <x:color rgb=\"FFFFFFFF\"/></x:stop></x:gradientFill></x:fill></x:fills>"
        ),
        "fills: {got}"
    );
}

/// The refusal of `style` over no base, with nothing recorded, and the
/// part's own index still free afterwards.
#[track_caller]
fn assert_refused(style: Style, message: &str) {
    let (_, mut package) = opened(styled(&with_collections("<cellXfs><xf/></cellXfs>"), ""));
    assert_package(
        package.add_style(None, &style).expect_err("refused"),
        message,
    );
    assert_nothing_recorded(&package);
    add_styled(&mut package, None, &Style::default(), 1);
}

#[test]
fn a_gradient_beside_a_pattern_or_a_colour_is_refused_and_records_nothing() {
    // The schema gives a fill one child.
    let colour = Some(Color::Indexed(2));
    for fill in [
        Fill {
            pattern: Pattern::Solid,
            ..linear(90.0)
        },
        Fill {
            foreground: colour,
            ..linear(90.0)
        },
        Fill {
            background: colour,
            ..path(0.5, 0.5, 0.5, 0.5)
        },
    ] {
        assert_refused(filled(fill), "a style field is out of range");
    }
}

#[test]
fn each_gradient_range_is_refused_and_records_nothing() {
    let stops = |stops: Vec<Stop>| Fill {
        gradient: Some(Gradient {
            kind: GradientKind::Linear { degree: 90.0 },
            stops,
        }),
        ..linear(0.0)
    };
    let stop = |position: f64| Stop {
        position,
        color: Color::Rgb(0xFF00_0000),
    };
    let mut refused = Vec::new();
    for degree in [-0.5, 360.5, f64::NAN, f64::INFINITY] {
        refused.push(linear(degree));
    }
    for side in [-0.1, 1.1, f64::NAN, f64::NEG_INFINITY] {
        refused.push(path(side, 0.5, 0.5, 0.5));
        refused.push(path(0.5, side, 0.5, 0.5));
        refused.push(path(0.5, 0.5, side, 0.5));
        refused.push(path(0.5, 0.5, 0.5, side));
    }
    // Fewer than two stops.
    refused.push(stops(Vec::new()));
    refused.push(stops(vec![stop(0.0)]));
    // A position out of range, and stops out of order.
    for position in [-0.1, 1.5, f64::NAN, f64::INFINITY] {
        refused.push(stops(vec![stop(0.0), stop(position)]));
    }
    refused.push(stops(vec![stop(0.6), stop(0.4)]));
    // A stop's colour, checked as a fill's colour is.
    refused.push(stops(vec![
        stop(0.0),
        Stop {
            position: 1.0,
            color: Color::Theme {
                index: 0,
                tint: Some(1.5),
            },
        },
    ]));
    for fill in refused {
        assert_refused(filled(fill), "a style field is out of range");
    }
    // The edges are taken.
    let (_, mut package) = opened(styled(&with_collections("<cellXfs><xf/></cellXfs>"), ""));
    for (at, fill) in [
        linear(0.0),
        linear(360.0),
        path(0.0, 0.0, 1.0, 1.0),
        stops(vec![stop(0.0), stop(0.0)]),
        stops(vec![stop(1.0), stop(1.0), stop(1.0)]),
    ]
    .into_iter()
    .enumerate()
    {
        let index = u32::try_from(at).expect("small") + 1;
        add_styled(&mut package, None, &filled(fill), index);
    }
}

#[test]
fn the_gradient_is_checked_after_the_cell_format_s_own_children() {
    // `add_style`'s existing order first: a border on a part without
    // `<borders>` refuses with the collection message, though the fill's
    // gradient, recorded before it, is out of range.
    let (_, mut package) = opened(styled("<fonts/><fills/><cellXfs><xf/></cellXfs>", ""));
    let style = Style {
        fill: Some(linear(400.0)),
        border: Some(Border::default()),
        ..Style::default()
    };
    assert_package(
        package.add_style(None, &style).expect_err("refused"),
        "a style addition needs a collection the styles part lacks",
    );
    assert_nothing_recorded(&package);
}

// --- the typed read (the Cargo feature `read`) ---

/// The fill a styles part's second `<fill>`, `element`, reads as.
#[cfg(feature = "read")]
fn read_fill(element: &str) -> Result<Option<Fill>, Error> {
    let body = format!(
        "<fonts count=\"1\"><font/></fonts><fills count=\"2\"><fill/>{element}</fills>\
         <borders count=\"1\"><border/></borders><cellStyleXfs count=\"1\"><xf/></cellStyleXfs>\
         <cellXfs count=\"1\"><xf/></cellXfs>"
    );
    let (_, mut package) = opened(styled(&body, ""));
    package
        .styles()
        .map(|styles| styles.typed_fills()[1].clone())
}

#[cfg(feature = "read")]
#[test]
fn each_form_reads_back_as_the_writer_takes_it() {
    let read = |element: &str| read_fill(element).expect("read").expect("a fill");
    let stops = STOPS;
    assert_eq!(
        read(&format!("<fill><gradientFill degree=\"90\">{stops}")),
        linear(90.0)
    );
    // `type="linear"` and no `type` are one form; no `degree` reads as 0.
    assert_eq!(
        read(&format!("<fill><gradientFill type=\"linear\">{stops}")),
        linear(0.0)
    );
    assert_eq!(read(&format!("<fill><gradientFill>{stops}")), linear(0.0));
    assert_eq!(
        read(&format!(
            "<fill><gradientFill type=\"path\" left=\"0.5\" right=\"0.5\" top=\"0.5\" \
             bottom=\"0.5\">{stops}"
        )),
        path(0.5, 0.5, 0.5, 0.5)
    );
    // Each of the four absent reads as 0.
    assert_eq!(
        read(&format!(
            "<fill><gradientFill type=\"path\" top=\"1\">{stops}"
        )),
        path(0.0, 0.0, 1.0, 0.0)
    );
    // A gradient with no stop reads with none; the writer would refuse it.
    assert_eq!(
        read("<fill><gradientFill/></fill>").gradient,
        Some(Gradient {
            kind: GradientKind::Linear { degree: 0.0 },
            stops: Vec::new(),
        })
    );
}

#[cfg(feature = "read")]
#[test]
fn a_gradient_that_cannot_be_read_is_refused() {
    let stop = "<stop position=\"0\"><color rgb=\"FF000000\"/></stop>";
    for element in [
        // An unreadable number.
        format!("<fill><gradientFill degree=\"ninety\">{stop}</gradientFill></fill>"),
        format!("<fill><gradientFill degree=\"inf\">{stop}</gradientFill></fill>"),
        format!("<fill><gradientFill type=\"path\" left=\"NaN\">{stop}</gradientFill></fill>"),
        "<fill><gradientFill><stop position=\"x\"><color rgb=\"FF000000\"/></stop>\
         </gradientFill></fill>"
            .to_owned(),
        // A stop without a position, without a colour, or with two.
        "<fill><gradientFill><stop><color rgb=\"FF000000\"/></stop></gradientFill></fill>"
            .to_owned(),
        "<fill><gradientFill><stop position=\"0\"/></gradientFill></fill>".to_owned(),
        "<fill><gradientFill><stop position=\"0\"></stop></gradientFill></fill>".to_owned(),
        "<fill><gradientFill><stop position=\"0\"><color rgb=\"FF000000\"/>\
         <color rgb=\"FF000000\"/></stop></gradientFill></fill>"
            .to_owned(),
        // A child other than a stop, a stop's child other than a colour, a
        // type the schema does not have.
        format!("<fill><gradientFill>{stop}<extLst/></gradientFill></fill>"),
        "<fill><gradientFill><stop position=\"0\"><fgColor rgb=\"FF000000\"/></stop>\
         </gradientFill></fill>"
            .to_owned(),
        format!("<fill><gradientFill type=\"radial\">{stop}</gradientFill></fill>"),
        // A second child beside the gradient.
        format!("<fill><gradientFill>{stop}</gradientFill><patternFill/></fill>"),
    ] {
        match read_fill(&element) {
            Err(error) => assert_package(error, "a style cannot be read"),
            Ok(fill) => panic!("{element} read as {fill:?}"),
        }
    }
}

#[cfg(feature = "read")]
#[test]
fn a_written_gradient_reads_back_equal_to_what_was_given() {
    let given = [linear(90.0), linear(0.0), path(0.25, 0.75, 0.0, 1.0)];
    let parts = styled(
        &format!(
            "{}<cellStyleXfs count=\"1\"><xf/></cellStyleXfs><cellXfs count=\"1\"><xf/></cellXfs>",
            with_collections("")
        ),
        "",
    );
    let output = edited_output(parts, |p| {
        for (at, fill) in given.iter().enumerate() {
            let index = u32::try_from(at).expect("small") + 1;
            add_styled(p, None, &filled(fill.clone()), index);
        }
    });
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    for (at, fill) in given.iter().enumerate() {
        let index = u32::try_from(at).expect("small") + 1;
        assert_eq!(
            package.cell_style(index).expect("read").fill.as_ref(),
            Some(fill),
            "the format at {index}"
        );
    }
}

#[cfg(all(feature = "read", feature = "rules"))]
#[test]
fn a_differential_style_s_gradient_is_written_back_through_the_same_composer() {
    let kept = "<dxf><fill><gradientFill type=\"path\" left=\"0.5\" right=\"0.5\" top=\"0.5\" \
                bottom=\"0.5\"><stop position=\"0\"><color theme=\"0\"/></stop>\
                <stop position=\"1\"><color theme=\"4\" tint=\"0.5\"/></stop></gradientFill>\
                </fill></dxf>";
    let body = format!(
        "{}<cellStyleXfs count=\"1\"><xf/></cellStyleXfs><cellXfs count=\"1\"><xf/></cellXfs>\
         <dxfs count=\"1\">{kept}</dxfs>",
        with_collections("")
    );
    let parts = styled(&body, "");
    let (_, mut package) = opened(parts.clone());
    let read = package.differential_styles().expect("read")[0].clone();
    assert_eq!(
        read.fill
            .as_ref()
            .and_then(|fill| fill.gradient.as_ref())
            .map(|g| g.kind),
        Some(GradientKind::Path {
            left: 0.5,
            right: 0.5,
            top: 0.5,
            bottom: 0.5
        })
    );
    let part = added_styles(&body, |p| {
        assert_eq!(p.add_differential_style(&read).expect("recorded"), 1);
    });
    assert_eq!(
        part,
        style_sheet(&body.replace(
            &format!("<dxfs count=\"1\">{kept}</dxfs>"),
            &format!("<dxfs count=\"2\">{kept}{kept}</dxfs>")
        ))
    );
}
