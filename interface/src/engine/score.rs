use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::types::PlyCount;

pub trait PositionalScore: Sized + Ord + PartialOrd + Eq + PartialEq + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Neg<Output = Self> {
    const MATE_SCORE: Self;
    fn new(centipawns: i32) -> Self;
    fn new_mate(plies_to_mate: PlyCount) -> Self;
    fn is_mate(&self) -> bool;
    fn mate_in(&self) -> Option<PlyCount>;
    fn centipawns(&self) -> i32;
}