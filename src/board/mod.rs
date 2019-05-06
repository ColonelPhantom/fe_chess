use std::num::Wrapping;

#[derive(Copy, Clone, Debug, PartialEq)]
//#[repr(u8)]
pub enum PieceType {
    None = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
    Any = 7
}
impl Default for PieceType {
    fn default() -> Self { PieceType::None }
}
pub type Side = bool;
pub const WHITE: Side = false;
pub const BLACK: Side = true;
#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Side
}
pub mod pieces;

pub type Coord0x88 = std::num::Wrapping<usize>;
pub type Coord8x8 = usize;

// c0x88:h1 is a number now
#[allow(non_upper_case_globals)] #[allow(dead_code)] pub mod c0x88;
pub fn c0x88(file: isize, rank: isize) -> Coord0x88 {
    Wrapping(16 * rank as usize) + Wrapping( file as usize )
}

// allow (-1, 1) format to determine offset from file and rank difference (same order as h5 but with ints)
pub fn o0x88(file: isize, rank: isize) -> Coord0x88 {
    Wrapping(((rank * 0x10) + file) as usize)
} 

pub fn coord0x88_to8x8(sq0x88: Coord0x88) -> Coord8x8 {
    (sq0x88.0 + (sq0x88.0 & 0x7)) >> 1
}
pub fn coord8x8_to0x88(sq8x8: Coord8x8) -> Coord0x88 {
    std::num::Wrapping(sq8x8 + (sq8x8 & 0xF8))
}

#[derive(Clone, Debug, PartialEq)]
pub enum ThreatInfo {
    Safe,
    Single { c: Coord0x88 },
    Multiple { c: Vec<Coord0x88> },
}



#[derive(Debug)]
pub enum EnPassantState {
    None,
    Possible ( Coord0x88 ),
    Capture ( Coord0x88 ),
}

#[derive(Debug)]
pub struct Move {
    pub from: Coord0x88,
    pub to: Coord0x88,
    pub promote_to: PieceType,
    pub en_passant: EnPassantState,
}
impl Move {
    pub fn new(from: Coord0x88, to: Coord0x88) -> Move {
        Move {
            from: from,
            to: to,
            promote_to: PieceType::None,
            en_passant: EnPassantState::None,
        }
    }
}

pub struct Unmove {
    pub from: Coord0x88,
    pub to: Coord0x88,
    pub captured: Piece,
    pub promoted: bool,
    pub revmov_clock: usize,
    pub in_check: Option<ThreatInfo>,
    pub en_passant: EnPassantState,
}



pub struct Board {
    pub mailbox: [Piece; 128],      // 0x88
    //pub bitboards: [u64; 16],
    pub unmake_stack: Vec<Unmove>,
    pub side_to_move: Side,
    pub en_passant: Option<Coord0x88>,
    pub revmov_clock: usize,
    pub king_pos: [Coord0x88; 2],
    pub in_check: Option<ThreatInfo>,
}

impl Board {
    pub fn new() -> Board {
        use pieces::*;
        Board {
            mailbox: [
                WROOK,  WKNIGHT,    WBISHOP,    WQUEEN, WKING,  WBISHOP,    WKNIGHT,    WROOK,  NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   
                WPAWN,  WPAWN,      WPAWN,      WPAWN,  WPAWN,  WPAWN,      WPAWN,      WPAWN,  NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   
                NONE,   NONE,       NONE,       NONE,   NONE,   NONE,       NONE,       NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,             
                NONE,   NONE,       NONE,       NONE,   NONE,   NONE,       NONE,       NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,             
                NONE,   NONE,       NONE,       NONE,   NONE,   NONE,       NONE,       NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,             
                NONE,   NONE,       NONE,       NONE,   NONE,   NONE,       NONE,       NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,             
                BPAWN,  BPAWN,      BPAWN,      BPAWN,  BPAWN,  BPAWN,      BPAWN,      BPAWN,  NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   
                BROOK,  BKNIGHT,    BBISHOP,    BQUEEN, BKING,  BBISHOP,    BKNIGHT,    BROOK,  NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   NONE,   

            ],

            //bitboards: [0; 16],  // Use empty bitboards for now (yes that's gross)

            unmake_stack: Vec::new(),

            side_to_move: WHITE,

            revmov_clock: 0,

            en_passant: None,

            king_pos: [c0x88::e1, c0x88::e8],

            in_check: None,
        }
    }

    pub fn make(&mut self, cmove: Move) {
        // Begin with determining info on the move
        let captured: Piece = self[cmove.to];
        let promoted;
        let revmov_clock = self.revmov_clock;
        let mut undo_ep = EnPassantState::None;

        if self.en_passant.is_some() {
            undo_ep = EnPassantState::Possible(self.en_passant.unwrap());
        }

        match cmove.en_passant {
            EnPassantState::None => {
                self.en_passant = None;
            }
            EnPassantState::Possible(c) => {
                self.en_passant = Some(c);
            }
            EnPassantState::Capture(c) => {
                self[c] = pieces::NONE;
                undo_ep = EnPassantState::Capture(c);
            }
        }


        // Reversible move clock
        if self.occupied(cmove.to) || self[cmove.from].piece_type == PieceType::Pawn {
            self.revmov_clock += 1;
        } else {
            self.revmov_clock = 0;
        }

        if self[cmove.from].piece_type == PieceType::King {
            // TODO: remove castling rights

            // Update kingpos
            self.king_pos[self.side_to_move as usize] = cmove.to;
        }

        // TODO: remove castling rights when rook moves from starting square

        // Pawn promotion
        if cmove.promote_to != PieceType::None {
            self[cmove.to] = Piece {piece_type: cmove.promote_to, color: self.side_to_move};
            self[cmove.from] = pieces::NONE;
            promoted = true;

        } else { // Non-promoting move: business as usual
            self[cmove.to] = self[cmove.from];
            self[cmove.from] = pieces::NONE;
            promoted = false;
        }

        // Add the information to undo the move to the stack

        // Vague but optimized code equivalent to but doesn't conflict with the borrow checker:
        //let in_check = self.in_check
        //self.in_check = None
        let in_check = std::mem::replace(&mut self.in_check, None);
        // Now actually push
        self.unmake_stack.push( Unmove{
            from: cmove.from,
            to: cmove.to,
            captured: captured,
            promoted: promoted,
            in_check: in_check,
            revmov_clock: revmov_clock,
            en_passant: undo_ep,
        });

        // Update 'trivial' field(s)
        self.side_to_move = !self.side_to_move;
    }
    
