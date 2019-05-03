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

pub mod pieces {
    use super::*;
    use PieceType::*;
    //use Side::*;
    pub const NONE: Piece = Piece {piece_type:PieceType::None, color:WHITE};
    pub const WPAWN: Piece = Piece {piece_type:Pawn, color: WHITE};
    pub const WKNIGHT: Piece = Piece {piece_type:Knight, color: WHITE};
    pub const WBISHOP: Piece = Piece {piece_type:Bishop, color: WHITE};
    pub const WROOK: Piece = Piece {piece_type:Rook, color: WHITE};
    pub const WQUEEN: Piece = Piece {piece_type:Queen, color: WHITE};
    pub const WKING: Piece = Piece {piece_type:King, color: WHITE};
    pub const BPAWN: Piece = Piece {piece_type:Pawn, color: BLACK};
    pub const BKNIGHT: Piece = Piece {piece_type:Knight, color: BLACK};
    pub const BBISHOP: Piece = Piece {piece_type:Bishop, color: BLACK};
    pub const BROOK: Piece = Piece {piece_type:Rook, color: BLACK};
    pub const BQUEEN: Piece = Piece {piece_type:Queen, color: BLACK};
    pub const BKING: Piece = Piece {piece_type:King, color: BLACK};

    
}

pub type Coord0x88 = std::num::Wrapping<usize>;
pub type Coord8x8 = usize;

// h5 format to 0x88 index
#[macro_export]
macro_rules! c0x88 {
    (a $rank:expr) => {$rank-1 + 0x00};
    (b $rank:expr) => {$rank-1 + 0x10};
    (c $rank:expr) => {$rank-1 + 0x20};
    (d $rank:expr) => {$rank-1 + 0x30};
    (e $rank:expr) => {$rank-1 + 0x40};
    (f $rank:expr) => {$rank-1 + 0x50};
    (g $rank:expr) => {$rank-1 + 0x60};
    (h $rank:expr) => {$rank-1 + 0x70};

    (1 $rank:expr) => {$rank-1 + 0x00};
    (2 $rank:expr) => {$rank-1 + 0x10};
    (3 $rank:expr) => {$rank-1 + 0x20};
    (4 $rank:expr) => {$rank-1 + 0x30};
    (5 $rank:expr) => {$rank-1 + 0x40};
    (6 $rank:expr) => {$rank-1 + 0x50};
    (7 $rank:expr) => {$rank-1 + 0x60};
    (8 $rank:expr) => {$rank-1 + 0x70};
}

// allow (-1, 1) format to determine offset from file and rank difference (same order as h5 but with ints)
#[macro_export]
macro_rules! o0x88 {
    ($file:expr, $rank:expr) => {
        (rank * 0x10) + file
    }
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
        let captured: Piece = self.mailbox[cmove.to.0];
        if self.mailbox[cmove.from.0].piece_type == PieceType::Pawn && (cmove.to.0 > c0x88!(h 1)) {
            
        }

        // First add the information to undo the move to the stack
        self.unmake_stack.push( Unmove{
            from: cmove.from,
            to: cmove.to,
            captured: captured,
            promoted: false,
        });

        // Now move the piece on the mailbox
        self.mailbox[cmove.to.0] = self.mailbox[cmove.from.0];
        self.mailbox[cmove.from.0] = pieces::NONE;

        self.side_to_move = !self.side_to_move;
    }
    
    pub fn unmake(&mut self) {
        let u = self.unmake_stack.pop().unwrap();
        if u.promoted {
            self.mailbox[u.from.0] = Piece{piece_type: PieceType::Pawn, color: self.side_to_move};
        } else {
            self.mailbox[u.from.0] = self.mailbox[u.to.0];
        }
        self.mailbox[u.to.0] = u.captured;
    }
}