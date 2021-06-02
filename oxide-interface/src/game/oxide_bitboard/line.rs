use crate::game::{OxideBitboard, OxideSquare};
use interface::game::{LineMask, PieceArrangement, Position, Square, BoardMask};
use crate::engine::OxidePosition;

#[inline]
fn line_bb<P: Position>(from_square: P::Square, to_square: P::Square) -> P::BoardMask {
    let from_mask = from_square.to_mask();
    let to_mask = to_square.to_mask();
    // Ignore from == to (no line between same square)
    if from_mask ^ to_mask == P::BoardMask::EMPTY {
        return from_mask;
    }
    // Get rook attacks from the from square blocked by to
    let from_rook = from_mask.cardinal_fill();
    // Get rook attacks from the to square blocked by from
    let to_rook = to_mask.cardinal_fill();

    // Get bishop attacks from the from square blocked by to
    let from_bishop = from_mask.diagonal_fill();
    // Get bishop attacks from the to square blocked by from
    let to_bishop = to_mask.diagonal_fill();

    if from_rook & to_mask != P::BoardMask::EMPTY {
        // Cardinal aligned so get between the two rooks and include origin squares
        (from_rook & to_rook) | from_mask | to_mask
    } else if from_bishop & to_mask != P::BoardMask::EMPTY {
        // Diagonally aligned so get between the two bishops and include origin squares
        (from_bishop & to_bishop) | from_mask | to_mask
    } else {
        // Not aligned diagonally or cardinally
        P::BoardMask::EMPTY
    }
}

#[cfg(feature = "line_table")]
mod line_table {
    use lazy_static::lazy_static;
    use crate::game::{OxideBitboard, OxideSquare};
    use interface::game::{Square, BoardMask};
    use crate::game::oxide_bitboard::line::line_bb;
    use crate::engine::OxidePosition;

    lazy_static! {
        static ref LINE_TABLE: [[OxideBitboard; 64]; 64] = {
            let mut table = [[OxideBitboard::EMPTY; 64]; 64];

            for &from_square in &OxideSquare::SQUARES {
                for &to_square in &OxideSquare::SQUARES {
                    table[from_square.offset() as usize][to_square.offset() as usize] = line_bb::<OxidePosition>(from_square, to_square);
                }
            }

            table
        };
    }

    #[inline]
    pub fn line_table_lookup(a: OxideSquare, b: OxideSquare) -> OxideBitboard {
        LINE_TABLE[a.offset() as usize][b.offset() as usize]
    }
}

impl LineMask<OxidePosition> for OxideBitboard {
    fn line_fill(a: OxideSquare, b: OxideSquare) -> Self {
        #[cfg(not(feature = "line_table"))] {
            line_bb(a, b)
        }
        #[cfg(feature = "line_table")] {
            use line_table::line_table_lookup;

