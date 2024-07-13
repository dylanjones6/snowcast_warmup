use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use snowcast_dilly::structs::{Hello, Welcome, TypeInfo};

fn main() {
    let port = "16800";
    let ip = format!("127.0.0.1:{}", &port);
    let client_message = Hello {
        commandType: 0,
        udpPort: 16801,
    };
    match TcpStream::connect(&ip) {
        Ok(mut stream) => {
            println!("Connection made on port {}", &port);
            let message_serialized = serde_json::to_string(&client_message).unwrap();
            let message_ser_bytes = message_serialized.as_bytes();
            stream.write_all(message_ser_bytes).unwrap();

            let mut buffer = [0; 33];
            let bytes_read = stream.read(&mut buffer).unwrap();
            println!("{:?}", &buffer);
            let buffer_str = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
            println!("{:?}", &buffer_str);
            let response = serde_json::from_str::<Welcome>(&buffer_str[..]);
            println!("{response:?}");
            println!("{:?}", response);
        }
        Err(_) => {
            println!("there's something wrong!");
        }
    }
}
