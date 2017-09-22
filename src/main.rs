extern crate clap;
extern crate reqwest;

use clap::{App, Arg};
use std::io::Read;

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
    println!("{}", content);
}
