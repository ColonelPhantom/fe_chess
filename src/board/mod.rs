use std::num::Wrapping;

mod zobrist;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

pub fn c8x8(file: isize, rank: isize) -> Coord8x8 {
    (8*rank + file) as usize
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
impl ThreatInfo {
    pub fn is_safe(&self) -> bool {
        match self {
            ThreatInfo::Safe => true,
            ThreatInfo::Single{c: _} => false,
            ThreatInfo::Multiple{c: _} => false,
        }
    }
}

pub type CastlingRights = [bool; 4];
pub const CR_QUEEN: usize = 2;
pub const CR_KING: usize = 0;
// CastlingRights[CR_{QUEEN|KING} + side_to_move as usize]

#[derive(Debug, Clone)]
pub enum EnPassantState {
    None,
    Possible ( Coord0x88 ),
    Capture ( Coord0x88 ),
}

#[derive(Debug, Clone)]
pub struct Move {
    pub from: Coord0x88,
    pub to: Coord0x88,
    pub promote_to: PieceType,
    pub en_passant: EnPassantState,
    pub castling: Option<usize>,
}
impl Move {
    pub fn new(from: Coord0x88, to: Coord0x88) -> Self {
        Move {
            from: from,
            to: to,
            promote_to: PieceType::None,
            en_passant: EnPassantState::None,
            castling: None,
        }
    }

    pub fn from_str(s: &str, b: &Board) -> Result<Self, ()> {
        match s {
            "O-O" => {
                let from = b.king_pos[b.side_to_move as usize];
                let to = from + o0x88(2, 0);

                return Ok(Move{
                    castling: Some(CR_KING),
                    from, to,

                    en_passant: EnPassantState::None,
                    promote_to: PieceType::None,
                });
            }
            "O-O-O" => {
                let from = b.king_pos[b.side_to_move as usize];
                let to = from + o0x88(-2, 0);

                return Ok(Move{
                    castling: Some(CR_QUEEN),
                    from, to,

                    en_passant: EnPassantState::None,
                    promote_to: PieceType::None,
                });
            }
            _ => {}
        }
        
        let from = c0x88(
            s.chars().nth(0).expect("Error parsing move") as isize - 'a' as isize,
            s.chars().nth(1).expect("Error parsing move") as isize - '1' as isize
        );
        let to = c0x88(
            s.chars().nth(2).expect("Error parsing move") as isize - 'a' as isize,
            s.chars().nth(3).expect("Error parsing move") as isize - '1' as isize
        );
        
        // Pawn handling
        let enp;
        let prom;
        if b[from].piece_type == PieceType::Pawn {
            if ( to - from == o0x88(0, 2) ) || ( from - to == o0x88(0, 2) ) {
                enp = EnPassantState::Possible(to);
            } else if b.en_passant.is_some() &&
                    (( b.en_passant.unwrap() == to + o0x88(0,  1)  && b.side_to_move == BLACK ) ||
                     ( b.en_passant.unwrap() == to + o0x88(0, -1)  && b.side_to_move == WHITE ))
            {
                enp = EnPassantState::Capture(b.en_passant.unwrap());
            } else {
                enp = EnPassantState::None;
            }

            if to.0 >> 4 == 0 || to.0 >> 4 == 7 {
                if s.len() < 5 {
                    return Err(());
                }
                match s.chars().nth(4).unwrap() {
                    'q' => prom = PieceType::Queen,
                    'n' => prom = PieceType::Knight,
                    'r' => prom = PieceType::Rook,
                    'b' => prom = PieceType::Bishop,
                    _ => return Err(()),
                };
            } else {
                prom = PieceType::None;
            }
        } else {
            enp = EnPassantState::None;
            prom = PieceType::None;
        }     
        
        Ok(Move{
            castling: None,
            en_passant:enp,
            from, to,
            promote_to: prom
        })
    }
}
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}",
            std::char::from_u32('a' as u32 + (self.from.0 & 0x7) as u32).expect("Error trying to turn move into text"),
            std::char::from_u32('1' as u32 + ((self.from.0 >> 4) & 0x7) as u32).expect("Error trying to turn move into text"),
            std::char::from_u32('a' as u32 + (self.to.0 & 0x7) as u32).expect("Error trying to turn move into text"),
            std::char::from_u32('1' as u32 + ((self.to.0 >> 4) & 0x7) as u32).expect("Error trying to turn move into text"),
            match self.promote_to {
                PieceType::Queen => 'q',
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                _ => '\0',
            }            
        )
    }
}

pub struct Unmove {
    pub from: Coord0x88,
    pub to: Coord0x88,
    pub captured: Piece,
    pub promoted: bool,
    pub revmov_clock: usize,
    pub check_cache: Option<ThreatInfo>,
    pub en_passant: EnPassantState,
    pub castling: Option<usize>,
    pub castling_rights: CastlingRights,
    pub zobrist: u64,
}



