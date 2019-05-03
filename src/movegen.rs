use crate::board;
use board::Piece;
use board::PieceType;

fn movegen(b: board::Board) {
    let moves: Vec<board::Move>;
    for file in 0..7 {
        for rank in 0..7 {
            let c = board::c0x88(file,rank);
            let p: Piece = b.mailbox[c as usize];
            match p.piece_type {
                PieceType::None => {},
                PieceType::Any => {},
                PieceType::Pawn => {
                    match p.color {
                        board::WHITE => {
                            if !b.occupied(std::num::Wrapping( (c + board::o0x88(0,1)) as usize)) {
                                // Piece can move ahead
                                if rank == 7 {
                                    moves.push(board::Move{})
                                }
                                moves.push(board::Move {
                                    
                                });
                                if rank == 2  {

                                }
                            }
                            
                        },
                        board::BLACK => {

                        }
                    }
                },
                PieceType::Knight => {

                },
                PieceType::Bishop => {

                },
                PieceType::Rook => {

                },
                PieceType::Queen => {

                },
                PieceType::King => {

                }
            }
        }
    }
} 