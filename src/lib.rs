extern crate chrono;

use chrono::*;

#[test]
fn hours_served__rounds_up() {
    assert_eq!(hours_served(NaiveTime::from_hms(6, 30, 0), NaiveTime::from_hms(7, 29, 0)), Ok(1));
}
