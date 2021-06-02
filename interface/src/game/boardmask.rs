use crate::game::{Shiftable, Position};
use std::ops::{BitOr, BitXor, Not, BitAnd, BitOrAssign, BitAndAssign, BitXorAssign};
use std::fmt::{UpperHex, LowerHex, Debug};
use crate::game::line::LineMask;

pub trait BoardMask<P: Position>: Shiftable + LineMask<P> + UpperHex + LowerHex + BitOr<Output=Self> + BitXor<Output=Self> + BitAnd<Output=Self> + Not<Output=Self> + BitOrAssign + BitAndAssign + BitXorAssign + Eq + PartialEq + Debug + IntoIterator + Copy + Clone + Iterator<Item=P::Square> {
    // First square (A1 mask)
    const SQUARE: Self;
    // Full board masks
    const EMPTY: Self;
    const FULL: Self;
    // Files
    const A_FILE: Self;
    const B_FILE: Self;
    const C_FILE: Self;
    const D_FILE: Self;
    const E_FILE: Self;
    const F_FILE: Self;
    const G_FILE: Self;
    const H_FILE: Self;
    // Ranks
    const RANK_1: Self;
    const RANK_2: Self;
    const RANK_3: Self;
    const RANK_4: Self;
    const RANK_5: Self;
    const RANK_6: Self;
    const RANK_7: Self;
    const RANK_8: Self;

    // Fills
    fn file_fill(self) -> Self;
    fn rank_fill(self) -> Self;
    fn south_fill(self) -> Self;
    fn north_fill(self) -> Self;
    fn east_fill(self) -> Self;
    fn west_fill(self) -> Self;
    fn north_west_fill(self) -> Self;
    fn north_east_fill(self) -> Self;
    fn south_west_fill(self) -> Self;
    fn south_east_fill(self) -> Self;
    fn cardinal_fill(self) -> Self;
    fn diagonal_fill(self) -> Self;
    // Occluded Fills
    fn south_occluded_fill(self, empty: Self) -> Self;
    fn north_occluded_fill(self, empty: Self) -> Self;
    fn east_occluded_fill(self, empty: Self) -> Self;
    fn west_occluded_fill(self, empty: Self) -> Self;
    fn north_east_occluded_fill(self, empty: Self) -> Self;
    fn north_west_occluded_fill(self, empty: Self) -> Self;
    fn south_east_occluded_fill(self, empty: Self) -> Self;
    fn south_west_occluded_fill(self, empty: Self) -> Self;
    // Ray attacks
    fn south_ray_attacks(self, empty: Self) -> Self;
    fn north_ray_attacks(self, empty: Self) -> Self;
    fn east_ray_attacks(self, empty: Self) -> Self;
    fn west_ray_attacks(self, empty: Self) -> Self;
    fn north_west_ray_attacks(self, empty: Self) -> Self;
    fn north_east_ray_attacks(self, empty: Self) -> Self;
    fn south_west_ray_attacks(self, empty: Self) -> Self;
    fn south_east_ray_attacks(self, empty: Self) -> Self;
    fn cardinal_ray_attacks(self, empty: Self) -> Self;
    fn diagonal_ray_attacks(self, empty: Self) -> Self;
}
