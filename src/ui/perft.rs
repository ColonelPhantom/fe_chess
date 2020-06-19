use crate::*;

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
        if b.is_check(!b.side_to_move).is_safe() {
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
        if b.is_check(!b.side_to_move).is_safe() {
            let c = perft(b, depth - 1);
            perft_count += c;
            println!("{:02x} to {:02x}: {}", m.from, m.to, c);
        }

        b.unmake()
    }

    return perft_count;
}

pub fn main() {
    let mut b = board::Board::new();

    // println!("Perft test, depth 1: {}", perft(&mut b, 1));
    // println!("Perft test, depth 2: {}", perft(&mut b, 2));
    // println!("Perft test, depth 3: {}", perft(&mut b, 3));
    // println!("Perft test, depth 4: {}", perft(&mut b, 4));
    // println!("Perft test, depth 5: {}", perft(&mut b, 5));
    // println!("Perft test, depth 6: {}", perft(&mut b, 6));
    // println!("Perft test, depth 7: {}", pretty_perft(&mut b, 7));

    for i in 1..=5 {
        let time = measure_time!(perft(&mut b, i));
        println!("Perft({}) time: {}ms", i, time);
    }
}
