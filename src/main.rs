mod fswatch;
mod webdav;

fn main() {
    match fswatch::run("/Users/tweissin/deletemesoon") {
        Err(err) => panic!("Problem watching: {:?}", err),
        _ => (),
    }
}
