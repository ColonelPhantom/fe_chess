use crate::board;
use crate::search::transtable::TransTable;
use crate::search::see;
use crate::search::Score;

pub fn move_priority(m: &board::Move, b: &mut board::Board, _tt: &TransTable) -> Score {
    b.make(m);
    let static_eval_score = crate::eval::eval(b) * match b.side_to_move {
        board::WHITE => -1,
        board::BLACK => 1,
    };
    b.unmake();
    // let static_eval_score = 0;
    return Score::Value(see::see_capt(b, m, b.side_to_move) + static_eval_score);
}