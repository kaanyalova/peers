use std::{
    io::{Cursor, Read},
    net::{IpAddr, Ipv4Addr},
    str::from_utf8,
};

use anyhow::{Error, Result};
use byteorder::{BigEndian, ReadBytesExt};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};

use crate::torrent_file::TorrentFile;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackerRequest {
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
    pub port: u64,
    pub uploaded: u64,
    pub downloaded: u64,
    pub compact: u8,
    pub left: u64,
}

impl TrackerRequest {
    pub fn new(t: &TorrentFile, peer_id: [u8; 20]) -> Result<Self> {
        // Generate the infohash
        let info = &t.info;
        let bencoded_info = serde_bencode::to_bytes(&info)?;

        let mut hasher = Sha1::new();
        hasher.update(&bencoded_info);
        let hash = hasher.finalize();

        let info_hash: [u8; 20] = hash.into();

        Ok(Self {
            info_hash,
            peer_id,
            port: 1337,
            uploaded: 0,
            downloaded: 0,
            compact: 0,
            left: 0,
        })
    }

    pub fn request(&self, torrent: &TorrentFile, reqwest: &Client) -> Result<TrackerResponse> {
        // i cannot use reqwest::get().query()... because serde_urlencoding doesn't support serializing bytes
        // https://github.com/nox/serde_urlencoded/issues/104

        let url = format!(
            "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact=1",
            torrent.announce,
            urlencoding::encode_binary(&self.info_hash),
            urlencoding::encode_binary(&self.peer_id),
            &self.port,
            &self.uploaded,
            &self.downloaded,
            &self.left,
        );

        let request = reqwest.get(url).send()?.bytes()?;
        let response: TrackerResponse = serde_bencode::from_bytes(&request)?;

        Ok(response)
    }
}

fn get_ufloaded() -> u64 {
    return 0;
}

/*

fn get_url(&self) -> String {
    format!(
        "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact=1",
        &self.
        urlencoding::encode_binary(&torrent.tracker_request.info_hash),
        urlencoding::encode_binary(&torrent.tracker_request.peer_id),
        torrent.tracker_request.port,
        torrent.tracker_request.uploaded,
        torrent.tracker_request.downloaded,
        torrent.tracker_request.left
    );
}


*/

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerResponse {
    // TODO implement optional stuff
    #[serde(rename = "failure reason")]
    failure_reason: Option<String>,
    interval: i64,
    #[serde(rename = "tracker id")]
    tracker_id: Option<String>,
    complete: Option<i64>,
    incomplete: Option<i64>,
    peers: ByteBuf,
}

#[derive(Debug)]
// remove the pub fields later
pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl TrackerResponse {
    pub fn get_peers(&self) -> Result<Vec<Peer>> {
        let mut peers: Vec<Peer> = Vec::new();

        // there is probably a more efficient way to do this
        let bytes = &self.peers.as_slice();

        // check if the bytes are divisible by 6 and valid
        if bytes.len() % 6 != 0 {
            return Err(Error::msg("invalid peer data"));
        }

        for peer in bytes.chunks_exact(6) {
            // wtf is going on with rust analyzer???? it stopped working
            let mut ip_address_bytes: &[u8] = &peer[0..4];
            let ip_raw = ip_address_bytes.read_u32::<BigEndian>()?;

            let ip = Ipv4Addr::from(ip_raw);

            let mut port_bytes: &[u8] = &peer[4..6];
            let port = port_bytes.read_u16::<BigEndian>()?;

            let new_peer = Peer { ip, port };
            peers.push(new_peer);
        }

        Ok(peers)
    }
}
