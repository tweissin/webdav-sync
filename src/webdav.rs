use hyperdav::Client;
use std::env;
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
        let copy = path_buf.to_path_buf();
        // self.mkdir(path_buf);
        self.make_directories_as_needed(path_buf);
        let pathstr = copy.clone().into_os_string().into_string().unwrap();
        match self.client.put(reader, &[pathstr]) {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("problem writing file {}", err);
                Ok(())
            }
        }
    }

    fn make_directories_as_needed(&self, path_buf: PathBuf) {
        let _whole_path = path_buf.as_path().to_str().unwrap();
        // let sub_path = whole_path.trim_start_matches(&self.dir_to_watch);

        let mut pb = PathBuf::new();
        pb.push("Users");
        self.mkdir(pb);

        let mut pb = PathBuf::new();
        pb.push("Users");
        pb.push("tweissin");
        self.mkdir(pb);

        let mut pb = PathBuf::new();
        pb.push("Users");
        pb.push("tweissin");
        pb.push("deletemesoon");
        self.mkdir(pb);
    }

    fn mkdir(&self, path_buf: PathBuf) {
        let whole_path = path_buf.as_path().to_str().unwrap();
        // how to split this into multiple "things"
        match self.client.mkcol(&[whole_path]) {
            Err(err) => {
                println!("problem making directory {}", err);
                return;
            }
            Ok(_) => return,
        }
    }
}

fn split_pathbuf(dir_to_watch: PathBuf, incoming_path: PathBuf, out: &mut Vec<String>) {
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
        split_pathbuf(dir_to_watch, incoming_path, &mut splitted_path);
        assert_eq!(3, splitted_path.len());
        assert_eq!("one", splitted_path[0]);
        assert_eq!("two", splitted_path[1]);
        assert_eq!("three.txt", splitted_path[2]);
    }
}

pub fn run() {
    let username = env::var("WEBDAV_USERNAME");
    let password = env::var("WEBDAV_PASSWORD");
    println!("{:?}", username);
    println!("{:?}", password);
    let client = Client::new()
        .credentials(username.unwrap(), password.unwrap())
        .build("http://192.168.1.15/")
        .unwrap();

    let resp = client.get(&["wow.txt"]);
    println!("{:?}", resp);
    let mut res = resp.unwrap();
    let mut buf = vec![];
    res.copy_to(&mut buf).unwrap();
    let string = std::str::from_utf8(&buf);
    println!("{}", string.unwrap());
}
