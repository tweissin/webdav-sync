use hyperdav::Client;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

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
        let f = File::open(path_buf.clone())?;
        let reader = BufReader::new(f);

        let mut path_vec: Vec<String> = vec![];
        let is_file = path_buf.is_file();
        make_path_vec(&self.dir_to_watch, path_buf, &mut path_vec);

        let mut pb = PathBuf::new();
        let mut iter = path_vec.iter();
        let mut path_and_file_vec: Vec<String> = vec![];
        loop {
            match iter.next() {
                Some(p) => {
                    let s = String::from(p);
                    path_and_file_vec.push(s);
                    pb.push(p)
                }
                None => break,
            }
        }
        if is_file {
            if path_vec.len() > 0 {
                path_vec.pop();
                self.mkdir(path_vec);
            }
            self.put_file(path_and_file_vec, reader);
            Ok(())
        } else {
            self.mkdir(path_vec);
            Ok(())
        }
    }

    fn put_file(&self, path_and_file_vec: Vec<String>, reader: BufReader<File>) {
        let pathstr = path_and_file_vec.join("/");
        println!("WebDav: write '{}'", pathstr);
        match self.client.put(reader, path_and_file_vec) {
            Err(err) => {
                println!("problem writing file {}", err);
            }
            _ => (),
        }
    }

    fn mkdir(&self, path_vec: Vec<String>) {
        let pathstr = path_vec.join("/");
        println!("WebDav: mkdir '{}'", pathstr);

        match self.client.mkcol(&path_vec) {
            Err(err) => {
                println!("problem making directory '{}' {}", pathstr, err);
            }
            _ => (),
        }
    }
}

fn make_path_vec(dir_to_watch: &PathBuf, incoming_path: PathBuf, path_vec: &mut Vec<String>) {
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
        make_path_vec(&dir_to_watch, incoming_path, &mut splitted_path);
        assert_eq!(3, splitted_path.len());
        assert_eq!("one", splitted_path[0]);
        assert_eq!("two", splitted_path[1]);
        assert_eq!("three.txt", splitted_path[2]);
    }
    /*
    const OWNCLOUD_URL: &'static str = "https://demo.owncloud.org/remote.php/webdav/";

    fn get_client() -> Client {
        let uuid = Uuid::new_v4();
        let url = format!("{}{}", OWNCLOUD_URL, uuid);
        ClientBuilder::default()
            .credentials("test", "test")
            .build(&url)
            .unwrap()
    }
        */
}
