use std::{collections::HashMap, io::{self, Read}, fs::{File, self}, time::Duration, sync::Arc, net::TcpListener};
use serde::{Serialize, Deserialize};
use protocol::{self, ServerToClientRead, ServerToClientWrite, ClientToServer, Node};

use std::io::prelude::*;
use ring::aead::{NONCE_LEN, MAX_TAG_LEN};
use ring::digest::SHA512_256_OUTPUT_LEN;

struct FileInfo {
    tag: [u8; MAX_TAG_LEN],
    nonce: [u8; NONCE_LEN],
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
        match deserialized {
            ClientToServer::Read(index) => {
                let file = memory.get(&index).expect("File not found");
                
                serde_json::to_writer(&stream,
                    &ServerToClientRead{
                        nonce: file.nonce,
                        tag: file.tag,
                        data: file.data.clone(),
                        merkle_tree: Node::Leaf { index: index, signature: file.tag }
                    }
                ).expect("Failed to send read to server");
            },
            ClientToServer::Write { index, nonce, tag, data } => {
                memory.insert(index, FileInfo {
                    nonce,
                    tag,
                    data,
                });

                serde_json::to_writer(&stream, 
                    &ServerToClientWrite {
                        merkle_tree: Node::Leaf { index: index, signature: tag }
                    }
                ).expect("Failed to send write to client");
            },
        }
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

fn merkle_tree_for_file() {
    todo!()
}