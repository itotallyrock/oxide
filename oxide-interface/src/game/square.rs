use std::fmt::{Display, Formatter, Result as FormatResult};
use std::ops::{Shl, Shr};

use interface::game::{BoardMask, Shiftable, Square};
use interface::types::SquareOffset;
use OxideSquare::*;

use crate::game::OxideBitboard;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OxideSquare {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Display for OxideSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{:?}", self)
    }
}

impl const Shl<SquareOffset> for OxideSquare {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: SquareOffset) -> Self::Output {
        let shifted = <OxideSquare as Square<OxideBitboard, 64>>::offset(self) + rhs;
        assert!(shifted <= <OxideSquare as Square<OxideBitboard, 64>>::offset(H8), "Shifted square left off board");

        <OxideSquare as Square<OxideBitboard, 64>>::from_offset(shifted).unwrap()
    }
}

impl const Shr<SquareOffset> for OxideSquare {
    type Output = Self;

    fn shr(self, rhs: SquareOffset) -> Self::Output {
        <OxideSquare as Square<OxideBitboard, 64>>::from_offset(<OxideSquare as Square<OxideBitboard, 64>>::offset(self).saturating_sub(rhs)).unwrap()
    }
}


impl const Shiftable for OxideSquare {

    #[inline]
    fn north_shift(self) -> Self {
        assert!(self.y_offset() <= 6, "Attempting to shift square north off of board");
        self << 8
    }

    #[inline]
    fn south_shift(self) -> Self {
        assert!(self.y_offset() > 0, "Attempting to shift square south off of board");
        self >> 8
    }

    #[inline]
    fn east_shift(self) -> Self {
        assert!(self.x_offset() <= 6, "Attempting to shift square east off of board");
        self << 1
    }

    #[inline]
    fn west_shift(self) -> Self {
        assert!(self.x_offset() > 0, "Attempting to shift square west off of board");
        self >> 1
    }

    #[inline]
    fn north_east_shift(self) -> Self {
        assert!(self.y_offset() <= 6, "Attempting to shift square north off of board");
        assert!(self.x_offset() <= 6, "Attempting to shift square east off of board");
        self << 9
    }

    #[inline]
    fn north_west_shift(self) -> Self {
        assert!(self.y_offset() <= 6, "Attempting to shift square north off of board");
        assert!(self.x_offset() > 0, "Attempting to shift square west off of board");
        self << 7
    }

    #[inline]
    fn south_east_shift(self) -> Self {
        assert!(self.y_offset() > 0, "Attempting to shift square south off of board");
        assert!(self.x_offset() <= 6, "Attempting to shift square east off of board");
        self >> 7
    }

    #[inline]
    fn south_west_shift(self) -> Self {
        assert!(self.y_offset() > 0, "Attempting to shift square south off of board");
        assert!(self.x_offset() > 0, "Attempting to shift square west off of board");
        self >> 9
    }
}

