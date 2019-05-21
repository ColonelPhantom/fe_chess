use crate::eval::eval;
use crate::movegen;
use crate::board;
use board::Board;

use super::Score;

const MAX_DELTA: i32 = 1000;
const SEE_DELTA: i32 = 100;

pub fn quiesce(b: &mut Board, mut alpha: Score, beta:Score, qdepth: i16, tt: &mut super::transtable::TransTable ) -> Score {
     let sign = match b.side_to_move {
        board::WHITE => 1,
        board::BLACK => -1,
    };
    
    let sp = sign * eval(b);
    let stand_pat = Score::Value(sp);

    if stand_pat >= beta  {
        return beta;
    }
    if alpha < stand_pat  {
        alpha = stand_pat;
    }

    match tt.get(b.zobrist) {
        None => {},
        Some(tt_entry) => {
            // TODO: maybe TT move ordering?
            return tt_entry.eval_score;
        }
    };

    // Delta pruning
    match alpha {
        Score::Value(a) => if sp < a - MAX_DELTA {
            return alpha;
        },
        Score::Draw => if sp < 0 - MAX_DELTA {
            return alpha;
        }
        Score::Win(_d) => {return stand_pat;},
        Score::Loss(_d) => {}
    };

    

    let cap_moves = movegen::capturegen::cap_gen(b);
    for m in cap_moves {
        let see = super::see::see_capt(b, &m, b.side_to_move);
        if see < 0 {
            //println!("SEE cut");
            continue;
        }
        if let Score::Value(a) = alpha {
            if sp + see < a - SEE_DELTA {
                //println!("SEE delta prune");
                continue;
            }
        }
        b.make(&m);
        if !b.is_check(!b.side_to_move).is_safe() {
            b.unmake();
            continue;
        }
        let score = -quiesce(b, -beta, -alpha, qdepth + 1, tt);
        b.unmake();

        if score >= beta  {
            tt.put(b.zobrist, Some(m), -qdepth, score);
            return beta;
        }
        if score > alpha  {
           alpha = score;
        }
    }
    tt.put(b.zobrist, None, -qdepth, alpha);
    return alpha;
}