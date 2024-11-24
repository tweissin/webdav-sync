use crate::webdav::WebDav;
use notify_debouncer_full::new_debouncer;
use notify::RecursiveMode;
use std::sync::mpsc::channel;
use std::time::Duration;

// credit: https://github.com/notify-rs/notify/blob/main/examples/debouncer_full.rs
pub fn run(
    hostname: &str,
    username: &str,
    password: &str,
    dir_to_watch: &str,
) -> Result<(), notify::Error> {
    let (tx, rx) = channel();

    let mut watcher = new_debouncer(Duration::from_secs(2), None, tx)?;

    watcher.watch(dir_to_watch, RecursiveMode::Recursive)?;

    let wd = WebDav::new(
        String::from(hostname),
        String::from(username),
        String::from(password),
        String::from(dir_to_watch),
    );

    println!();
    println!("==========================");
    println!("Beginning filesystem watch of: {}", dir_to_watch);
    println!("               and syncing to: {}", hostname);
    println!();

    for result in rx {
        match result {
            Ok(events) => {
                for event in events {
                    handle_event(&wd, &event);
                }
            }
            Err(errors) => {
                // Handle errors
                for error in errors {
                    println!("watch error: {:?}", error);
                }
            }
        }
    }
    Ok(())
}

fn handle_event(wd: &WebDav, event: &notify::event::Event) {
    for path in &event.paths {
        if let Some(file_name) = path.file_name() {
            if file_name == ".DS_Store" {
                println!("Skipping macOS system file: {}", path.display());
                continue;
            }
        }

        match &event.kind {
            notify::event::EventKind::Create(_) => {
                println!("Create: {}", path.display());
                if let Err(e) = wd.write_file(path.clone()) {
                    eprintln!("Failed to handle create event: {}", e);
                }
            }
            notify::event::EventKind::Modify(modify_kind) => match modify_kind {
                notify::event::ModifyKind::Name(_) => {
                    if event.paths.len() == 1 {
                        // Single path: Check if it still exists
                        if path.exists() {
                            println!("Single path modify: rename or move detected: {}", path.display());
                        } else {
                            println!("Single path modify: file or directory deleted: {}", path.display());
                            if let Err(e) = wd.delete(path.clone()) {
                                eprintln!("Failed to handle delete event: {}", e);
                            }
                        }
                    } else if event.paths.len() == 2 {
                        // Two paths: Likely a rename or move
                        let src_path = &event.paths[0];
                        let dst_path = &event.paths[1];
                        println!("Rename or move detected from {} to {}", src_path.display(), dst_path.display());
                        if let Err(e) = wd.rename(src_path.clone(), dst_path.clone()) {
                            eprintln!("Failed to handle rename: {}", e);
                        }
                    } else {
                        eprintln!("Unexpected number of paths for modify: {:?}", event.paths);
                    }
                }
                _ => println!("Unhandled Modify kind: {:?}", modify_kind),
            },
            notify::event::EventKind::Remove(_) => {
                println!("Remove: {}", path.display());
                if let Err(e) = wd.delete(path.clone()) {
                    eprintln!("Failed to handle delete event: {}", e);
                }
            }
            _ => println!("Unhandled event kind: {:?}", event.kind),
        }
    }
}
