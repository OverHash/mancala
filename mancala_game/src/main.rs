use mancala_game::{board::Board, game::Game};

fn main() {
    let game = Game::with_starting_position(Board::with_custom_pits([0; 12], [24; 2]));
    println!("Current game board status:{:?}", game.current_board);
}
