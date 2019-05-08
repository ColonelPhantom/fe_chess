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

fn print_board(b: &board::Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            use board::PieceType::*;
            let p = b[board::c0x88(file, rank)];
            let mut t = match p.piece_type {
                Pawn => 'p',
                Knight => 'n',
                Bishop => 'b',
                Rook => 'r',
                Queen => 'q',
                King => 'k',
                _ => ' ',
            };
            match p.color {
                board::WHITE => { t.make_ascii_lowercase(); }
                board::BLACK => { t.make_ascii_uppercase(); }
            }
            print!("{}", t);
        }
        println!("");
    }
    println!("Kingpos: {:02x}, {:02x}", b.king_pos[board::WHITE as usize], b.king_pos[board::BLACK as usize]);
}

fn main() {
    let mut b = board::Board::new();

    /*let moves = movegen::movegen(&b);
    println!("{:?}", moves);
    println!("{}", moves.len());*/


    //b = board::Board::new();

    //println!("\n\n");
    println!("Perft test, depth 1: {}", perft(&mut b, 1));
    println!("Perft test, depth 2: {}", perft(&mut b, 2));
    println!("Perft test, depth 3: {}", perft(&mut b, 3));
    println!("Perft test, depth 4: {}", perft(&mut b, 4));
    //b.make(&board::Move::new(board::c0x88::a2, board::c0x88::a4));
    println!("Perft test, depth 5: {}", perft(&mut b, 5));
    //print_board(&b);
    /*b.make(&board::Move::new(board::c0x88::a2, board::c0x88::a4));
    b.make(&board::Move::new(board::c0x88::c7, board::c0x88::c5));
    b.make(&board::Move::new(board::c0x88::a4, board::c0x88::a5));
    b.make(&board::Move{
        from: board::c0x88::b7,
        to: board::c0x88::b5,
        en_passant: board::EnPassantState::Possible(board::c0x88::b5),
        promote_to: board::PieceType::None,
    });
    b.make(&board::Move{
        from: board::c0x88::a5,
        to: board::c0x88::b6,
        en_passant: board::EnPassantState::Capture(board::c0x88::b5),
        promote_to: board::PieceType::None,
    });
    print_board(&b);*/
    println!("Perft test, depth 6: {}", perft(&mut b, 6));
    println!("Perft test, depth 7: {}", pretty_perft(&mut b, 7));
    print_board(&b);
}
