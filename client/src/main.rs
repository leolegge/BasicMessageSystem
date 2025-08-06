use std::net::{TcpStream};
use std::io::{Read, Write};

use loading_bar::{loading_bar::LoadingBar, Color};

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            loop {
                let mut sending_buffer = [0 as u8; 128];

                let mut my_message = String::new();

                std::io::stdin().read_line(&mut my_message).expect("Failed to read line");

                my_message = my_message.trim().parse().unwrap();


                let message_bytes: &[u8] = my_message.as_bytes(); // This always works

                if message_bytes.len() >= 128 {
                    println!("Message too long ({} bytes)", message_bytes.len());
                    continue;
                }

                sending_buffer[..message_bytes.len()].copy_from_slice(message_bytes);

                stream.write(&sending_buffer).unwrap();
                println!("Sent {}, awaiting reply...", my_message);
                

                let mut data = [0 as u8; 128];
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        println!("Reply is ok!");
                        let received_message = std::str::from_utf8(&data)
                            .expect("Invalid UTF-8 bytes").to_string();
                        println!("Got message -> {}", received_message);

                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }


        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
