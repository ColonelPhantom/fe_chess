use crate::movegen;
use crate::board;
use board::Board;

use super::quiesce::quiesce;
use super::SearchInfo;

pub fn alpha_beta(b: &mut Board, mut alpha: isize, beta: isize, depthleft: usize )
 -> SearchInfo
{
   let mut pv: Vec<board::Move> = vec![];

   if depthleft == 0 {
       return SearchInfo{
            score: quiesce(b, alpha, beta ),
            pv: vec![],
       };
   }

   let moves = movegen::movegen(b);

   for m in moves {
      b.make(&m);
      let si = alpha_beta(b, -beta, -alpha, depthleft - 1 );
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
      }
   }
   return SearchInfo {
       score: alpha,
       pv
   }
}