pub struct Board {
    pub mailbox: [Piece; 128],      // 0x88
    //pub bitboards: [u64; 16],
    pub unmake_stack: Vec<Unmove>,
    pub side_to_move: Side,
    pub en_passant: Option<Coord0x88>,
    pub revmov_clock: usize,
    pub king_pos: [Coord0x88; 2],
    check_cache: Option<ThreatInfo>,
    pub castling: CastlingRights,
    pub zobrist: u64
}

impl Board {
    pub fn new() -> Board {
        use pieces::*;
        let mut b = Board {
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
            unmake_stack: Vec::new(),
            side_to_move: WHITE,
            revmov_clock: 0,
            en_passant: None,
            king_pos: [c0x88::e1, c0x88::e8],
            check_cache: None,
            castling: [true, true, true, true],
            zobrist: 0,
        };
        b.zobrist = b.zobrist_init();
        return b;
    }

    pub fn make(&mut self, cmove: &Move) {
        // Begin with determining info on the move
        let captured: Piece = self[cmove.to];
        let promoted;
        let revmov_clock = self.revmov_clock;
        let mut undo_ep = EnPassantState::None;
        let undo_castlerights = self.castling;
        let undo_zobrist = self.zobrist;
        let undo_castling;

        if captured.piece_type != PieceType::None {
            let zob_tab = match captured.piece_type {
                PieceType::Pawn => match self.side_to_move {
                    WHITE => &zobrist::WPAWN,
                    BLACK => &zobrist::BPAWN,
                }
                PieceType::Knight => match self.side_to_move {
                    WHITE => &zobrist::WKNIGHT,
                    BLACK => &zobrist::BKNIGHT,
                }
                PieceType::Bishop => match self.side_to_move {
                    WHITE => &zobrist::WBISHOP,
                    BLACK => &zobrist::BBISHOP,
                }
                PieceType::Rook => match self.side_to_move {
                    WHITE => &zobrist::WROOK,
                    BLACK => &zobrist::BROOK,
                }
                PieceType::Queen => match self.side_to_move {
                    WHITE => &zobrist::WQUEEN,
                    BLACK => &zobrist::BQUEEN,
                }
                PieceType::King => match self.side_to_move {
                    WHITE => &zobrist::WKING,
                    BLACK => &zobrist::BKING,
                }
                _ => panic!("Capture of unexpected piece type"),
            };
            self.zobrist ^= zob_tab[coord0x88_to8x8(cmove.to)];
        }

        if let Some(c) = cmove.castling {
            undo_castling = Some(c);

            // Only handle rook movement and castling rights
            // King movement is handled by regular code
            let kc = self.king_pos[self.side_to_move as usize];
            let zob_tab = match self.side_to_move {
                WHITE => &zobrist::WROOK,
                BLACK => &zobrist::BROOK,
            };
            match c {
                CR_KING => {
                    self[kc + o0x88(1, 0)] = self[kc + o0x88(3, 0)];
                    self[kc + o0x88(3, 0)] = pieces::NONE;
                    self.zobrist ^= zob_tab[coord0x88_to8x8(kc + o0x88(1, 0))];
                    self.zobrist ^= zob_tab[coord0x88_to8x8(kc + o0x88(3, 0))];
                }
                CR_QUEEN => {
                    self[kc + o0x88(-1, 0)] = self[kc + o0x88(-4, 0)];
                    self[kc + o0x88(-4, 0)] = pieces::NONE;
                    self.zobrist ^= zob_tab[coord0x88_to8x8(kc + o0x88(-1, 0))];
                    self.zobrist ^= zob_tab[coord0x88_to8x8(kc + o0x88(-4, 0))];
                }
                _ => panic!("Castling not with value CR_KING or CR_QUEEN")
            }
        } else {
            undo_castling = None;
        }

        if self.en_passant.is_some() {
            undo_ep = EnPassantState::Possible(self.en_passant.unwrap());
            self.zobrist ^= zobrist::ENPASSANT[self.en_passant.unwrap().0 & 0x7];
        }

        match cmove.en_passant {
            EnPassantState::None => {
                self.en_passant = None;
            }
            EnPassantState::Possible(c) => {
                self.en_passant = Some(c);
                self.zobrist ^= zobrist::ENPASSANT[c.0 & 0x7];
            }
            EnPassantState::Capture(c) => {
                self[c] = pieces::NONE;
                undo_ep = EnPassantState::Capture(c);
                self.en_passant = None;
            }
        }


        // Reversible move clock
        if self.occupied(cmove.to) || self[cmove.from].piece_type == PieceType::Pawn {
            self.revmov_clock += 1;
        } else {
            self.revmov_clock = 0;
        }

        if self[cmove.from].piece_type == PieceType::King {
            if self.castling[CR_KING + self.side_to_move as usize] {
                self.castling[CR_KING + self.side_to_move as usize] = false;
                self.zobrist ^= zobrist::CASTLING[CR_KING + self.side_to_move as usize];
            }
            if self.castling[CR_QUEEN + self.side_to_move as usize] {
                self.castling[CR_QUEEN + self.side_to_move as usize] = false;
                self.zobrist ^= zobrist::CASTLING[CR_QUEEN + self.side_to_move as usize];
            }
            // Update kingpos
            self.king_pos[self.side_to_move as usize] = cmove.to;
        }

        if self[cmove.from].piece_type == PieceType::Rook {
            match cmove.from {
                c0x88::a1 => if self.castling[CR_QUEEN + WHITE as usize] { 
                   self.castling[CR_QUEEN + WHITE as usize] = false;
                   self.zobrist ^= zobrist::CASTLING[CR_QUEEN + WHITE as usize];
                },
                c0x88::h1 => if self.castling[CR_KING + WHITE as usize] { 
                   self.castling[CR_KING + WHITE as usize] = false;
                   self.zobrist ^= zobrist::CASTLING[CR_KING + WHITE as usize];
                },
                c0x88::a8 => if self.castling[CR_QUEEN + BLACK as usize] { 
                   self.castling[CR_QUEEN + BLACK as usize] = false;
                   self.zobrist ^= zobrist::CASTLING[CR_QUEEN + BLACK as usize];
                },
                c0x88::h8 => if self.castling[CR_KING + BLACK as usize] { 
                   self.castling[CR_KING + BLACK as usize] = false;
                   self.zobrist ^= zobrist::CASTLING[CR_KING + BLACK as usize];
                },
                _ => {}

            }
        }

        // Pawn promotion
        if cmove.promote_to != PieceType::None {
            self[cmove.to] = Piece {piece_type: cmove.promote_to, color: self.side_to_move};
            self[cmove.from] = pieces::NONE;

            use PieceType::*;
            let zob_from_tab = match self.side_to_move {
                WHITE => &zobrist::WPAWN,
                BLACK => &zobrist::BPAWN,
            };
            let zob_to_tab = match cmove.promote_to {
                Knight => match self.side_to_move {
                    WHITE => &zobrist::WKNIGHT,
                    BLACK => &zobrist::BKNIGHT,
                }
                Bishop => match self.side_to_move {
                    WHITE => &zobrist::WBISHOP,
                    BLACK => &zobrist::BBISHOP,
                }
                Rook => match self.side_to_move {
                    WHITE => &zobrist::WROOK,
                    BLACK => &zobrist::BROOK,
                }
                Queen => match self.side_to_move {
                    WHITE => &zobrist::WQUEEN,
                    BLACK => &zobrist::BQUEEN,
                }
                _ => panic!("invalid promotion"),
            };
            self.zobrist ^= zob_from_tab[coord0x88_to8x8(cmove.from)];
            self.zobrist ^= zob_to_tab[coord0x88_to8x8(cmove.to)];
            promoted = true;

        } else { // Non-promoting move: business as usual
            let zob_tab = match self[cmove.from].piece_type {
                PieceType::Pawn => match self.side_to_move {
                    WHITE => &zobrist::WPAWN,
                    BLACK => &zobrist::BPAWN,
                }
                PieceType::Knight => match self.side_to_move {
                    WHITE => &zobrist::WKNIGHT,
                    BLACK => &zobrist::BKNIGHT,
                }
                PieceType::Bishop => match self.side_to_move {
                    WHITE => &zobrist::WBISHOP,
                    BLACK => &zobrist::BBISHOP,
                }
                PieceType::Rook => match self.side_to_move {
                    WHITE => &zobrist::WROOK,
                    BLACK => &zobrist::BROOK,
                }
                PieceType::Queen => match self.side_to_move {
                    WHITE => &zobrist::WQUEEN,
                    BLACK => &zobrist::BQUEEN,
                }
                PieceType::King => match self.side_to_move {
                    WHITE => &zobrist::WKING,
                    BLACK => &zobrist::BKING,
                }
                _ => panic!("Attempting to move a non-piece")
            };
            self[cmove.to] = self[cmove.from];
            self[cmove.from] = pieces::NONE;
            self.zobrist ^= zob_tab[coord0x88_to8x8(cmove.from)];
            self.zobrist ^= zob_tab[coord0x88_to8x8(cmove.to)];
            promoted = false;
        }

        // Add the information to undo the move to the stack

        // Vague but optimized code equivalent to but doesn't conflict with the borrow checker:
        //let in_check = self.in_check
        //self.in_check = None
        let in_check = std::mem::replace(&mut self.check_cache, None);
        // Now actually push
        self.unmake_stack.push( Unmove{
            from: cmove.from,
            to: cmove.to,
            captured: captured,
            promoted: promoted,
            check_cache: in_check,
            revmov_clock: revmov_clock,
            en_passant: undo_ep,
            castling: undo_castling,
            castling_rights: undo_castlerights,
            zobrist: undo_zobrist
        });

        // Update 'trivial' field(s)
        self.side_to_move = !self.side_to_move;
        self.zobrist ^= zobrist::SIDE_TO_MOVE;
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
        self.check_cache = u.check_cache;
        self.revmov_clock = u.revmov_clock;
        self.castling = u.castling_rights;
        self.zobrist = u.zobrist;

        match u.en_passant {
            EnPassantState::None => {
                self.en_passant = None;
            }
            EnPassantState::Possible(c) => {
                self.en_passant = Some(c);
            }
            EnPassantState::Capture(c) => {
                self[c] = Piece{piece_type: PieceType::Pawn, color: !self.side_to_move};
                self.en_passant = Some(c);
            }
        }

        if self[u.from].piece_type == PieceType::King {
            self.king_pos[self.side_to_move as usize] = u.from;
        }

        if u.castling.is_some() {
            // Kingpos is already updated to what it was before move in the above if{}
            let kc = self.king_pos[self.side_to_move as usize];

            match u.castling.unwrap() {
                CR_KING => {
                    self[kc + o0x88(3, 0)] = self[kc + o0x88(1, 0)];
                    self[kc + o0x88(1, 0)] = pieces::NONE;
                }
                CR_QUEEN => {
                    self[kc + o0x88(-4, 0)] = self[kc + o0x88(-1, 0)];
                    self[kc + o0x88(-1, 0)] = pieces::NONE;
                }
                _ => panic!("Uncastling not with value CR_KING or CR_QUEEN")
            }
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
        
        macro_rules! nonslide_threat {($offset:expr, $types:pat) => {
            let to = c+$offset;
            if to.0 & 0x88 == 0 && self.occupied(to) && self[to].color != side {
                match self[to].piece_type {
                    $types => { threats.push(to) }
                    _ => {}
                }
            }
        };}

        macro_rules! slide_threat { ($offset:expr, $types:pat) =>  {
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
            }
        }}
        

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

        match threats.len() {
            0 => ThreatInfo::Safe,
            1 => ThreatInfo::Single{ c: threats[0] },
            _ => ThreatInfo::Multiple { c: threats },
        }

    }

