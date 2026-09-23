//! The date arithmetic against the values `specs/0012-styles.md` pins.
//!
//! "Module `dates`" gives the pinned serials, computed by planning from
//! Python's `date` ordinals; "P3" lists the refusals; "Planning rulings
//! during P3" rulings 2 and 3 fix the rounding order and what a civil date
//! and time is. Every expected value here is one of those, written out, never
//! computed by the code under test.

use super::*;
use crate::package::Workbook;

/// A date at midnight.
fn date(year: i32, month: u32, day: u32) -> DateTime {
    at(year, month, day, 0, 0, 0, 0)
}

/// A date and time.
fn at(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
) -> DateTime {
    DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond,
    }
}

/// A pinned value: the date as year, month and day, its 1900 serial, and
/// its 1904 serial where the date is not before 1904-01-01.
type Pinned = ((i32, u32, u32), f64, Option<f64>);

/// The pinned values of "Module `dates`".
const PINNED: [Pinned; 8] = [
    ((1900, 1, 1), 1.0, None),
    ((1900, 2, 28), 59.0, None),
    ((1900, 3, 1), 61.0, None),
    ((1904, 1, 1), 1462.0, Some(0.0)),
    ((1970, 1, 1), 25569.0, Some(24107.0)),
    ((2000, 1, 1), 36526.0, Some(35064.0)),
    ((2024, 2, 29), 45351.0, Some(43889.0)),
    ((9999, 12, 31), 2958465.0, Some(2957003.0)),
];

/// One millisecond as a fraction of a day.
const ONE_MS: f64 = 1.0 / 86_400_000.0;

#[test]
fn every_pinned_date_has_its_serial_in_both_epochs() {
    for ((y, m, d), s1900, s1904) in PINNED {
        assert_eq!(
            date(y, m, d).serial(Epoch::Y1900),
            Some(s1900),
            "{y}-{m}-{d}"
        );
        assert_eq!(date(y, m, d).serial(Epoch::Y1904), s1904, "{y}-{m}-{d}");
    }
}

#[test]
fn noon_adds_a_half_and_a_millisecond_adds_one_part_in_86400000() {
    for ((y, m, d), s1900, s1904) in PINNED {
        assert_eq!(
            at(y, m, d, 12, 0, 0, 0).serial(Epoch::Y1900),
            Some(s1900 + 0.5)
        );
        assert_eq!(
            at(y, m, d, 12, 0, 0, 0).serial(Epoch::Y1904),
            s1904.map(|s| s + 0.5)
        );
        assert_eq!(
            at(y, m, d, 0, 0, 0, 1).serial(Epoch::Y1900),
            Some(s1900 + ONE_MS)
        );
        assert_eq!(
            at(y, m, d, 0, 0, 0, 1).serial(Epoch::Y1904),
            s1904.map(|s| s + ONE_MS)
        );
    }
}

#[test]
fn every_pinned_value_reads_back_as_its_date_in_both_epochs() {
    for ((y, m, d), s1900, s1904) in PINNED {
        assert_eq!(
            DateTime::from_serial(Epoch::Y1900, s1900),
            Some(date(y, m, d))
        );
        assert_eq!(
            DateTime::from_serial(Epoch::Y1900, s1900 + 0.5),
            Some(at(y, m, d, 12, 0, 0, 0))
        );
        if let Some(s1904) = s1904 {
            assert_eq!(
                DateTime::from_serial(Epoch::Y1904, s1904),
                Some(date(y, m, d))
            );
            assert_eq!(
                DateTime::from_serial(Epoch::Y1904, s1904 + 0.5),
                Some(at(y, m, d, 12, 0, 0, 0))
            );
        }
    }
}

