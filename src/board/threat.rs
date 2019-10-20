use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ThreatInfo {
    Safe,
    Single(Coord0x88),
    Multiple(Vec<Coord0x88>),
}
impl ThreatInfo {
    pub fn is_safe(&self) -> bool {
        match self {
            ThreatInfo::Safe => true,
            ThreatInfo::Single(_c) => false,
            ThreatInfo::Multiple(_c) => false,
        }
    }
}

impl Board {
    pub fn under_attack(&self, c: Coord0x88, side: Side) -> ThreatInfo {
        let mut threats: Vec<Coord0x88> = vec![];

        macro_rules! nonslide_threat {
            ($offset:expr, $types:pat) => {
                let to = c + $offset;
                if to.0 & 0x88 == 0 && self.occupied(to) && self[to].color != side {
                    match self[to].piece_type {
                        $types => threats.push(to),
                        _ => {}
                    }
                }
            };
        }

        macro_rules! slide_threat {
            ($offset:expr, $types:pat) => {
                let mut to = c + $offset;
                while to.0 & 0x88 == 0 {
                    if self.occupied(to) {
                        if self[to].color != side {
                            match self[to].piece_type {
                                $types | PieceType::Queen => threats.push(to),
                                _ => {}
                            }
                        }
                        break;
                    }
                    to += $offset;
                }
            };
        }

        // Pawns
        match side {
            WHITE => {
                nonslide_threat!(o0x88(1, 1), PieceType::Pawn);
                nonslide_threat!(o0x88(-1, 1), PieceType::Pawn);
            }
            BLACK => {
                nonslide_threat!(o0x88(1, -1), PieceType::Pawn);
                nonslide_threat!(o0x88(-1, -1), PieceType::Pawn);
            }
        }
        // Knights
        nonslide_threat!(o0x88(1, 2), PieceType::Knight);
        nonslide_threat!(o0x88(-1, 2), PieceType::Knight);
        nonslide_threat!(o0x88(1, -2), PieceType::Knight);
        nonslide_threat!(o0x88(-1, -2), PieceType::Knight);
        nonslide_threat!(o0x88(2, 1), PieceType::Knight);
        nonslide_threat!(o0x88(-2, 1), PieceType::Knight);
        nonslide_threat!(o0x88(2, -1), PieceType::Knight);
        nonslide_threat!(o0x88(-2, -1), PieceType::Knight);

        // Kings
        nonslide_threat!(o0x88(-1, -1), PieceType::King);
        nonslide_threat!(o0x88(-1, 0), PieceType::King);
        nonslide_threat!(o0x88(-1, 1), PieceType::King);
        nonslide_threat!(o0x88(0, -1), PieceType::King);
        nonslide_threat!(o0x88(0, 1), PieceType::King);
        nonslide_threat!(o0x88(1, -1), PieceType::King);
        nonslide_threat!(o0x88(1, 0), PieceType::King);
        nonslide_threat!(o0x88(1, 1), PieceType::King);

        // Diagonal (bishop + queen)
        slide_threat!(o0x88(1, 1), PieceType::Bishop);
        slide_threat!(o0x88(1, -1), PieceType::Bishop);
        slide_threat!(o0x88(-1, 1), PieceType::Bishop);
        slide_threat!(o0x88(-1, -1), PieceType::Bishop);

        // Hor/vertical (rook + queen)
        slide_threat!(o0x88(1, 0), PieceType::Rook);
        slide_threat!(o0x88(-1, 0), PieceType::Rook);
        slide_threat!(o0x88(0, 1), PieceType::Rook);
        slide_threat!(o0x88(0, -1), PieceType::Rook);

        match threats.len() { 
            0 => ThreatInfo::Safe,
            1 => ThreatInfo::Single(threats[0]),
            _ => ThreatInfo::Multiple(threats),
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
            Some(x) => x.clone(),
        }
    }
}