            line_table_lookup(a, b)
        }
    }

    fn between_fill(a: OxideSquare, b: OxideSquare) -> Self {
        let line_mask = Self::line_fill(a, b);
        if line_mask != OxideBitboard::EMPTY {
            let mask = line_mask & ((Self::FULL << a.offset()) ^ (Self::FULL << b.offset()));

            OxideBitboard(mask.0 & (mask.0 - 1))
        } else {
            Self::EMPTY
        }
    }

    fn aligned(a: OxideSquare, b: OxideSquare, c: OxideSquare) -> bool {
        Self::line_fill(a, b) & c.to_mask() != OxideBitboard::EMPTY
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::square::OxideSquare::*;

    #[test]
    fn aligned_works() {
        assert!(OxideBitboard::aligned(A2, A4, A6));
        assert!(OxideBitboard::aligned(A2, A4, A8));
        assert!(!OxideBitboard::aligned(B2, A4, A8));
        assert!(!OxideBitboard::aligned(A2, B4, A8));
        assert!(!OxideBitboard::aligned(A2, A4, B8));
        assert!(!OxideBitboard::aligned(A2, B4, B8));
        assert!(OxideBitboard::aligned(B2, B4, B8));
        assert!(OxideBitboard::aligned(H1, A1, C1));
        assert!(!OxideBitboard::aligned(H1, A1, C2));
        assert!(OxideBitboard::aligned(H8, A1, D4));
        assert!(!OxideBitboard::aligned(H8, A1, D5));
        assert!(!OxideBitboard::aligned(H8, A2, D4));
        assert!(!OxideBitboard::aligned(H7, A1, D4));
    }

    #[test]
    fn between_fill_works() {
        // A1-H8 diagonal
        assert_eq!(OxideBitboard::between_fill(A1, H8), OxideBitboard(0x40201008040200));
        assert_eq!(OxideBitboard::between_fill(A1, G7), OxideBitboard(0x201008040200));
        assert_eq!(OxideBitboard::between_fill(A1, F6), OxideBitboard(0x1008040200));
        assert_eq!(OxideBitboard::between_fill(A1, E5), OxideBitboard(0x8040200));
        assert_eq!(OxideBitboard::between_fill(B2, E5), OxideBitboard(0x8040000));
        assert_eq!(OxideBitboard::between_fill(B2, D4), OxideBitboard(0x40000));
        assert_eq!(OxideBitboard::between_fill(B3, D4), OxideBitboard(0x0));
        // G2-G6 vertical
        assert_eq!(OxideBitboard::between_fill(G2, G6), OxideBitboard(0x4040400000));
        assert_eq!(OxideBitboard::between_fill(G3, G6), OxideBitboard(0x4040000000));
        assert_eq!(OxideBitboard::between_fill(G4, G6), OxideBitboard(0x4000000000));
        assert_eq!(OxideBitboard::between_fill(G4, G5), OxideBitboard(0x0));
        // F5-A5 horizontal
        assert_eq!(OxideBitboard::between_fill(F5, A5), OxideBitboard(0x1e00000000));
        assert_eq!(OxideBitboard::between_fill(E5, A5), OxideBitboard(0xe00000000));
        assert_eq!(OxideBitboard::between_fill(D5, A5), OxideBitboard(0x600000000));
        assert_eq!(OxideBitboard::between_fill(D5, B5), OxideBitboard(0x400000000));
        assert_eq!(OxideBitboard::between_fill(D5, C5), OxideBitboard(0x0));
        // Non aligned between
        assert_eq!(OxideBitboard::between_fill(A5, B7), OxideBitboard(0x0));
        assert_eq!(OxideBitboard::between_fill(H1, C8), OxideBitboard(0x0));
        assert_eq!(OxideBitboard::between_fill(E4, C1), OxideBitboard(0x0));
        assert_eq!(OxideBitboard::between_fill(E4, D1), OxideBitboard(0x0));
        assert_eq!(OxideBitboard::between_fill(E4, F1), OxideBitboard(0x0));
        assert_eq!(OxideBitboard::between_fill(E4, G1), OxideBitboard(0x0));
    }

    #[test]
    fn line_fill_works() {
        // Non aligned
        assert_eq!(OxideBitboard::line_fill(A1, B5), OxideBitboard(0x0));
        assert_eq!(OxideBitboard::line_fill(A1, B4), OxideBitboard(0x0));
        assert_eq!(OxideBitboard::line_fill(A1, C4), OxideBitboard(0x0));
        // Diagonal A1-H8
        assert_eq!(OxideBitboard::line_fill(A1, D4), OxideBitboard(0x8040201008040201));
        assert_eq!(OxideBitboard::line_fill(B2, D4), OxideBitboard(0x8040201008040201));
        assert_eq!(OxideBitboard::line_fill(C3, D4), OxideBitboard(0x8040201008040201));
        assert_eq!(OxideBitboard::line_fill(D4, C3), OxideBitboard(0x8040201008040201));
        assert_eq!(OxideBitboard::line_fill(D4, E5), OxideBitboard(0x8040201008040201));
        assert_eq!(OxideBitboard::line_fill(D4, H8), OxideBitboard(0x8040201008040201));
        assert_eq!(OxideBitboard::line_fill(A1, H8), OxideBitboard(0x8040201008040201));
        // Diagonal A8-H1
        assert_eq!(OxideBitboard::line_fill(A8, D5), OxideBitboard(0x102040810204080));
        assert_eq!(OxideBitboard::line_fill(B7, D5), OxideBitboard(0x102040810204080));
        assert_eq!(OxideBitboard::line_fill(C6, D5), OxideBitboard(0x102040810204080));
        assert_eq!(OxideBitboard::line_fill(D5, C6), OxideBitboard(0x102040810204080));
        assert_eq!(OxideBitboard::line_fill(D5, E4), OxideBitboard(0x102040810204080));
        assert_eq!(OxideBitboard::line_fill(D5, H1), OxideBitboard(0x102040810204080));
        assert_eq!(OxideBitboard::line_fill(A8, H1), OxideBitboard(0x102040810204080));
        // Non-major diagonal D8-H4
        assert_eq!(OxideBitboard::line_fill(E7, G5), OxideBitboard(0x810204080000000));
        assert_eq!(OxideBitboard::line_fill(G5, E7), OxideBitboard(0x810204080000000));
        assert_eq!(OxideBitboard::line_fill(G5, H4), OxideBitboard(0x810204080000000));
        assert_eq!(OxideBitboard::line_fill(D8, H4), OxideBitboard(0x810204080000000));
        // Vertical G1-G4
        assert_eq!(OxideBitboard::line_fill(G1, G4), OxideBitboard(0x4040404040404040));
        assert_eq!(OxideBitboard::line_fill(G1, G3), OxideBitboard(0x4040404040404040));
        assert_eq!(OxideBitboard::line_fill(G1, G2), OxideBitboard(0x4040404040404040));
        assert_eq!(OxideBitboard::line_fill(G4, G1), OxideBitboard(0x4040404040404040));
        // Horizontal A5-F5
        assert_eq!(OxideBitboard::line_fill(A5, F5), OxideBitboard(0xff00000000));
        assert_eq!(OxideBitboard::line_fill(A5, E5), OxideBitboard(0xff00000000));
        assert_eq!(OxideBitboard::line_fill(A5, D5), OxideBitboard(0xff00000000));
        assert_eq!(OxideBitboard::line_fill(A5, C5), OxideBitboard(0xff00000000));
        assert_eq!(OxideBitboard::line_fill(B5, C5), OxideBitboard(0xff00000000));
        assert_eq!(OxideBitboard::line_fill(C5, F5), OxideBitboard(0xff00000000));
    }
}

