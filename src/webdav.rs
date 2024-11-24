use rustydav::client;
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct WebDav {
    client: rustydav::client::Client,
    hostname: String,
}

impl WebDav {
    pub fn new(host: String, username: String, password: String) -> WebDav {
        WebDav {
            client: client::Client::init(&username, &password),
            hostname: host,
        }
    }
    pub fn write_file(&self, path_buf: PathBuf) {
        match Self::load_file_to_bytes(&path_buf) {
            Ok(file_content) => {
                // Generate the remote path
                let remote_path = format!(
                    "http://{}/{}",
                    self.hostname,
                    path_buf.file_name().unwrap().to_string_lossy()
                );
                println!("Uploading to: {}", remote_path);
    
                // Upload the file
                // (&self, body: B, path: &str)
                match self.client.put(file_content, &remote_path) {
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
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

}
