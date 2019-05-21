use crate::board;

pub fn see(b: &mut board::Board, sq: board::Coord0x88, side: board::Side) -> i32 {
    let att = b.under_attack(sq, !side);    // Who of us can attack the opponent in sq?
    // Get the least valuable attacker
    let lva = match att {
        board::ThreatInfo::Safe => {return 0;}, 
        board::ThreatInfo::Single(a) => a,
        board::ThreatInfo::Multiple(mut va) => {
            va.sort_unstable_by(|c1,c2| {
                crate::eval::piece_val(b[*c1].piece_type).cmp(&crate::eval::piece_val(b[*c2].piece_type))
            });
            *va.first().unwrap()
        }
    };

    // TODO: make see consider pawn promotion

    b.make(&board::Move::new(lva, sq));
    let value = std::cmp::max(0, crate::eval::piece_val(b[sq].piece_type) - see(b, sq, !side));
    b.unmake();

    return value;
}

pub fn see_capt(b: &mut board::Board, m: &board::Move, side: board::Side) -> i32 {
    let captured = crate::eval::piece_val(b[m.to].piece_type);
    b.make(m);
    let value = captured - see(b, m.to, !side);
    b.unmake();
    return value;

}