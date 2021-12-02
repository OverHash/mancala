use mancala_game::board::Board;

fn main() {
    let mut board = Board::new();
    board.stores = [24; 2];
    board.pits = [0; 12];
    println!("Current board status:\n{:?}", board);
}
