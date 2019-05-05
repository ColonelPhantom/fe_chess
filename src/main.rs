#[macro_use]
mod board;

mod movegen;

fn perft(b: &mut board::Board, depth: usize) -> usize {
    let mut perft_count = 0;

    if depth == 0 {
        return 1;
    }


    let moves = movegen::movegen(b);
    for m in moves {
        b.make(m);
        perft_count += perft(b, depth - 1);
        b.unmake()
    }

    return perft_count;
}

fn main() {
    println!("Hello, world!");

    let mut b = board::Board::new();

    let moves = movegen::movegen(&b);
    println!("{:?}", moves);
    println!("{}", moves.len());


    b = board::Board::new();

    println!("\n\n");
    println!("Perft test, depth 1: {}", perft(&mut b, 1));
    println!("Perft test, depth 2: {}", perft(&mut b, 2));
    println!("Perft test, depth 3: {}", perft(&mut b, 3));
}
