use crate::game::oxide_bitboard::pop_square::pop_square;
use crate::game::{OxideSquare, OxideBitboard};

pub struct BoardMaskSquareIter(pub(crate) u64);

impl Iterator for BoardMaskSquareIter {
    type Item = OxideSquare;

    fn next(&mut self) -> Option<Self::Item> {
        pop_square::<OxideSquare, OxideBitboard, 64>(&mut self.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.0 == 0 {
            (0, Some(0))
        } else {
            let ones = self.0.count_ones() as usize;

            (ones, Some(ones))
        }
    }
}

impl IntoIterator for OxideBitboard {
    type Item = OxideSquare;
    type IntoIter = BoardMaskSquareIter;

    fn into_iter(self) -> Self::IntoIter {
        BoardMaskSquareIter(self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::square::OxideSquare::*;
    use interface::game::BoardMask;

    #[test]
    fn works_for_empty() {
        let raw_mask = OxideBitboard::EMPTY.0;
        assert_eq!(BoardMaskSquareIter(raw_mask).size_hint(), (0, Some(0)));
        assert_eq!(BoardMaskSquareIter(raw_mask).next(), None);
    }

    #[test]
    fn works_for_single_square() {
        let raw_mask = OxideBitboard::SQUARE.0;
        assert_eq!(BoardMaskSquareIter(raw_mask).next(), Some(A1));
        assert_eq!(BoardMaskSquareIter(raw_mask).size_hint(), (1, Some(1)));
        let raw_mask = 0x100000u64;
        assert_eq!(BoardMaskSquareIter(raw_mask).next(), Some(E3));
        assert_eq!(BoardMaskSquareIter(raw_mask).size_hint(), (1, Some(1)));
    }

    #[test]
    fn works_for_multiple_squares() {
        let raw_mask = 0x8002000200100080u64;
        let mut iter = BoardMaskSquareIter(raw_mask);
        assert_eq!(BoardMaskSquareIter(raw_mask).size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some(H1));
        assert_eq!(iter.next(), Some(E3));
        assert_eq!(iter.next(), Some(B5));
        assert_eq!(iter.next(), Some(B7));
        assert_eq!(iter.next(), Some(H8));
    }

    #[test]
    fn is_equivalent_to_from_square_iter() {
        let raw_mask = 0x8002000200100080u64;
        let bitboard = BoardMaskSquareIter(raw_mask).collect::<OxideBitboard>();
        assert_eq!(bitboard.0, raw_mask);
        let raw_mask = OxideBitboard::FULL.0;
        let bitboard = BoardMaskSquareIter(raw_mask).collect::<OxideBitboard>();
        assert_eq!(bitboard.0, raw_mask);
        let raw_mask = OxideBitboard::EMPTY.0;
        let bitboard = BoardMaskSquareIter(raw_mask).collect::<OxideBitboard>();
        assert_eq!(bitboard.0, raw_mask);
    }
}