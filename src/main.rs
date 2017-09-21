extern crate clap;

use clap::{App, Arg};

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
    println!("username = {}", username);
    println!("verbose  = {}", verbose);
}
