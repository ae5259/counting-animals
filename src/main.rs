mod server;

use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::net::TcpStream;

mod test;

#[derive(Debug, Serialize, Deserialize)]
struct RequestItem {
    socket_address: String,
    request: Vec<u8>,
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(num) => println!("Read {} bytes from stdin", num),
        Err(error) => println!("Error occurred: {}", error),
    };

    let parsed_data = match serde_json::from_str::<Vec<RequestItem>>(&input) {
        Ok(requests) => requests,
        Err(err) => {
            println!("Error occured while reading file: {}", err);

            vec![]
        }
    };

    for request in parsed_data {
        let mut stream = TcpStream::connect(request.socket_address)
            .expect("Error occured while connection to a host: ");

        match stream.write_all(&request.request) {
            Ok(ok) => {
                println!(
                    "Text: {:?}",
                    str::from_utf8(&request.request)
                        .unwrap()
                        .split(" ")
                        .collect::<Vec<&str>>()
                );
                println!("Request successfull: {:?}", ok)
            }
            Err(err) => {
                println!("Error occured while sending a TCP request: {}", err)
            }
        };
    }
}
