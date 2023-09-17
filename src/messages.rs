use hmac::{Hmac, Mac};
use sha2::Sha256;

use serde::Serialize;
// enum MessageType {
//     ExecuteRequest,
// }
//
pub fn message_to_bytes(message: Message, hmac_key: &str, zmq_identity: Vec<u8>) -> Vec<u8> {
    // byte stream builder
    let mut message_bytes = Vec::new();

    let mut mac = Hmac::<Sha256>::new_from_slice(hmac_key.as_bytes()).unwrap();

    mac.update(serde_json::to_string(&message.header).unwrap().as_bytes());
    mac.update(
        serde_json::to_string(&message.parent_header)
            .unwrap()
            .as_bytes(),
    );
    mac.update(serde_json::to_string(&message.metadata).unwrap().as_bytes());
    mac.update(serde_json::to_string(&message.content).unwrap().as_bytes());
    let mac_signature = mac.finalize().into_bytes();
    let hex_signature: String = mac_signature.iter().map(|b| format!("{:02x}", b)).collect();

    message_bytes.extend_from_slice(zmq_identity.as_slice());
    // message_bytes.push(b',');
    message_bytes.extend_from_slice("<IDS|MSG>".as_bytes());
    // message_bytes.push(b',');
    message_bytes.extend_from_slice(hex_signature.as_bytes());
    // message_bytes.push(b',');
    message_bytes.extend_from_slice(serde_json::to_string(&message.header).unwrap().as_bytes());
    // message_bytes.push(b',');

    if let Some(parent_header) = message.parent_header {
        message_bytes.extend_from_slice(serde_json::to_string(&parent_header).unwrap().as_bytes());
    }
    // message_bytes.push(b',');
    // message_bytes.push(b',');

    message_bytes.extend_from_slice(serde_json::to_string(&message.metadata).unwrap().as_bytes());
    message_bytes.extend_from_slice(serde_json::to_string(&message.content).unwrap().as_bytes());

    // return message_bytes;
    return "".as_bytes().to_vec();
}

#[derive(Debug, Serialize)]
pub struct MessageHeader {
    pub msg_id: String,
    pub session: String,
    pub username: String,
    pub date: String,
    pub msg_type: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub header: MessageHeader, // You can use appropriate types for keys and value
    pub parent_header: Option<MessageHeader>,
    pub metadata: (),
    pub content: ContentType,
    pub buffers: Vec<String>, // Assuming "buffers" contains strings
}

#[derive(Debug, Serialize)]
pub struct ExecuteRequestContent {
    pub code: String,
    pub silent: bool,
    pub store_history: bool,
    pub user_expressions: (),
    pub allow_stdin: bool,
    pub stop_on_error: bool,
}

#[derive(Debug, Serialize)]
pub enum ContentType {
    ExecuteRequest(ExecuteRequestContent),
}
