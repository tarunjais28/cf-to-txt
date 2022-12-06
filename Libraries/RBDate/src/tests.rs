use chrono::NaiveDate;
use super::increment_date_by_months_unchecked;
use super::increment_date_by_months;


#[test]
pub fn test_month_addition() {
    for c in get_test_candidates() {
        assert_eq!(
            c.expected_date,
            increment_date_by_months(c.date, c.difference)
        );
    }
}

#[test]
pub fn test_unchecked_month_addition() {
    for c in get_test_candidates() {
        assert_eq!(
            c.expected_date,
            increment_date_by_months_unchecked(c.date, c.difference)
        );
    }
}


// MARK: Test Candidates


struct TestCandidate {
    date: NaiveDate,
    difference: u16,
    expected_date: NaiveDate
}

impl TestCandidate {
    fn new(d: NaiveDate, diff: u16, e: NaiveDate) -> TestCandidate {
        TestCandidate {
            date: d,
            difference: diff,
            expected_date: e,
        }
    }
}

fn get_test_candidates() -> Vec<TestCandidate> {

    let c1 = TestCandidate::new(
        NaiveDate::from_ymd(2007, 7, 15),
        1,
        NaiveDate::from_ymd(2007, 8, 15)
    );

    let c2 = TestCandidate::new(
        NaiveDate::from_ymd(2007, 7, 15),
        25,
        NaiveDate::from_ymd(2009, 8, 15)
    );

    let c3 = TestCandidate::new(
        NaiveDate::from_ymd(2008, 2, 29),
        12,
        NaiveDate::from_ymd(2009, 2, 28)
    );

    let c4 = TestCandidate::new(
        NaiveDate::from_ymd(2007, 3, 31),
        1,
        NaiveDate::from_ymd(2007, 4, 30)
    );

    vec!(c1, c2, c3, c4)
}