use std::ops::{Shr, Shl};
use crate::types::SquareOffset;

pub trait Shiftable: Shl<SquareOffset, Output = Self> + Shr<SquareOffset, Output = Self> + Sized {
    fn north_shift(self) -> Self;
    fn south_shift(self) -> Self;
    fn east_shift(self) -> Self;
    fn west_shift(self) -> Self;
    fn north_east_shift(self) -> Self;
    fn north_west_shift(self) -> Self;
    fn south_east_shift(self) -> Self;
    fn south_west_shift(self) -> Self;
}