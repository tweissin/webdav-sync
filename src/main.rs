use clap::Parser;
use std::env;
use std::io;

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

fn main() {
    // let args = Args::parse();
    let args = Args {
        username: String::from("foo"),
        password: String::from("pwd"),
        hostname: String::from("127.0.0.1"),
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
}