#[cfg(test)]
mod bench {
    extern crate test;

    use test::Bencher;
    use super::*;
    use crate::game::square::OxideSquare::*;

    #[bench]
    fn between_fill_connected_bench(bencher: &mut Bencher) {
        let from = test::black_box(A4);
        let to = test::black_box(H4);
        bencher.iter(|| OxideBitboard::between_fill(from, to));
    }

    #[bench]
    fn between_fill_disconnected_bench(bencher: &mut Bencher) {
        let from = test::black_box(A4);
        let to = test::black_box(B7);
        bencher.iter(|| OxideBitboard::between_fill(from, to));
    }

    #[bench]
    #[cfg(feature = "low_memory")]
    fn line_bb_connected_bench(bencher: &mut Bencher) {
        let from = test::black_box(A4);
        let to = test::black_box(H4);
        bencher.iter(|| OxideBitboard::line_fill(from, to));
    }

    #[bench]
    #[cfg(feature = "low_memory")]
    fn line_bb_disconnected_bench(bencher: &mut Bencher) {
        let from = test::black_box(A4);
        let to = test::black_box(B7);
        bencher.iter(|| OxideBitboard::line_fill(from, to));
    }

    #[bench]
    fn line_fill_connected_bench(bencher: &mut Bencher) {
        let from = test::black_box(A4);
        let to = test::black_box(H4);
        bencher.iter(|| OxideBitboard::line_fill(from, to));
    }

    #[bench]
    fn line_fill_disconnected_bench(bencher: &mut Bencher) {
        let from = test::black_box(A4);
        let to = test::black_box(B7);
        bencher.iter(|| OxideBitboard::line_fill(from, to));
    }
}