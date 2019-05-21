use crate::movegen;
use crate::board;
use board::Board;

pub fn see(b: &board::Board, sq: board::Coord0x88, side: board::Side) -> isize {
    let mut value = 0;
    let mut att = b.under_attack(sq, !side);    // Are we attacking the opponent in sq?
    let lva = match att {
        board::ThreatInfo::Safe => {return value;}, 
        board::ThreatInfo::Single(a) => a,
        board::ThreatInfo::Multiple(mut va) => {
            va.sort_unstable_by(|c1,c2| {
                (b[*c1].piece_type as usize).cmp( &(b[*c2].piece_type as usize ))
            });
            *va.first().unwrap()
        }
    };

    return value;
}