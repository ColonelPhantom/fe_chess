use crate::board;
use board::*;

fn gen_promotions(from: Coord0x88, to: Coord0x88) -> Vec<Move> {
    vec![
        Move{from: from, to:to, promote_to: PieceType::Knight, en_passant: None, ep_capture: false},
        Move{from: from, to:to, promote_to: PieceType::Bishop, en_passant: None, ep_capture: false},
        Move{from: from, to:to, promote_to: PieceType::Rook, en_passant: None, ep_capture: false},
        Move{from: from, to:to, promote_to: PieceType::Queen, en_passant: None, ep_capture: false},
    ]
}

pub fn movegen(b: &Board) {
    let mut moves: Vec<Move> = vec![];
    for file in 0..7 { for rank in 0..7 {
        let c: Coord0x88 = c0x88(file,rank);
        let p: Piece = b[c];
        match p.piece_type {
            PieceType::None => {},
            PieceType::Any => {},
            PieceType::Pawn => {
                match p.color {
                    WHITE => {
                        // Move ahead
                        if !b.occupied(c + o0x88(0,1)) {
                            if rank == 6 {  // 6 is second last
                                moves.extend(gen_promotions( c, c+o0x88(0,1)));
                            } else {
                                moves.push( Move::new( c, c+o0x88(0,1) ) );
                                if rank == 1 && !b.occupied( c + o0x88(0,2)) {
                                    // Pawn first move, two ahead
                                    moves.push( Move {
                                        from: c, to: c+o0x88(0,2),
                                        promote_to: PieceType::None,
                                        en_passant: Some( c+o0x88(0,1) ),
                                        ep_capture: false
                                    });
                                }
                            }
                        }
                        // En passant capture
                        if
                                b.en_passant.is_some() &&
                                (( b.en_passant.unwrap() - (c+o0x88(0, 1)) ).0 == 1 ||
                                 ( (c+o0x88(0, 1)) - b.en_passant.unwrap() ).0 == 1 )
                        {
                            moves.push(Move {
                                from: c,
                                to: b.en_passant.unwrap(),
                                promote_to: PieceType::None,
                                en_passant: None,
                                ep_capture: true,
                            })
                        }
                    },
                    BLACK => {
                        // Move ahead
                        if !b.occupied(c + o0x88(0,-1)) {
                            if rank == 7-6 {  // 6 is second last
                                moves.extend(gen_promotions( c, c+o0x88(0,-1)));
                            } else {
                                moves.push( Move::new( c, c+o0x88(0,-1) ) );
                                if rank == 7-1 && !b.occupied( c + o0x88(0,-2)) {
                                    // Pawn first move, two ahead
                                    moves.push( Move {
                                        from: c, to: c+o0x88(0,-2),
                                        promote_to: PieceType::None,
                                        en_passant: Some( c+o0x88(0,-1) ),
                                        ep_capture: false
                                    });
                                }
                            }
                        }
                        // En passant capture
                        if
                                b.en_passant.is_some() &&
                                (( b.en_passant.unwrap() - (c+o0x88(0, -1)) ).0 == 1 ||
                                 ( (c+o0x88(0, -1)) - b.en_passant.unwrap() ).0 == 1 )
                        {
                            moves.push(Move {
                                from: c,
                                to: b.en_passant.unwrap(),
                                promote_to: PieceType::None,
                                en_passant: None,
                                ep_capture: true,
                            })
                        }
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
    }}
} 