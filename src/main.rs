#[macro_use]
mod board;

mod movegen;

fn main() {
    println!("Hello, world!");

    let mut b = board::Board::new();

    movegen::movegen(&b);


    b.make(board::Move::new(board::c0x88::e2, board::c0x88::e4));
    b.unmake();
}
