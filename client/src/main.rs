use std::{collections::HashMap, io::{self, Read}, fs::{File, self}, time::Duration, sync::Arc, net::TcpListener, path};
use serde::{Serialize, Deserialize};
use protocol::{self, ServerToClientRead, ServerToClientWrite, ClientToServer, Node, sha_512_256};

use std::io::prelude::*;
use std::net::TcpStream;
use ring::{aead::{self, Nonce, LessSafeKey, UnboundKey, Tag, Aad, NONCE_LEN, MAX_TAG_LEN, AES_256_GCM}, rand};
use ring::rand::{SystemRandom, SecureRandom};
use ring::digest::SHA512_256_OUTPUT_LEN;
use ring::error::Unspecified;

fn main() -> Result<(), Unspecified> {
    let mut merkle_hash: [u8; SHA512_256_OUTPUT_LEN];

    let rand = SystemRandom::new();

    let mut key_bytes = vec![0; AES_256_GCM.key_len()];
    rand.fill(&mut key_bytes).unwrap();

    let private_key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &key_bytes).unwrap());

    let stream = TcpStream::connect("127.0.0.1:5000").unwrap();
    let mut de = serde_json::Deserializer::from_reader(&stream);

    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        println!("Enter a action: [(read <filename>), (write <path to data> <data>)]");
        input.clear();
        stdin.read_line(&mut input).unwrap();

        let mut input_iter = input.split_whitespace();

        match input_iter.next().unwrap() {
            "read" => {
                let index = sha_512_256(input_iter
                        .next()
                        .expect("No name provided")
                        .as_bytes()
                    );

                //send
                serde_json::to_writer(&stream, &ClientToServer::Read(index))
                    .expect("Failed to send read to server");

                //receive
                let deserialized = ServerToClientRead::deserialize(&mut de).unwrap();
                println!("Received: {:?}", deserialized);

                // check tree

                let mut data_and_tag = deserialized.data.clone();
                data_and_tag.append(&mut deserialized.tag.to_vec());

                println!("Data: {}", 
                    std::str::from_utf8(
                        private_key.open_in_place(
                            Nonce::assume_unique_for_key(deserialized.nonce),
                            make_aad(&index, &deserialized.nonce),
                            &mut data_and_tag,
                        )?
                    ).unwrap()
                );
            },
            "write" => {
                let path = input_iter.next().expect("No path provided");
                let name = *path.split("\\").collect::<Vec<&str>>().last().unwrap();
                let mut data = input_iter.next().unwrap().as_bytes().to_vec();

                let nonce_bytes: [u8; NONCE_LEN] = rand::generate(&rand).unwrap().expose();

                let hashed_name = sha_512_256(name.as_bytes());

                let tag = private_key.seal_in_place_separate_tag(
                        Nonce::assume_unique_for_key(nonce_bytes),
                        make_aad(&hashed_name, &nonce_bytes),
                        &mut data,
                    ).unwrap();

                //send
                serde_json::to_writer(&stream,
                    &ClientToServer::Write {
                        index: hashed_name.clone(),
                        nonce: nonce_bytes,
                        tag: tag.as_ref().try_into().unwrap(),
                        data: data,
                    }
                ).expect("Failed to send write to server");

                //receive
                let deserialized = ServerToClientWrite::deserialize(&mut de).unwrap();

                // check tree

                println!("Successful write");
            },
            _ => {
                println!("Invalid action");
                continue;
            },
        }
    }
}

fn make_aad(index: &[u8; SHA512_256_OUTPUT_LEN], nonce: &[u8; NONCE_LEN]) -> Aad<Vec<u8>> {
    let mut aad = index.to_vec();
    aad.append(&mut nonce.to_vec());

    Aad::from(aad)
}

fn check_merkle_tree(merkle_tree: Node, merkle_hash: [u8; SHA512_256_OUTPUT_LEN]) -> bool {
    todo!()
}