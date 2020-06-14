use clap::{App, Arg, ArgMatches};
use rpassword;
use std::env;
use std::io;
use std::io::Result;

mod fswatch;
mod webdav;

fn main() -> Result<()> {
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
        _ => hostname = parameter_check(&matches, "hostname (i.e. http://192.168.1.2)", false)?,
    };
    match env::var("WEBDAV_USERNAME") {
        Ok(val) => username = val,
        _ => username = parameter_check(&matches, "username", false)?,
    };
    match env::var("WEBDAV_PASSWORD") {
        Ok(val) => password = val,
        _ => password = parameter_check(&matches, "password", true)?,
    };
    let dir = parameter_check(&matches, "dir", false)?;

    match fswatch::run(&hostname, &username, &password, &dir) {
        Err(err) => panic!("Problem watching: {:?}", err),
        _ => Ok(()),
    }
}

fn parameter_check(matches: &ArgMatches, name: &str, password: bool) -> Result<String> {
    match matches.value_of(name) {
        Some(val) => Ok(val.to_string()),
        None => Ok(read_parameter(name, password)),
    }
}

// Read paramter from stdin
fn read_parameter(name: &str, password: bool) -> String {
    let msg = format!("Enter {}: ", name);
    if password {
        rpassword::prompt_password_stdout(&msg).unwrap()
    } else {
        println!("{}", msg);
        let mut val = String::new();
        match io::stdin().read_line(&mut val) {
            _ => String::from(val.trim_end()),
        }
    }
}
