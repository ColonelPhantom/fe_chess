use crate::board;
use board::*;

fn move_std(from: Coord0x88, to: Coord0x88) -> Move {
    Move{
        from: from,
        to: to,
        promote_to: pieces::NONE
    }
}

pub fn movegen(b: &Board) {
    let mut moves: Vec<Move> = vec![];
    for file in 0..7 {
        for rank in 0..7 {
            let c: Coord0x88 = c0x88(file,rank);
            let p: Piece = b[c];
            match p.piece_type {
                PieceType::None => {},
                PieceType::Any => {},
                PieceType::Pawn => {
                    match p.color {
                        WHITE => {
                            if !b.occupied(c + o0x88(0,1)) {
                                // Piece can move ahead
                                if rank == 7 {
                                    moves.push(move_std(
                                        c, c + o0x88(0,1)
                                    ));
                                }
                                //moves.push(Move {
                                    
                                //});
                                if rank == 2  {

                                }
                            }
                            
                        },
                        BLACK => {

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