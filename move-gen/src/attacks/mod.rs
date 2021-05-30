use interface::game::{Piece, Square, Position};

use crate::attacks::knight::knight_attacks;
use crate::attacks::sliding::{bishop_attacks, queen_attacks, rook_attacks};

mod knight;
mod sliding;

#[inline]
pub fn pseudo_attacks<P: Position>(piece: P::Piece, from_square: P::Square, occupied: P::BoardMask) -> P::BoardMask {
    // Im effectively doing a `#[structural_match]` but generically by requiring each enum have the same number of variants
    if piece == P::Piece::PAWN {
        panic!("pawn attacks unsupported in pseudo attacks")
    } else if piece == P::Piece::BISHOP {
        bishop_attacks::<P>(from_square, occupied) }else if piece == P::Piece::ROOK {
        rook_attacks::<P>(from_square, occupied)
    } else if piece == P::Piece::KING {
        panic!("king attacks unsupported in pseudo attacks")
    } else if piece == P::Piece::KNIGHT {
        knight_attacks::<P>(from_square.to_mask()) }else if piece == P::Piece::QUEEN {
        queen_attacks::<P>(from_square, occupied)
    } else {
        panic!("cannot get pseudo attacks for none piece")
    }
}