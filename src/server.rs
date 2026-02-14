use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::{env, fs, time};

use std::sync::{Arc, Mutex};

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let ports = args[1..].to_vec();

    let m = Mutex::new(HashMap::new());
    let state = Arc::new(m);

    let mut handles = vec![];

    let check_state = state.clone();

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
    thread::spawn(move || {
        let state = check_state.lock().unwrap();

        let five_seconds = time::Duration::from_secs(5);

        loop {
            thread::sleep(five_seconds);

            println!("{:?}", state);

            match fs::write("./dummy", format!("{:?}", state)) {
                Ok(_) => println!("State updated"),
                Err(_) => println!("Failed to update state"),
            }
        }
    });

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
            println!(
                "Received text: {:?}",
                line.split(" ").collect::<Vec<&str>>()
            );
            let data = line.split(" ").collect::<Vec<&str>>();
            let animal = data.first().unwrap();

            let mut state_guard = state.lock().unwrap();
            let count = state_guard.entry((*animal).to_string()).or_insert(1);
            *count += 1;

            let response = "message received!\n";
            stream
                .write_all(response.as_bytes())
                .expect("failed to write response");
        }
        Err(e) => {
            eprintln!("failed to read from stream: {}", e);
        }
    }
}
