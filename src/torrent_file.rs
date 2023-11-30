use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::{
    fs::read,
    io::{self, Read},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentFile {
    pub info: Info,
    pub announce: String,
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<i64>,
    pub comment: Option<String>,
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
    pub encoding: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct Info {
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub pieces: ByteBuf,
    pub private: Option<i64>,
    pub name: String,
    // SINGE FILE ONLY
    pub md5sum: Option<String>,
    pub length: Option<i64>,
    // MULTI FILE ONLY
    pub files: Option<Vec<File>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
    pub md5sum: Option<String>,
}

impl TorrentFile {
    pub fn from_byte_vec(file: Vec<u8>) -> Result<Self> {
        let torrent: TorrentFile = serde_bencode::from_bytes(&file)?;
        Ok(torrent)
    }
    pub fn from_file(path: &str) -> Result<Self> {
        let file = read(path)?;
        let torrent: TorrentFile = serde_bencode::from_bytes(&file)?;

        Ok(torrent)
    }
}
