use crate::movegen;
use crate::board;
use board::Board;

use super::quiesce::quiesce;
use super::SearchInfoIntm;
use super::Score;
use super::transtable::TransTable;
use super::NodeType;

const NODES_REDUCE: [usize; 6] = [2, 4, 8, 16, 32, 64];

pub fn alpha_beta(b: &mut Board, mut alpha: Score, beta: Score, depthleft: usize, tt: &mut TransTable)
 -> SearchInfoIntm
{
    let mut local_alpha = Score::Loss(0);
    let mut best_move: Option<board::Move> = None;
    let mut nodes = 1;

    if depthleft > std::usize::MAX / 2 {
        panic!("Depth < 0");
    }

    if depthleft == 0 {
        return SearchInfoIntm{
            score: quiesce(b, alpha, beta, 1, tt),
            nodes: 1
        };
    }

    let eval;
    let baseline_eval;

    let tt_e = tt.get(b.zobrist);

    match tt_e {
        None => {
            baseline_eval = crate::eval::eval(b);
            eval = baseline_eval * match b.side_to_move {
                board::WHITE => 1,
                board::BLACK => -1,
            };
        },
        Some(tt_entry) => {
            match tt_entry.node_type {
                NodeType::None => panic!("Tt.get returned some but type is None"),
                NodeType::QuiesceEval | NodeType::QuiesceFull | NodeType::QuiesceCut => {
                    eval = tt_entry.eval.unwrap();
                    baseline_eval = eval * match b.side_to_move {
                        board::WHITE => 1,
                        board::BLACK => -1,
                    };
                }
                NodeType::AllNode | NodeType::PvNode => {
                    // TODO: use decided position even when insufficient depth
                    if tt_entry.depthleft >= depthleft as i16 {
                        if beta <= tt_entry.beta {
                            return SearchInfoIntm{
                                score: tt_entry.eval_score,
                                nodes
                            };
                        } else {
                            local_alpha = tt_entry.eval_score;
                            if local_alpha > alpha {
                                alpha = local_alpha;
                            }
                        }
                    }
                    eval = tt_entry.eval.unwrap();
                    baseline_eval = eval * match b.side_to_move {
                        board::WHITE => 1,
                        board::BLACK => -1,
                    }
                }
                NodeType::CutNode => {
                    if tt_entry.depthleft >= depthleft as i16 && tt_entry.eval_score >= beta {
                        return SearchInfoIntm {
                            score: tt_entry.eval_score,
                            nodes
                        };
                    } else {
                        eval = tt_entry.eval.unwrap();
                        baseline_eval = eval * match b.side_to_move {
                            board::WHITE => 1,
                            board::BLACK => -1,
                        };
                    }
                }
            }
        }
    }
    
    let mut moves = movegen::movegen(b);

    // Sort decending by priority given
    if depthleft > 1 {
        moves.sort_by_cached_key(|m| {
            -super::moveord::move_priority(m, b, tt, baseline_eval, tt_e)
        });
    }

    // Late move reduction
    let mut lmr_reduction = 0;
    let mut nodes_searched = 0;

    if moves.len() == 0 {
        if !b.is_check(b.side_to_move).is_safe() {
            return SearchInfoIntm {
                score: Score::Loss(0),
                nodes,
            }
        } else {
            return SearchInfoIntm {
                score: Score::Draw,
                nodes,
            }
        }
    }

    // if let Some(m) = tt_move {
    //     moves.insert(0, m);
    // }

    for m in moves {
        b.make(&m);
        if !b.is_check(!b.side_to_move).is_safe() {
            b.unmake();
            continue;
        }
        let si = alpha_beta(b, -beta, -alpha, (depthleft - 1) - lmr_reduction, tt);
        let score_lmr = match -si.score {
            Score::Win(d) => Score::Win(d+1),
            Score::Loss(d) => Score::Loss(d+1),
            Score::Value(p) => Score::Value(p),
            Score::Draw => Score::Draw,
        };
        nodes += si.nodes;

        let score;
        if lmr_reduction > 0 && (score_lmr >= beta || score_lmr > alpha || score_lmr > local_alpha) {
            let si = alpha_beta(b, -beta, -alpha, depthleft - 1, tt);
            score = match -si.score {
                Score::Win(d) => Score::Win(d+1),
                Score::Loss(d) => Score::Loss(d+1),
                Score::Value(p) => Score::Value(p),
                Score::Draw => Score::Draw,
            };
            nodes += si.nodes;
            // println!("Score meaningful with LMR {} (lmr_score = {}, score = {}", lmr_reduction, score_lmr, score);
        } else {
            score = score_lmr;
        }

        b.unmake();

        if score >= beta  {
            // Store self move in TT, move field is refutation move
            tt.put(b.zobrist, Some(m), depthleft as i16, score, NodeType::CutNode, beta, Some(eval));
            return SearchInfoIntm {
                score: beta,
                nodes,
            };
        }
        if score > alpha  {
            // Remember this move to be stored in TT
            best_move = Some(m.clone());
            alpha = score;
            local_alpha = score;
        } else if score > local_alpha {
            best_move = Some(m.clone());
            local_alpha = score;
        }
        // LMR reduction update
        if depthleft > 2 && lmr_reduction < NODES_REDUCE.len() && lmr_reduction < depthleft - 1 && nodes_searched >= NODES_REDUCE[lmr_reduction] {
            lmr_reduction += 1;
        }
        nodes_searched += 1;
    }

    match !(local_alpha < alpha) {  // match alpha_raised
        false => tt.put(b.zobrist, best_move, depthleft as i16, local_alpha, NodeType::AllNode, beta, Some(eval)),
        true => tt.put(b.zobrist, best_move, depthleft as i16, local_alpha, NodeType::PvNode, beta, Some(eval)),
    };

    return SearchInfoIntm {
        score: local_alpha,
        nodes
    }
}
