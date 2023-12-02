use anyhow::{Ok, Result};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::blocking::Client as ReqwestClient;
use std::{collections::HashMap, future::Future, process::Output};

use crate::{
    connection::{self, Handshake},
    torrent_file::TorrentFile,
    tracker::{self, TrackerRequest, TrackerResponse},
};

// remove the pubs laters
/// Various values created when the client first starts,
pub struct Client {
    // string is an identifier for unique torrent, something like a sha256 hash
    pub torrents: HashMap<String, Torrent>,
    pub peer_id: [u8; 20],
    pub reqwest: ReqwestClient,
    // Max number of operations should be configurable the default for qBittorent is 100 per torrent and
    // 500 in total we will want to get the connections per torrent using
    //
    //
    //
    // if torrents.len() > 5 {
    //     connections = MAX_CONNECTIONS / torrents.len()
    // }
    // else {
    //     connections = MAX_PER_TORRENT
    // }
    //
    //
    //
    //
}

// remove the pubs later
pub struct Torrent {
    pub torrent_file: TorrentFile,
    pub tracker_response: TrackerResponse,
    pub tracker_request: TrackerRequest,
    pub handshake: Vec<u8>,
    // peers: Vec<NetworkedPeer>
}

/// TODO
/// This should try to connect all peers and do the initial handshake, removing peers from tracker_request.get_peers()
/// adding them to networked peers
impl Torrent {
    fn new(torrent_file: TorrentFile, peer_id: [u8; 20], reqwest: &ReqwestClient) -> Result<Self> {
        let tracker_request = TrackerRequest::new(&torrent_file, peer_id)?;
        let tracker_response = tracker_request.request(&torrent_file, reqwest)?;
        let handshake = connection::Handshake::new_buf(peer_id, tracker_request.info_hash);
        //let peers = todo!();
        //let networked_peers = todo()!;

        Ok(Self {
            torrent_file,
            tracker_response,
            tracker_request,
            handshake,
            // peers,
            // networked_peers
        })
    }
}

impl Client {
    pub fn new() -> Result<Self> {
        // Peer id
        let version = "0000";
        let client_id = "ct";
        let random: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        let id = format!("-{client_id}{version}-{random}");
        let peer_id = id.as_bytes().try_into()?;
        let reqwest = ReqwestClient::new();

        Ok(Client {
            torrents: HashMap::new(),
            peer_id,
            reqwest,
        })
    }

    pub fn add_torrent(&mut self, file: TorrentFile) -> Result<()> {
        let torrent = Torrent::new(file, self.peer_id, &self.reqwest)?;
        let name = torrent.torrent_file.info.name.clone();

        let torrent = self.torrents.insert(name, torrent);
        Ok(())
    }
}
