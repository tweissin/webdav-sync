use crate::webdav::WebDav;
use notify::DebouncedEvent::{Create, Remove, Rename, Write};
use notify::{watcher, RecursiveMode, Watcher};
use std::env;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn run(dir_to_watch: &str) -> Result<(), notify::Error> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    watcher.watch(dir_to_watch, RecursiveMode::Recursive)?;

    let hostname = env::var("WEBDAV_HOSTNAME");
    let username = env::var("WEBDAV_USERNAME");
    let password = env::var("WEBDAV_PASSWORD");
    let wd = WebDav::new(
        hostname.unwrap(),
        username.unwrap(),
        password.unwrap(),
        String::from(dir_to_watch),
    );

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Create(path) => {
                    println!("Create: {}", path.as_path().display());
                    match wd.write_file(path) {
                        Ok(_) => (),
                        Err(err) => println!("Create: there was an error! {}", err),
                    };
                }
                Write(path) => {
                    println!("Write: {}", path.as_path().display());
                    match wd.write_file(path) {
                        Ok(_) => (),
                        Err(err) => println!("Write: there was an error! {}", err),
                    };
                }
                Remove(path) => println!("Remove: {}", path.as_path().display()),
                Rename(orig, _) => println!("Rename: {}", orig.as_path().display()),
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
