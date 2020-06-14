use notify::DebouncedEvent::{Create, Remove, Rename, Write};
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn run(dir_to_watch: &str) -> Result<(), notify::Error> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    watcher.watch(dir_to_watch, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Create(path) => println!("Create: {}", path.as_path().display()),
                Write(path) => println!("Write: {}", path.as_path().display()),
                Remove(path) => println!("Remove: {}", path.as_path().display()),
                Rename(orig, _) => println!("Rename: {}", orig.as_path().display()),
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
