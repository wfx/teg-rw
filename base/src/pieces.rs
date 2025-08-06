use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PiecesDefinition {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub sets: Vec<PieceSet>,
}

#[derive(Debug, Deserialize)]
pub struct PieceSet {
    pub id: u8,
    pub name: String,
    pub pieces: Vec<Piece>,
}

#[derive(Debug, Deserialize)]
pub struct Piece {
    pub value: u8,
    pub image: String,
}
