use std::fmt::{Debug, Display};

/// Side representing a turn to move as well as set of owned pieces
pub trait Side: Copy + Sized + Debug + Display + Debug + Eq + PartialEq {
    /// Array of sides for iteration
    const SIDES: [Self; 2];
    /// Trait constant representing white
    const WHITE: Self;
    /// Trait constant representing black
    const BLACK: Self;
    /// Get the opposite side, (ie. white -> black and black -> white)
    fn opposite_side(&self) -> Self;
    /// If this side is white
    fn is_white(&self) -> bool;
    /// If this side is black
    fn is_black(&self) -> bool;
}