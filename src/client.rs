use anyhow::{Ok, Result};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::blocking::Client as ReqwestClient;
use std::collections::HashMap;

use crate::{
    torrent_file::TorrentFile,
    tracker::{self, TrackerRequest, TrackerResponse},
};

pub struct Client {
    // string is an identifier for unique torrent, something like a sha256 hash
    torrents: HashMap<String, Torrent>,
    peer_id: [u8; 20],
    reqwest: ReqwestClient,
}

struct Torrent {
    torrent_file: TorrentFile,
    tracker_response: TrackerResponse,
    tracker_request: TrackerRequest,
    handshake: Vec<u8>,
    // peers: Vec<NetworkedPeer>
}

impl Torrent {
    fn new(torrent_file: TorrentFile, peer_id: [u8; 20], reqwest: &ReqwestClient) -> Result<Self> {
        let tracker_request = TrackerRequest::new(&torrent_file, peer_id)?;
        let tracker_response = tracker_request.request(&torrent_file, reqwest)?;
        let handshake = todo!();

        Ok(Self {
            torrent_file,
            tracker_response,
            tracker_request,
            handshake,
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

    fn add_torrent(mut self, file: TorrentFile) -> Result<Self> {
        let torrent = Torrent::new(file, self.peer_id, &self.reqwest)?;
        let name = torrent.torrent_file.info.name.clone();

        let torrent = self.torrents.insert(name, torrent);

        Ok(self)
    }

    // -> Torrent
    // iterate over torrents check stuff , update if necessary
}

// stuff like requests
struct TorrentFunctions;
