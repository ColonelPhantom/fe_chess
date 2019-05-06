#[macro_use]
mod board;

mod movegen;

fn perft(b: &mut board::Board, depth: usize) -> usize {
    let mut perft_count = 0;

    //if depth == 1 {
    //    return movegen::movegen(b).len();
    //}
    if depth == 0 {
        return 1;
    }

    let moves = movegen::movegen(b);
    for m in moves {
        b.make(&m);
        let check = b.is_check(!b.side_to_move);
        if let board::ThreatInfo::Safe = check {
            perft_count += perft(b, depth - 1);
        }
        b.unmake()
    }

    return perft_count;
}

fn pretty_perft(b: &mut board::Board, depth: usize) -> usize {
    let mut perft_count = 0;


    if depth == 0 {
        return 1;
    }

    let moves = movegen::movegen(b);
    for m in moves {
        b.make(&m);
        let check = b.is_check(!b.side_to_move);
        if let board::ThreatInfo::Safe = check {
            let c = perft(b, depth - 1);
            perft_count += c;
            println!("{:02x} to {:02x}: {}", m.from, m.to, c);
        }
        
        b.unmake()
    }

    return perft_count;
}

fn main() {
    let mut b = board::Board::new();

    /*let moves = movegen::movegen(&b);
    println!("{:?}", moves);
    println!("{}", moves.len());*/


    //b = board::Board::new();

    //println!("\n\n");
    println!("Perft test, depth 1: {}", perft(&mut b, 1));

    b = board::Board::new();
    println!("Perft test, depth 2: {}", perft(&mut b, 2));
    //println!("Perft test, depth 2: {}", perft(&mut b, 1));
    // let moves = movegen::movegen(&b);
    // for m in moves {
    //     println!("{:02x} to {:02x}, promote: {:?}, enpassant: {:?}", m.from, m.to, m.promote_to, m.en_passant);
    // }

    //for i in 1..32 {
    //    println!("{:02x}", i);
    //}

    b = board::Board::new();
    println!("Perft test, depth 3: {}", perft(&mut b, 3));
    b = board::Board::new();
    println!("Perft test, depth 4: {}", pretty_perft(&mut b, 4));

    b = board::Board::new();
    println!("Perft test, depth 5: {}", perft(&mut b, 5));
    b = board::Board::new();
    println!("Perft test, depth 6: {}", perft(&mut b, 6));

}
