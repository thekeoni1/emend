//! The visual conditional formats on byte-built worksheets
//! (`specs/0020-validation-and-conditional-formatting.md`, "Conditional
//! formatting", "P3", and the P3 rulings): colour scales, data bars and icon
//! sets, each checked against literal bytes, the value rule refused case by
//! case, and the kinds read back.

use super::*;
use crate::styles::Color;

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

/// The worksheet after `rule` is recorded on A1:A3 with no style, no
/// priority and no stop, as text; every other part is asserted untouched.
#[track_caller]
fn written_rule(rule: Rule<'_>) -> String {
    let parts = with_body("<sheetData/>");
    let output = edited_output(parts.clone(), |p| {
        p.add_conditional_format(SHEET, &[area("A1:A3")], rule, None, None, false)
            .expect("recorded")
    });
    assert_untouched(&output, &parts, &[SHEET]);
    String::from_utf8(stored_data(&output, SHEET)).expect("UTF-8")
}

/// The worksheet a visual rule of `kind` with `child` is written into.
fn sheet_with(kind: &str, child: &str) -> String {
    worksheet(&format!(
        "<sheetData/><conditionalFormatting sqref=\"A1:A3\"><cfRule type=\"{kind}\" \
         priority=\"1\">{child}</cfRule></conditionalFormatting>"
    ))
}

#[test]
fn a_colour_scale_is_written_with_its_points_then_its_colours() {
    let sheet = written_rule(Rule::ColorScale {
        min: Cfvo {
            kind: CfvoKind::Min,
            color: Color::Rgb(0xFFF8_696B),
        },
        mid: None,
        max: Cfvo {
            kind: CfvoKind::Max,
            color: Color::Rgb(0xFF63_BE7B),
        },
    });
    assert_eq!(
        sheet,
        sheet_with(
            "colorScale",
            "<colorScale><cfvo type=\"min\"/><cfvo type=\"max\"/><color rgb=\"FFF8696B\"/>\
             <color rgb=\"FF63BE7B\"/></colorScale>"
        )
    );
    // Three points, every other kind of value and every other colour form.
    let sheet = written_rule(Rule::ColorScale {
        min: Cfvo {
            kind: CfvoKind::Num(-1.5),
            color: Color::Theme {
                index: 4,
                tint: Some(-0.25),
            },
        },
        mid: Some(Cfvo {
            kind: CfvoKind::Percentile(50.0),
            color: Color::Indexed(10),
        }),
        max: Cfvo {
            kind: CfvoKind::Formula("$B$1&\"x\""),
            color: Color::Auto,
        },
    });
    assert_eq!(
        sheet,
        sheet_with(
            "colorScale",
            "<colorScale><cfvo type=\"num\" val=\"-1.5\"/><cfvo type=\"percentile\" val=\"50\"/>\
             <cfvo type=\"formula\" val=\"$B$1&amp;&quot;x&quot;\"/><color theme=\"4\" \
             tint=\"-0.25\"/><color indexed=\"10\"/><color auto=\"1\"/></colorScale>"
        )
    );
}

#[test]
fn a_data_bar_is_written_in_the_2007_form_its_value_shown_or_hidden() {
    for (show_value, attribute) in [(true, ""), (false, " showValue=\"0\"")] {
        let sheet = written_rule(Rule::DataBar {
            min: CfvoKind::Min,
            max: CfvoKind::Percent(90.0),
            color: Color::Rgb(0xFF63_8EC6),
            show_value,
        });
        assert_eq!(
            sheet,
            sheet_with(
                "dataBar",
                &format!(
                    "<dataBar{attribute}><cfvo type=\"min\"/><cfvo type=\"percent\" val=\"90\"/>\
                     <color rgb=\"FF638EC6\"/></dataBar>"
                )
            )
        );
    }
}

