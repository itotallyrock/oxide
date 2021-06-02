use std::fmt::{Debug, Display};
use crate::game::Position;
use std::ops::{BitOr, BitAnd, Not};
use std::convert::TryFrom;

/// Castle rights representation (which side can still legally castle and queen-side vs king-side)
pub trait CastleRights<P: Position>: Sized + Copy + Debug + Display + Eq + PartialEq + BitOr + BitAnd + Not {
    /// No castle rights either side (-)
    const NONE: Self;
    /// White king-side (K)
    const WHITE_KING: Self;
    /// White queen-side (Q)
    const WHITE_QUEEN: Self;
    /// White all (KQ)
    const WHITE_ALL: Self;
    /// Black king-side (k)
    const BLACK_KING: Self;
    /// Both sides king-side (Kk)
    const BOTH_KINGS: Self;
    /// White queen-side, black king-side (Qk)
    const WHITE_QUEEN_BLACK_KING: Self;
    /// White all, black king-side (KQk)
    const WHITE_ALL_BLACK_KING: Self;
    /// Black queen-side (q)
    const BLACK_QUEEN: Self;
    /// White king-side, black queen-side (Kq)
    const WHITE_KING_BLACK_QUEEN: Self;
    /// Both sides can queen-side (Qq)
    const BOTH_QUEENS: Self;
    /// White all, black queen-side (KQq)
    const WHITE_ALL_BLACK_QUEEN: Self;
    /// Black all (kq)
    const BLACK_ALL: Self;
    /// White king-side, black all (Kkq)
    const BLACK_ALL_WHITE_KING: Self;
    /// White queen-side, black all (Qkq)
    const BLACK_ALL_WHITE_QUEEN: Self;
    /// All possible castles (KQkq)
    const ALL: Self;

    // Helpers
    /// Get the castle rights for only a single side
    fn for_side(&self, side: P::Side) -> Self;
    /// If one set of castle rights entirely contains another (ie. KQkq contains Kq)
    fn contains(&self, other: Self) -> bool;
    /// If one set of castle rights shares a castle in common with another (ie. KQ intersects Q)
    fn intersects(&self, other: Self) -> bool;
    // Mutation
    /// Add a set of castle rights to another
    fn insert(&mut self, other: Self);
    /// Remove a set of castle rights from another
    fn remove(&mut self, other: Self);
    /// Get a board mask for the path required for a single castle right
    fn castle_path(&self) -> P::BoardMask;
}