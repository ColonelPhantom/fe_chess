use crate::board;
use board::*;

fn gen_sliding(b: &Board, from: Coord0x88, offset: Coord0x88, side: Side) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let mut c = from + offset;
    while c.0 & 0x88 == 0 {
        if b.occupied(c) {
            if b[c].color != side {
                moves.push(Move::new(from, c));
            }
            break;
        } else {
            moves.push(Move::new(from, c));
        }
        c += offset;
    }
    return moves;
}

pub fn movegen(b: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for rank in 0..8 { for file in 0..8 { 
        let c: Coord0x88 = c0x88(file,rank);
        let p: Piece = b[c];
        if p.color != b.side_to_move { continue; }   // Piece not of side to move: cannot move
        match p.piece_type {
            PieceType::None => {},
            PieceType::Any => {},
            PieceType::Pawn => {
                macro_rules! gen_promotions {
                    ($from:expr, $to:expr) => {
                        moves.push(Move{from: $from, to:$to, promote_to: PieceType::Knight, en_passant: None, ep_capture: false});
                        moves.push(Move{from: $from, to:$to, promote_to: PieceType::Bishop, en_passant: None, ep_capture: false});
                        moves.push(Move{from: $from, to:$to, promote_to: PieceType::Rook, en_passant: None, ep_capture: false});
                        moves.push(Move{from: $from, to:$to, promote_to: PieceType::Queen, en_passant: None, ep_capture: false});                        
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
                macro_rules! nonslide_move { ($to:expr) => {
                    if $to.0 & 0x88 == 0 && (!b.occupied($to) || b[$to].color != p.color ) {
                        moves.push(Move::new(c, $to));
                    }
                };}
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
                moves.extend(gen_sliding(b, c, o0x88( 1,  1), p.color));
                moves.extend(gen_sliding(b, c, o0x88( 1, -1), p.color));
                moves.extend(gen_sliding(b, c, o0x88(-1,  1), p.color));
                moves.extend(gen_sliding(b, c, o0x88(-1, -1), p.color));
            },
            PieceType::Rook => {
                moves.extend(gen_sliding(b, c, o0x88( 1,  0), p.color));
                moves.extend(gen_sliding(b, c, o0x88(-1,  0), p.color));
                moves.extend(gen_sliding(b, c, o0x88( 0,  1), p.color));
                moves.extend(gen_sliding(b, c, o0x88( 0, -1), p.color));
            },
            PieceType::Queen => {
                moves.extend(gen_sliding(b, c, o0x88( 1,  1), p.color));
                moves.extend(gen_sliding(b, c, o0x88( 1, -1), p.color));
                moves.extend(gen_sliding(b, c, o0x88(-1,  1), p.color));
                moves.extend(gen_sliding(b, c, o0x88(-1, -1), p.color));
                moves.extend(gen_sliding(b, c, o0x88( 1,  0), p.color));
                moves.extend(gen_sliding(b, c, o0x88(-1,  0), p.color));
                moves.extend(gen_sliding(b, c, o0x88( 0,  1), p.color));
                moves.extend(gen_sliding(b, c, o0x88( 0, -1), p.color));
            },
            PieceType::King => {
                macro_rules! nonslide_move { ($to:expr) => {
                    if $to.0 & 0x88 == 0 && (!b.occupied($to) || b[$to].color != p.color ) {
                        moves.push(Move::new(c, $to));
                    }
                };}
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