use crate::board;
use board::*;

pub fn movegen(b: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(218);
    for rank in 0..8 { for file in 0..8 { 
        let c: Coord0x88 = c0x88(file,rank);
        let p: Piece = b[c];
        if p.color != b.side_to_move { continue; }   // Piece not of side to move: cannot move

        macro_rules! nonslide_move { ($to:expr) => {
            if $to.0 & 0x88 == 0 && (!b.occupied($to) || b[$to].color != p.color ) {
                moves.push(Move::new(c, $to));
            }
        };}

        macro_rules! gen_sliding {
            ($offset:expr) => {
                let mut to = c + $offset;
                while to.0 & 0x88 == 0 {
                    if b.occupied(c) {
                        if b[c].color != p.color {
                            moves.push(Move::new(c, to));
                        }
                        break;
                    } else {
                        moves.push(Move::new(c, to));
                    }
                    to += $offset;
                }
            };
        }


        match p.piece_type {
            PieceType::None => {},
            PieceType::Any => {},
            PieceType::Pawn => {
                macro_rules! gen_promotions {
                    ($from:expr, $to:expr) => {
                        moves.push(Move{from: $from, to:$to, promote_to: PieceType::Knight, en_passant: EnPassantState::None});
                        moves.push(Move{from: $from, to:$to, promote_to: PieceType::Bishop, en_passant: EnPassantState::None});
                        moves.push(Move{from: $from, to:$to, promote_to: PieceType::Rook, en_passant: EnPassantState::None});
                        moves.push(Move{from: $from, to:$to, promote_to: PieceType::Queen, en_passant: EnPassantState::None});                        
                    };
                }
                match p.color {
                    WHITE => {
                        // Move ahead
                        if !b.occupied(c + o0x88(0,1)) {
                            if rank == 6 {  // 6 is second last
                                gen_promotions!(c, c+o0x88(0, 1));
                            } else {
                                moves.push( Move::new( c, c+o0x88(0,1) ) );
                                if rank == 1 && !b.occupied( c + o0x88(0,2)) {
                                    // Pawn first move, two ahead
                                    moves.push( Move {
                                        from: c, to: c+o0x88(0,2),
                                        promote_to: PieceType::None,
                                        en_passant: EnPassantState::Possible( c+o0x88(0,1) ),
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
                                en_passant: EnPassantState::Capture(b.en_passant.unwrap()),
                            })
                        }
                        // Regular capture
                        let lcap_c = c+o0x88(-1, 1);
                        if b.occupied(lcap_c) && b[lcap_c].color != p.color {
                            moves.push(Move::new(c, lcap_c));
                        }
                        let rcap_c = c+o0x88(-1, 1);
                        if b.occupied(rcap_c) && b[rcap_c].color != p.color {
                            moves.push(Move::new(c, rcap_c));
                        }
                    },
                    BLACK => {
                        // Move ahead
                        if !b.occupied(c + o0x88(0,-1)) {
                            if rank == 7-6 {  // 6 is second last
                                gen_promotions!(c, c+o0x88(0, -1));
                            } else {
                                moves.push( Move::new( c, c+o0x88(0,-1) ) );
                                if rank == 7-1 && !b.occupied( c + o0x88(0,-2)) {
                                    // Pawn first move, two ahead
                                    moves.push( Move {
                                        from: c, to: c+o0x88(0,-2),
                                        promote_to: PieceType::None,
                                        en_passant: EnPassantState::Possible( c+o0x88(0,-1) ),
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
                                en_passant: EnPassantState::Capture(b.en_passant.unwrap()),
                            })
                        }
                    }
                }
            },
            PieceType::Knight => {
                nonslide_move!(c+o0x88( 1,  2));
                nonslide_move!(c+o0x88(-1,  2));
                nonslide_move!(c+o0x88( 1, -2));
                nonslide_move!(c+o0x88(-1, -2));
                nonslide_move!(c+o0x88( 2,  1));
                nonslide_move!(c+o0x88(-2,  1));
                nonslide_move!(c+o0x88( 2, -1));
                nonslide_move!(c+o0x88(-2, -1));
            },

            PieceType::Bishop => {
                gen_sliding!(o0x88( 1,  1));
                gen_sliding!(o0x88( 1, -1));
                gen_sliding!(o0x88(-1,  1));
                gen_sliding!(o0x88(-1, -1));
            },
            PieceType::Rook => {
                gen_sliding!(o0x88( 1,  0));
                gen_sliding!(o0x88(-1,  0));
                gen_sliding!(o0x88( 0,  1));
                gen_sliding!(o0x88( 0, -1));
            },
            PieceType::Queen => {
                gen_sliding!(o0x88( 1,  1));
                gen_sliding!(o0x88( 1, -1));
                gen_sliding!(o0x88(-1,  1));
                gen_sliding!(o0x88(-1, -1));
                gen_sliding!(o0x88( 1,  0));
                gen_sliding!(o0x88(-1,  0));
                gen_sliding!(o0x88( 0,  1));
                gen_sliding!(o0x88( 0, -1));
            },
            PieceType::King => {
                nonslide_move!(c+o0x88( 1,  1));
                nonslide_move!(c+o0x88( 1, -1));
                nonslide_move!(c+o0x88(-1,  1));
                nonslide_move!(c+o0x88(-1, -1));
                nonslide_move!(c+o0x88( 1,  0));
                nonslide_move!(c+o0x88(-1,  0));
                nonslide_move!(c+o0x88( 0,  1));
                nonslide_move!(c+o0x88( 0, -1));
            }
        }
    }}
    return moves;
} 