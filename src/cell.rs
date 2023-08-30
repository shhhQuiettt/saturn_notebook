use jupyter_client::Client;
use jupyter_client::responses::Response;
use jupyter_client::commands::Command;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Cell {
    code: String,
}

impl Cell {
    pub fn new(code: String) -> Self {
        return Self {code};
    }
    pub fn execute(&self, client: &Client) {
        let command = Command::Execute{
        code: self.code.clone(),
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
        };

        let response = client.send_shell_command(command).unwrap();
        match response {
            Response::Shell(res) => {
                println!("Shell response: {:#?}", res);
            }
            Response::IoPub(_) => {
                panic!("IoPub in Shell response?")
            }
        }
    }
}
