use serde::{Serialize, Deserialize};

/* ///This is the first message that the client sends to the server just tells it what color it wants the server to play as.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClientToServerHandshake {
    pub server_color: Color,
}

/**
This is the first message that the server sends to the client after receiving the `ClientToServerHandshake`.
Includes the start state of the game and what features the server supports.
*/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServerToClientHandshake {
    ///The start state of the game. The board is row major and the first index is the rank and the second index is the file.
    pub board: [[Piece; 8]; 8],
    pub moves: Vec<Move>,
    pub joever: Joever,
    ///The features that the server supports. Completely optional to handle and the client can just ignore it if it wants to.
    pub features: Vec<Features>,
} */

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ServerToClient {
    Read(Vec<u8>),
    Write,//(u64),
    Error(String),
    Status/* {
        // hash: u64,
        // memory_used: u64,
    },*/
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ClientToServer {
    Read(u64),
    Write {
        index: u64,
        data: Vec<u8>,
    },
    Status,
}