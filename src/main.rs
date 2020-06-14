use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::DebouncedEvent::{NoticeWrite, NoticeRemove, Create, Write, Remove, Rename};

mod webdav;

fn watch() {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
    watcher
        .watch("/Users/tweissin/deletemesoon", RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                NoticeWrite(path) => println!("NoticeWrite: {}", path.as_path().display()),
                NoticeRemove(path) => println!("NoticeRemove: {}", path.as_path().display()),
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

fn main() {
    webdav::run();
}
