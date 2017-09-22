extern crate clap;
extern crate reqwest;
extern crate json;
extern crate chrono;

use clap::{App, Arg};
use std::io::Read;
use chrono::prelude::*;

fn main() {
    let matches = App::new("bakt")
        .version("1.0")
        .about("An app to tell you if you've done anything on Github today")
        .author("Phil B.")
        .arg(Arg::with_name("username")
            .help("Your Github username")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Enable verbose output"))
        .get_matches();

    let username = matches.value_of("username").unwrap();
    let verbose = matches.occurrences_of("verbose");
    let url = format!("https://api.github.com/users/{}/events", username);

    // TODO: Set user-agent header
    let mut resp = reqwest::get(&url).unwrap(); // TODO: Don't unwrap here!!
    assert!(resp.status().is_success());  // TODO: Don't assert here

    let mut content = String::new();
    if let Err(error) = resp.read_to_string(&mut content) {
        panic!("{:?}", error); // TODO: Handle this better
    }

    let today = Local::now();
    let parsed = json::parse(&content).unwrap(); // TODO: Don't unwrap here!!
    if let json::JsonValue::Array(data) = parsed {

        let todays_events = data.iter().filter(|x| {
            let dt_str = x["created_at"].as_str().unwrap();
            let dt = DateTime::parse_from_rfc3339(dt_str).unwrap(); // TODO: Is the the right date time standard
            let dt_local = dt.with_timezone(&Local);
            let is_today = dt_local.year() == today.year()
                        && dt_local.month() == today.month()
                        && dt_local.day() == today.day();
            if is_today && verbose > 0 {
                let event_type = x["type"].as_str().unwrap();
                println!("{} at {}", event_type, dt_local.format("%H:%M:%S").to_string());
            }
            is_today
        });

        // In addition to it's obvious purpuse, I'm using the count method call
        // below to cause side-effects in the filter above (print out today's
        // events when verbose > 0) ... that doesn't seem so nice but it does
        // save me from having to parse the datetime and convert it to local
        // time again. Perhaps a filter_map would be better ... that could
        // filter the events, returning today's events in local time along with
        // the event type, ready to be printed when verbose > 0. Then hopefully
        // I can find some way to short circuit evaluation of the filter_map
        // in cases where verbose < 1.
        if todays_events.count() > 0 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}
