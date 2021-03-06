mod pop_square;
mod numeric_impl;
mod mask_iter;
mod shiftable;
#[cfg(test)]
mod test;
#[cfg(test)]
mod bench;
#[macro_use]
mod fill_macros;
mod line;

use interface::game::{Shiftable, BoardMask};
use std::fmt::{Formatter, Result as FormatResult, Debug};
use crate::game::oxide_bitboard::shiftable::{NOT_A_FILE, NOT_H_FILE};
use crate::engine::OxidePosition;

const NOT_A_FILE_BB: u64 = NOT_A_FILE.0;
const NOT_H_FILE_BB: u64 = NOT_H_FILE.0;
const FULL_BB: u64 = OxideBitboard::FULL.0;

#[derive(Copy, Clone, Ord, PartialOrd)]
pub struct OxideBitboard(pub u64);

impl const PartialEq for OxideBitboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl const Eq for OxideBitboard {}

impl Debug for OxideBitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "0x{:X}", self)
    }
}

impl const BoardMask<OxidePosition> for OxideBitboard {
    const SQUARE: Self = Self(1u64);
    const EMPTY: Self = Self(0u64);
    const FULL: Self = Self(0xFFFFFFFFFFFFFFFFu64);

    // TODO: Figure out what in const_trait_impl is wrong with this
    const A_FILE: Self = Self(!(Self::FULL.0 << 1 & NOT_A_FILE.0)); // !Self::east_shift(Self::FULL);
    const B_FILE: Self = Self(Self::A_FILE.0 << 1 & NOT_A_FILE.0); // Self::east_shift(Self::A_FILE);
    const C_FILE: Self = Self(Self::B_FILE.0 << 1 & NOT_A_FILE.0); // Self::east_shift(Self::B_FILE);
    const D_FILE: Self = Self(Self::C_FILE.0 << 1 & NOT_A_FILE.0); // Self::east_shift(Self::C_FILE);
    const E_FILE: Self = Self(Self::D_FILE.0 << 1 & NOT_A_FILE.0); // Self::east_shift(Self::D_FILE);
    const F_FILE: Self = Self(Self::E_FILE.0 << 1 & NOT_A_FILE.0); // Self::east_shift(Self::E_FILE);
    const G_FILE: Self = Self(Self::F_FILE.0 << 1 & NOT_A_FILE.0); // Self::east_shift(Self::F_FILE);
    const H_FILE: Self = Self(Self::G_FILE.0 << 1 & NOT_A_FILE.0); // Self::east_shift(Self::G_FILE);

    const RANK_1: Self = Self(!(Self::FULL.0 << 8));// !Self::north_shift(Self::FULL);
    const RANK_2: Self = Self(Self::RANK_1.0 << 8);// Self::north_shift(Self::RANK_1);
    const RANK_3: Self = Self(Self::RANK_2.0 << 8);// Self::north_shift(Self::RANK_2);
    const RANK_4: Self = Self(Self::RANK_3.0 << 8);// Self::north_shift(Self::RANK_3);
    const RANK_5: Self = Self(Self::RANK_4.0 << 8);// Self::north_shift(Self::RANK_4);
    const RANK_6: Self = Self(Self::RANK_5.0 << 8);// Self::north_shift(Self::RANK_5);
    const RANK_7: Self = Self(Self::RANK_6.0 << 8);// Self::north_shift(Self::RANK_6);
    const RANK_8: Self = Self(Self::RANK_7.0 << 8);// Self::north_shift(Self::RANK_7);

    // Fills
    #[inline]
    fn file_fill(self) -> Self {
        Self::south_fill(self) | Self::north_fill(self)
    }

    #[inline]
    fn rank_fill(self) -> Self {
        Self::east_fill(self) | Self::west_fill(self)
    }

    #[inline]
    fn south_fill(self) -> Self {
        let mut bb = self.0;
        bb |= bb >> 8;
        bb |= bb >> 16;
        bb |= bb >> 32;

        Self(bb)
    }

    #[inline]
    fn north_fill(self) -> Self {
        let mut bb = self.0;
        bb |= bb << 8;
        bb |= bb << 16;
        bb |= bb << 32;

        Self(bb)
    }

    #[inline]
    fn east_fill(self) -> Self {
        let mut bb = self.0;
        fill_masked!(bb, NOT_A_FILE_BB << 1);

        Self(bb)
    }

    #[inline]
    fn west_fill(self) -> Self {
        let mut bb = self.0;
        fill_masked!(bb, NOT_H_FILE_BB >> 1);

        Self(bb)
    }
    #[inline]
    fn north_west_fill(self) -> Self {
        let mut bb = self.0;
        fill_masked!(bb, NOT_H_FILE_BB << 7);

        Self(bb)
    }
    #[inline]
    fn north_east_fill(self) -> Self {
        let mut bb = self.0;
        fill_masked!(bb, NOT_A_FILE_BB << 9);

        Self(bb)
    }
    #[inline]
    fn south_west_fill(self) -> Self {
        let mut bb = self.0;
        fill_masked!(bb, NOT_H_FILE_BB >> 9);

        Self(bb)
    }
    #[inline]
    fn south_east_fill(self) -> Self {
        let mut bb = self.0;
        fill_masked!(bb, NOT_A_FILE_BB >> 7);

        Self(bb)
    }

