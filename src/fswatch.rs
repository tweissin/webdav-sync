use std::{io, time::Duration};

use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;

use rustydav::client;
use std::fs;

// see https://github.com/notify-rs/notify/blob/main/examples/debouncer_full.rs
pub fn run(
    hostname: &str,
    username: &str,
    password: &str,
    dir_to_watch: &str,
) -> Result<(), notify::Error> {
    let client = client::Client::init(username, password);

    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // no specific tickrate, max debounce time 2 seconds
    let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;

    debouncer.watch(dir_to_watch, RecursiveMode::Recursive)?;

    // print all events and errors
    for result in rx {
        match result {
            Ok(events) => {
                for event in events {
                    for path in &event.event.paths {
                        let path_clone = path.clone();
                        if event.event.kind.is_modify() {
                            println!("File modified: {}", path_clone.display());
                            upload_to_webdav(&client, hostname, path_clone);
                        } else if event.event.kind.is_create() {
                            println!("File created: {}", path_clone.display());
                            upload_to_webdav(&client, hostname, path_clone);
                        } else if event.event.kind.is_other() {
                            println!("File renamed???: {}", path_clone.display());
                            // Optionally handle renames
                        } else if event.event.kind.is_remove() {
                            println!("File deleted: {}", path_clone.display());
                            // Optionally handle deletions
                        }
                    }
                }
            }
            Err(errors) => {
                // Handle errors
                for error in errors {
                    println!("Error: {:?}", error);
                }
            }
        }
        println!();
    }
    Ok(())
}

fn upload_to_webdav(client: &client::Client, hostname: &str, path: std::path::PathBuf) {
    match load_file_to_bytes(&path) {
        Ok(file_content) => {
            // Generate the remote path
            let remote_path = format!("http://{}/{}", hostname, path.file_name().unwrap().to_string_lossy());
            println!("Uploading to: {}", remote_path);

            // Upload the file
            // (&self, body: B, path: &str)
            match client.put(file_content, &remote_path) {
                Ok(_) => println!("Uploaded: {}", remote_path),
                Err(e) => eprintln!("Failed to upload {}: {:?}", remote_path, e),
            }
        }
        Err(e) => {
            eprintln!("Failed to load file: {}", e);
        }
    }
}

fn load_file_to_bytes(path: &std::path::PathBuf) -> io::Result<Vec<u8>> {
    let error = format!("Unable to read file {}", path.display());
    let buffer = fs::read(path).expect(&error);
    Ok(buffer)                       
}
