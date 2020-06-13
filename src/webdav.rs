use hyperdav::Client;
use std::env;

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
