use base::board::Board;
use base::game::GameDefinition;
use base::loader::load_and_validate_ron;

fn main() {
    let game = load_and_validate_ron::<GameDefinition>("wad/game/teg.game.ron")
        .expect("Failed to load or validate game");
    println!("Game loaded and validated: {:#?}", game);

    let board = load_and_validate_ron::<Board>("wad/game/teg.board.ron")
        .expect("Failed to load or validate board");
    println!("Board loaded and validated: {:?}", board);
}
