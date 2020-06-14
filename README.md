# WebDAV sync

This application can be used to synchronize files from your local filesystem to a remote WebDAV server.

## Prerequisites

* MacOS
* Rust
* (optional) Docker - for testing/debugging

## Build and run

Build it

```
cargo build --release
```

Run it and it will prompt you for all parameters.

```
./target/release/webdav-sync 
```

You may otionally cnfigure your `.bash_profile`/`.zprofile` and restart your shell so it doesn't ask you for every parameter.

```
export WEBDAV_HOSTNAME=YOUR_HOSTNAME
export WEBDAV_USERNAME=YOUR_USERNAME
export WEBDAV_PASSWORD=YOUR_PASSWORD
```

You can specify all options at the command-line (not recommended because of shell history):

```
./target/release/webdav-sync -h YOUR_HOSTNAME -d /tmp/dir -u YOUR_USERNAME -p YOUR_PASSWORD 
```

Note:
* YOUR_HOSTNAME should be an HTTP URL string. For instance http://192.168.1.2

## Limitations

* Only creates and writes are watched. Renames, moves, and deletes are not supported.
* Only syncs from local filesystem to WebDav server (one-way sync, not bi-directional).
* The client is using Basic auth credentials. Digest credentials are not supported at this time.
* Could use Apple's Keychain services for credentials. See [this link](https://crates.io/crates/keychain-services).
* No logging. All output goes to stdout.

## Local testing

You can install a WebDav server locally to test.

1. Make a directory to serve your files. For instance, `/tmp/webdav/data`. Add a file or two there for testing.
2. Start up your WebDAV server in a Docker container.
```shell
docker run --restart always -v /tmp/webdav:/var/lib/dav \
    -e AUTH_TYPE=Basic -e USERNAME=YOUR_USERNAME -e PASSWORD=YOUR_PASSWORD \
    --publish 80:80 -d bytemark/webdav
```
3. In a web browser, try connecting and make sure your credentials work and that you see your files: http://localhost

You should be set to try it.

## Testing

Limited unit tests are available.

```
cargo test
```

## References

* https://hub.docker.com/r/bytemark/webdav/
* https://docs.rs/hyperdav/0.2.0/hyperdav/
* https://docs.rs/notify/4.0.15/notify/
* https://github.com/seanmonstar/reqwest/issues/483
