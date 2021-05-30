
#![cfg_attr(test, allow(soft_unstable))]
#![cfg_attr(test, feature(test))]
#![cfg_attr(feature = "sliding_attack_lookup", const_eval_limit ="0")]
#![cfg_attr(feature = "sliding_attack_lookup", feature(const_eval_limit))]

mod attacks;

use interface::game::{ChessMove, Piece, SimpleChessMove, Board, Position};
use smallvec::SmallVec;
use std::hash::Hasher;

// TODO: Tune this value (be just above average for number of moves so that most move generation calls don't need any reallocation on the heap)
const BASE_MOVES_CAPACITY: usize = 50;

pub fn legal_moves<P: Position, M: ChessMove<P>>(position: &P) -> impl Iterator<Item=M> {
    let mut list = SmallVec::<[M; BASE_MOVES_CAPACITY]>::new();

    let side = position.side_to_move();
    for from_square in position.piece_mask_for_side(side) {

    }

    list.push(M::BLACK_KING_CASTLE);

    list.into_iter()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
