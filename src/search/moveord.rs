use crate::board;
use crate::search::transtable::TransTable;
use crate::search::transtable::TtEntry;
use crate::search::see;

const SEE_WEIGHT: i32 = 1;
const EVAL_WEIGHT: i32 = 1;
const CHECK_BONUS: i32 = 10000;
const TT_BONUS: i32 = 500;
const TT_QUIESCE_BONUS: i32 = 1;

pub fn move_priority(m: &board::Move, b: &mut board::Board, _tt: &TransTable, baseline_eval: crate::eval::ValCp, tt_entry: Option<&TtEntry>) -> i32 {
    let tt_bonus = match tt_entry {
        Some(e) if e.depthleft > 1 => {
            match e.get_move() {
                None => 0,
                Some(tm) => if tm == *m {
                    TT_BONUS * (e.depthleft - 1) as i32
                } else {
                    0
                }
            }
        }
        Some(e) => {
            match e.get_move() {
                None => 0,
                Some(tm) => if tm == *m {
                    TT_QUIESCE_BONUS
                } else {
                    0
                }
            }
        }
        _ => 0
    };
    b.make(m);
    let static_eval_score = (crate::eval::eval(b) - baseline_eval) as i32 * match !b.side_to_move {
        // If white is making m, higher is better
        board::WHITE => 1,
        board::BLACK => -1,
    };
    let check_bonus = CHECK_BONUS * match b.is_check(b.side_to_move) {
        board::ThreatInfo::Safe => 0,
        board::ThreatInfo::Single(_c) => 1,
        board::ThreatInfo::Multiple(v) => v.len() as i32,
    };
    b.unmake();
    // let static_eval_score = 0;
    return 
        see::see_capt(b, m, b.side_to_move) as i32 * SEE_WEIGHT +
        static_eval_score * EVAL_WEIGHT +
        check_bonus +
        tt_bonus
    ;
}