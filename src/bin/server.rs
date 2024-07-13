use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
// use serde::ser::{Serialize, SerializeStruct, Serializer};
use snowcast_dilly::structs::{Hello, Welcome};

// #[derive(Serialize)]
// pub struct Welcome {
//     commandType: u8,
//     udpPort: u16,
// }
// 
// #[derive(Deserialize)]
// pub struct Hello {
//     replyType: u8,
//     numStations: u16,
// }
// 
// impl Serialize for Welcome {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("Welcome", 2)?;
//         state.serialize_field("commandType", &self.commandType)?;
//         state.serialize_field("udpPort", &self.udpPort)?;
//     }
// }

fn handle_client(mut stream: TcpStream) {
    // let mut welcome: [u8; 3] = [0; 3];
    // welcome[0] = 2; // replyType = 2 for welcome message
    // welcome[1] = 
    let new_welcome = Welcome {
        replyType: 2,
        numStations: 150,
    };
    // let welcome1 = 2 as u8;
    // let welcome2 = 17775 as u16;
    println!("new_welcome: {:?}", &new_welcome);
    let msg = serde_json::to_string(&new_welcome).unwrap().into_bytes();
    //println!("{:?}", &msg);
    let mut data = [0 as u8; 33];
    match stream.read(&mut data) {
        Ok(size) => {
            println!("received data from client: {:?}", &data);
            println!("sending msg: {:?}", &msg);
            stream.write_all(&msg[..]).unwrap();
        },
        Err(_) => {
            println!("An error occurred, \
                terminating connection with {}",
                stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    } 
}

fn main() {
    let port = "16800";
    let ip = format!("127.0.0.1:{}", &port);
    let listener = TcpListener::bind(&ip).unwrap();
    println!("Listening on port: {}", &port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
