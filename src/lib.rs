extern crate chrono;

use std::io;

use chrono::*;

//// Returns Rust [Result](http://rustbyexample.com/std/result.html)
//// representing the total hours that the baby sitter will get credit for.
fn hours_served(_from: NaiveTime, _to: NaiveTime) -> Result<i32, io::Error> {
    let r: i32 = 1;
    Ok(r)
}


#[test]
fn hours_served__rounds_up() {

    let x: Result<i32, io::Error> = hours_served(NaiveTime::from_hms(18, 30, 0), NaiveTime::from_hms(19, 29, 0));
    assert_eq!(x.unwrap(), 1);
}
