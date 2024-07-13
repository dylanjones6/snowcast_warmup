use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use snowcast_dilly::structs::{Hello, TypeInfo, Welcome};

// #[derive(Serialize)]
// pub struct Hello {
//     pub commandType: u8,
//     pub udpPort: u16,
// }
// 
// #[derive(Deserialize)]
// struct Welcome {
//     replyType: u8,
//     numStations: u16,
// }
// 
// trait TypeInfo {
//     fn type_of(&self) -> &'static str;
// }
// 
// impl TypeInfo for Hello {
//     fn type_of(&self) -> &'static str {
//         "Hello"
//     }
// }
// 
// impl TypeInfo for Welcome {
//     fn type_of(&self) -> &'static str {
//         "Welcome"
//     }
// }

fn main() {
    let port = "16800";
    let ip = format!("127.0.0.1:{}", &port);
    // let stream = TcpStream::connect(&ip);
    let message = Hello {
        commandType: 0,
        udpPort: 1234,
    };
    match TcpStream::connect(&ip) {
        Ok(mut stream) => {
            println!("Connection made on port {}", &port);

            //let msg = b"Hello!";
            let msg_serialized = serde_json::to_string(&message).unwrap();
            let msg_ser_bytes = msg_serialized.as_bytes();
            stream.write_all(msg_ser_bytes).unwrap();

            let dummy_welcome = Welcome {
                replyType: 0,
                numStations: 0,
            };
            println!("Sent message, waiting for response...");

            let mut data = [0 as u8; 33];
            //let mut data = Vec::new();
            //match stream.read_exact(&mut data) {
            //let thing = stream.read(&mut data);
            match stream.read(&mut data) { // Vec<u8> --> &str
                Ok(n_bytes) => {
                    //let welcome_msg = serde_json::from_str::<Welcome>(std::str::from_utf8(&data[0..n_bytes]).unwrap()).unwrap();
                    let welcome_str = std::str::from_utf8(&data[..]).unwrap();
                    println!("welcome_str: {:?}", welcome_str);
                    println!("n_bytes: {n_bytes}");
                    let welcome_msg = serde_json::from_str::<Welcome>(&welcome_str[..]).unwrap();
                    println!("welcome_msg: {:?}", &welcome_msg);
                    if welcome_msg.type_of() == dummy_welcome.type_of() {
                        println!("Welcome to Snowcast! The server has {} stations.", welcome_msg.numStations);
                        //println!("{:?}", msg_in);
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    };

                    // if &data == msg {
                    //     println!("Reply is ok!");
                    // } else {
                    //     let text = from_utf8(&data).unwrap();
                    //     println!("Unexpected reply: {}", text);
                    // }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            /* match stream.read(&mut data) { // Vec<u8> --> &str
                Ok(_n_bytes) => {
                    if data == hello_ser_bytes {
                        println!("Reply is good!");
                    } else if serde_json::from_str::<Welcome>(&String::from_utf8(data.clone()).unwrap()).unwrap().type_of() == dummy_welcome.type_of() {
                        println!("hey they're the same type!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }

                    // if &data == msg {
                    //     println!("Reply is ok!");
                    // } else {
                    //     let text = from_utf8(&data).unwrap();
                    //     println!("Unexpected reply: {}", text);
                    // }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }*/

        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }

}
