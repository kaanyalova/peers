use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender};
use std::vec;

use client::Torrent;
use futures::executor::block_on;
use tracker::{Peer, TrackerRequest};
mod client;
mod connection;
mod handshake;
mod torrent_file;
mod tracker;

use crate::client::Client;
use crate::torrent_file::TorrentFile;

#[tokio::main]
async fn main() {
    let mut client = Client::new().unwrap();
    let file = TorrentFile::from_file("stuff/debian-12.2.0-amd64-netinst.iso.torrent").unwrap();

    client.add_torrent(file).await.unwrap();

    for (key, torrent) in client.torrents.iter() {
        let handshake = &torrent.handshake;
        let peers: &Vec<Peer> = &torrent.tracker_response.get_peers().unwrap();

        for peer in peers {
            // Networking code
            // Move this to connection.rs
            //
            // first "thing" received should be the handshake with a fixed size
            // terminate the connection if it isn't
            //
            // after that get the first byte of the stream the length, get the rest of the message as [u8; len]
            // serialize into a message struct
            dbg!("sent");
            let addr = format!("{}:{}", peer.ip, peer.port);

            let a = connection::NetworkedPeer::new(peer.clone(), handshake)
                .await
                .unwrap();

            dbg!(a);

            // dont ddos the tracker pls
            return;
        }
    }
}
