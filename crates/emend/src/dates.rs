//! Dates as the serial numbers a cell holds, in the workbook's own epoch.
//!
//! `specs/0012-styles.md`, "Module `dates` (feature `dates`)". This module
//! is the Cargo feature `dates`, off by default, and has no dependency of
//! its own. ECMA-376 Part 1, 18.17.4.1 and 18.17.4.2 define the two date
//! systems:
//!
//! - in the 1900 system serial 1 is 1900-01-01, and serial 60 is the day
//!   1900-02-29, which the calendar does not have, so every later day is one
//!   more than the civil count;
//! - in the 1904 system serial 0 is 1904-01-01, and every date is 1462 less
//!   than in the 1900 system.
//!
//! The time of day is the serial's fraction. [`DateTime::serial`] turns a
//! civil date and time into that number and [`DateTime::from_serial`] turns
//! it back; [`Epoch::of`] says which system a workbook uses.
//!
//! A dated cell is a [`crate::cells::Value::Number`] holding the serial,
//! with a cell format whose number format is a date. Nothing in the core
//! knows which numbers are dates.
//!
//! # Errors
//!
//! None. Both functions answer `None` for what they refuse, and raise no
//! [`crate::Error`]:
//!
//! - [`DateTime::serial`]: a month outside 1 to 12; a day outside its month,
//!   1900-02-29 included; an hour above 23, a minute or second above 59, a
//!   millisecond above 999; a date before the epoch's first day, 1900-01-01
//!   or 1904-01-01; a date after 9999-12-31.
//! - [`DateTime::from_serial`]: a serial that is not finite; after rounding
//!   to the millisecond, one below the epoch's first day (1 or 0), one at or
//!   after the day after 9999-12-31 (2958466 or 2957004), and the 1900
//!   system's day 60.
//!
//! # Limits
//!
//! - The module is the Cargo feature `dates`, off by default and outside
//!   the core: the core's size budget is measured without it, its cost is
//!   measured beside the budget, and nothing in the core calls it
//!   (`specs/0012-styles.md`, "Decisions for the maintainer", 2).
//! - The calendar is the proleptic Gregorian one, with no time zone and no
//!   leap second, to the millisecond. A serial's fraction is rounded to the
//!   nearest millisecond, carrying into the next day when it rounds to
//!   midnight, and every refusal above applies to the rounded value
//!   ("Planning rulings during P3", ruling 2).
//! - Serials 1 to 59 of the 1900 system are 1900-01-01 to 1900-02-28, as the
//!   system's definition counts them. The day 60 has no date, so
//!   [`DateTime::serial`] never answers it and [`DateTime::from_serial`]
//!   refuses it.
//! - A number format is not consulted: whether a number is a date is the
//!   caller's to decide, from the cell's format or otherwise. A `t="d"`
//!   cell's ISO 8601 text ([`crate::cells::CellKind::Date`]) is not converted.

use crate::package::Workbook;

#[cfg(test)]
mod tests;

/// Milliseconds in a day.
const DAY_MS: i64 = 86_400_000;

/// The day after 9999-12-31, as a serial in the 1900 system.
const END_1900: i64 = 2_958_466;

/// The day after 9999-12-31, as a serial in the 1904 system.
const END_1904: i64 = END_1900 - 1462;

/// The 1900 system's serial for the day 1900-02-29, which no calendar has.
const PHANTOM_DAY: i64 = 60;

/// A workbook's date system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Epoch {
    /// Serial 1 is 1900-01-01, with the phantom day 60.
    Y1900,
    /// Serial 0 is 1904-01-01.
    Y1904,
}

impl Epoch {
    /// The date system `workbook` uses: [`Epoch::Y1904`] when its
    /// `date1904` is set, else [`Epoch::Y1900`].
    #[must_use]
    pub fn of(workbook: &Workbook) -> Epoch {
        if workbook.date1904 {
            Epoch::Y1904
        } else {
            Epoch::Y1900
        }
    }

    /// The serial of the epoch's first day.
    fn first(self) -> i64 {
        match self {
            Epoch::Y1900 => 1,
            Epoch::Y1904 => 0,
        }
    }

    /// The serial of the day after 9999-12-31.
    fn end(self) -> i64 {
        match self {
            Epoch::Y1900 => END_1900,
            Epoch::Y1904 => END_1904,
        }
    }
}

/// A civil date and time in the proleptic Gregorian calendar, to the
/// millisecond.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    /// The year.
    pub year: i32,
    /// The month, 1 to 12.
    pub month: u32,
    /// The day of the month, from 1.
    pub day: u32,
    /// The hour, 0 to 23.
    pub hour: u32,
    /// The minute, 0 to 59.
    pub minute: u32,
    /// The second, 0 to 59.
    pub second: u32,
    /// The millisecond, 0 to 999.
    pub millisecond: u32,
}

