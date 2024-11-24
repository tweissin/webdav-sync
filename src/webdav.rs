use rustydav::client;
use std::fs;
use std::io::{Error, ErrorKind, Result};
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
            self.create_dir(path_buf)?; 
        }
        Ok(())
    }

    pub fn rename(&self, src: PathBuf, dst: PathBuf) -> Result<()> {
        // Convert the source and destination paths to relative paths
        let relative_src = src
            .strip_prefix(&self.dir_to_watch)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;
        let relative_dst = dst
            .strip_prefix(&self.dir_to_watch)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;

        // Construct the full WebDav URLs
        let remote_src = format!("http://{}/{}", self.hostname, relative_src.display());
        let remote_dst = format!("http://{}/{}", self.hostname, relative_dst.display());

        println!("Renaming on server: {} -> {}", remote_src, remote_dst);

        // Use the "mv" method instead of "move_item"
        match self.client.mv(&remote_src, &remote_dst) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                e.to_string(),
            )),
        }
    }

    pub fn delete(&self, path: PathBuf) -> Result<()> {
        // Convert the path to a relative path
        let relative_path = path.strip_prefix(&self.dir_to_watch)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;
        
        // Construct the full WebDav URL
        let remote_path = format!("http://{}/{}", self.hostname, relative_path.display());

        println!("Deleting: {}", remote_path);

        // Call the WebDav client delete method
        match self.client.delete(&remote_path) {
            Ok(_) => {
                println!("Successfully deleted: {}", remote_path);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to delete {}: {:?}", remote_path, e);
                Err(Error::new(ErrorKind::Other, e.to_string()))
            }
        }
    }

    /// Load file contents into a vector of bytes
    fn load_file_to_bytes(path: &Path) -> Result<Vec<u8>> {
        fs::read(path).map_err(|err| {
            eprintln!("Unable to read file {}: {}", path.display(), err);
            err
        })
    }

    pub fn create_dir(&self, dir_path: PathBuf) -> Result<()> {
        // Convert the local directory path to a relative path
        let relative_path = dir_path
            .strip_prefix(&self.dir_to_watch)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;
    
        // Construct the remote WebDAV URL
        let remote_path = format!("http://{}/{}", self.hostname, relative_path.display());
        println!("WebDav: mkdir '{}'", remote_path);
    
        // Make the directory on the WebDAV server
        match self.client.mkcol(&remote_path) {
            Ok(_) => {
                println!("Successfully created directory: {}", remote_path);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to create directory '{}': {}", remote_path, e);
                Err(Error::new(ErrorKind::Other, e.to_string()))
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
