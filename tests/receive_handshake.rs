use std::time::Duration;

use ntest::timeout;
use peers_core::{self, client::Client, connection, torrent_file::TorrentFile};
use tokio::time::timeout;

#[tokio::test]
#[timeout(30000)]
async fn test_receive_handshake() {
    let mut client = Client::new().unwrap();
    let file = TorrentFile::from_file("stuff/debian-12.2.0-amd64-netinst.iso.torrent").unwrap();

    client.add_torrent(file).await.unwrap();

    for (_key, torrent) in client.torrents.iter() {
        let handshake = &torrent.handshake;
        let peers = torrent.tracker_response.get_peers().unwrap();

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

            let future = connection::NetworkedPeer::new(peer, handshake);

            let timeout = timeout(Duration::from_secs(5), future).await;

            // Timeout
            if let Ok(unwrapped_timeout) = timeout {
                // Network Error
                if let Ok(res) = unwrapped_timeout {
                    dbg!(res);
                    return;
                } else if let Err(e) = unwrapped_timeout {
                    dbg!("network error?");
                    dbg!(e);
                }
            } else {
                dbg!("timed out");
            }
        }
    }
}
