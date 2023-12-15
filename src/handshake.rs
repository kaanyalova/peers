use anyhow::{Ok, Result};
use serde::{
    de::{DeserializeOwned, Visitor},
    Deserialize, Serialize,
};
use thiserror::Error;

use crate::consts::{self, BITTORRENT_PROTOCOL, PSTRLEN};

#[derive(Debug, Clone)]
pub struct Handshake {
    pstrlen: u8,
    pstr: String,
    features: [u8; 8],
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
    #[error("{0}")]
    NonDefaultHandshakeValue(String),
    #[error("{0}")]
    EmptyField(String),
    #[error("{0}")]
    InputWrongSize(String),
    #[error("{0}")]
    ConversionToSizedError(String),
}

impl Handshake {
    pub fn parse_handshake(input: &[u8]) -> Result<Handshake> {
        if input.len() != 68 {
            return Err(HandshakeError::InputWrongSize(format!(
                "the size for Handshake must be 68 bytes, it was {}",
                input.len()
            )))?;
        }

        let pstrlen = &input[0];
        let pstr = &input[1..20];
        let features = &input[20..28];
        let info_hash = &input[28..48];
        let peer_id = &input[48..];

        // check predetermined values

        if pstrlen != &(PSTRLEN as u8) {
            return Err(HandshakeError::NonDefaultHandshakeValue(format!(
                r#"pstrlen must be {}, it was "{}""#,
                PSTRLEN, pstrlen
            )))?;
        }

        if pstr != BITTORRENT_PROTOCOL.as_bytes() {
            return Err(HandshakeError::NonDefaultHandshakeValue(format!(
                r#"pstr must be "{}" it was "{}""#,
                BITTORRENT_PROTOCOL,
                String::from_utf8_lossy(pstr)
            )))?;
        }

        // check if info hash or peer_id is empty
        if info_hash == &[0u8; 20] {
            return Err(HandshakeError::EmptyField("empty info hash".into()))?;
        }

        if peer_id == &[0u8; 20] {
            return Err(HandshakeError::EmptyField("empty peer id".into()))?;
        }

        let features_sized: [u8;8] = features.try_into()
        .map_err(|_| HandshakeError::ConversionToSizedError("cannot convert 'features' to sized byte array , something really went wrong if you are seeing this".into()))?;

        let info_hash_sized: [u8;20] = info_hash.try_into()
        .map_err(|_| HandshakeError::ConversionToSizedError("cannot convert 'info_hash' to sized byte array ,something really went wrong if you are seeing this".into()))?;

        let peer_id_sized: [u8; 20] = peer_id.try_into()
        .map_err(|_| HandshakeError::ConversionToSizedError("cannot convert 'peer_id' to sized array, something really went wrong if you are seeing this".into()))? ;

        let handshake = Handshake {
            pstrlen: 19,
            pstr: BITTORRENT_PROTOCOL.to_string(),
            features: features_sized,
            info_hash: info_hash_sized,
            peer_id: peer_id_sized,
        };

        Ok(handshake)
    }
}
