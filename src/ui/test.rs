use crate::*;

pub fn main() {
    let mut b = board::Board::new();

    let mut searchinfo = search::search(&mut b, 5);
    while let Some(m) = searchinfo.pv.pop() {
        println!(
            "{:02x} {:02x} {:?} {:?} {:?}",
            m.from, m.to, m.promote_to, m.en_passant, m.castling
        );
    }
    println!("Score: {}", searchinfo.score);

}