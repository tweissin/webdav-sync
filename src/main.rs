use clap::Parser;
mod fswatch;
mod webdav;
use std::{env, io::Result};

/// Filesystem watcher that syncs with WebDav
#[derive(Parser, Debug)]
#[command(version, author = env!("CARGO_PKG_AUTHORS"))]
struct Args {
    /// WebDav username (or set WEBDAV_USERNAME env var)
    #[arg(short, long, default_value_t=get_default_from_env("WEBDAV_USERNAME"))]
    username: String,

    /// WebDav password (or set WEBDAV_PASSWORD env var)
    #[arg(short, long, default_value_t=get_default_from_env("WEBDAV_PASSWORD"))]
    password: String,

    /// WebDav hostname (or set WEBDAV_HOSTNAME env var)
    #[arg(long, default_value_t=get_default_from_env("WEBDAV_HOSTNAME"))]
    hostname: String,

    /// Local filesystem directory to watch
    #[arg(short, long, default_value_t=get_default_dir())]
    dir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(err) = fswatch::run(&args.hostname, &args.username, &args.password, &args.dir) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }

    Ok(())
}

fn get_default_from_env(name: &str) -> String {
    match env::var(name) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Warning: Environment variable '{}' is not set", name);
            "".to_string()
        },
    }
}

fn get_default_dir() -> String {
    let home = dirs_next::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
    format!("{}/deletemesoon", home.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_from_env() {
        // Test when environment variable is set
        env::set_var("MY_VAR", "test_value");
        assert_eq!(get_default_from_env("MY_VAR"), "test_value");

        // Test when environment variable is not set
        assert_eq!(get_default_from_env("UNKNOWN_VAR"), "");
    }
}