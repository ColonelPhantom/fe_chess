#[macro_use]
mod board;

mod movegen;

fn main() {
    println!("Hello, world!");

    let mut b = board::Board::new_board_startpos();

    b.make(board::Move {
        from: std::num::Wrapping(17),
        to: std::num::Wrapping(27),
        promote_to: board::pieces::NONE
    });
}
