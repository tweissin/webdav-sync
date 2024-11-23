use clap::Parser;
mod fswatch;
use std::io::Result;

/// Filesystem watcher that syncs with WebDav
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// WebDav username
    #[arg(short, long)]
    username: String,

    /// WebDav password
    #[arg(short, long)]
    password: String,

    /// WebDav hostname
    #[arg(long)]
    hostname: String,

    /// Local filesystem directory to watch
    #[arg(short, long)]
    dir: String,
}

fn main() -> Result<()> {
    // let args = Args::parse();
    let args = Args {
        username: String::from("YOUR_USERNAME"),
        password: String::from("YOUR_PASSWORD"),
        hostname: String::from("localhost"),
        dir: String::from("/tmp")
    };

    let hostname: String = args.hostname;
    let username: String = args.username;
    let password: String = args.password;
    let dir: String = args.dir;

    println!("{}", &hostname);
    println!("{}", &username);
    println!("{}", &password);
    println!("{}", &dir);

    match fswatch::run(&hostname, &username, &password, &dir) {
        err => panic!("Problem watching: {:?}", err),
        _ => Ok(()),
    }
}