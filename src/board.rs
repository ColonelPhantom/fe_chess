#[derive(Copy, Clone)]
//#[repr(u8)]
pub enum PieceType {
    None = 0,       // if(Piece)
    WPawn = 1,      // all odd can move diagonally
    WKnight = 2,    // &2 is minor piece
    WBishop = 3,    // odd and &2
    WRook = 4,      // &4 = major piece
    WQueen = 5,     // &4 and odd
    WKing = 6,      // Above queen
    WAny = 7,       // Any piece (bitboard index)
    BPawn = 9,      // &8 is black, &7 to get type without color.
    BKnight = 10,
    BBishop = 11,
    BRook = 12,
    BQueen = 13,
    BKing = 14,
    BAny = 15
}

pub type Coord0x88 = std::num::Wrapping<usize>;
pub type Coord8x8 = usize;

pub fn coord0x88_to8x8(sq0x88: Coord0x88) -> Coord8x8 {
    (sq0x88.0 + (sq0x88.0 & 0x7)) >> 1
}
pub fn coord8x8_to0x88(sq8x8: Coord8x8) -> Coord0x88 {
    std::num::Wrapping(sq8x8 + (sq8x8 & 0xF8))
}

pub struct Move {
    from: Coord0x88,
    to: Coord0x88,
    promote_to: PieceType
}

pub struct Unmove {
    from: Coord0x88,
    to: Coord0x88,
    captured: PieceType,
    promoted: bool,

}

pub struct Board {
    // Mailbox: 0x88
    mailbox: [PieceType; 128],
    bitboards: [u64; 16],
    unmake_stack: Vec<Unmove>

}

impl Board {
    pub fn new_board_startpos() -> Board {
        //use PieceType::{None, WPawn, WKnight, WBishop, WRook, WQueen, WKing, BPawn, BKnight, BBishop, BRook, BQueen, BKing};
        use PieceType::*;
        Board {
            mailbox: [
                WRook,  WKnight,    WBishop,    WQueen, WKing,  WBishop,    WKnight,    WRook,  None,   None,   None,   None,   None,   None,   None,   None,   
                WPawn,  WPawn,      WPawn,      WPawn,  WPawn,  WPawn,      WPawn,      WPawn,  None,   None,   None,   None,   None,   None,   None,   None,   
                None,   None,       None,       None,   None,   None,       None,       None,   None,   None,   None,   None,   None,   None,   None,   None,             
                None,   None,       None,       None,   None,   None,       None,       None,   None,   None,   None,   None,   None,   None,   None,   None,             
                None,   None,       None,       None,   None,   None,       None,       None,   None,   None,   None,   None,   None,   None,   None,   None,             
                None,   None,       None,       None,   None,   None,       None,       None,   None,   None,   None,   None,   None,   None,   None,   None,             
                BPawn,  BPawn,      BPawn,      BPawn,  BPawn,  BPawn,      BPawn,      BPawn,  None,   None,   None,   None,   None,   None,   None,   None,   
                BRook,  BKnight,    BBishop,    BQueen, BKing,  BBishop,    BKnight,    BRook,  None,   None,   None,   None,   None,   None,   None,   None,   

            ],

            bitboards: [0; 16],  // Use empty bitboards for now (yes that's gross)

            unmake_stack: Vec::new()
        }
    }

    pub fn make(&mut self, cmove: Move) {
        // First add the information to undo the move to the stack
        self.unmake_stack.push( Unmove{
            from: cmove.from,
            to: cmove.to,
            captured: self.mailbox[cmove.to.0],
            promoted: false,
        });

        // Now move the piece on the mailbox
        self.mailbox[cmove.to.0] = self.mailbox[cmove.from.0];
    }
}