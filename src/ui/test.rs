use crate::*;

pub fn main() {
    let mut b = board::Board::new();

    let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 4, &mut tt), "Time to depth 4: {} ms");
    measure_time!(search::search(&mut b, 5, &mut tt), "Time to depth 5: {} ms");
    let mut tt = search::transtable::TransTable::new(22);
    measure_time!(search::search(&mut b, 6, &mut tt), "Time to depth 6: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 7, &mut tt), "Time to depth 7: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 8, &mut tt), "Time to depth 8: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 9, &mut tt), "Time to depth 9: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 10, &mut tt), "Time to depth 10: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 11, &mut tt), "Time to depth 11: {} ms");
    println!("Transposition table pressure: {}/{}", tt.filled(), 2_u64.pow(22));



    // let m = &search::search(&mut b, 5, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 5, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 5, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 5, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 5, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 5, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 6, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);

    //super::print_board(&b);

    // let mut searchinfo = search::search(&mut b, 8, &mut search::transtable::TransTable::new(24));
    // while let Some(m) = searchinfo.pv.pop() {
    //     println!(
    //         "{:02x} {:02x} {:?} {:?} {:?}",
    //         m.from, m.to, m.promote_to, m.en_passant, m.castling
    //     );
    // }
    // println!("Score: {}", searchinfo.score);

    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 5, &mut tt), "Time to depth 5: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 6, &mut tt), "Time to depth 6: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 7, &mut tt), "Time to depth 7: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 8, &mut tt), "Time to depth 8: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 9, &mut tt), "Time to depth 9: {} ms");
    // let mut tt = search::transtable::TransTable::new(22);
    // measure_time!(search::search(&mut b, 10, &mut tt), "Time to depth 10: {} ms");
    // println!("Transposition table pressure: {}/{}", tt.filled(), 2_u64.pow(22));

    // let m = &search::search(&mut b, 6, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 6, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 6, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 6, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 6, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // let m = &search::search(&mut b, 9, &mut search::transtable::TransTable::new(18)).pv.pop().expect("Did not get a move");
    // b.make(m);
    // println!("Board: {:x}", b.zobrist);


    b = board::Board::from_fen("q2k2q1/2nqn2b/1n1P1n1b/2rnr2Q/1NQ1QN1Q/3Q3B/2RQR2B/Q2K2Q1 w - -");
    let mut tt = search::transtable::TransTable::new(22);
    measure_time!(search::search(&mut b, 5, &mut tt), "Time to depth 5: {} ms");
    let mut tt = search::transtable::TransTable::new(22);
    measure_time!(search::search(&mut b, 6, &mut tt), "Time to depth 6: {} ms");
    let mut tt = search::transtable::TransTable::new(22);
    measure_time!(search::search(&mut b, 7, &mut tt), "Time to depth 7: {} ms");

    

    // println!("Sizeof board: {}", std::mem::size_of::<board::Board>());
    // println!("Sizeof move: {}", std::mem::size_of::<board::Move>());
    // println!("Sizeof zobrist: {}", std::mem::size_of::<u64>());
    // println!("Sizeof score: {}", std::mem::size_of::<search::Score>());
    println!("Sizeof ttEntry: {}", std::mem::size_of::<search::transtable::TtEntry>());
    // println!("Sizeof PieceType: {}", std::mem::size_of::<board::PieceType>());
    // println!("Sizeof Piece: {}", std::mem::size_of::<board::Piece>());
    // println!("Sizeof Coord0x88: {}", std::mem::size_of::<board::Coord0x88>());
    // println!("Sizeof EPState: {}", std::mem::size_of::<board::EnPassantState>());


    

}