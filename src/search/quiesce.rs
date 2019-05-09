use crate::eval::eval;
use crate::movegen;
use crate::board;
use board::Board;


pub fn quiesce(b: &mut Board, mut alpha: isize, beta:isize ) -> isize {
    let sign = match b.side_to_move {
        board::WHITE => 1,
        board::BLACK => -1,
    };
    
    let e = sign * eval(b);
    if e >= beta  {
        return beta;
    }
    if alpha < e  {
        alpha = e;
    }

    let cap_moves = movegen::capturegen::cap_gen(b);
    for m in cap_moves {
        b.make(&m);
        if !b.is_check(!b.side_to_move).is_safe() {
            b.unmake();
            continue;
        }
        let score = -quiesce(b, -beta, -alpha );
        b.unmake();

        if score >= beta  {
            return beta;
        }
        if score > alpha  {
           alpha = score;
        }
    }
    return alpha;
}