use std::time::Duration;

use ntest::timeout;
use peers_core::connection::Command;
use peers_core::{self, client::Client, connection, torrent_file::TorrentFile};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;

#[cfg(not(feature = "no_p2p_tests"))]
#[tokio::test]
#[timeout(30000)]
async fn test_receive_handshake() {
    use tokio::sync::mpsc::Receiver;

    let mut client = Client::new().unwrap();
    let file = TorrentFile::from_file("torrents/debian-12.2.0-amd64-netinst.iso.torrent").unwrap();

    client.add_torrent(file).await.unwrap();

    for (_key, torrent) in client.torrents.iter() {
        let handshake = &torrent.handshake;
        let peers = torrent.tracker_response.get_peers().unwrap();

        for peer in peers {
            dbg!("sent");

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
