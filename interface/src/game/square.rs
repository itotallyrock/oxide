use std::fmt::Display;

use crate::game::{BoardMask, Shiftable};
use crate::types::SquareOffset;

pub trait Square<BB: BoardMask, const COUNT: usize>: Display + Shiftable + Copy {
    /// List of all available squares
    const SQUARES: [Self; COUNT];
    /// Get a single square from a little endian offset from A1 (None if offset is off board)
    fn from_offset(offset: SquareOffset) -> Option<Self>;
    /// Get a single square from a mask
    fn from_mask(mask: BB) -> Option<Self>;
    /// Get the 0 based x-offset (columns from A File) of a square
    fn x_offset(self) -> SquareOffset;
    /// Get the 0 based y-offset (rows from Rank 1) of a square
    fn y_offset(self) -> SquareOffset;
    /// Get a board mask for the current square
    fn to_mask(self) -> BB;
    /// Get the little endian offset
    fn offset(self) -> SquareOffset;
}