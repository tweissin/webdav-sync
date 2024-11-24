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
                // Skip macOS .DS_Store files
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
                    if event.paths.len() == 2 {
                        // Handle rename events
                        let src_path = &event.paths[0];
                        let dst_path = &event.paths[1];
                        println!(
                            "Rename detected from: {} to: {}",
                            src_path.display(),
                            dst_path.display()
                        );

                        // Optionally handle the rename with your WebDav client
                        if let Err(e) = wd.rename(src_path.clone(), dst_path.clone()) {
                            eprintln!("Failed to handle rename: {}", e);
                        }
                    } else {
                        println!(
                            "Unexpected number of paths for rename: {:?}",
                            event.paths
                        );
                    }
                }
                notify::event::ModifyKind::Metadata(_) => {
                    println!("Metadata changed: {}", path.display());
                }
                _ => {
                    println!("Other modify event: {:?}", modify_kind);
                }
            },
            notify::event::EventKind::Remove(_) => {
                println!("Remove: {}", path.display());
            }
            _ => {
                println!("Unhandled event: {:?}", event.kind);
            }
        }
    }
}


