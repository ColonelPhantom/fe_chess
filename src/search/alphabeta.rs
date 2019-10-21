use crate::board;
use crate::movegen;
use board::Board;

use super::quiesce::quiesce;
use super::transtable::TransTable;
use super::NodeType;
use super::Score;
use super::SearchInfoIntm;

const NODES_REDUCE: [usize; 6] = [2, 4, 8, 16, 32, 64];

pub fn alpha_beta(
    b: &mut Board,
    mut alpha: Score,
    beta: Score,
    depthleft: usize,
    tt: &mut TransTable,
    treedump: &mut std::fs::File,
) -> SearchInfoIntm {
    let mut local_alpha = Score::Loss(0);
    let mut best_move: Option<board::Move> = None;
    let mut nodes = 1;

    use std::io::Write;
    write!(treedump,
        "{{\"id\": \"{:X}\", \"depth\": {}, \"alpha\": \"{}\", \"beta\": \"{}\", ",
        b.zobrist, depthleft, alpha, beta
        );

    if depthleft > std::usize::MAX / 2 {
        panic!("Depth < 0");
    }

    if depthleft == 0 {
        let score = quiesce(b, alpha, beta, 1, tt);
        writeln!(treedump, "\"transtable\": \"-\", \"nodetype\": \"terminus\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, score);
        return SearchInfoIntm{score, nodes};
    }

    let eval;
    let baseline_eval;

    let tt_e = tt.get(b.zobrist);
    write!(treedump, "\"transtable\": \"{:?}\", ", tt_e);

    match tt_e {
        None => {
            baseline_eval = crate::eval::eval(b);
            eval = baseline_eval * match b.side_to_move {
                board::WHITE => 1,
                board::BLACK => -1,
            };
        }
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
                            writeln!(treedump, "\"nodetype\": \"tt-beta\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, tt_entry.beta);
                            return SearchInfoIntm {
                                score: tt_entry.eval_score,
                                nodes,
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
                        writeln!(treedump, "\"nodetype\": \"tt-beta\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, tt_entry.beta);
                        return SearchInfoIntm {
                            score: tt_entry.eval_score,
                            nodes,
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
        moves.sort_by_cached_key(|m| -super::moveord::move_priority(m, b, tt, baseline_eval, tt_e));
    }

    // Late move reduction
    let mut lmr_reduction = 0;
    let mut nodes_searched = 0;

    if moves.len() == 0 {
        if !b.is_check(b.side_to_move).is_safe() {
            writeln!(treedump, "\"nodetype\": \"mate\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, Score::Loss(0));
            return SearchInfoIntm {
                score: Score::Loss(0),
                nodes,
            };
        } else {
            writeln!(treedump, "\"nodetype\": \"pat\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, Score::Draw);
            return SearchInfoIntm {
                score: Score::Draw,
                nodes,
            };
        }
    }

    // if let Some(m) = tt_move {
    //     moves.insert(0, m);
    // }

    writeln!(treedump, "\"children\": [");
    let mut seperator = "";
    for m in moves {
        // LMR reduction update
        nodes_searched += 1;
        if depthleft > 2
            && lmr_reduction < NODES_REDUCE.len()
            && lmr_reduction < depthleft - 3
            && nodes_searched > NODES_REDUCE[lmr_reduction]
        {
            lmr_reduction += 1;
        }

        write!(treedump, "{}", seperator);
        seperator = ", ";
        b.make(&m);
        if !b.is_check(!b.side_to_move).is_safe() {
            write!(treedump,
                "{{\"id\": \"{:X}\", \"depth\": {}, \"alpha\": \"{}\", \"beta\": \"{}\", ",
                b.zobrist, depthleft - 1, -beta, -alpha
            );
            writeln!(treedump, "\"nodetype\": \"selfcheck\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, Score::Loss(0));
            seperator = ", ";
            b.unmake();
            continue;
        }
        let si = alpha_beta(b, -beta, -alpha, (depthleft - 1) - lmr_reduction, tt, treedump);
        let score_lmr = match -si.score {
            Score::Win(d) => Score::Win(d + 1),
            Score::Loss(d) => Score::Loss(d + 1),
            Score::Value(p) => Score::Value(p),
            Score::Draw => Score::Draw,
        };
        nodes += si.nodes;

        let score;
        if lmr_reduction > 0 && (score_lmr >= beta || score_lmr > alpha || score_lmr > local_alpha)
        {
            write!(treedump, ", ");
            let si = alpha_beta(b, -beta, -alpha, depthleft - 1, tt, treedump);
            score = match -si.score {
                Score::Win(d) => Score::Win(d + 1),
                Score::Loss(d) => Score::Loss(d + 1),
                Score::Value(p) => Score::Value(p),
                Score::Draw => Score::Draw,
            };
            nodes += si.nodes;
            // println!("Score meaningful with LMR {} (lmr_score = {}, score = {}", lmr_reduction, score_lmr, score);
        } else {
            score = score_lmr;
        }

        b.unmake();

        if score >= beta {
            // Store self move in TT, move field is refutation move
            tt.put(
                b.zobrist,
                Some(m),
                depthleft as i16,
                score,
                NodeType::CutNode,
                beta,
                Some(eval),
            );
            writeln!(treedump, "], \"nodetype\": \"beta\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, score);
            return SearchInfoIntm { score: beta, nodes };
        }
        if score > alpha {
            // Remember this move to be stored in TT
            best_move = Some(m.clone());
            alpha = score;
            local_alpha = score;
        } else if score > local_alpha {
            best_move = Some(m.clone());
            local_alpha = score;
        }
    }
    write!(treedump, "], ");


    match !(local_alpha < alpha) {
        // match alpha_raised
        false => {
            tt.put(
                b.zobrist,
                best_move,
                depthleft as i16,
                local_alpha,
                NodeType::AllNode,
                beta,
                Some(eval),
            );
            writeln!(treedump, "\"nodetype\": \"allnode\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, local_alpha);
        },
        true => {
            tt.put(
                b.zobrist,
                best_move,
                depthleft as i16,
                local_alpha,
                NodeType::PvNode,
                beta,
                Some(eval),
            );
            writeln!(treedump, "\"nodetype\": \"pvnode\", \"nodes\": {}, \"score\": \"{}\"}}", nodes, local_alpha);
        },
    };

    return SearchInfoIntm {
        score: local_alpha,
        nodes,
    };
}
