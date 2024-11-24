use rustydav::client;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::time::Instant;

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

    /// Writes a file to the remote WebDAV server
    pub fn write_file(&self, path_buf: PathBuf) -> Result<()> {
        if path_buf.is_file() {
            self.upload_file(&path_buf)?;
        } else {
            self.mkdir(&path_buf);
        }
        Ok(())
    }

    /// Load file contents into a vector of bytes
    fn load_file_to_bytes(path: &Path) -> Result<Vec<u8>> {
        fs::read(path).map_err(|err| {
            eprintln!("Unable to read file {}: {}", path.display(), err);
            err
        })
    }

    /// Create a directory on the remote WebDAV server
    fn mkdir(&self, path_buf: &Path) {
        if let Some(parent) = path_buf.parent() {
            let remote_path = self.generate_remote_path(parent);
            println!("WebDav: mkdir '{}'", remote_path);

            if let Err(err) = self.client.mkcol(&remote_path) {
                eprintln!("Failed to create directory '{}': {}", remote_path, err);
            }
        }
    }

    /// Upload a file to the remote WebDAV server
    fn upload_file(&self, path_buf: &Path) -> Result<()> {
        let size = fs::metadata(path_buf)?.len();
        let start_time = Instant::now();

        let file_content = Self::load_file_to_bytes(path_buf)?;
        let remote_path = self.generate_remote_path(path_buf);

        println!("Uploading to: {}", remote_path);

        match self.client.put(file_content, &remote_path) {
            Ok(_) => println!("Uploaded: {}", remote_path),
            Err(e) => eprintln!("Failed to upload {}: {:?}", remote_path, e),
        }

        let duration = start_time.elapsed();
        let kbps = (size as f64 / duration.as_secs_f64()) / 1000.0;
        println!(
            "Elapsed: {} ms, Speed: {:.2} KBps, Size: {} bytes",
            duration.as_millis(),
            kbps,
            size
        );

        Ok(())
    }

    /// Generate a remote path for a given local path
    fn generate_remote_path(&self, path: &Path) -> String {
        match path.strip_prefix(&self.dir_to_watch) {
            Ok(relative_path) => format!("http://{}/{}", self.hostname, relative_path.display()),
            Err(_) => format!("http://{}/{}", self.hostname, path.display()), // Fallback if strip_prefix fails
        }
    }
}