#[test]
fn every_pinned_value_round_trips_through_both_functions_with_noon_and_a_millisecond() {
    for ((y, m, d), _, s1904) in PINNED {
        for time in [
            at(y, m, d, 0, 0, 0, 0),
            at(y, m, d, 12, 0, 0, 0),
            at(y, m, d, 0, 0, 0, 1),
            at(y, m, d, 23, 59, 59, 999),
        ] {
            let serial = time.serial(Epoch::Y1900).expect("a 1900 serial");
            assert_eq!(DateTime::from_serial(Epoch::Y1900, serial), Some(time));
            if s1904.is_some() {
                let serial = time.serial(Epoch::Y1904).expect("a 1904 serial");
                assert_eq!(DateTime::from_serial(Epoch::Y1904, serial), Some(time));
            }
        }
    }
}

#[test]
fn the_day_1900_02_29_is_no_date_and_day_60_is_refused() {
    assert_eq!(date(1900, 2, 29).serial(Epoch::Y1900), None);
    assert_eq!(date(1900, 2, 29).serial(Epoch::Y1904), None);
    assert_eq!(DateTime::from_serial(Epoch::Y1900, 60.0), None);
    assert_eq!(DateTime::from_serial(Epoch::Y1900, 60.5), None);
    // The same serial is an ordinary day in the other system.
    assert_eq!(
        DateTime::from_serial(Epoch::Y1904, 60.0),
        Some(date(1904, 3, 1))
    );
}

#[test]
fn the_day_before_each_epoch_and_the_day_after_9999_12_31_are_refused() {
    assert_eq!(date(1899, 12, 31).serial(Epoch::Y1900), None);
    assert_eq!(date(1903, 12, 31).serial(Epoch::Y1904), None);
    assert_eq!(date(10000, 1, 1).serial(Epoch::Y1900), None);
    assert_eq!(date(10000, 1, 1).serial(Epoch::Y1904), None);
    assert_eq!(DateTime::from_serial(Epoch::Y1900, 0.0), None);
    assert_eq!(DateTime::from_serial(Epoch::Y1904, -1.0), None);
    assert_eq!(DateTime::from_serial(Epoch::Y1900, 2958466.0), None);
    assert_eq!(DateTime::from_serial(Epoch::Y1904, 2957004.0), None);
}

#[test]
fn a_date_the_calendar_does_not_have_is_refused() {
    for epoch in [Epoch::Y1900, Epoch::Y1904] {
        assert_eq!(date(2023, 2, 29).serial(epoch), None);
        assert_eq!(date(2023, 13, 1).serial(epoch), None);
        assert_eq!(date(2023, 0, 1).serial(epoch), None);
        assert_eq!(date(2023, 1, 0).serial(epoch), None);
        assert_eq!(date(2023, 4, 31).serial(epoch), None);
    }
}

#[test]
fn a_time_outside_the_day_is_refused() {
    for epoch in [Epoch::Y1900, Epoch::Y1904] {
        assert_eq!(at(2024, 2, 29, 24, 0, 0, 0).serial(epoch), None);
        assert_eq!(at(2024, 2, 29, 0, 60, 0, 0).serial(epoch), None);
        assert_eq!(at(2024, 2, 29, 0, 0, 60, 0).serial(epoch), None);
        assert_eq!(at(2024, 2, 29, 0, 0, 0, 1000).serial(epoch), None);
        // The largest of each is a time.
        assert!(at(2024, 2, 29, 23, 59, 59, 999).serial(epoch).is_some());
    }
}

#[test]
fn a_serial_that_is_not_finite_is_refused() {
    for epoch in [Epoch::Y1900, Epoch::Y1904] {
        assert_eq!(DateTime::from_serial(epoch, f64::NAN), None);
        assert_eq!(DateTime::from_serial(epoch, f64::INFINITY), None);
        assert_eq!(DateTime::from_serial(epoch, f64::NEG_INFINITY), None);
    }
}

