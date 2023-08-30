use jupyter_client::Client;
use std::time;
use std::fs::File;
use std::thread;
mod cell;

const KERNEL_NO: &str = "922269";

fn get_kernel_file() -> File {
    let path = "/home/krzys/.local/share/jupyter/runtime/kernel-".to_owned() + KERNEL_NO + ".json";
    return File::open(path).unwrap();
}

fn main() {
    let file = get_kernel_file();
    let client = Client::from_reader(file).unwrap();

    let io_receiver = client.iopub_subscribe().unwrap();
    thread::spawn(move || {
        for msg in io_receiver {
            println!("Received message from kernel: {:#?}", msg);

            // println!("Received message from kernel");
        }
    });

    let c = cell::Cell::new("410 + 10".to_owned());
    c.execute(&client);

    // Set up the heartbeat watcher
    // let hb_receiver = client.heartbeat();
    // thread::spawn(move || {
    //     for _ in hb_receiver {
    //         println!("Received heartbeat from kernel");
    //     }
    // });
    // let command = Command::Execute {
    //     code: "3+8".to_string(),
    //     silent: false,
    //     store_history: true,
    //     user_expressions: HashMap::new(),
    //     allow_stdin: true,
    //     stop_on_error: false,
    // };

    // // Run some code on the kernel
    // let response = client.send_shell_command(command);
    // match response {
    //     Ok(_r) => {
    //         // println!("{:?}", r);
    //         // match r {
    //         //     Response::Shell(m) => println!("{:?}", m),
    //         //     Response::IoPub(m) => println!("{:?}", m),
    //         // }
    //     }
    //     Err(e) => {
    //         println!("{}", e)
    //     }
    // }
    thread::sleep(time::Duration::new(1, 0));
    // }
}
