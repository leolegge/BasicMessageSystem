use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

use loading_bar::{loading_bar::LoadingBar, Color};

fn handle_client(mut stream: TcpStream) {
    println!("Connected to {} and chat started!", stream.peer_addr().unwrap());
    let mut data = [0 as u8; 128];
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            let received_message = std::str::from_utf8(&data)
                .expect("Invalid UTF-8 bytes").to_string();
            println!("Message said -> {}",received_message);
            
            let mut sending_buffer = [0u8; 128];

            let mut my_message = String::new();

            std::io::stdin().read_line(&mut my_message).expect("Failed to read line");

            my_message = my_message.trim().parse().unwrap();
            
            let message_bytes: &[u8] = my_message.as_bytes(); // This always works

            if message_bytes.len() >= 128 {
                println!("Message too long ({} bytes)", message_bytes.len());
                
            }

            sending_buffer[..message_bytes.len()].copy_from_slice(message_bytes);
            
            stream.write(&sending_buffer).unwrap();
            println!("Message sent awaiting reply!");
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}