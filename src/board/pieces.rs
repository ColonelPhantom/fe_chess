use super::*;
use PieceType::*;
pub const NONE: Piece = Piece {
    piece_type: PieceType::None,
    color: WHITE,
};
pub const WPAWN: Piece = Piece {
    piece_type: Pawn,
    color: WHITE,
};
pub const WKNIGHT: Piece = Piece {
    piece_type: Knight,
    color: WHITE,
};
pub const WBISHOP: Piece = Piece {
    piece_type: Bishop,
    color: WHITE,
};
pub const WROOK: Piece = Piece {
    piece_type: Rook,
    color: WHITE,
};
pub const WQUEEN: Piece = Piece {
    piece_type: Queen,
    color: WHITE,
};
pub const WKING: Piece = Piece {
    piece_type: King,
    color: WHITE,
};
pub const BPAWN: Piece = Piece {
    piece_type: Pawn,
    color: BLACK,
};
pub const BKNIGHT: Piece = Piece {
    piece_type: Knight,
    color: BLACK,
};
pub const BBISHOP: Piece = Piece {
    piece_type: Bishop,
    color: BLACK,
};
pub const BROOK: Piece = Piece {
    piece_type: Rook,
    color: BLACK,
};
pub const BQUEEN: Piece = Piece {
    piece_type: Queen,
    color: BLACK,
};
pub const BKING: Piece = Piece {
    piece_type: King,
    color: BLACK,
};
