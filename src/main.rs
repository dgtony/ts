//! Simple utility for dealing with UNIX timestamps
//!
//! There are two modes of operation:
//! 1) no arguments - print current UNIX timestamp
//! 2) timestamp provided - parse timestamp and print as UTC datetime
//!
extern crate chrono;

use chrono::prelude::*;
use std::env;
use std::process;

fn current_ts() -> i64 {
    Utc::now().timestamp()
}

fn parse_ts(ts: &str) -> Option<DateTime<Utc>> {
    let moment_ts: i64 = ts.trim().parse().ok()?;
    let moment = NaiveDateTime::from_timestamp_opt(moment_ts, 0)?;

    Some(DateTime::from_utc(moment, Utc))
}

fn process_ts(ts: &str) {
    match parse_ts(ts) {
        Some(moment) => println!("{} => {}", ts.trim(), moment),
        None => {
            println!("cannot parse timestamp");
            process::exit(1);
        }
    }
}

fn main() {
    match env::args().skip(1).next() {
        Some(timestamp) => process_ts(&timestamp),
        None => {
            let ts = current_ts();
            println!("Current timestamp: {}", ts);
        }
    }
}
