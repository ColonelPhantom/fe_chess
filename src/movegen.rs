#[macro_use]
use crate::board;
use board::Piece;


fn movegen(b: board::Board) {
    for file in 1..8 {
        for rank in 1..8 {
            let p: Piece = board::c0x88!(file rank);
        }
    }
} 