    #[inline]
    fn cardinal_fill(self) -> Self {
        self.north_fill() | self.south_fill() | self.east_fill() | self.west_fill()
    }

    #[inline]
    fn diagonal_fill(self) -> Self {
        self.north_east_fill() | self.north_west_fill() | self.south_east_fill() | self.south_west_fill()
    }

    // Occluded fills
    #[inline]
    fn south_occluded_fill(self, empty: Self) -> Self {
        let mut bb = self.0;
        let mut empty_bb = empty.0;
        fill_occluded_mask!(bb, empty_bb, FULL_BB >> 8);

        Self(bb)
    }

    #[inline]
    fn north_occluded_fill(self, empty: Self) -> Self {
        let mut bb = self.0;
        let mut empty_bb = empty.0;
        fill_occluded_mask!(bb, empty_bb, FULL_BB << 8);

        Self(bb)
    }

    #[inline]
    fn east_occluded_fill(self, empty: Self) -> Self {
        let mut bb = self.0;
        let mut empty_bb = empty.0;
        fill_occluded_mask!(bb, empty_bb, NOT_A_FILE_BB << 1);

        Self(bb)
    }

    #[inline]
    fn west_occluded_fill(self, empty: Self) -> Self {
        let mut bb = self.0;
        let mut empty_bb = empty.0;
        fill_occluded_mask!(bb, empty_bb, NOT_H_FILE_BB >> 1);

        Self(bb)
    }

    #[inline]
    fn north_east_occluded_fill(self, empty: Self) -> Self {
        let mut bb = self.0;
        let mut empty_bb = empty.0;
        fill_occluded_mask!(bb, empty_bb, NOT_A_FILE_BB << 9);

        Self(bb)
    }

    #[inline]
    fn north_west_occluded_fill(self, empty: Self) -> Self {
        let mut bb = self.0;
        let mut empty_bb = empty.0;
        fill_occluded_mask!(bb, empty_bb, NOT_H_FILE_BB << 7);

        Self(bb)
    }

    #[inline]
    fn south_east_occluded_fill(self, empty: Self) -> Self {
        let mut bb = self.0;
        let mut empty_bb = empty.0;
        fill_occluded_mask!(bb, empty_bb, NOT_A_FILE_BB >> 7);

        Self(bb)
    }

    #[inline]
    fn south_west_occluded_fill(self, empty: Self) -> Self {
        let mut bb = self.0;
        let mut empty_bb = empty.0;
        fill_occluded_mask!(bb, empty_bb, NOT_H_FILE_BB >> 9);

        Self(bb)
    }

    // Ray attacks
    #[inline]
    fn south_ray_attacks(self, empty: Self) -> Self {
        Self::south_shift(Self::south_occluded_fill(self, empty))
    }

    #[inline]
    fn north_ray_attacks(self, empty: Self) -> Self {
        Self::north_shift(Self::north_occluded_fill(self, empty))
    }

    #[inline]
    fn east_ray_attacks(self, empty: Self) -> Self {
        Self::east_shift(Self::east_occluded_fill(self, empty))
    }

    #[inline]
    fn west_ray_attacks(self, empty: Self) -> Self {
        Self::west_shift(Self::west_occluded_fill(self, empty))
    }

    #[inline]
    fn north_west_ray_attacks(self, empty: Self) -> Self {
        Self::north_west_shift(Self::north_west_occluded_fill(self, empty))
    }

    #[inline]
    fn north_east_ray_attacks(self, empty: Self) -> Self {
        Self::north_east_shift(Self::north_east_occluded_fill(self, empty))
    }

    #[inline]
    fn south_west_ray_attacks(self, empty: Self) -> Self {
        Self::south_west_shift(Self::south_west_occluded_fill(self, empty))
    }

    #[inline]
    fn south_east_ray_attacks(self, empty: Self) -> Self {
        Self::south_east_shift(Self::south_east_occluded_fill(self, empty))
    }

    #[inline]
    fn cardinal_ray_attacks(self, empty: Self) -> Self {
        Self::north_ray_attacks(self, empty) | Self::south_ray_attacks(self, empty) | Self::east_ray_attacks(self, empty) | Self::west_ray_attacks(self, empty)
    }

    #[inline]
    fn diagonal_ray_attacks(self, empty: Self) -> Self {
        Self::north_west_ray_attacks(self, empty) | Self::north_east_ray_attacks(self, empty) | Self::south_west_ray_attacks(self, empty) | Self::south_east_ray_attacks(self, empty)
    }
}

