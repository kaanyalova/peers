use anyhow::Result;
use serde::{de::Visitor, Serialize};
use thiserror::Error;

use crate::consts::{self, BITTORRENT_PROTOCOL, PSTRLEN};

#[derive(Debug, Clone)]
pub struct Handshake {
    pstrlen: u8,
    pstr: String,
    reserved: [u8; 8],
    info_hash: [u8; 20],
    peer_id: [u8; 20],
}

/// Might use serde for this, this looks dumb, i don't know
///
impl Handshake {
    pub fn new_buf(peer_id: [u8; 20], info_hash: [u8; 20]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(19);
        buffer.extend(consts::BITTORRENT_PROTOCOL.as_bytes());
        buffer.extend([0u8; 8]);
        buffer.extend(info_hash);
        buffer.extend(peer_id);
        buffer
    }
    pub fn serialize() {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("Handshake has non-standard values")]
    NonExpectedValue,
    #[error("wrong info hash")]
    WrongInfoHash,
    #[error("wrong peer id")]
    WrongPeerId,
    // surely won't happen
    #[error("")]
    ConversionError,
}

struct HandshakeVisitor;

impl<'de> Visitor<'de> for HandshakeVisitor {
    type Value = Handshake;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Vec<u8>")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> std::prelude::v1::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() != 68 {
            return Err(E::custom(format!(
                "the size for Handshake must be 68 bytes, it was {}",
                v.len()
            )));
        }

        let pstrlen = &v[0];
        let pstr = &v[1..20];
        let features = &v[20..28];
        let infohash = &v[28..48];
        let peer_id = &v[48..];

        // check predetermined values

        if pstrlen != &(PSTRLEN as u8) {
            return Err(E::custom(format!("pstrlen must be 19, it was {}", pstrlen)));
        }

        let expected_pstr = BITTORRENT_PROTOCOL.as_bytes();
        if pstr != expected_pstr {
            return Err(E::custom(format!(
                r#"pstr must be "BitTorrent Protocol" it was {} "#,
                String::from_utf8_lossy(pstr)
            )));
        }

        let handshake = Handshake {
            pstrlen: 19,
            pstr: BITTORRENT_PROTOCOL.to_string(),
            reserved: todo!(),
            info_hash: todo!(),
            peer_id: todo!(),
        };

        Ok(handshake)
    }
}

// TODO: use serde
pub fn deserialize_handshake(bytes: [u8; 68]) -> Result<Handshake> {
    // check the expected fields
    //if bytes[0] != 19 {
    //    return Err(HandshakeError::NonExpectedValue.into());
    //}

    //let str = &bytes[0..19];

    // TODO: fix this
    //if str != "BitTorrent protocol" as &[u8] {
    //    return Err(HandshakeError::NonExpectedValue.into());
    //}

    let info_hash = &bytes[38..58];
    let peer_id = &bytes[58..];

    dbg!(bytes);

    // this is so bad that anyhow refuses to process the error
    let converted_info_hash = info_hash
        .to_owned()
        .try_into()
        .map_err(|_| HandshakeError::ConversionError)?;
    let converted_peer_id = peer_id
        .to_owned()
        .try_into()
        .map_err(|_| HandshakeError::ConversionError)?;

    let handshake = Handshake {
        pstrlen: 19,
        pstr: BITTORRENT_PROTOCOL.to_string(),
        reserved: [0; 8],
        info_hash: converted_info_hash,
        peer_id: converted_peer_id,
    };

    Ok(handshake)
}
