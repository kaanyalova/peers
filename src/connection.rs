struct Handshake;

impl Handshake {
    fn new(peer_id: [u8; 20], info_hash: [u8; 20]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(19);
        buffer.extend("BitTorrent protocol".as_bytes());
        buffer.extend([0u8; 8]);
        buffer.extend(info_hash);
        buffer.extend(peer_id);
        buffer
    }
}

trait SendMessage {
    fn send() -> [u8];
}
