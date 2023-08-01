use std::{
    io::{stdin, Read, Write},
    net::TcpStream,
    str::from_utf8,
};

use jdcp::{
    character::character_data::{health_points::HealthPoints, CharacterData},
    message::{info_type::InfoType, Message, MessageType},
};

fn main() {
    println!("Please enter the server port number");
    let port = "12345";
    match TcpStream::connect(format!("127.0.0.1:{}", port)) {
        Ok(mut stream) => {
            println!("Successfully connected!");

            loop {
                let mut msg = String::new();
                println!("Enter '1 ' to send jdcp message");
                stdin()
                    .read_line(&mut msg)
                    .expect("Failed to read message to write");
                if msg.trim().eq("1") {
                    let jdcp_msg = Message {
                        message_type: MessageType::RESPONSE,
                        character_name: "Bart",
                        info_type: InfoType::HP,
                        data_size: 2,
                        data: Some(CharacterData::HP(HealthPoints {
                            current: 34,
                            max: 42,
                        })),
                    };

                    let jdcp_vec: Vec<u8> = jdcp_msg.encode_jdcp();
                    stream.write(&jdcp_vec).unwrap();
                    println!("Awaiting response...  sent bytes: {:?}", jdcp_vec);
                    println!(
                        "Awaiting response...  sent message: {:?}",
                        jdcp::decode_jdcp(&jdcp_vec)
                    );
                } else {
                    println!("No message sent. You entered: {}", msg);
                }
                let mut data: [u8; 10] = [0 as u8; 10];
                match stream.read(&mut data) {
                    Ok(_) => {
                        println!(
                            "Reply: {}",
                            from_utf8(&data).expect("Error unwrapping response")
                        );
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e)
        }
    }
    println!("Terminated.");
}
