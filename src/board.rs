#[derive(Copy, Clone)]
#[repr(u8)]
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

type Coord88 = u8;

struct Unmake {
    from: Coord88,
    tp: Coord88,
    captured: PieceType
}

pub struct Board {
    // Mailbox: 0x88
    mailbox: [PieceType; 128],
    bitboards: [u64; 16],
    unmake_stack: Vec<Unmake>

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
}