pub mod interactive;
pub mod perft;
pub mod test;
pub mod uci;

use crate::*;

pub fn print_board(b: &board::Board) {
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
                board::WHITE => {
                    t.make_ascii_lowercase();
                }
                board::BLACK => {
                    t.make_ascii_uppercase();
                }
            }
            print!("{}", t);
        }
        println!("");
    }
    //println!("Kingpos: {:02x}, {:02x}", b.king_pos[board::WHITE as usize], b.king_pos[board::BLACK as usize]);
}
