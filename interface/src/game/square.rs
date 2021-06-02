use std::fmt::Display;

use crate::game::{Shiftable, Position};
use crate::types::SquareOffset;

/// Square representation, a coordinate system for the board
pub trait Square<P: Position>: Display + Shiftable + Copy + PartialOrd + Ord + Eq + PartialEq {
    /// List of all available squares
    const SQUARES: [Self; 64];
    /// Get a single square from a little endian offset from A1 (None if offset is off board)
    fn from_offset(offset: SquareOffset) -> Option<Self>;
    /// Get a single square from a mask
    fn from_mask(mask: P::BoardMask) -> Option<Self>;
    /// Get the 0 based x-offset (columns from A File) of a square
    fn x_offset(&self) -> SquareOffset;
    /// Get the 0 based y-offset (rows from Rank 1) of a square
    fn y_offset(&self) -> SquareOffset;
    /// Get a board mask for the current square
    fn to_mask(&self) -> P::BoardMask;
    /// Get the little endian offset
    fn offset(&self) -> SquareOffset;
}