    pub fn is_check(&mut self, side: Side) -> ThreatInfo {
        match &self.check_cache {
            None => {
                let c = self.under_attack(self.king_pos[side as usize], side);
                if side == self.side_to_move {
                    self.check_cache = Some(c.clone());
                }
                return c;
            }
            Some(x) => x.clone()
        }
    }

    pub fn zobrist_init(&self) -> u64 {
        let mut z: u64 = 0;
        for file in 0..8 {
            for rank in 0..8 {
                use pieces::*;
                let c = c0x88(file, rank);
                z ^= match self[c] {
                    NONE => 0,
                    WPAWN => zobrist::WPAWN[c8x8(file, rank)],
                    WKNIGHT => zobrist::WKNIGHT[c8x8(file, rank)],
                    WBISHOP => zobrist::WBISHOP[c8x8(file, rank)],
                    WROOK => zobrist::WROOK[c8x8(file, rank)],
                    WQUEEN => zobrist::WQUEEN[c8x8(file, rank)],
                    WKING => zobrist::WKING[c8x8(file, rank)],
                    BPAWN => zobrist::BPAWN[c8x8(file, rank)],
                    BKNIGHT => zobrist::BKNIGHT[c8x8(file, rank)],
                    BBISHOP => zobrist::BBISHOP[c8x8(file, rank)],
                    BROOK => zobrist::BROOK[c8x8(file, rank)],
                    BQUEEN => zobrist::BQUEEN[c8x8(file, rank)],
                    BKING => zobrist::BKING[c8x8(file, rank)],
                    _ => 0,
                }
            }
        }
        z ^= match self.side_to_move {
            WHITE => 0,
            BLACK => zobrist::SIDE_TO_MOVE,
        };
        if self.en_passant.is_some() {
            z ^= zobrist::ENPASSANT[self.en_passant.unwrap().0 & 7];
        }
        for cr in 0..4 {
            if self.castling[cr] {
                z ^= zobrist::CASTLING[cr];
            }
        }

        return z;
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
