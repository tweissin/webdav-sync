use rustydav::client;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::fs;

pub struct WebDav {
    client: rustydav::client::Client,
    hostname: String,
    dir_to_watch: PathBuf,
}

impl WebDav {
    pub fn new(host: String, username: String, password: String, dir_to_watch: String) -> WebDav {
        WebDav {
            client: client::Client::init(&username, &password),
            hostname: host,
            dir_to_watch: PathBuf::from(dir_to_watch),
        }
    }

    pub fn write_file(&self, path_buf: PathBuf) -> Result<()> {
        let is_file = path_buf.is_file();

        if is_file {
            let size: u64 = fs::metadata(&path_buf)?.len();
            let now = Instant::now();

            match Self::load_file_to_bytes(&path_buf) {
                Ok(file_content) => {
                    // Generate the remote path
                    let relative_path = path_buf.strip_prefix(self.dir_to_watch.clone()).unwrap();
                    let remote_path = format!(
                        "http://{}/{}",
                        self.hostname,
                        relative_path.display()
                    );
                    println!("Uploading to: {}", remote_path);

                    // Upload the file
                    match self.client.put(file_content, &remote_path) {
                        Ok(_) => println!("Uploaded: {}", remote_path),
                        Err(e) => eprintln!("Failed to upload {}: {:?}", remote_path, e),
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load file: {}", e);
                }
            }
            let duration = now.elapsed();
            let kbps = (size as u128 / duration.as_millis()) as f64 / 1000.0;
            println!(
                "elapsed {} ms, {} Kbps, {} bytes",
                duration.as_millis(),
                kbps,
                size
            );
            Ok(())
        } else {
            self.mkdir(&path_buf);
            Ok(())
        }
    }

    fn load_file_to_bytes(path: &std::path::PathBuf) -> Result<Vec<u8>> {
        let error = format!("Unable to read file {}", path.display());
        let buffer = fs::read(path).expect(&error);
        Ok(buffer)
    }

    fn mkdir(&self, path_buf: &Path) {
        if let Some(parent) = path_buf.parent() {
            let parent_str = parent.to_string_lossy();
            let length = parent_str.len();
            if length == 0 {
                // nothing to do
                return;
            }
            let remote_path = format!(
                "http://{}/{}",
                self.hostname,
                path_buf.file_name().unwrap().to_string_lossy()
            );
            println!("WebDav: mkdir '{}'", remote_path);

            match self.client.mkcol(&remote_path) {
                Err(err) => {
                    println!("problem making directory '{}' {}", remote_path, err);
                }
                _ => (),
            }
        } else {
            // nothing to do
            return;
        }
    }
}