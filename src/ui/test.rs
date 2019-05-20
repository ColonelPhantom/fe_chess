use crate::*;

pub fn main() {
    let mut b = board::Board::new();

    let mut searchinfo = search::search(&mut b, 8, &mut search::transtable::TransTable::new(24));
    while let Some(m) = searchinfo.pv.pop() {
        println!(
            "{:02x} {:02x} {:?} {:?} {:?}",
            m.from, m.to, m.promote_to, m.en_passant, m.castling
        );
    }
    println!("Score: {}", searchinfo.score);

    // println!("Sizeof board: {}", std::mem::size_of::<board::Board>());
    // println!("Sizeof move: {}", std::mem::size_of::<board::Move>());
    // println!("Sizeof zobrist: {}", std::mem::size_of::<u64>());
    // println!("Sizeof score: {}", std::mem::size_of::<search::Score>());
    // println!("Sizeof ttEntry: {}", std::mem::size_of::<search::transtable::TtEntry>());
    // println!("Sizeof PieceType: {}", std::mem::size_of::<board::PieceType>());
    // println!("Sizeof Piece: {}", std::mem::size_of::<board::Piece>());
    // println!("Sizeof Coord0x88: {}", std::mem::size_of::<board::Coord0x88>());
    // println!("Sizeof EPState: {}", std::mem::size_of::<board::EnPassantState>());
    

}