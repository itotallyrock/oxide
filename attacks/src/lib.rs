#![cfg_attr(test, allow(soft_unstable))]
#![cfg_attr(test, feature(test))]

use interface::game::{Piece, Square, Position};

pub use crate::king::king_attacks;
pub use crate::knight::knight_attacks;
pub use crate::sliding::{bishop_attacks, queen_attacks, rook_attacks};
pub use crate::pawn::{pawn_attacks, pawn_east_attacks, pawn_west_attacks, pawn_pushes};

mod knight;
mod sliding;
mod king;
mod pawn;

#[inline]
pub fn pseudo_attacks<P: Position>(piece: P::Piece, from_square: P::Square, occupied: P::BoardMask) -> P::BoardMask {
    let from_mask = from_square.to_mask();
    // Im effectively doing a `#[structural_match]` but generically by requiring each enum have the same number of variants
    if piece == P::Piece::PAWN {
        panic!("Pawn attacks unsupported in pseudo-attacks as which side the pawn belongs to is unknown")
    } else if piece == P::Piece::BISHOP {
        bishop_attacks::<P>(from_mask, occupied)
    } else if piece == P::Piece::ROOK {
        rook_attacks::<P>(from_mask, occupied)
    } else if piece == P::Piece::KING {
        king_attacks::<P>(from_mask)
    } else if piece == P::Piece::KNIGHT {
        knight_attacks::<P>(from_mask)
    } else if piece == P::Piece::QUEEN {
        queen_attacks::<P>(from_mask, occupied)
    } else {
        panic!("Cannot get pseudo attacks for empty piece")
    }
}