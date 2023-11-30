use tracker::TrackerRequest;
mod client;
mod connection;
mod torrent_file;
mod tracker;

use crate::client::Client;
use crate::torrent_file::TorrentFile;

fn main() {
    struct Test {
        f1: i32,
        f2: i32,
    }

    impl Test {
        fn new(f1: i32, f2: i32) -> Self {
            Test { f1, f2 }
        }
    }

    let str_1 = Test::new(1, 3);

    let refer = &str_1.f1;

    let derefer = *refer;

    let referenced = &str_1.f1;
    let owned = str_1.f1;
}
