use std::net::TcpStream;

use anyhow::{Ok, Result};
use sha1::digest::typenum::bit;

use crate::tracker::Peer;

pub struct Handshake {
    pstrlen: u8,
    pstr: String,
    reserved: u8,
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

// Messages

/*
enum MessageType{
    KeepAliveMessage,
    ChokeMessage,
    UnchokeMessage,
    ...
}

struct Message{
    type: MessageType,
    id: u32,

}


bytes -> Message

*/

fn serialize_handshake() {}

/// bytes are <message ID><Payload>
/// without the length prefix, length is only used for networking to know   
fn read_message(bytes: &[u8]) -> impl Message {
    let id = bytes[0];
    match id {
        0 => KeepAliveMessage,
        // parse the bytes in the impl's of Messages not here
        // like this
        // n => HaveMessage::parse(&bytes),
        _ => todo!(),
    }
}

fn _alt_read_message(id: u8, bytes: Vec<u8>) -> _alt_Msg {
    match id {
        0 => _alt_Msg::KeepAlive(KeepAliveMessage),
        _ => todo!(),
    }
}

enum _alt_Msg {
    KeepAlive(KeepAliveMessage),
}

trait Message {}

pub struct KeepAliveMessage;
impl Message for KeepAliveMessage {}
pub struct ChokeMessage;
impl Message for ChokeMessage {}
pub struct UnchokeMessage;
impl Message for UnchokeMessage {}
pub struct InterestedMessage;
impl Message for InterestedMessage {}
pub struct NotInterestedMessage;
impl Message for NotInterestedMessage {}
pub struct HaveMessage;
impl Message for HaveMessage {}
pub struct BitFieldMessage;
impl Message for BitFieldMessage {}
pub struct RequestMessage;
impl Message for RequestMessage {}
pub struct PieceMessage;
impl Message for PieceMessage {}
pub struct CancelMessage;
impl Message for CancelMessage {}
pub struct PortMessage;
impl Message for PortMessage {}

pub fn new_base_message(length_prefix: u32, id: u8) -> Vec<u8> {
    let mut buffer = vec![];
    buffer.extend(length_prefix.to_be_bytes());
    buffer.push(id);
    buffer
}

impl ChokeMessage {
    fn new_buffer() -> Vec<u8> {
        new_base_message(1, 0)
    }
}

impl UnchokeMessage {
    fn new_buffer(bytes: Option<Vec<u8>>) -> Vec<u8> {
        new_base_message(1, 1)
    }
}

impl InterestedMessage {
    fn new_buffer(bytes: Option<Vec<u8>>) -> Vec<u8> {
        new_base_message(1, 2)
    }
}

impl NotInterestedMessage {
    fn new_buffer(bytes: Option<Vec<u8>>) -> Vec<u8> {
        new_base_message(1, 3)
    }
}

impl HaveMessage {
    pub fn new_buffer(index: u64) -> Vec<u8> {
        let mut buffer = new_base_message(1, 5);
        buffer.extend(index.to_be_bytes());
        buffer
    }
}

impl BitFieldMessage {
    fn new_buffer(bitfield: Vec<u8>) -> Vec<u8> {
        let length = bitfield.len() + 1;
        let mut buffer = new_base_message(length as u32, 5);
        buffer.extend(bitfield);
        buffer
    }
}

impl RequestMessage {
    fn new_buffer() {
        todo!()
    }
}

impl PieceMessage {
    fn new_buffer() {
        todo!()
    }
}

struct HandShakeResponse {}

/// Once the client tries to connect to peers they should be removed from the Peers vec, and added to here
struct NetworkedPeer {
    peer: Peer,
    /// changed by Have messages and bitfields , bitfields might be set as "don't have" for some clients but later
    /// sent as Have messages
    have_pieces: Vec<bool>,
    ignore: bool,
    handshake_response: Handshake,
}

impl NetworkedPeer {
    fn new(peer: Peer) -> Result<Self> {
        // Do the initial handshake here
        // This is blocking code for now but will be converted to async

        // this is dumb , change it later
        let addr = format!("{}:{}", peer.ip, peer.port);
        TcpStream::connect(addr);

        let networked_peer = NetworkedPeer {
            peer,
            have_pieces: todo!(),
            ignore: todo!(),
            handshake_response: todo!(),
        };

        Ok(networked_peer);
    }
}