#[test]
fn an_icon_set_of_three_four_and_five_icons_is_written_reversed_or_not() {
    let three = [
        CfvoKind::Percent(0.0),
        CfvoKind::Percent(33.0),
        CfvoKind::Percent(67.0),
    ];
    let sheet = written_rule(Rule::IconSet {
        set: IconSetKind::ThreeArrows,
        thresholds: &three,
        reverse: false,
        show_value: true,
    });
    assert_eq!(
        sheet,
        sheet_with(
            "iconSet",
            "<iconSet iconSet=\"3Arrows\"><cfvo type=\"percent\" val=\"0\"/><cfvo \
             type=\"percent\" val=\"33\"/><cfvo type=\"percent\" val=\"67\"/></iconSet>"
        )
    );
    let four = [
        CfvoKind::Num(0.0),
        CfvoKind::Num(10.0),
        CfvoKind::Percentile(20.0),
        CfvoKind::Formula("C1"),
    ];
    let sheet = written_rule(Rule::IconSet {
        set: IconSetKind::FourTrafficLights,
        thresholds: &four,
        reverse: true,
        show_value: false,
    });
    assert_eq!(
        sheet,
        sheet_with(
            "iconSet",
            "<iconSet iconSet=\"4TrafficLights\" reverse=\"1\" showValue=\"0\"><cfvo \
             type=\"num\" val=\"0\"/><cfvo type=\"num\" val=\"10\"/><cfvo type=\"percentile\" \
             val=\"20\"/><cfvo type=\"formula\" val=\"C1\"/></iconSet>"
        )
    );
    let five = [CfvoKind::Percent(0.0); 5];
    let sheet = written_rule(Rule::IconSet {
        set: IconSetKind::FiveQuarters,
        thresholds: &five,
        reverse: false,
        show_value: true,
    });
    assert_eq!(
        sheet,
        sheet_with(
            "iconSet",
            &format!(
                "<iconSet iconSet=\"5Quarters\">{}</iconSet>",
                "<cfvo type=\"percent\" val=\"0\"/>".repeat(5)
            )
        )
    );
    // Every set's name, and its icons numbering its thresholds.
    let sets = [
        (IconSetKind::ThreeArrows, "3Arrows"),
        (IconSetKind::ThreeArrowsGray, "3ArrowsGray"),
        (IconSetKind::ThreeFlags, "3Flags"),
        (IconSetKind::ThreeTrafficLights1, "3TrafficLights1"),
        (IconSetKind::ThreeTrafficLights2, "3TrafficLights2"),
        (IconSetKind::ThreeSigns, "3Signs"),
        (IconSetKind::ThreeSymbols, "3Symbols"),
        (IconSetKind::ThreeSymbols2, "3Symbols2"),
        (IconSetKind::FourArrows, "4Arrows"),
        (IconSetKind::FourArrowsGray, "4ArrowsGray"),
        (IconSetKind::FourRedToBlack, "4RedToBlack"),
        (IconSetKind::FourRating, "4Rating"),
        (IconSetKind::FourTrafficLights, "4TrafficLights"),
        (IconSetKind::FiveArrows, "5Arrows"),
        (IconSetKind::FiveArrowsGray, "5ArrowsGray"),
        (IconSetKind::FiveRating, "5Rating"),
        (IconSetKind::FiveQuarters, "5Quarters"),
    ];
    for (set, name) in sets {
        let icons = usize::from(name.as_bytes()[0] - b'0');
        let thresholds = vec![CfvoKind::Num(1.0); icons];
        let sheet = written_rule(Rule::IconSet {
            set,
            thresholds: &thresholds,
            reverse: false,
            show_value: true,
        });
        assert!(
            sheet.contains(&format!("<iconSet iconSet=\"{name}\">")),
            "{sheet}"
        );
    }
}

