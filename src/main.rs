use clap::Parser;
mod fswatch;
mod webdav;
use std::{env, io::Result};

/// Filesystem watcher that syncs with WebDav
#[derive(Parser, Debug)]
#[command(version, author = env!("CARGO_PKG_AUTHORS"))]
struct Args {
    /// WebDav username
    #[arg(short, long, default_value_t=get_default_from_env("WEBDAV_USERNAME"))]
    username: String,

    /// WebDav password
    #[arg(short, long, default_value_t=get_default_from_env("WEBDAV_PASSWORD"))]
    password: String,

    /// WebDav hostname
    #[arg(long, default_value_t=get_default_from_env("WEBDAV_HOSTNAME"))]
    hostname: String,

    /// Local filesystem directory to watch
    #[arg(short, long, default_value_t=get_default_dir())]
    dir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let hostname: String = args.hostname;
    let username: String = args.username;
    let password: String = args.password;
    let dir: String = args.dir;

    println!("{}", hostname);
    println!("{}", username);
    println!("{}", password);
    println!("{}", dir);

    match fswatch::run(&hostname, &username, &password, &dir) {
        Ok(()) => Ok(()),
        Err(err) => panic!("Problem watching: {:?}", err),
    }
}

fn get_default_from_env(name: &str) -> String {
    match env::var(name) {
        Ok(value) => value,
        Err(_) => "".to_string(),
    }
}

fn get_default_dir() -> String {
    format!(
        "{}/deletemesoon",
        env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())
    )
}