#[test]
fn a_time_that_rounds_to_midnight_carries_into_the_next_day() {
    // Four tenths of a millisecond before midnight rounds up too.
    let serial = 45351.0 + (86_400_000.0 - 0.4) / 86_400_000.0;
    assert_eq!(
        DateTime::from_serial(Epoch::Y1900, serial),
        Some(date(2024, 3, 1))
    );
    // Six tenths before midnight rounds down to the last millisecond.
    let serial = 45351.0 + (86_400_000.0 - 0.6) / 86_400_000.0;
    assert_eq!(
        DateTime::from_serial(Epoch::Y1900, serial),
        Some(at(2024, 2, 29, 23, 59, 59, 999))
    );
}

// Ruling 2: the serial is rounded to the millisecond first, and each refusal
// applies to the rounded value. Each boundary is approached from both sides:
// by a serial that rounds onto it and by one that stays clear of it.

#[test]
fn a_serial_that_rounds_onto_day_60_is_refused_and_one_that_stays_clear_is_not() {
    // From below: a hair under 60 rounds to day 60.
    assert_eq!(DateTime::from_serial(Epoch::Y1900, 59.999_999_999_99), None);
    // From below, clear of it: the last millisecond of 1900-02-28.
    assert_eq!(
        DateTime::from_serial(Epoch::Y1900, 60.0 - ONE_MS),
        Some(at(1900, 2, 28, 23, 59, 59, 999))
    );
    // From above: a hair over 60 rounds down to day 60.
    assert_eq!(DateTime::from_serial(Epoch::Y1900, 60.000_000_000_01), None);
    // From above, onto day 61: a hair under 61 rounds up to 1900-03-01.
    assert_eq!(
        DateTime::from_serial(Epoch::Y1900, 60.999_999_999_99),
        Some(date(1900, 3, 1))
    );
}

#[test]
fn a_serial_that_rounds_up_to_the_epochs_first_day_is_that_day() {
    // 1900: a hair under 1 reads as 1900-01-01 00:00:00.000.
    assert_eq!(
        DateTime::from_serial(Epoch::Y1900, 0.999_999_999_99),
        Some(date(1900, 1, 1))
    );
    // 1900: a millisecond under 1 is before the first day.
    assert_eq!(DateTime::from_serial(Epoch::Y1900, 1.0 - ONE_MS), None);
    // 1904: a hair under 0 reads as 1904-01-01.
    assert_eq!(
        DateTime::from_serial(Epoch::Y1904, -0.000_000_000_01),
        Some(date(1904, 1, 1))
    );
    // 1904: a millisecond under 0 is before the first day.
    assert_eq!(DateTime::from_serial(Epoch::Y1904, -ONE_MS), None);
}

#[test]
fn a_serial_that_rounds_up_to_the_day_after_9999_12_31_is_refused() {
    // The largest f64 below 2958466 rounds up to it.
    assert_eq!(
        DateTime::from_serial(Epoch::Y1900, f64::from_bits(2958466.0f64.to_bits() - 1)),
        None
    );
    assert_eq!(
        DateTime::from_serial(Epoch::Y1904, f64::from_bits(2957004.0f64.to_bits() - 1)),
        None
    );
    // The last millisecond of 9999-12-31 is a date.
    assert_eq!(
        DateTime::from_serial(Epoch::Y1900, 2958466.0 - ONE_MS),
        Some(at(9999, 12, 31, 23, 59, 59, 999))
    );
    assert_eq!(
        DateTime::from_serial(Epoch::Y1904, 2957004.0 - ONE_MS),
        Some(at(9999, 12, 31, 23, 59, 59, 999))
    );
}

#[test]
fn the_epoch_is_the_workbooks_date1904() {
    let workbook = |date1904| Workbook {
        part: b"xl/workbook.xml".to_vec(),
        sheets: Vec::new(),
        shared_strings: None,
        styles: None,
        calc_chain: None,
        theme: None,
        date1904,
        full_calc_on_load: false,
    };
    assert_eq!(Epoch::of(&workbook(false)), Epoch::Y1900);
    assert_eq!(Epoch::of(&workbook(true)), Epoch::Y1904);
}
