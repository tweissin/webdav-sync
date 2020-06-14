use clap::{App, Arg, ArgMatches};
use std::env;
use std::process;

mod fswatch;
mod webdav;

fn main() {
    let matches = App::new("WebDav synchronizer")
        .version("0.1.0")
        .author("Tom Weissinger")
        .about("Filesystem watcher that syncs with WebDav")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .takes_value(true)
                .help("WebDav username"),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .takes_value(true)
                .help("WebDav password"),
        )
        .arg(
            Arg::with_name("hostname")
                .short("h")
                .long("hostname")
                .takes_value(true)
                .help("WebDav hostname"),
        )
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .takes_value(true)
                .help("Local filesystem directory to watch"),
        )
        .get_matches();

    let hostname: String;
    let username: String;
    let password: String;

    match env::var("WEBDAV_HOSTNAME") {
        Ok(val) => hostname = val,
        _ => hostname = parameter_check(&matches, "hostname"),
    };
    match env::var("WEBDAV_USERNAME") {
        Ok(val) => username = val,
        _ => username = parameter_check(&matches, "username"),
    };
    match env::var("WEBDAV_PASSWORD") {
        Ok(val) => password = val,
        _ => password = parameter_check(&matches, "password"),
    };
    let dir = parameter_check(&matches, "dir");

    match fswatch::run(&hostname, &username, &password, &dir) {
        Err(err) => panic!("Problem watching: {:?}", err),
        _ => (),
    }
}

fn parameter_check(matches: &ArgMatches, name: &str) -> String {
    matches
        .value_of(name)
        .unwrap_or_else(|| {
            println!("Need to specify {}", name);
            process::exit(1);
        })
        .to_string()
}
