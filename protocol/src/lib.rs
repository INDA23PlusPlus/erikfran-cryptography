use serde::{Serialize, Deserialize};
use ring;

pub fn sha_512_256(data: &[u8]) -> [u8; ring::digest::SHA512_256_OUTPUT_LEN] {
    ring::digest::digest(&ring::digest::SHA512_256, data).as_ref().try_into().unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Node {
    Leaf {
        index: [u8; ring::digest::SHA512_256_OUTPUT_LEN],
        signature: [u8; ring::aead::MAX_TAG_LEN],
    },
    Branch {
        hash: [u8; ring::digest::SHA512_256_OUTPUT_LEN],
        left: Box<Node>,
        right: Box<Node>,
    },
}

/* impl Node {
    // Calculate the merkle hash of a merkle tree
    fn calculate_merkle_hash(&self) -> [u8; ring::digest::SHA512_256_OUTPUT_LEN] {
        match self {
            Node::Leaf{ index, signature } => {
                [signature, signature].concat().try_into().unwrap()
            },
            Node::Branch{ hash, left, right } => {
                calculate_merkle_hash(*left)
                    .iter()
                    .enumerate()
                    .map(|(i, left)| left ^ calculate_merkle_hash(*right)[i])
                    .collect::<Vec<u8>>()

                sha_512_256(&hash.iter().map(|x| x.to_string()).collect::<String>())
            },
        }
    }
} */

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerToClientRead {
    pub tag: [u8; ring::aead::MAX_TAG_LEN],
    pub nonce: [u8; ring::aead::NONCE_LEN],
    pub data: Vec<u8>,
    pub merkle_tree: Node,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerToClientWrite {
    pub merkle_tree: Node,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ClientToServer {
    Read([u8; ring::digest::SHA512_256_OUTPUT_LEN]),
    Write {
        index: [u8; ring::digest::SHA512_256_OUTPUT_LEN],
        nonce: [u8; ring::aead::NONCE_LEN],
        tag: [u8; ring::aead::MAX_TAG_LEN],
        data: Vec<u8>,
    },
}