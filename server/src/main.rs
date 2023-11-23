use std::{collections::HashMap, io::{self, Read}, fs::{File, self}, time::Duration, sync::Arc, net::TcpListener};
use serde::{Serialize, Deserialize};
use protocol::{self, ServerToClient, ClientToServer};

use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> io::Result<()> {
    fs::create_dir_all(".\\db")?;

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
                ServerToClient::Read (
                    fs::read(
                        format!("db\\{}", index)
                    ).unwrap()
                )
            },
            ClientToServer::Write { index, data } => {
                print!("Writing to .\\db\\{}...", index);
                fs::write(
                    format!(".\\db\\{}", index),
                    data
                ).unwrap();

                ServerToClient::Write
            },
            ClientToServer::Status => {
                ServerToClient::Status
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