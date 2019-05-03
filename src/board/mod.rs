use std::num::Wrapping;

#[derive(Copy, Clone)]
//#[repr(u8)]
#[derive(PartialEq)]
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
type Side = bool;
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





pub struct Move {
    pub from: Coord0x88,
    pub to: Coord0x88,
    pub promote_to: Piece,
}

pub struct Unmove {
    pub from: Coord0x88,
    pub to: Coord0x88,
    pub captured: Piece,
    pub promoted: bool,

}

pub struct Board {
    pub mailbox: [Piece; 128],      // 0x88
    //pub bitboards: [u64; 16],
    pub unmake_stack: Vec<Unmove>,
    pub side_to_move: Side,
}

impl Board {
    pub fn new_board_startpos() -> Board {
        //use PieceType::{None, WPawn, WKnight, WBishop, WRook, WQueen, WKing, BPawn, BKnight, BBishop, BRook, BQueen, BKing};
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
        }
    }

    pub fn make(&mut self, cmove: Move) {
        // Begin with determining info on the move
        let captured: Piece = self[cmove.to];

        // Test for pawn promotion
        if self[cmove.from].piece_type == PieceType::Pawn && (cmove.to >= c0x88::h1) {
            
        }

        //Time to do the move
        // First add the information to undo the move to the stack
        self.unmake_stack.push( Unmove{
            from: cmove.from,
            to: cmove.to,
            captured: captured,
            promoted: false,
        });

        // Now move the piece on the mailbox
        self[cmove.to] = self[cmove.from];
        self[cmove.from] = pieces::NONE;

        self.side_to_move = !self.side_to_move;
    }
    
    pub fn unmake(&mut self) {
        let u = self.unmake_stack.pop().unwrap();
        if u.promoted {
            self[u.from] = Piece{piece_type: PieceType::Pawn, color: self.side_to_move};
        } else {
            self[u.from] = self[u.to];
        }
        self[u.to] = u.captured;
    }



    // Helper functions
    pub fn occupied(&self, c: Coord0x88) -> bool {
        match self[c].piece_type {
            PieceType::None => false,
            _ => true
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