impl const Square<OxideBitboard, 64> for OxideSquare {
    const SQUARES: [OxideSquare; 64] = [
        A1, B1, C1, D1, E1, F1, G1, H1,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A8, B8, C8, D8, E8, F8, G8, H8,
    ];
    #[inline]
    fn from_offset(offset: SquareOffset) -> Option<Self> {
        if offset < 64 {
            Some(unsafe { std::mem::transmute(offset) })
        } else {
            None
        }
    }
    #[inline]
    fn from_mask(mask: OxideBitboard) -> Option<Self> {
        if mask.0 > 0 {
            OxideSquare::from_offset(mask.0.trailing_zeros() as u8)
        } else {
            None
        }
    }
    #[inline]
    fn x_offset(self) -> SquareOffset {
        Square::<OxideBitboard, 64>::offset(self) % 8
    }
    #[inline]
    fn y_offset(self) -> SquareOffset {
        Square::<OxideBitboard, 64>::offset(self) / 8
    }
    #[inline]
    fn to_mask(self) -> OxideBitboard {
        let offset = <Self as Square::<OxideBitboard, 64>>::offset(self);

        OxideBitboard(OxideBitboard::SQUARE.0 << offset)
    }
    #[inline]
    fn offset(self) -> SquareOffset {
        self as SquareOffset
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn offset_works() {
        // Test corners are correct
        assert_eq!(Square::<OxideBitboard, 64>::offset(A1), 0);
        assert_eq!(Square::<OxideBitboard, 64>::offset(H1), 7);
        assert_eq!(Square::<OxideBitboard, 64>::offset(A8), 56);
        assert_eq!(Square::<OxideBitboard, 64>::offset(H8), 63);
        // Test something in the middle
        assert_eq!(Square::<OxideBitboard, 64>::offset(E3), 20);
    }

    #[test]
    fn from_offset_works() {
        assert_eq!(<OxideSquare as Square::<OxideBitboard, 64>>::from_offset(64), None);
        assert_eq!(<OxideSquare as Square::<OxideBitboard, 64>>::from_offset(SquareOffset::MAX), None);
        // assert_eq!(Square::<OxideBitboard, 64>::from_offset(SquareOffset::MAX), None);
        for square in Square::<OxideBitboard, 64>::SQUARES.iter().copied::<OxideSquare>() {
            let offset = Square::<OxideBitboard, 64>::offset(square);
            assert_eq!(Square::<OxideBitboard, 64>::from_offset(offset), Some(square));
        }
    }

    #[test]
    fn from_mask_works() {
        assert_eq!(<OxideSquare as Square::<OxideBitboard, 64>>::from_mask(OxideBitboard(0)), None);
        assert_eq!(<OxideSquare as Square::<OxideBitboard, 64>>::from_mask(OxideBitboard(0x10000000)), Some(E4));
        assert_eq!(<OxideSquare as Square::<OxideBitboard, 64>>::from_mask(OxideBitboard::FULL), Some(A1));
    }

    #[test]
    fn from_mask_matches_to_mask() {
        for square in OxideSquare::SQUARES {
            assert_eq!(<OxideSquare as Square::<OxideBitboard, 64>>::from_mask(<OxideSquare as Square::<OxideBitboard, 64>>::to_mask(square)), Some(square));
        }
    }

    #[test]
    fn to_bitboard_works() {
        // Test corners are correct
        assert_eq!(Square::<OxideBitboard, 64>::to_mask(A1), OxideBitboard(0x1u64));
        assert_eq!(Square::<OxideBitboard, 64>::to_mask(H1), OxideBitboard(0x80u64));
        assert_eq!(Square::<OxideBitboard, 64>::to_mask(A8), OxideBitboard(0x100000000000000u64));
        assert_eq!(Square::<OxideBitboard, 64>::to_mask(H8), OxideBitboard(0x8000000000000000u64));
        // Test something in the middle
        assert_eq!(Square::<OxideBitboard, 64>::to_mask(E3), OxideBitboard(0x100000u64));
    }

    #[test]
    fn x_offset_works() {
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(A1), 0);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(A3), 0);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(A8), 0);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(B2), 1);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(B8), 1);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(C4), 2);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(D4), 3);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(E1), 4);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(F7), 5);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(G3), 6);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(H3), 7);
        assert_eq!(Square::<OxideBitboard, 64>::x_offset(H8), 7);
    }

    #[test]
    fn y_offset_works() {
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(A1), 0);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(A3), 2);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(A8), 7);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(B2), 1);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(B8), 7);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(C4), 3);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(D4), 3);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(E1), 0);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(F7), 6);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(G3), 2);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(H3), 2);
        assert_eq!(Square::<OxideBitboard, 64>::y_offset(H8), 7);
    }


    #[test]
    fn north_shift_works() {
        // Left vertical
        assert_eq!(<OxideSquare as Shiftable>::north_shift(A1), A2);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(A2), A3);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(A3), A4);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(A4), A5);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(A5), A6);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(A6), A7);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(A7), A8);
        // Random squares
        assert_eq!(<OxideSquare as Shiftable>::north_shift(E1), E2);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(H6), H7);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(H7), H8);
        assert_eq!(<OxideSquare as Shiftable>::north_shift(B4), B5);
    }

    #[test]
    #[should_panic]
    fn north_shift_off_board_panics() {
        <OxideSquare as Shiftable>::north_shift(C8);
    }

    #[test]
    fn south_shift_works() {
        // Left vertical
        assert_eq!(<OxideSquare as Shiftable>::south_shift(A8), A7);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(A7), A6);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(A6), A5);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(A5), A4);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(A4), A3);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(A3), A2);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(A2), A1);
        // Random squares
        assert_eq!(<OxideSquare as Shiftable>::south_shift(E2), E1);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(H7), H6);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(H8), H7);
        assert_eq!(<OxideSquare as Shiftable>::south_shift(B5), B4);
    }

    #[test]
    #[should_panic]
    fn south_shift_off_board_panics() {
        <OxideSquare as Shiftable>::south_shift(B1);
    }

    #[test]
    fn east_shift_works() {
        // Rank 3 east shift
        assert_eq!(<OxideSquare as Shiftable>::east_shift(A3), B3);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(B3), C3);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(C3), D3);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(D3), E3);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(E3), F3);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(F3), G3);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(G3), H3);
        // Random squares
        assert_eq!(<OxideSquare as Shiftable>::east_shift(E2), F2);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(B4), C4);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(D7), E7);
        assert_eq!(<OxideSquare as Shiftable>::east_shift(B5), C5);
    }

    #[test]
    #[should_panic]
    fn east_shift_off_board_panics() {
        <OxideSquare as Shiftable>::east_shift(H3);
    }

    #[test]
    fn west_shift_works() {
        // Rank 3 east shift
        assert_eq!(<OxideSquare as Shiftable>::west_shift(H3), G3);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(G3), F3);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(F3), E3);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(E3), D3);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(D3), C3);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(C3), B3);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(B3), A3);
        // Random squares
        assert_eq!(<OxideSquare as Shiftable>::west_shift(F2), E2);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(C4), B4);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(E7), D7);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(C5), B5);
        assert_eq!(<OxideSquare as Shiftable>::west_shift(H8), G8);
    }

    #[test]
    #[should_panic]
    fn west_shift_off_board_panics() {
        <OxideSquare as Shiftable>::west_shift(A7);
    }

    #[test]
    fn north_east_shift_works() {
        // South-West to North-East diagonal
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(A1), B2);
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(B2), C3);
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(C3), D4);
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(D4), E5);
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(E5), F6);
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(F6), G7);
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(G7), H8);
        // Random squares
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(B1), C2);
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(C7), D8);
        assert_eq!(<OxideSquare as Shiftable>::north_east_shift(F5), G6);
    }

    #[test]
    #[should_panic]
    fn north_east_shift_off_east_board_panics() {
        <OxideSquare as Shiftable>::north_east_shift(H4);
    }

    #[test]
    #[should_panic]
    fn north_east_shift_off_north_board_panics() {
        <OxideSquare as Shiftable>::north_east_shift(F8);
    }

    #[test]
    #[should_panic]
    fn north_east_shift_off_board_panics() {
        <OxideSquare as Shiftable>::north_east_shift(H8);
    }

    #[test]
    fn north_west_shift_works() {
        // South-East to North-West diagonal
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(H1), G2);
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(G2), F3);
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(F3), E4);
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(E4), D5);
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(D5), C6);
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(C6), B7);
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(B7), A8);
        // Random squares
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(B1), A2);
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(C7), B8);
        assert_eq!(<OxideSquare as Shiftable>::north_west_shift(F5), E6);
    }

    #[test]
    #[should_panic]
    fn north_west_shift_off_west_board_panics() {
        <OxideSquare as Shiftable>::north_west_shift(A7);
    }

    #[test]
    #[should_panic]
    fn north_west_shift_off_north_board_panics() {
        <OxideSquare as Shiftable>::north_west_shift(D8);
    }

    #[test]
    #[should_panic]
    fn north_west_shift_off_board_panics() {
        <OxideSquare as Shiftable>::north_west_shift(A8);
    }

    #[test]
    fn south_east_shift_works() {
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(G8), H7);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(A2), B1);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(B3), C2);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(C4), D3);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(D5), E4);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(E6), F5);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(F7), G6);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(B2), C1);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(C8), D7);
        assert_eq!(<OxideSquare as Shiftable>::south_east_shift(F6), G5);
    }

    #[test]
    #[should_panic]
    fn south_east_shift_off_east_board_panics() {
        <OxideSquare as Shiftable>::south_east_shift(H4);
    }

    #[test]
    #[should_panic]
    fn south_east_shift_off_south_board_panics() {
        <OxideSquare as Shiftable>::south_east_shift(F1);
    }

    #[test]
    #[should_panic]
    fn south_east_shift_off_board_panics() {
        <OxideSquare as Shiftable>::south_east_shift(H1);
    }

    #[test]
    fn south_west_shift_works() {
        // North-East to South-West diagonal
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(H8), G7);
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(G7), F6);
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(F6), E5);
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(E5), D4);
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(D4), C3);
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(C3), B2);
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(B2), A1);
        // Random squares
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(B2), A1);
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(C7), B6);
        assert_eq!(<OxideSquare as Shiftable>::south_west_shift(F5), E4);
    }

    #[test]
    #[should_panic]
    fn south_west_shift_off_west_board_panics() {
        <OxideSquare as Shiftable>::south_west_shift(A5);
    }

    #[test]
    #[should_panic]
    fn south_west_shift_off_south_board_panics() {
        <OxideSquare as Shiftable>::south_west_shift(F1);
    }

    #[test]
    #[should_panic]
    fn south_west_shift_off_board_panics() {
        <OxideSquare as Shiftable>::south_west_shift(A1);
    }
}