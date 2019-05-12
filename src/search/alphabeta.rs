use crate::movegen;
use crate::board;
use board::Board;

use super::quiesce::quiesce;
use super::SearchInfo;

pub fn alpha_beta(b: &mut Board, mut alpha: isize, beta: isize, depthleft: usize, prev_pv: &mut Vec<board::Move>, )
 -> SearchInfo
{
    let mut pv: Vec<board::Move> = vec![];

    if depthleft == 0 {
        return SearchInfo{
            score: quiesce(b, alpha, beta ),
            pv: vec![],
        };
    }

    if let Some(m) = prev_pv.pop() {
        b.make(&m);
        let si = alpha_beta(b, -beta, -alpha, depthleft - 1, prev_pv);
        let score = -si.score;
        b.unmake();
        if score >= beta  {
            return SearchInfo {
                score: beta,
                pv
            };
        }
        if score > alpha  {
            alpha = score;
            pv = si.pv;
            pv.push(m);
        }
    }

    let moves = movegen::movegen(b);

    for m in moves {
        b.make(&m);
        if !b.is_check(!b.side_to_move).is_safe() {
            b.unmake();
            continue;
        }
        let si = alpha_beta(b, -beta, -alpha, depthleft - 1, prev_pv );
        let score = -si.score;
        b.unmake();
        if score >= beta  {
            return SearchInfo {
                score: beta,
                pv
            };
        }
        if score > alpha  {
            alpha = score;
            pv = si.pv;
            pv.push(m);
        }
    }
    return SearchInfo {
        score: alpha,
        pv
    }
}
