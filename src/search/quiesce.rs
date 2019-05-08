use crate::eval::eval;
use crate::movegen;
use crate::board;
use board::Board;


pub fn quiesce(b: &mut Board, mut alpha: isize, beta:isize ) -> isize {
    return eval(b);
    
    let e = eval(b);
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