#[test]
fn the_value_rule_and_a_visual_rule_with_a_style_are_refused_at_the_call() {
    let value = "a colour scale, data bar or icon set value is not valid";
    let red = Color::Rgb(0xFFFF_0000);
    let point = |kind| Cfvo { kind, color: red };
    let scale = |min, max| Rule::ColorScale {
        min: point(min),
        mid: None,
        max: point(max),
    };
    let three_for = |thresholds: &'static [CfvoKind<'static>]| Rule::IconSet {
        set: IconSetKind::ThreeArrows,
        thresholds,
        reverse: false,
        show_value: true,
    };
    static FOUR: [CfvoKind<'static>; 4] = [CfvoKind::Percent(0.0); 4];
    static FIVE: [CfvoKind<'static>; 5] = [CfvoKind::Percent(0.0); 5];
    static MIN_IN_SET: [CfvoKind<'static>; 3] = [
        CfvoKind::Min,
        CfvoKind::Percent(33.0),
        CfvoKind::Percent(67.0),
    ];
    let refusals: Vec<(Rule<'static>, &str)> = vec![
        // `Min` as a scale's last point, `Max` as its first.
        (scale(CfvoKind::Min, CfvoKind::Min), value),
        (scale(CfvoKind::Max, CfvoKind::Max), value),
        // A percent of 101, and a percentile below 0.
        (scale(CfvoKind::Min, CfvoKind::Percent(101.0)), value),
        (scale(CfvoKind::Percentile(-1.0), CfvoKind::Max), value),
        // A number that is not finite, and a percent that is not a number.
        (scale(CfvoKind::Num(f64::INFINITY), CfvoKind::Max), value),
        (scale(CfvoKind::Min, CfvoKind::Percent(f64::NAN)), value),
        // An empty formula, and one with a leading `=`.
        (scale(CfvoKind::Formula(""), CfvoKind::Max), value),
        (scale(CfvoKind::Min, CfvoKind::Formula("=A1")), value),
        // A middle point may not be `Min` or `Max`.
        (
            Rule::ColorScale {
                min: point(CfvoKind::Min),
                mid: Some(point(CfvoKind::Max)),
                max: point(CfvoKind::Max),
            },
            value,
        ),
        // A theme tint outside -1 to 1.
        (
            Rule::DataBar {
                min: CfvoKind::Min,
                max: CfvoKind::Max,
                color: Color::Theme {
                    index: 1,
                    tint: Some(1.5),
                },
                show_value: true,
            },
            value,
        ),
        // Four thresholds for a three-icon set; five for a four-icon set
        // (P3 ruling 1); `Min` in an icon set.
        (three_for(&FOUR), value),
        (
            Rule::IconSet {
                set: IconSetKind::FourArrows,
                thresholds: &FIVE,
                reverse: false,
                show_value: true,
            },
            value,
        ),
        (three_for(&MIN_IN_SET), value),
    ];
    let parts = with_body("<sheetData/>");
    let (_, mut package) = opened(parts.clone());
    for (rule, message) in refusals {
        assert_package(
            package
                .add_conditional_format(SHEET, &[area("A1")], rule, None, None, false)
                .expect_err("refused"),
            message,
        );
    }
    // A visual rule with a style (P2 ruling 2).
    assert_package(
        package
            .add_conditional_format(
                SHEET,
                &[area("A1")],
                scale(CfvoKind::Min, CfvoKind::Max),
                Some(0),
                None,
                false,
            )
            .expect_err("refused"),
        "a conditional format rule and its differential style do not agree",
    );
    // Nothing recorded: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
}

#[test]
fn the_three_kinds_read_back_with_no_formulas_and_no_style() {
    let red = Color::Rgb(0xFFFF_0000);
    let thresholds = [
        CfvoKind::Percent(0.0),
        CfvoKind::Percent(33.0),
        CfvoKind::Percent(67.0),
    ];
    let (_, mut package) = opened(with_body("<sheetData/>"));
    for (at, rule) in [
        Rule::ColorScale {
            min: Cfvo {
                kind: CfvoKind::Min,
                color: red,
            },
            mid: None,
            max: Cfvo {
                kind: CfvoKind::Max,
                color: red,
            },
        },
        Rule::DataBar {
            min: CfvoKind::Min,
            max: CfvoKind::Max,
            color: red,
            show_value: true,
        },
        Rule::IconSet {
            set: IconSetKind::ThreeArrows,
            thresholds: &thresholds,
            reverse: false,
            show_value: true,
        },
    ]
    .into_iter()
    .enumerate()
    {
        let column = ["A1", "B1", "C1"][at];
        package
            .add_conditional_format(SHEET, &[area(column)], rule, None, None, false)
            .expect("recorded");
    }
    let out = written(package);
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    let entries = reopened.conditional_formats(SHEET).expect("read");
    let kinds: Vec<(&str, Vec<Area>, u32)> = entries
        .iter()
        .map(|entry| (entry.kind.as_str(), entry.areas.clone(), entry.priority))
        .collect();
    assert_eq!(
        kinds,
        [
            ("colorScale", vec![area("A1")], 1),
            ("dataBar", vec![area("B1")], 2),
            ("iconSet", vec![area("C1")], 3),
        ]
    );
    for entry in &entries {
        assert_eq!(entry.formulas, Vec::<String>::new());
        assert_eq!(entry.dxf, None);
        assert_eq!(entry.operator, None);
        assert!(!entry.extended);
    }
}
