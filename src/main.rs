// extern crate serde;
// extern crate serde_json;

use serde::Deserialize;
use serde_json;
use std::fs::File;
use uuid::Uuid;
use zmq;


mod cell;
mod messages;
use std::thread;
use cell::Cell;
use chrono::{DateTime, Utc}; // 0.4.15
use std::time::SystemTime;

//TODO: get it automatically
const KERNEL_NO: &str = "78269";

#[derive(Deserialize, Debug)]
struct KernelData {
    shell_port: u16,
    iopub_port: u16,
    stdin_port: u16,
    control_port: u16,
    hb_port: u16,
    ip: String,
    key: String,
    transport: String,
    signature_scheme: String,
    kernel_name: String,
}

impl KernelData {
    fn new() -> KernelData {
        let file = get_kernel_file();
        let kernel_data: KernelData = serde_json::from_reader(file).unwrap();
        return kernel_data;
    }
}

struct ShellSubscriber {
    // zmq_context: zmq::Context,
    zmq_socket: zmq::Socket,
    hmac_key: String,
}

impl ShellSubscriber {
    fn send_cell(&self, cell: &Cell) {
        let code = cell.code.clone();
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        let zmq_identity = self
            .zmq_socket
            .get_identity()
            .expect("Cannot get zmq identity");

        let message = messages::Message {
            header: messages::MessageHeader {
                msg_id: Uuid::new_v4().to_string(),
                session: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")
                    .unwrap()
                    .to_string(),
                username: "".to_owned(),
                date: now,
                msg_type: "execute_request".to_owned(),
                version: "5.0".to_owned(),
            },
            parent_header: None,
            metadata: (),
            content: messages::ContentType::ExecuteRequest({
                messages::ExecuteRequestContent {
                    code: code,
                    silent: false,
                    store_history: true,
                    user_expressions: (),
                    allow_stdin: false,
                    stop_on_error: false,
                }
            }),
            buffers: vec![],
        };

        // self.zmq_socket
        //     .send(&serde_json::to_string(&message).unwrap().as_bytes(), 0)
        //     .expect("Cannot send message");
        //
        let message_bytes =
            messages::message_to_bytes(message, &self.hmac_key.to_owned(), zmq_identity);
        let message_str = String::from_utf8(message_bytes.to_owned()).unwrap();
        self.zmq_socket
            .send(&message_str, 0)
            .expect("Cannot send message");
        // println!("{}", String::from_utf8(messages::message_to_bytes(message, &self.hmac_key.to_owned(), zmq_identity)).unwrap());
        println!("Sent: {}", message_str);
        let reply = self.zmq_socket.recv_string(0).unwrap().unwrap();
        println!("Received reply");
        println!("{}", reply);
    }
}

struct IOPubSubscriber {
    zmq_socket: zmq::Socket,
    hmac_key: String,
}

struct Client {
    kernel_data: KernelData,
    zmq_context: zmq::Context,
    // zmq_socket: zmq::Socket,
}

impl Client {
    fn new(kernel_data: KernelData) -> Client {
        let zmq_context = zmq::Context::new();
        return Client {
            kernel_data,
            zmq_context,
        };
    }

    fn subscribe_to_shell(&self) -> ShellSubscriber {
        let zmq_socket = self
            .zmq_context
            .socket(zmq::REQ)
            .expect("Failed to create shell socket");
        let shell_port = self.kernel_data.shell_port;
        let ip = &self.kernel_data.ip;
        let kernel_shell_adress = format!("tcp://{}:{}", ip, shell_port);

        zmq_socket
            .connect(&kernel_shell_adress)
            .expect("Cannot connect to the kernel shell");
        // zmq_socket
        //     .set_subscribe(b"")
        //     .expect("Cannot subscribe to the kernel shell");

        return ShellSubscriber {
            // zmq_context: self.zmq_context,
            hmac_key: self.kernel_data.key.clone(),
            zmq_socket,
        };
    }

    fn subscribe_to_iopub(&self) -> IOPubSubscriber {
        let zmq_socket = self
            .zmq_context
            .socket(zmq::SUB)
            .expect("Failed to create iopub socket");
        let iopub_port = self.kernel_data.iopub_port;
        let ip = &self.kernel_data.ip;
        let kernel_iopub_adress = format!("tcp://{}:{}", ip, iopub_port);

        zmq_socket
            .connect(&kernel_iopub_adress)
            .expect("Cannot connect to the kernel iopub");

        zmq_socket
            .set_subscribe(b"")
            .expect("Cannot subscribe to the kernel iopub");

        return IOPubSubscriber {
            hmac_key: self.kernel_data.key.clone(),
            zmq_socket,
        };
    }
}

fn get_kernel_file() -> File {
    let path = "/home/krzys/.local/share/jupyter/runtime/kernel-".to_owned() + KERNEL_NO + ".json";
    return File::open(path).unwrap();
}

fn main() {
    let kernel_data = KernelData::new();
    // println!("{:#?}", kernel_data);
    let client = Client::new(kernel_data);
    let shell_subscriber = client.subscribe_to_shell();
    let iopub_subscriber = client.subscribe_to_iopub();

    thread::spawn(move || {
        loop {
            println!("Waiting for reply");
            let reply = iopub_subscriber.zmq_socket.recv_string(0).unwrap().unwrap();
            println!("Received reply");
            println!("{}", reply);
        }
    });

    let cell = Cell::new("print('Hello World!')".to_owned());
    shell_subscriber.send_cell(&cell);

    thread::sleep(std::time::Duration::from_secs(10));
}
