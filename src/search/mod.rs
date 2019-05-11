use crate::board;
use board::Board;

mod alphabeta;
mod quiesce;

#[derive(Debug, Clone)]
pub struct SearchInfo {
    pub score: isize,
    pub pv: Vec<board::Move>,
}

pub fn search(b: &mut Board, depth: usize)
 -> SearchInfo
{
    let mut si = alphabeta::alpha_beta(b, std::isize::MIN + 100, std::isize::MAX - 100, 1, &mut vec![]);
    for d in 2..depth+1 {
        si = alphabeta::alpha_beta(b, std::isize::MIN + 100, std::isize::MAX - 100, d, &mut si.pv);
    }

    return si;
}