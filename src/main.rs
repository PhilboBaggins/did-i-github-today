#![forbid(unsafe_code)]

extern crate chrono;
extern crate clap;
extern crate json;
extern crate ureq;

//use chrono::prelude::*;
use clap::{crate_version, Command, Arg, ArgAction};
use serde::Deserialize;
use chrono::{DateTime, Local};
use crate::chrono::Datelike;

#[derive(Deserialize, Debug)]
struct UserEventsApiResponse {
    created_at: String,
    r#type: String,
    payload: UserEventsPayload,
}

#[derive(Deserialize, Debug)]
struct UserEventsPayload {
    ref_type: Option<String>,
}

fn describe_event(event: &UserEventsApiResponse) -> String {
    match (event.r#type.as_str(), &event.payload.ref_type) {
        ("CreateEvent", Some(ref_type)) => format!("CreateEvent ({})", ref_type),
        // TODO: Consider other event types: https://developer.github.com/v3/activity/events/types/
        _ => format!("{}", event.r#type),
    }
}

fn look_for_events(data: Vec<UserEventsApiResponse>, verbose: u64) {
    let today = Local::now();
    let todays_events = data.iter().filter(|x| {
        let dt = DateTime::parse_from_rfc3339(&x.created_at).unwrap(); // TODO: Is this the right date/time standard?
        let dt_local = dt.with_timezone(&Local);
        let is_today = dt_local.year() == today.year()
                    && dt_local.month() == today.month()
                    && dt_local.day() == today.day();
        if is_today && verbose > 0 {
            println!("{} at {}", describe_event(x), dt_local.format("%H:%M:%S").to_string());
        }
        is_today
    });

    // In addition to it's obvious purpose, I'm using the count method call
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

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Ureq(ureq::Error),
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::Io(err)
    }
}

impl From<ureq::Error> for MyError {
    fn from(err: ureq::Error) -> MyError {
        MyError::Ureq(err)
    }
}

fn get_and_parse_json(url: &str, verbose: u64) -> Result<Vec<UserEventsApiResponse>, MyError> {
    if verbose > 1 {
        println!("Fetching {}", url);
    }

    // TODO: Set user-agent header - https://developer.github.com/v3/#user-agent-required
    let resp: Vec<UserEventsApiResponse> = ureq::get(url)
        .set("Accept", "application/vnd.github.v3+json")
        .call()?
        .into_json()?;

    if verbose > 2 {
        // Super verbose!
        println!("{:?}", resp);
    }

    Ok(resp)
}

macro_rules! die {
    ($($tt:tt)*) => {{
        eprintln!($($tt)*);
        ::std::process::exit(1)
    }}
}

fn main() {
    let matches = App::new("Did I Github today?")
        .version(crate_version!())
        .about("An command line application to tell you if you've done anything on Github today")
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

    match get_and_parse_json(&url, verbose) {
        Ok(data) => look_for_events(data, verbose),

        Err(MyError::Io(err)) => die!("IO error: {}", err),
        Err(MyError::Ureq(err)) => die!("HTTP request error: {}", err),
    }
}
