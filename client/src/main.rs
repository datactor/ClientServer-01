use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::str;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let msg = input.as_bytes();

                stream.write(msg).unwrap();

                let mut data = [0 as u8; 256]; // using 256 byte buffer
                match stream.read(&mut data) {
                    Ok(_) => {
                        let text = str::from_utf8(&data[0..input.len()]).unwrap();
                        if text.eq(&input) {
                            println!("Receiving Message : {}" , from_utf8(&data[0..input.len()-1]).unwrap());
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        break;
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