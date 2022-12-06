//! # RBDate
//!
//! RBDate provides a collection of libraries that make it easy to work with dates
// TODO: Change the line above when the purpose of the crate is defined better. ^
extern crate chrono;

#[allow(dead_code)]
#[allow(unused_imports)]
mod tests;

pub use chrono::NaiveDate;
pub use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::Datelike;
use chrono::Duration;
use std::cmp::min;

/// Adds `month_count` number of months to `date` and returns a new date.
///
/// This method doesn't check for integer overflows.
/// Please refer to `increment_date_by_months` for integer overflow safety.
pub fn increment_date_by_months_unchecked(date: NaiveDate, month_count: u16) -> NaiveDate {
    let next_month_0 = (date.month0() as i64) + (month_count as i64);
    let additional_years = (next_month_0 / 12) as i32;
    let next_month_0 = (next_month_0 % 12) as u32;
    let next_year = date.year() + (additional_years);
    let next_day = min(date.day(), last_day_of_month_0(next_year, next_month_0));

    NaiveDate::from_ymd_opt(next_year, next_month_0 + 1, next_day).unwrap()
}


/// Adds `month_count` number of months to `date` and returns a new date.
pub fn increment_date_by_months(date: NaiveDate, month_count: u16) -> NaiveDate {
    // Slight modification of: https://github.com/kosta/date-iterator/blob/master/src/calendar_duration.rs
    // Thank you @kosta
    let next_month_0 = (date.month0() as i64).checked_add(month_count as i64).unwrap();
    let additional_years = next_month_0 / 12;
    let next_month_0 = (next_month_0 % 12) as u32;
    let additional_years =
        if additional_years >= (i32::max_value() as i64) {
            panic!("Date too far in the future. Date: {:?}, Month Count: {}", date, month_count);
        } else {
            additional_years as i32
        };
    let next_year = date.year().checked_add(additional_years).unwrap();
    let next_day = min(date.day(), last_day_of_month_0(next_year, next_month_0));

    NaiveDate::from_ymd_opt(next_year, next_month_0 + 1, next_day).unwrap()
}

/// Returns the number of days between the `start` and `end` dates.
///
/// This method assumes the `start` date is greater than the `end` date. Otherwise the function will panic.
pub fn num_days_start_to_end(start: &NaiveDate, end: &NaiveDate) -> i64 {

    // TODO: This is obviously a horrible implementation.
    // Calculating the number of days between two dates should be a constant operation.

    if start > end {
        panic!("Start date cannot be greater than end date");
    }

    let mut i = 0;
    let start_date = start.clone();

    loop {
        if &(start_date + Duration::days(i)) == end {
            return i as i64;
        }

        i += 1;
    }

}

/// Returns a timestamp that corresponds to the start of the `date`.
pub fn timestamp(date: &NaiveDate) -> i64 {
    date.and_hms(0, 0, 0).timestamp()
}

pub fn date_from_timestamp(t: i64) -> NaiveDate {
    // TODO: Obviously wasteful!
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}

/// A DateParser parses Dates from Strings.
///
/// The `date_format` property specifies the format of the String that represents a date.
///
/// If a date object cannot be parsed, the `use_today_on_error` specifies whether the `parse` method should return
/// today's date or `panic` instead.
pub struct DateParser {
    date_format: String,
    use_today_on_error: bool
}

impl DateParser {

    /// Creates a new date parser.
    ///
    /// Use the `parse` method to parse a date from a String.
    pub fn new(date_format: String, use_today_on_error: bool) -> DateParser {
        DateParser {
            date_format,
            use_today_on_error
        }
    }
}

impl DateParser {

    /// Parses the `string` into a date.
    ///
    /// If the `DateParser`'s `use_today_on_error` property is set to true, and the parser cannot parse the
    /// `string`, then you're returned today's date instead.
    pub fn parse(&self, string: &str) -> NaiveDate {
        match NaiveDate::parse_from_str(string, &self.date_format) {
            Ok(parsed_date) => {
                return parsed_date;
            },
            Err(error) => {
                if self.use_today_on_error {
                    return Utc::now().naive_utc().date();
                }

                panic!(
                    "Could not parse date string. String: '{}', Expected format: '{}'. Parse error: '{}'",
                     string, self.date_format, error
                );
            }
        }
    }

    /// Parses the `string` into an `Option<NaiveDate>`.
    pub fn parse_opt(&self, string: &str) -> Option<NaiveDate> {
        match NaiveDate::parse_from_str(string, &self.date_format) {
            Ok(parsed_date) => {
                return Some(parsed_date);
            },
            Err(_error) => {
                return None;
            }
        }
    }
}

/// Outputs the current (local) time in the format `2001-07-08T00:34:60.026490+09:30`
pub fn current_time_utc() -> String {
    format!("{}", chrono::Local::now().format("%+"))
}

fn last_day_of_month_0(year: i32, month_0: u32) -> u32 {
    last_day_of_month(year, month_0 + 1)
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
        .day()
}