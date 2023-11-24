use std::{collections::HashMap, io::{self, Read}, fs::{File, self}, time::Duration, sync::Arc, net::TcpListener};
use serde::{Serialize, Deserialize};
use protocol::{self, ServerToClient, ClientToServer, Node};

use std::io::prelude::*;

struct FileInfo {
    tag: [u8; 16],
    data: Vec<u8>,
}

fn main() -> io::Result<()> {
    let mut merkle_tree: Node;

    let mut memory: HashMap<[u8; 32], FileInfo> = HashMap::new();

    //fs::create_dir_all(".\\db")?;

    let listener = TcpListener::bind("127.0.0.1:5000")?;

    // accept connections and process them serially
    let (stream, _addr) = listener.accept()?;
    let mut de = serde_json::Deserializer::from_reader(&stream);

    loop {
        //receive
        let deserialized = ClientToServer::deserialize(&mut de)?;
        println!("Received: {:?}", deserialized);

        //send
        serde_json::to_writer(&stream, &match deserialized {
            ClientToServer::Read(index) => {
                let file = memory.get(&index).expect("File not found");
                
                ServerToClient::Read{
                    index,
                    tag: file.tag,
                    data: file.data.clone(),
                }
            },
            ClientToServer::Write { index, tag, data } => {
                memory.insert(index, FileInfo {
                    tag,
                    data,
                });

                ServerToClient::Write
            },
        }).unwrap();
    }

    /* let server = Server::new(|request, mut response| {
            let mut path = request.uri().path().trim_start_matches("/").split("/");

            match path.next().expect("No path provided") {
                "write" => {
                    let index = path.next().expect("No index provided")
                        .parse::<i32>()
                        .unwrap();
            
                    Ok(response.status(200)
                        .body(fs::read(format!("./db/{}", index)).unwrap())?)
                },
                "read" => {
                    let index = path.next().expect("No index provided")
                        .parse::<i32>()
                        .unwrap();

                    fs::write(
                        format!("./db/{}", index),
                        request.body()
                    ).unwrap();

                    Ok(response.status(200)
                        .body("".as_bytes().to_vec())?)
                },
                _ => {
                    Ok(response.status(404).body("".as_bytes().to_vec())?)
                },
            }
        });

    server.listen("localhost", "9090"); */
}

fn add_to_merkle_tree() {
    todo!()
}