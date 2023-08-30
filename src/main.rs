// extern crate serde;
// extern crate serde_json;

use serde::Deserialize;
use serde_json;
use std::fs::File;
use zmq;

//TODO: get it automatically
const KERNEL_NO: &str = "277756";

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
        let zmq_socket = self.zmq_context.socket(zmq::SUB).expect("Failed to create shell socket");
        let shell_port = self.kernel_data.shell_port;
        let ip = &self.kernel_data.ip;
        let address = format!("tcp://{}:{}", ip, shell_port);
        zmq_socket.connect(&address).expect("Cannot connect to the kernel shell");
        zmq_socket.set_subscribe(b"").expect("Cannot subscribe to the kernel shell");
        return ShellSubscriber {
            // zmq_context: self.zmq_context,
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
    println!("{:#?}", kernel_data);
    let client = Client::new(kernel_data);
    let _shell_subscriber = client.subscribe_to_shell();
}
