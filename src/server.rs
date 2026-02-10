use std::collections::HashMap;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use std::sync::{Arc, Mutex};

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let ports = args[1..].to_vec();

    let m = Mutex::new(HashMap::new());
    let state = Arc::new(m);

    let mut handles = vec![];

    let state1 = Arc::clone(&state);
    let state2 = state.clone();

    let mut a = state2.lock().unwrap();
    a.insert(String::from(""), 10);

    for port in ports {
        let state_clone = Arc::clone(&state);

        let handle = thread::spawn(move || {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
                .expect("Error occured while binding a port.");

            println!("Server listening on 127.0.0.1:{}", port);

            for request in listener.incoming() {
                match request {
                    Ok(stream) => {
                        println!("New connection established!");
                        let connection_state = Arc::clone(&state_clone);
                        thread::spawn(move || {
                            handle_connection(stream, connection_state);
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        });

        handles.push(handle);
    }

    // Update global state
    // thread::spawn(move || {
    //     let state = state2.lock().unwrap();
    // });
    for handle in handles {
        handle.join().expect("Server thread panicked")
    }
}

fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<HashMap<String, u8>>>) {
    let mut reader = BufReader::new(&stream);
    let mut line = String::new();

    match reader.read_line(&mut line) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                println!("Client disconnected.");
                return;
            }
            println!("Received text: {}", line.split(" ").collect::<String>());
            let mut state_guard = state.lock().unwrap();

            state_guard.insert("dogs".to_string(), 1);
            println!("Current state: {:?}", state_guard);

            let response = "Message received!\n";
            stream
                .write_all(response.as_bytes())
                .expect("Failed to write response");
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}
