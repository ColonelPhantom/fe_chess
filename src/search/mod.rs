use crate::board;
use board::Board;

mod alphabeta;
mod quiesce;
pub mod transtable;

#[derive(Debug, Clone, Copy)]
pub enum Score {
    Draw,
    Value(i32),
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
        Some( self.cmp(o) )
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
            }
            Score::Value(p) => match o {
                Score::Draw => p.cmp(&0),
                Score::Value(px) => p.cmp(px),
                Score::Win(_) => Ordering::Less,
                Score::Loss(_) => Ordering::Greater,
            }
            Score::Win(d) => match o {
                Score::Win(dx) => dx.cmp(d),
                _ => Ordering::Greater,
            }
            Score::Loss(d) => match o {
                Score::Loss(dx) => d.cmp(dx),
                _ => Ordering::Less,
            }
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
            }
            Score::Value(p) => match o {
                Score::Draw => *p == 0,
                Score::Value(px) => p == px,
                _ => false
            }
            Score::Win(d) => match o {
                Score::Win(dx) => d == dx,
                _ => false
            }
            Score::Loss(d) => match o {
                Score::Loss(dx) => d == dx,
                _ => false
            }
        }
    }
}
impl std::cmp::Eq for Score {}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            Score::Draw => write!(f, "Draw"),
            Score::Value(p) => match p {
                p if p > &0 => write!(f, "+{}", p),
                p => write!(f, "{}", p),
            }
            Score::Win(d) => write!(f, "#{}", d),
            Score::Loss(d) => write!(f, "#-{}", d),
        }
    }
}



#[derive(Debug, Clone)]
pub struct SearchInfo {
    pub score: Score,
    pub pv: Vec<board::Move>,
}

pub fn search(b: &mut Board, depth: usize, tt: &mut transtable::TransTable) -> SearchInfo
{
    let mut si = alphabeta::alpha_beta(b, Score::Loss(0), Score::Win(0), 1, &mut vec![], tt);
    for d in 1..depth+1 {
        si = alphabeta::alpha_beta(b, Score::Loss(0), Score::Win(0), d, &mut si.pv, tt);
    }
    println!("Transtable filled with {} entries (capacity {})", tt.filled(), tt.len);
    return si;
}