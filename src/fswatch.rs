use crate::webdav::WebDav;
use notify_debouncer_full::new_debouncer;
use notify::RecursiveMode;
use std::sync::mpsc::channel;
use std::time::Duration;

// see https://github.com/notify-rs/notify/blob/main/examples/debouncer_full.rs
pub fn run(
    hostname: &str,
    username: &str,
    password: &str,
    dir_to_watch: &str,
) -> Result<(), notify::Error> {
    // setup debouncer
    let (tx, rx) = channel();

    // no specific tickrate, max debounce time 2 seconds
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
                    if event.event.kind.is_create() {
                        let path_clone = event.event.paths[0].clone();
                        println!("Create: {}", path_clone.display());
                        let _ = wd.write_file(path_clone);
                    } else if event.event.kind.is_modify() {
                        if event.event.paths.len() == 1 {
                            let path_clone = event.event.paths[0].clone();
                            println!("Modify: {}", path_clone.display());
                            let _ = wd.write_file(path_clone);
                        } else {
                            let src_path = event.event.paths[0].clone();
                            let dst_path = event.event.paths[1].clone();
                            println!(
                                "Rename from: {} to: {}",
                                src_path.display(),
                                dst_path.display()
                            );
                            // Optionally handle renames
                        }
                    } else if event.event.kind.is_remove() {
                        let path_clone = event.event.paths[0].clone();
                        println!("Remove: {}", path_clone.display());
                        // Optionally handle deletions
                    }
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


