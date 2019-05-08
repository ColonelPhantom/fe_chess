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
        return alphabeta::alpha_beta(b, std::isize::MIN + 100, std::isize::MAX - 100, depth)
}