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

        let mut splitted_path: Vec<String> = vec![];
        let is_file = path_buf.is_file();
        split_pathbuf(&self.dir_to_watch, path_buf, &mut splitted_path);

        let mut pb = PathBuf::new();
        let mut iter = splitted_path.iter();
        let mut pathstr: Vec<String> = vec![];
        loop {
            match iter.next() {
                Some(p) => {
                    println!("adding {}", p);
                    let s = String::from(p);
                    pathstr.push(s);
                    pb.push(p)
                }
                None => break,
            }
        }
        if is_file {
            if splitted_path.len() > 0 {
                splitted_path.pop();
                self.mkdir(splitted_path);
            }

            println!("writing file {:?}", pathstr);
            match self.client.put(reader, pathstr) {
                Ok(_) => Ok(()),
                Err(err) => {
                    println!("problem writing file {}", err);
                    Ok(())
                }
            }
        } else {
            self.mkdir(splitted_path);
            Ok(())
        }
    }

    fn mkdir(&self, splitted_path: Vec<String>) {
        let whole_path = splitted_path.join("/");
        println!("making dir {}", whole_path);

        match self.client.mkcol(&splitted_path) {
            Err(err) => {
                println!("problem making directory {} {}", whole_path, err);
                return;
            }
            Ok(_) => return,
        }
    }
}

fn split_pathbuf(dir_to_watch: &PathBuf, incoming_path: PathBuf, out: &mut Vec<String>) {
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
                out.push(p)
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
        split_pathbuf(&dir_to_watch, incoming_path, &mut splitted_path);
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
