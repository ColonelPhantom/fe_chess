mod board;

fn main() {
    println!("Hello, world!");

    let b = board::Board::new_board_startpos();

    let mut a = std::num::Wrapping(0usize);
    a -= std::num::Wrapping(1usize);
    println!("{}",a);
}
