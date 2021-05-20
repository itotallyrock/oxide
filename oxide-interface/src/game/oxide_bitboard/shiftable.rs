use interface::game::{SquareOffset, Shiftable};
use crate::game::OxideBitboard;
use std::ops::{Shl, Shr};

// Must be defined by number to avoid circular dependency
pub(crate) const NOT_A_FILE: OxideBitboard = OxideBitboard(0xfefefefefefefefeu64);
pub(crate) const NOT_H_FILE: OxideBitboard = OxideBitboard(0x7f7f7f7f7f7f7f7fu64);

impl const Shr<SquareOffset> for OxideBitboard {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: SquareOffset) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs >> rhs)
    }
}

impl const Shl<SquareOffset> for OxideBitboard {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: SquareOffset) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs << rhs)
    }
}

impl const Shiftable for OxideBitboard {
    #[inline]
    fn north_shift(self) -> Self {
        self << 8
    }
    #[inline]
    fn south_shift(self) -> Self {
        self >> 8
    }
    #[inline]
    fn east_shift(self) -> Self {
        self << 1 & NOT_A_FILE
    }
    #[inline]
    fn west_shift(self) -> Self {
        self >> 1 & NOT_H_FILE
    }
    #[inline]
    fn north_east_shift(self) -> Self {
        self << 9 & NOT_A_FILE
    }
    #[inline]
    fn north_west_shift(self) -> Self {
        self << 7 & NOT_H_FILE
    }
    #[inline]
    fn south_east_shift(self) -> Self {
        self >> 7 & NOT_A_FILE
    }
    #[inline]
    fn south_west_shift(self) -> Self {
        self >> 9 & NOT_H_FILE
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use interface::game::BoardMask;

    #[test]
    fn edge_consts_match() {
        assert_eq!(NOT_A_FILE, !OxideBitboard::A_FILE);
        assert_eq!(NOT_H_FILE, !OxideBitboard::H_FILE);
    }

    #[test]
    fn shift_north_works() {
        assert_eq!(OxideBitboard::north_shift(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x40a102440880000));
        assert_eq!(OxideBitboard::north_shift(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xffffffffffffff00));
        assert_eq!(OxideBitboard::north_shift(OxideBitboard(0x0)), OxideBitboard(0x0));
    }

    #[test]
    fn shift_south_works() {
        assert_eq!(OxideBitboard::south_shift(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x3040a10244088));
        assert_eq!(OxideBitboard::south_shift(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xffffffffffffff));
        assert_eq!(OxideBitboard::south_shift(OxideBitboard(0x0)), OxideBitboard(0x0));
    }

    #[test]
    fn shift_east_works() {
        assert_eq!(OxideBitboard::east_shift(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x608142048801000));
        assert_eq!(OxideBitboard::east_shift(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xfefefefefefefefe));
        assert_eq!(OxideBitboard::east_shift(OxideBitboard(0x0)), OxideBitboard(0x0));
    }

    #[test]
    fn shift_west_works() {
        assert_eq!(OxideBitboard::west_shift(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x102050812204400));
        assert_eq!(OxideBitboard::west_shift(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x7f7f7f7f7f7f7f7f));
        assert_eq!(OxideBitboard::west_shift(OxideBitboard(0x0)), OxideBitboard(0x0));
    }

    #[test]
    fn shift_north_east_works() {
        assert_eq!(OxideBitboard::north_east_shift(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x814204880100000));
        assert_eq!(OxideBitboard::north_east_shift(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xfefefefefefefe00));
        assert_eq!(OxideBitboard::north_east_shift(OxideBitboard(0x0)), OxideBitboard(0x0));
    }

    #[test]
    fn shift_north_west_works() {
        assert_eq!(OxideBitboard::north_west_shift(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x205081220440000));
        assert_eq!(OxideBitboard::north_west_shift(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x7f7f7f7f7f7f7f00));
        assert_eq!(OxideBitboard::north_west_shift(OxideBitboard(0x0)), OxideBitboard(0x0));
    }

    #[test]
    fn shift_south_east_works() {
        assert_eq!(OxideBitboard::south_east_shift(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x6081420488010));
        assert_eq!(OxideBitboard::south_east_shift(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xfefefefefefefe));
        assert_eq!(OxideBitboard::south_east_shift(OxideBitboard(0x0)), OxideBitboard(0x0));
    }

    #[test]
    fn shift_south_west_works() {
        assert_eq!(OxideBitboard::south_west_shift(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x1020508122044));
        assert_eq!(OxideBitboard::south_west_shift(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x7f7f7f7f7f7f7f));
        assert_eq!(OxideBitboard::south_west_shift(OxideBitboard(0x0)), OxideBitboard(0x0));
    }
}