    pub fn unmake(&mut self) {
        self.side_to_move = !self.side_to_move; // At start of function so it's the side that made the move.
        let u = self.unmake_stack.pop().unwrap();
        if u.promoted {
            self[u.from] = Piece{piece_type: PieceType::Pawn, color: self.side_to_move};
        } else {
            self[u.from] = self[u.to];
        }
        self[u.to] = u.captured;
        self.in_check = u.in_check;
        self.revmov_clock = u.revmov_clock;

        match u.en_passant {
            EnPassantState::None => {
                self.en_passant = None;
            }
            EnPassantState::Possible(c) => {
                self.en_passant = Some(c);
            }
            EnPassantState::Capture(c) => {
                self[c] = Piece{piece_type: PieceType::Pawn, color: !self.side_to_move};
            }
        }

        if self[u.from].piece_type == PieceType::King {
            self.king_pos[self.side_to_move as usize] = u.from;
        }
    }



    // Helper functions
    pub fn occupied(&self, c: Coord0x88) -> bool {
        match self[c].piece_type {
            PieceType::None => false,
            _ => true
        }
    }

    pub fn under_attack(&self, c: Coord0x88, side: Side) -> ThreatInfo {
        let mut threats: Vec<Coord0x88> = vec![];
        
        macro_rules! nonslide_threat {
            ($offset:expr, $types:pat) => {
                let to = c+$offset;
                if to.0 & 0x88 == 0 && self.occupied(to) && self[to].color != side {
                    match self[to].piece_type {
                        $types => { threats.push(to) }
                        _ => {}
                    }
                }
            };
        }

        macro_rules! slide_threat {
           ($offset:expr, $types:pat) => 
           {
                let mut to = c+$offset;
                if to.0 & 0x88 == 0 && self[to].piece_type == PieceType::King && self[to].color != side {
                    threats.push(to);
                } else {
                    while to.0 & 0x88 == 0 {
                        if self.occupied(to) {
                            if self[to].color != side {
                                match self[to].piece_type {
                                    $types | PieceType::Queen => { threats.push(to) }
                                    _ => {}
                                }
                            }
                            break;
                        }
                        to += $offset;
                    }
                    if to.0 & 0x88 == 0 && self[to].color != side {
                        match self[to].piece_type {
                            $types | PieceType::Queen => { threats.push(to) }
                            _ => {}
                        }
                    }
                }
           }
        }
        

        // Pawns
        match side {
            WHITE => {
                nonslide_threat!(o0x88( 1,  1), PieceType::Pawn);
                nonslide_threat!(o0x88(-1,  1), PieceType::Pawn);
            }
            BLACK => {
                nonslide_threat!(o0x88( 1, -1), PieceType::Pawn);
                nonslide_threat!(o0x88(-1, -1), PieceType::Pawn);
            }
        }
        // Knights
        nonslide_threat!(o0x88( 1,  2), PieceType::Knight);
        nonslide_threat!(o0x88(-1,  2), PieceType::Knight);
        nonslide_threat!(o0x88( 1, -2), PieceType::Knight);
        nonslide_threat!(o0x88(-1, -2), PieceType::Knight);
        nonslide_threat!(o0x88( 2,  1), PieceType::Knight);
        nonslide_threat!(o0x88(-2,  1), PieceType::Knight);
        nonslide_threat!(o0x88( 2, -1), PieceType::Knight);
        nonslide_threat!(o0x88(-2, -1), PieceType::Knight);

        // Diagonal (bishop + queen)
        slide_threat!(o0x88( 1,  1), PieceType::Bishop);
        slide_threat!(o0x88( 1, -1), PieceType::Bishop);
        slide_threat!(o0x88(-1,  1), PieceType::Bishop);
        slide_threat!(o0x88(-1, -1), PieceType::Bishop);

        // Hor/vertical (rook + queen)
        slide_threat!(o0x88( 1,  0), PieceType::Rook);
        slide_threat!(o0x88(-1,  0), PieceType::Rook);
        slide_threat!(o0x88( 0,  1), PieceType::Rook);
        slide_threat!(o0x88( 0, -1), PieceType::Rook);


        return ThreatInfo::Safe;

    }

    pub fn is_check(&self, side: Side) -> ThreatInfo {
        match &self.in_check {
            None => { self.under_attack(self.king_pos[side as usize], side) }
            Some(x) => x.clone()
        }
    }
}
impl std::ops::Index<Coord0x88> for Board {
    type Output = Piece;

    fn index(&self, c: Coord0x88) -> &Piece {
        &self.mailbox[c.0]
    }
}
impl std::ops::IndexMut<Coord0x88> for Board {
    fn index_mut(&mut self, c: Coord0x88) -> &mut Piece {
        &mut self.mailbox[c.0]
    }
}