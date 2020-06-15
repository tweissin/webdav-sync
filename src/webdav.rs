use hyperdav::Client;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;
use std::thread;
use std::time::Instant;

pub struct WebDav {
    client: Client,
    dir_to_watch: PathBuf,
}

impl WebDav {
    pub fn new(host: String, username: String, password: String, dir_to_watch: String) -> WebDav {
        WebDav {
            client: Client::new()
                .credentials(username, password)
                .build(&host)
                .unwrap(),
            dir_to_watch: PathBuf::from(dir_to_watch),
        }
    }
    pub fn write_file(&self, path_buf: PathBuf) -> io::Result<()> {
        let f = File::open(&path_buf)?;
        let reader = BufReader::new(f);

        let mut path_vec: Vec<String> = vec![];
        let is_file = path_buf.is_file();
        make_path_vec(&self.dir_to_watch, &path_buf, &mut path_vec);

        if is_file {
            let path_and_file_vec = path_vec.clone();
            if path_vec.len() > 0 {
                path_vec.pop();
                self.mkdir(path_vec);
            }
            let size: u64 = fs::metadata(&path_buf)?.len();
            let now = Instant::now();

            // put the file!
            self.put_file(path_and_file_vec, reader);

            let duration = now.elapsed();
            let kbps = (size as u128 / duration.as_millis()) as f64 / 1000.0;
            println!(
                "{:?} elapsed {} ms, {} Kbps, {} bytes",
                thread::current().id(),
                duration.as_millis(),
                kbps,
                size
            );
            Ok(())
        } else {
            self.mkdir(path_vec);
            Ok(())
        }
    }

    fn put_file(&self, path_and_file_vec: Vec<String>, reader: BufReader<File>) {
        let pathstr = path_and_file_vec.join("/");
        println!("{:?} WebDav: write '{}'", thread::current().id(), pathstr);
        match self.client.put(reader, path_and_file_vec) {
            Err(err) => {
                println!("problem writing file {}", err);
            }
            _ => (),
        }
    }

    fn mkdir(&self, path_vec: Vec<String>) {
        let pathstr = path_vec.join("/");
        if pathstr.len() == 0 {
            // nothing to do
            return;
        }
        println!("{:?} WebDav: mkdir '{}'", thread::current().id(), pathstr);

        match self.client.mkcol(&path_vec) {
            Err(err) => {
                println!("problem making directory '{}' {}", pathstr, err);
            }
            _ => (),
        }
    }
}

fn make_path_vec(dir_to_watch: &PathBuf, incoming_path: &PathBuf, path_vec: &mut Vec<String>) {
    let mut iter1 = dir_to_watch.iter();
    let mut iter2 = incoming_path.iter();
    loop {
        match iter1.next() {
            Some(_) => iter2.next(),
            None => break,
        };
    }
    loop {
        match iter2.next() {
            Some(val) => {
                let p = String::from(val.to_str().unwrap());
                path_vec.push(p)
            }
            None => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_split_pathbuf() {
        let mut splitted_path: Vec<String> = vec![];
        let dir_to_watch = PathBuf::from("/Users/tweissin/deletemesoon");
        let incoming_path = PathBuf::from("/Users/tweissin/deletemesoon/one/two/three.txt");
        make_path_vec(&dir_to_watch, &incoming_path, &mut splitted_path);
        assert_eq!(3, splitted_path.len());
        assert_eq!("one", splitted_path[0]);
        assert_eq!("two", splitted_path[1]);
        assert_eq!("three.txt", splitted_path[2]);
    }
}
