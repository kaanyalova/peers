use anyhow::Result;
use thiserror::Error;

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
        buffer.extend("BitTorrent protocol".as_bytes());
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
    #[error("you fucked up")]
    ConversionError,
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
        pstr: "BitTorrent Protocol".to_string(),
        reserved: [0; 8],
        info_hash: converted_info_hash,
        peer_id: converted_peer_id,
    };

    Ok(handshake)
}
