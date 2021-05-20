use std::fmt::{Debug, Display};
use crate::game::Side;
use std::ops::{BitOr, BitAnd, BitXor};

pub trait CastleRights<SideType: Side>: Sized + Debug + Display + Default + Eq + PartialEq + BitOr + BitAnd + BitXor {
    const NONE: Self;
    const WHITE_KING: Self;
    const WHITE_QUEEN: Self;
    const WHITE_ALL: Self;
    const BLACK_KING: Self;
    const BOTH_KINGS: Self;
    const WHITE_QUEEN_BLACK_KING: Self;
    const WHITE_ALL_BLACK_KING: Self;
    const BLACK_QUEEN: Self;
    const WHITE_KING_BLACK_QUEEN: Self;
    const BOTH_QUEENS: Self;
    const WHITE_ALL_BLACK_QUEEN: Self;
    const BLACK_ALL: Self;
    const BLACK_ALL_WHITE_KING: Self;
    const BLACK_ALL_WHITE_QUEEN: Self;
    const ALL: Self;

    fn for_side(self, side: SideType) -> Self;
    fn contains(self, other: Self) -> bool;
    fn intersects(self, other: Self) -> bool;
    fn insert(&mut self, other: Self);
    fn remove(&mut self, other: Self);
}