use std::{io, net::TcpStream};

use anyhow::Result;
use sha1::digest::typenum::bit;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::unix::pipe::Receiver;

use crate::handshake::{deserialize_handshake, Handshake};
use crate::tracker::Peer;

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
#[derive(Clone, Debug)]
pub struct NetworkedPeer {
    peer: Peer,
    /// changed by Have messages and bitfields , bitfields might be set as "don't have" for some clients but later
    /// sent as Have messages
    have_pieces: Vec<bool>,
    ignore: bool,
    handshake_response: Handshake,
}

impl NetworkedPeer {
    pub async fn new(peer: Peer, handshake_bytes: &Vec<u8>) -> Result<Self> {
        // This is blocking code for now but will be converted to async

        // this is dumb , change it later
        let addr = format!("{}:{}", peer.ip, peer.port);

        let mut peer_connection = tokio::net::TcpStream::connect(addr).await?;
        peer_connection.write_all(&handshake_bytes).await?;

        // try to read the handshake , it is always 68 bytes long
        let mut buffer = [0u8; 68];

        peer_connection.read_exact(&mut buffer).await?;

        let handshake = deserialize_handshake(buffer)?;

        let networked_peer = NetworkedPeer {
            peer,                // TODO remove the clone
            have_pieces: vec![], // TODO,
            ignore: false,       // TODO,
            handshake_response: handshake,
        };

        Ok(networked_peer)
    }
}

struct NetworkCommand {}
enum Command {
    Send { bytes: Vec<u8> },
    Receive { bytes: Vec<u8> },
}
