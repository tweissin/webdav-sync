# WebDAV sync

## Prerequisites

* Docker
* MacOS
* Rust

## Getting started

1. Make a directory to serve your files. For instance, `/tmp/webdav/data`. Add a file or two there for testing.
2. Start up your WebDAV server in a Docker container.
```shell
docker run --restart always -v /tmp/webdav:/var/lib/dav \
    -e AUTH_TYPE=Basic -e USERNAME=YOUR_USERNAME -e PASSWORD=YOUR_PASSWORD \
    --publish 80:80 -d bytemark/webdav
```
3. In a web browser, try connecting and make sure your credentials work and that you see your files: http://localhost
4. Run the Rust client application.
  * In VSCode
  * or, `WEBDAV_USERNAME=YOUR_USERNAME WEBDAV_PASSWORD=YOUR_PASSWORD cargo run`

```rust
    let client = Client::new()
        .credentials("YOUR_USERNAME", "YOUR_PASSWORD")
        .build("http://192.168.1.2/")
        .unwrap();
```

## Build and run

Build it

```
cargo build --release
```

Run it

```
export WEBDAV_HOSTNAME=YOUR_HOSTNAME
export WEBDAV_USERNAME=YOUR_USERNAME
export WEBDAV_PASSWORD=YOUR_PASSWORD
./target/release/webdav-sync -d /tmp/dir
```

Or you can specify all options at the command-line:

```
./target/debug/webdav-sync -h YOUR_HOSTNAME -d /tmp/dir -u YOUR_USERNAME -p YOUR_PASSWORD 
```

Note:
* YOUR_HOSTNAME should be an HTTP URL string. For instance http://192.168.1.2

## Limintations

* The client is using Basic auth credentials. Digest credentials are not supported at this time.

## References

* https://hub.docker.com/r/bytemark/webdav/
* https://docs.rs/hyperdav/0.2.0/hyperdav/
* https://docs.rs/notify/4.0.15/notify/
* https://github.com/seanmonstar/reqwest/issues/483
