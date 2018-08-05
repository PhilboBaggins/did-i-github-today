extern crate clap;
extern crate reqwest;
extern crate json;
extern crate chrono;

use clap::{App, Arg};
use std::io::Read;
use chrono::prelude::*;

fn describe_event(event: &json::JsonValue) -> String {
    match event["type"].as_str() {
        Some("CreateEvent") => {
            let payload_ref_type = event["payload"]["ref_type"].as_str().unwrap_or("Unknown");
            format!("CreateEvent ({})", payload_ref_type)
        },
        // TODO: Consider other event types: https://developer.github.com/v3/activity/events/types/
        Some(event_type) => format!("{}", event_type),
        None => "Unknown event".to_string(),
    }
}

fn look_for_events(data: Vec<json::JsonValue>, verbose: u64) {
    let today = Local::now();
    let todays_events = data.iter().filter(|x| {
        let dt_str = x["created_at"].as_str().unwrap();
        let dt = DateTime::parse_from_rfc3339(dt_str).unwrap(); // TODO: Is this the right date/time standard?
        let dt_local = dt.with_timezone(&Local);
        let is_today = dt_local.year() == today.year()
                    && dt_local.month() == today.month()
                    && dt_local.day() == today.day();
        if is_today && verbose > 0 {
            println!("{} at {}", describe_event(x), dt_local.format("%H:%M:%S").to_string());
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
    } else {
        println!("No");
    }
}

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

    if verbose > 1 {
        println!("Fetching {}", url);
    }
    // TODO: Set user-agent header - https://developer.github.com/v3/#user-agent-required
    let mut resp = reqwest::get(&url).unwrap_or_else(|error| {
        eprintln!("{}", error.to_string());
        ::std::process::exit(1);
    });
    if resp.status().is_success() == false {
        eprintln!("Failed to access Github API, HTTP status code was {}", resp.status());
        ::std::process::exit(1);
    }

    let mut content = String::new();
    if let Err(error) = resp.read_to_string(&mut content) {
        panic!("{:?}", error); // TODO: Handle this better
    }
    if verbose > 2 { // Super verbose!
        println!("{}", content);
    }

    if let Ok(json::JsonValue::Array(data)) = json::parse(&content) {
        look_for_events(data, verbose);
    } else {
        eprintln!("Unable to understand response from Github API");
        ::std::process::exit(1);
    }
}
