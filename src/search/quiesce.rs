use crate::eval::eval;
use crate::movegen;
use crate::board;
use board::Board;


pub fn quiesce(b: &mut Board, mut alpha: isize, beta:isize ) -> isize {
    let sign = match b.side_to_move {
        board::WHITE => 1,
        board::BLACK => -1,
    };
    return sign * eval(b);
    
    let e = sign * eval(b);
    if e >= beta  {
        return beta;
    }
    if alpha < e  {
        alpha = e;
    }

    /*until( every_capture_has_been_examined )  {
        MakeCapture();
        score = -Quiesce( -beta, -alpha );
        TakeBackMove();

        if( score >= beta )
            return beta;
        if( score > alpha )
           alpha = score;
    }*/
    return alpha;
}