extern crate chrono;

// use duration::Duration;
use std::io;

use chrono::*;

//// Returns Rust [Result](http://rustbyexample.com/std/result.html)
//// representing the total hours that the baby sitter will get credit for.
fn hours_served(_from: NaiveTime, _to: NaiveTime) -> Result<i32, io::Error> {
    
    let diff = _from - _to;
    println!("Response: {}", diff.num_minutes());
    
    let r: i32 = 1;
    Ok(r)
}


#[test]
fn hours_served__rounds_up() {

    let start_hour:i32 = 18;

    let start =  NaiveTime::from_hms(start_hour as u32, 30, 0);

    for x in 19..23 {
        
        let end = NaiveTime::from_hms(x as u32, 29, 0);

        let r: Result<i32, io::Error> = hours_served(start, end);
        let diff = start_hour - x;
        assert_eq!(r.unwrap(), diff as i32);  

    }
}
