use std::{collections::HashMap, io::{self, Read}, fs::{File, self}, time::Duration, sync::Arc, net::TcpListener};
use serde::{Serialize, Deserialize};
use protocol::{self, ServerToClient, ClientToServer};

use std::io::prelude::*;
use std::net::TcpStream;
use ring;

struct FileInfo {
    /// The hash of the filename
    nonce: [u8; 12],
}


fn main()  -> std::io::Result<()> {
    let mut merkle_hash: [u8; 32];
    let mut nonce_index: HashMap<Vec<u8>, [u8; 12]> = HashMap::new();
    let private_key = ring::signature::Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
    let password = b"password";


    let stream = TcpStream::connect("127.0.0.1:5000")?;
    let mut de = serde_json::Deserializer::from_reader(&stream);

    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        println!("Enter a action: [(read <index>), (write <index> <path to data>), (status)]");
        input.clear();
        stdin.read_line(&mut input).unwrap();

        let mut input_iter = input.split_whitespace();

        //send
        serde_json::to_writer(
            &stream, 
            &match input_iter.next().unwrap() {
                "read" => {
                    ClientToServer::Read(
                        input_iter.next().unwrap().parse::<u64>().unwrap()
                    )
                },
                "write" => {
                    let name = input_iter.next().unwrap().parse::<u64>().unwrap();
                    let data = input_iter.next().unwrap().as_bytes().to_vec();
                    
                    let encrypted_name = ring::

                    ClientToServer::Write {
                        index: encrypted_name,
                        data: encrypted_data,//fs::read(input_iter.next().unwrap()).unwrap(),
                    }
                },
                "status" => {
                    ClientToServer::Status
                },
                _ => {
                    ClientToServer::Status
                },
            }
        ).unwrap();

        //receive
        let deserialized = ServerToClient::deserialize(&mut de)?;
        println!("Recieved: {:?}", deserialized);

        match deserialized {
            ServerToClient::Read(data) => {
                println!("Data: {:?}", data);
            },
            ServerToClient::Write => {
                println!("Write successful");
            },
            ServerToClient::Status => {
                println!("Status: {:?}", deserialized);
            },
            _ => {
                println!("Error: {:?}", deserialized);
            },
        }
    }
}