impl DateTime {
    /// The serial number of this date and time in `epoch`: the day count
    /// as the module header describes, plus the time of day as a fraction
    /// of 86400000 milliseconds.
    ///
    /// `None` when the fields are not a civil date and time (a month outside
    /// 1 to 12, a day outside the month, 1900-02-29 included, an hour above
    /// 23, a minute or a second above 59, a millisecond above 999), when the
    /// date is before the epoch's first day (1900-01-01 or 1904-01-01), and
    /// when it is after 9999-12-31.
    #[must_use]
    pub fn serial(&self, epoch: Epoch) -> Option<f64> {
        if !(1..=12).contains(&self.month)
            || self.day == 0
            || self.day > month_length(i64::from(self.year), self.month)
            || self.hour > 23
            || self.minute > 59
            || self.second > 59
            || self.millisecond > 999
        {
            return None;
        }
        let civil = days_from_civil(i64::from(self.year), self.month, self.day);
        let day = match epoch {
            Epoch::Y1900 => {
                let count = civil - days_from_civil(1899, 12, 31);
                if count >= PHANTOM_DAY {
                    count + 1
                } else {
                    count
                }
            }
            Epoch::Y1904 => civil - days_from_civil(1904, 1, 1),
        };
        if day < epoch.first() || day >= epoch.end() {
            return None;
        }
        let ms = i64::from(self.hour) * 3_600_000
            + i64::from(self.minute) * 60_000
            + i64::from(self.second) * 1000
            + i64::from(self.millisecond);
        // Both are exact in an f64: the day is below 2^22 and the
        // milliseconds below 2^27.
        Some(day as f64 + ms as f64 / DAY_MS as f64)
    }

    /// The civil date and time a serial number stands for in `epoch`.
    ///
    /// The serial is first rounded to the nearest millisecond, so a time
    /// that rounds up to midnight is the next day, and every refusal below
    /// applies to the rounded value (`specs/0012-styles.md`, "Planning
    /// rulings during P3", ruling 2). `None` when `serial` is not finite,
    /// when it is below the epoch's first day (1 in the 1900 system, 0 in
    /// the 1904 system), when it is at or after the day after 9999-12-31,
    /// and when it is the day 60 of the 1900 system, which stands for
    /// 1900-02-29.
    #[must_use]
    pub fn from_serial(epoch: Epoch, serial: f64) -> Option<DateTime> {
        if !serial.is_finite() {
            return None;
        }
        let total = (serial * DAY_MS as f64).round();
        // Compared as f64 first, so the conversion below is always in range.
        if total < (epoch.first() * DAY_MS) as f64 || total >= (epoch.end() * DAY_MS) as f64 {
            return None;
        }
        let total = total as i64;
        let day = total.div_euclid(DAY_MS);
        let ms = total.rem_euclid(DAY_MS);
        let civil = match epoch {
            Epoch::Y1900 if day == PHANTOM_DAY => return None,
            Epoch::Y1900 if day > PHANTOM_DAY => days_from_civil(1899, 12, 31) + day - 1,
            Epoch::Y1900 => days_from_civil(1899, 12, 31) + day,
            Epoch::Y1904 => days_from_civil(1904, 1, 1) + day,
        };
        let (year, month, day) = civil_from_days(civil);
        // Every field is in range by construction: the year is 1900 to 9999,
        // and the time of day is below 86400000 milliseconds.
        Some(DateTime {
            year: i32::try_from(year).ok()?,
            month,
            day,
            hour: u32::try_from(ms / 3_600_000).ok()?,
            minute: u32::try_from(ms / 60_000 % 60).ok()?,
            second: u32::try_from(ms / 1000 % 60).ok()?,
            millisecond: u32::try_from(ms % 1000).ok()?,
        })
    }
}

/// Whether `year` is a leap year in the proleptic Gregorian calendar.
fn is_leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// The number of days in `month` of `year`; 0 for a month outside 1 to 12.
fn month_length(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Days from 1970-01-01 to the civil date, negative before it, for a month
/// of 1 to 12 and any day (H. Hinnant's `days_from_civil`).
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let month = i64::from(month);
    let day_of_year =
        (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The civil date of a day count from 1970-01-01 (H. Hinnant's
/// `civil_from_days`).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let days = days + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    // month is 1 to 12 and day 1 to 31, so both fit.
    (year, month as u32, day as u32)
}
