use jdcp;
use std::{
    env::args,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) {
    let mut buff = [0 as u8; 50];
    while match stream.read(&mut buff) {
        Ok(size) => {
            println!("Recieved message of {} bytes: {:?}", size, buff);
            let jdcp_message = jdcp::decode_jdcp(&buff).expect("Unable to decode message");

            println!("Leftover Buffer: {:?}", jdcp_message.0);
            println!("Message: {:?}", jdcp_message.1);

            stream.write(b"okay...").unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occured, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    if args().count() < 2 {
        panic!("Not enough arguments")
    }
    let port = args().last().unwrap();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
