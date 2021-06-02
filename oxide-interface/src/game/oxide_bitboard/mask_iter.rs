use crate::game::{OxideBitboard, OxideSquare};
use crate::game::oxide_bitboard::pop_square;
use crate::engine::OxidePosition;
use std::iter::FromIterator;
use interface::game::{Square, BoardMask};

impl Iterator for OxideBitboard {
    type Item = OxideSquare;

    fn next(&mut self) -> Option<Self::Item> {
        pop_square::pop_square::<OxidePosition>(&mut self.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.0 == 0 {
            (0, Some(0))
        } else {
            let ones = self.count() as usize;

            (ones, Some(ones))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl FromIterator<OxideSquare> for OxideBitboard {
    fn from_iter<T: IntoIterator<Item = OxideSquare>>(iter: T) -> Self {
        iter.into_iter().fold(OxideBitboard::EMPTY, |bb, sq| {
            bb | <OxideSquare as Square<OxidePosition>>::to_mask(&sq)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::square::OxideSquare::*;

    #[test]
    fn works_for_empty() {
        let mut raw_mask = OxideBitboard::EMPTY;
        assert_eq!(<OxideBitboard as Iterator>::size_hint(&mut raw_mask), (0, Some(0)));
        assert_eq!(<OxideBitboard as Iterator>::next(&mut raw_mask), None);
    }

    #[test]
    fn works_for_single_square() {
        let mut raw_mask = OxideBitboard::SQUARE;
        assert_eq!(<OxideBitboard as Iterator>::size_hint(&raw_mask), (1, Some(1)));
        assert_eq!(<OxideBitboard as Iterator>::next(&mut raw_mask), Some(A1));
        assert_eq!(<OxideBitboard as Iterator>::size_hint(&raw_mask), (0, Some(0)));
        assert_eq!(<OxideBitboard as Iterator>::next(&mut raw_mask), None);
        let mut raw_mask = OxideBitboard(0x100000u64);
        assert_eq!(<OxideBitboard as Iterator>::size_hint(&raw_mask), (1, Some(1)));
        assert_eq!(<OxideBitboard as Iterator>::next(&mut raw_mask), Some(E3));
        assert_eq!(<OxideBitboard as Iterator>::size_hint(&raw_mask), (0, Some(0)));
        assert_eq!(<OxideBitboard as Iterator>::next(&mut raw_mask), None);
    }

    #[test]
    fn works_for_multiple_squares() {
        let mut iter = OxideBitboard(0x8002000200100080u64);
        assert_eq!(<OxideBitboard as Iterator>::size_hint(&iter), (5, Some(5)));
        assert_eq!(iter.next(), Some(H1));
        assert_eq!(iter.next(), Some(E3));
        assert_eq!(iter.next(), Some(B5));
        assert_eq!(iter.next(), Some(B7));
        assert_eq!(iter.next(), Some(H8));
    }

    #[test]
    fn is_equivalent_to_from_square_iter() {
        let raw_mask = OxideBitboard(0x8002000200100080u64);
        let bitboard = <OxideBitboard as Iterator>::collect::<OxideBitboard>(raw_mask);
        assert_eq!(bitboard, raw_mask);
        let raw_mask = OxideBitboard::FULL;
        let bitboard = <OxideBitboard as Iterator>::collect::<OxideBitboard>(raw_mask);
        assert_eq!(bitboard, raw_mask);
        let raw_mask = OxideBitboard::EMPTY;
        let bitboard = <OxideBitboard as Iterator>::collect::<OxideBitboard>(raw_mask);
        assert_eq!(bitboard, raw_mask);
    }
}