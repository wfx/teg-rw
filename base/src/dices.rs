#[derive(Deserialize)]
struct DicePiece {
    value: u8,
    image: String,
}

#[derive(Deserialize)]
struct DiceVariant {
    id: u8,
    name: String,
    pieces: Vec<DicePiece>,
}

#[derive(Deserialize)]
struct DiceSetCollection {
    id: String,
    name: String,
    author: String,
    version: String,
    description: String,
    dice_sets: Vec<DiceVariant>,
}
