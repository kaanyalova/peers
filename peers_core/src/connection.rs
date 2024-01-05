use std::vec;

use anyhow::{Error, Ok, Result};
use futures::channel::mpsc::Sender;
use futures::channel::oneshot;
use sha1::digest::typenum::bit;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver};

use crate::handshake::{self, Handshake};
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

enum Message {
    KeepAliveMessage,
    ChokeMessage,
    UnchokeMessage,
    InterestedMessage,
    NotInterestedMessage,
    HaveMessage(HaveMessageData),
    BitFieldMessage,
    PieceMessage,
    CancelMessage,
    PortMessage,
}

pub struct KeepAliveMessage;
pub struct ChokeMessage;
pub struct UnchokeMessage;
pub struct InterestedMessage;
pub struct NotInterestedMessage;
pub struct HaveMessageData {
    have: Option<u32>,
}

pub struct BitFieldMessage;
pub struct RequestMessage;
pub struct PieceMessage;
pub struct CancelMessage;
pub struct PortMessage;

pub fn new_base_message(length_prefix: u32, id: u8) -> Vec<u8> {
    let mut buffer = vec![];
    buffer.extend(length_prefix.to_be_bytes());
    buffer.push(id);
    buffer
}

impl KeepAliveMessage {
    fn new_buffer() -> Vec<u8> {
        vec![0, 0, 0, 0]
    }
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

impl HaveMessageData {
    pub fn new_buffer(index: u64) -> Vec<u8> {
        let mut buffer = new_base_message(1, 5);
        buffer.extend(index.to_be_bytes());
        buffer
    }

    pub fn new(have: Vec<u8>) -> Self {
        let have_parsed = u32::from_be_bytes(have.try_into().unwrap()); // this is temporary remove the unwrap
        Self {
            have: Some(have_parsed),
        }
    }

    pub fn empty() -> Self {
        Self { have: None }
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

struct MessageToParse {
    bytes_to_parse: Option<u32>,
    message: Message,
}

impl MessageToParse {
    fn new(len: u32, message: Message) -> MessageToParse {
        // if we have more than 2 bytes we need to parse the rest
        let bytes_to_parse = {
            if len <= 2 {
                None
            } else {
                Some(len - 2)
            }
        };
        MessageToParse {
            bytes_to_parse,
            message,
        }
    }
}

fn read_message_first(len: u32, id: Option<u8>) -> Result<MessageToParse> {
    match (len, id) {
        (0, None) => Ok(MessageToParse::new(len, Message::KeepAliveMessage)),
        (1, Some(0)) => Ok(MessageToParse::new(len, Message::ChokeMessage)),
        (1, Some(1)) => Ok(MessageToParse::new(len, Message::UnchokeMessage)),
        (1, Some(2)) => Ok(MessageToParse::new(len, Message::InterestedMessage)),
        (1, Some(3)) => Ok(MessageToParse::new(len, Message::NotInterestedMessage)),
        (1, Some(4)) => Ok(MessageToParse::new(
            len,
            Message::HaveMessage(HaveMessageData::empty()),
        )),

        _ => Err(Error::msg(
            "this message isn't implemented or doesn't exist",
        )),
    }
}

/// only for messages with data
fn read_message_final(bytes: Vec<u8>, message: Message) -> Message {
    match message {
        Message::HaveMessage(_) => Message::HaveMessage(HaveMessageData::new(bytes)),
        _ => todo!(),
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
    pub handshake_response: Handshake,
}

impl NetworkedPeer {
    pub async fn new(peer: Peer, handshake_bytes: &Vec<u8>) -> Result<NetworkedPeer> {
        let (handshake_tx, handshake_rx) = oneshot::channel();
        //let (send_cmd_tx, send_cmd_rx) = mpsc::channel(10); // is 10 a good value??
        //let (recv_cmd_tx, recv_cmd_rx) = mpsc::channel(10); // is 10 a good value??

        let handshake_bytes_new = handshake_bytes.clone();
        tokio::spawn(async move {
            // this is dumb , change it later
            let addr = format!("{}:{}", peer.ip, peer.port);

            let mut peer_connection = tokio::net::TcpStream::connect(addr).await.unwrap();
            peer_connection
                .write_all(&handshake_bytes_new)
                .await
                .unwrap();

            // try to read the handshake , it is always 68 bytes long
            let mut buffer = [0u8; 68];

            peer_connection.read_exact(&mut buffer).await.unwrap();

            let handshake = Handshake::parse_handshake(&buffer);

            dbg!(&handshake);

            handshake_tx.send(handshake).unwrap();

            let mut len_buffer = [0u8; 4];

            let mut is_choked = true;

            loop {
                // read the length, it is 4 bytes
                let len = peer_connection.read_u32().await.unwrap();

                // read the message id
                let message_id = peer_connection.read_u8().await.ok();

                let read_first = read_message_first(len, message_id).unwrap();

                if let Some(bytes_to_parse) = read_first.bytes_to_parse {
                    let mut rest_of_the_message_buffer = vec![0u8; bytes_to_parse as usize];
                    peer_connection
                        .read_exact(&mut rest_of_the_message_buffer)
                        .await
                        .unwrap();

                    let msg_final =
                        read_message_final(rest_of_the_message_buffer, read_first.message);

                    // some_channel.send(msg_final)
                } else {
                    // TODO
                    // just send the message, it doesn't have any data
                    // some_channel.send(read_first.message)
                }
            }
        });

        let networked_peer = NetworkedPeer {
            peer,
            have_pieces: vec![], // TODO,
            ignore: false,       // TODO,
            handshake_response: handshake_rx.await??,
        };

        Ok(networked_peer)
    }
}

struct NetworkCommand {}
pub enum Command {
    Send { bytes: Vec<u8> },
    Receive { bytes: Vec<u8> },
}
