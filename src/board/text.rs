use super::*;
impl Move {
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

impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut b = Board::new_empty();
        let fen = fen.trim();

        // Board occupation
        let mut rank: isize = 7;
        let mut file: isize = 0;
        let mut str_pos = 0;
        for (i, c) in fen.chars().enumerate() {
            str_pos = i;
            print!("{}", c);
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                    continue;
                }
                '1' ..= '8' => {
                    file += c.to_digit(10).unwrap() as isize;
                    continue;
                },
                'P' => b[c0x88(file, rank)] = pieces::WPAWN,
                'N' => b[c0x88(file, rank)] = pieces::WKNIGHT,
                'B' => b[c0x88(file, rank)] = pieces::WBISHOP,
                'R' => b[c0x88(file, rank)] = pieces::WROOK,
                'Q' => b[c0x88(file, rank)] = pieces::WQUEEN,
                'K' => b[c0x88(file, rank)] = {
                    b.king_pos[WHITE as usize] = c0x88(file, rank);
                    pieces::WKING
                },
                'p' => b[c0x88(file, rank)] = pieces::BPAWN,
                'n' => b[c0x88(file, rank)] = pieces::BKNIGHT,
                'b' => b[c0x88(file, rank)] = pieces::BBISHOP,
                'r' => b[c0x88(file, rank)] = pieces::BROOK,
                'q' => b[c0x88(file, rank)] = pieces::BQUEEN,
                'k' => b[c0x88(file, rank)] = {
                    b.king_pos[BLACK as usize] = c0x88(file, rank);
                    pieces::BKING
                },
                ' ' => break,
                _ => panic!("Invalid character {} in FEN at position {}", c, i),
            }
            file += 1;
        }

        // Side to move
        let fen = fen[str_pos+1 ..].trim();
        println!("{}", fen);
        b.side_to_move = match fen.chars().nth(0).expect("FEN too short") {
            'w' => WHITE,
            'b' => BLACK,
            wildcard => panic!("Invalid side to move {} (i = {}", wildcard, str_pos + 1),
        };

        // Castling
        let fen = fen[1..].trim();
        str_pos = 0;
        for (i,c) in fen.chars().enumerate() {
            str_pos = i;
            match c {
                ' ' => break,
                '-' => break,
                'k' => b.castling[CR_KING + 0],
                'K' => b.castling[CR_KING + 1],
                'q' => b.castling[CR_QUEEN + 0],
                'Q' => b.castling[CR_QUEEN + 1],
                _ => panic!("Invalid castling rights {}", c),
            };
        }

        // Enpassant
        let fen = fen[str_pos..].trim();
        match fen.chars().nth(0).expect("Fen too short") {
            '-' => {},
            'a'..='h' => {
                let file = fen.chars().nth(0).unwrap() as isize - 'a' as isize;
                let rank = fen.chars().nth(1).unwrap() as isize - '1' as isize;
                b.en_passant = Some(match rank {
                    2 => c0x88(file, 3),
                    5 => c0x88(file, 4),
                    _ => panic!("Wrong ep rank in fen! {}", rank),
                });
            }
            _ => panic!("Invalid enpassant"),
        }

        // TODO: halfmove and fullmove counter

        b.zobrist = b.zobrist_init();


        return b;
    }

    pub fn to_fen(&self) -> &str {
        return "";
    }
}