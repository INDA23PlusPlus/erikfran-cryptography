use std::{collections::HashMap, io::{self, Read}, fs::{File, self}, time::Duration, sync::Arc, net::TcpListener, path};
use serde::{Serialize, Deserialize};
use protocol::{self, ServerToClient, ClientToServer, Node};

use std::io::prelude::*;
use std::net::TcpStream;
use ring::aead::{self, NonceSequence, Nonce, OpeningKey, SealingKey, Tag, Aad, BoundKey, UnboundKey, NONCE_LEN, MAX_TAG_LEN};
use ring::rand::{SystemRandom, SecureRandom};
use ring::digest::{SHA256, digest, SHA256_OUTPUT_LEN};
use ring::error::Unspecified;

static AES_256_GCM: &aead::Algorithm = &aead::AES_256_GCM;

struct CounterNonceSequence(u32);

impl NonceSequence for CounterNonceSequence {
    // called once for each seal operation
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = [0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);

        self.0 += 1;
        Ok(Nonce::assume_unique_for_key(nonce_bytes))
    }
}

fn sha_256(data: &str) -> [u8; SHA256_OUTPUT_LEN] {
    digest(&SHA256, data.as_bytes()).as_ref().try_into().unwrap()
}

fn main() -> Result<(), Unspecified> {
    let mut merkle_hash: [u8; SHA256_OUTPUT_LEN];

    let rand = SystemRandom::new();
    let nonce_sequence_start = 1;

    let mut key_bytes = vec![0; AES_256_GCM.key_len()];
    rand.fill(&mut key_bytes).unwrap();

    let mut sealing_key = SealingKey::new(
        UnboundKey::new(&AES_256_GCM, &key_bytes)?, 
        CounterNonceSequence(nonce_sequence_start));
    let mut opening_key = OpeningKey::new(
        UnboundKey::new(&AES_256_GCM, &key_bytes)?, 
        CounterNonceSequence(nonce_sequence_start));

    let stream = TcpStream::connect("127.0.0.1:5000").unwrap();
    let mut de = serde_json::Deserializer::from_reader(&stream);

    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        println!("Enter a action: [(read <filename>), (write <path to data> <data>)]");
        input.clear();
        stdin.read_line(&mut input).unwrap();

        let mut input_iter = input.split_whitespace();

        //send
        serde_json::to_writer(
            &stream, 
            &match input_iter.next().unwrap() {
                "read" => {
                    ClientToServer::Read(
                        sha_256(input_iter.next().expect("No name provided"))
                    )
                },
                "write" => {
                    let path = input_iter.next().expect("No path provided");
                    let name = *path.split("\\").collect::<Vec<&str>>().last().unwrap();
                    let mut data = input_iter.next().unwrap().as_bytes().to_vec();

                    let hashed_name = sha_256(&name);
                    let tag = sealing_key.seal_in_place_separate_tag(
                            Aad::from(hashed_name.clone()),
                            &mut data,
                        ).unwrap();

                    ClientToServer::Write {
                        index: hashed_name.clone(),
                        tag: tag.as_ref().try_into().unwrap(),
                        data: data,
                    }
                },
                _ => {
                    unimplemented!("Invalid action")
                },
            }
        ).unwrap();

        //receive
        let deserialized = ServerToClient::deserialize(&mut de).unwrap();
        println!("Received: {:?}", deserialized);

        match deserialized {
            ServerToClient::Read{ index, tag, data } => {
                // check tree

                let mut data_and_tag = data.clone();
                data_and_tag.append(&mut tag.to_vec());

                println!("Data: {}", 
                    std::str::from_utf8(
                        opening_key.open_in_place(
                            Aad::from(index),
                            &mut data_and_tag,
                        )?
                    ).unwrap()
                );
            },
            ServerToClient::Write => {
                // check tree

                println!("Write successful");
            },
            _ => {
                println!("Error: {:?}", deserialized);
            },
        }
    }
}

fn check_merkle_tree(merkle_tree: Node, merkle_hash: [u8; SHA256_OUTPUT_LEN]) -> bool {
    todo!()
}