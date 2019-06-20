use crate::board;
use board::Board;

mod alphabeta;
mod moveord;
mod quiesce;
mod see;
pub mod transtable;

#[derive(Debug, Clone, Copy)]
pub enum Score {
    Draw,
    Value(crate::eval::ValCp),
    Win(u16),
    Loss(u16),
}
impl std::ops::Neg for Score {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Score::Draw => Score::Draw,
            Score::Value(p) => Score::Value(-p),
            Score::Win(d) => Score::Loss(d),
            Score::Loss(d) => Score::Win(d),
        }
    }
}

impl std::cmp::PartialOrd for Score {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl std::cmp::Ord for Score {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match self {
            Score::Draw => match o {
                Score::Draw => Ordering::Equal,
                Score::Value(px) => 0.cmp(px),
                Score::Win(_) => Ordering::Less,
                Score::Loss(_) => Ordering::Greater,
            },
            Score::Value(p) => match o {
                Score::Draw => p.cmp(&0),
                Score::Value(px) => p.cmp(px),
                Score::Win(_) => Ordering::Less,
                Score::Loss(_) => Ordering::Greater,
            },
            Score::Win(d) => match o {
                Score::Win(dx) => dx.cmp(d),
                _ => Ordering::Greater,
            },
            Score::Loss(d) => match o {
                Score::Loss(dx) => d.cmp(dx),
                _ => Ordering::Less,
            },
        }
    }
}
impl std::cmp::PartialEq for Score {
    fn eq(&self, o: &Self) -> bool {
        match self {
            Score::Draw => match o {
                Score::Draw => true,
                Score::Value(p) => *p == 0,
                _ => false,
            },
            Score::Value(p) => match o {
                Score::Draw => *p == 0,
                Score::Value(px) => p == px,
                _ => false,
            },
            Score::Win(d) => match o {
                Score::Win(dx) => d == dx,
                _ => false,
            },
            Score::Loss(d) => match o {
                Score::Loss(dx) => d == dx,
                _ => false,
            },
        }
    }
}
impl std::cmp::Eq for Score {}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Score::Draw => write!(f, "Draw"),
            Score::Value(p) => match p {
                p if p > &0 => write!(f, "+{}", p),
                p => write!(f, "{}", p),
            },
            Score::Win(d) => write!(f, "#{}", d),
            Score::Loss(d) => write!(f, "#-{}", d),
        }
    }
}

impl Score {
    pub fn is_decided(&self) -> bool {
        match self {
            Score::Value(_v) => false,
            _ => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NodeType {
    PvNode,  // TT: Score is exact
    AllNode, // All-Node: Score is exact (local_alpha)
    CutNode, // Cut-Node: Score is lower bound (might be higher)
    None,

    QuiesceEval,
    QuiesceCut,  // Score might be higher
    QuiesceFull, // Return this score immediately
}

#[derive(Debug, Clone)]
pub struct SearchInfo {
    pub score: Score,
    pub pv: Vec<board::Move>,
    pub nodes: u64,
}

#[derive(Debug, Clone)]
pub struct SearchInfoIntm {
    pub score: Score,
    pub nodes: u64,
}

const ASPIRATION_WIDTH: i16 = 100;

fn aspirate(b: &mut Board, depth: usize, tt: &mut transtable::TransTable, prev_info: SearchInfoIntm) -> SearchInfoIntm {
    // TODO: fix aspiration and mate detection (move maybe correct, score wrong)
    let alpha = match prev_info.score {
        Score::Draw => Score::Value(-ASPIRATION_WIDTH),
        Score::Value(v) => Score::Value(v - ASPIRATION_WIDTH),
        Score::Win(d) => Score::Win(d + 1),
        Score::Loss(d) => Score::Loss(std::cmp::max(d as i32 - 1, 0) as u16),
    };
    let beta = match prev_info.score {
        Score::Draw => Score::Value(ASPIRATION_WIDTH),
        Score::Value(v) => Score::Value (v + ASPIRATION_WIDTH),
        Score::Win(d) => Score::Win(std::cmp::max(d as i32 - 1, 0) as u16),
        Score::Loss(d) => Score::Loss(d + 1),
    };
    // println!("Alpha {}, beta {}, depth {}", alpha, beta, depth);
    let si = alphabeta::alpha_beta(b, alpha, beta, depth, tt, true);
    if si.score >= beta || si.score <= alpha {
        // Aspiration window failed; return full alphabeta
        let new_si = alphabeta::alpha_beta(b, Score::Loss(0), Score::Win(0), depth, tt, false);
        println!("Aspiration failed. Oldscore {}, aspscore {}, actual score {}", prev_info.score, si.score, new_si.score);

        return SearchInfoIntm {
            score: new_si.score,
            nodes: new_si.nodes + si.nodes,
        }
    }
    return si;
}

pub fn search(b: &mut Board, depth: usize, tt: &mut transtable::TransTable) -> SearchInfo
{
    let mut si = alphabeta::alpha_beta(b, Score::Loss(0), Score::Win(0), 0, tt, false);
    let mut nodes = si.nodes;
    for d in 1..=depth {
        si = aspirate(b, d, tt, si);
        // si = alphabeta::alpha_beta(b, Score::Loss(0), Score::Win(0), d, tt);
        nodes += si.nodes;
    }
    // let si = alphabeta::alpha_beta(b, Score::Loss(0), Score::Win(0), depth, tt);
    //println!("Transtable filled with {} entries (capacity {})", tt.filled(), tt.len);
    // if depth > 5 {println!("Node count {}", nodes)};
    return SearchInfo {
        score: si.score,
        pv: tt.get_pv(b),
        nodes: nodes,
    };
}
