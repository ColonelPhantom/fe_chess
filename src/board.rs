enum PieceType {
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

struct Board {
    // Mailbox: 0x88
    mailbox: [PieceType; 128]
}