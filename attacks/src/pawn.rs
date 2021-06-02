
use interface::game::{Position, Shiftable, Side};

#[inline]
pub fn pawn_pushes<P: Position>(pawn_mask: P::BoardMask, side: P::Side) -> P::BoardMask {
    if side.is_white() {
        pawn_mask.north_shift()
    } else {
        pawn_mask.south_shift()
    }
}

#[inline]
pub fn pawn_west_attacks<P: Position>(pawn_mask: P::BoardMask, side: P::Side) -> P::BoardMask {
    if side.is_white() {
        pawn_mask.north_west_shift()
    } else {
        pawn_mask.south_west_shift()
    }
}

#[inline]
pub fn pawn_east_attacks<P: Position>(pawn_mask: P::BoardMask, side: P::Side) -> P::BoardMask {
    if side.is_white() {
        pawn_mask.north_east_shift()
    } else {
        pawn_mask.south_east_shift()
    }
}

#[inline]
pub fn pawn_attacks<P: Position>(pawn_mask: P::BoardMask, side: P::Side) -> P::BoardMask {
    pawn_west_attacks::<P>(pawn_mask, side) | pawn_east_attacks::<P>(pawn_mask, side)
}

#[cfg(test)]
mod test {
    use super::*;
    use oxide_interface::game::{OxideBitboard, OxideSide};
    use oxide_interface::engine::OxidePosition;
    use interface::game::BoardMask;

    #[test]
    fn pawn_attacks_works() {
        assert_eq!(pawn_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideSide::White), OxideBitboard::EMPTY);
        assert_eq!(pawn_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideSide::Black), OxideBitboard::EMPTY);
        assert_eq!(pawn_attacks::<OxidePosition>(OxideBitboard(0x8000000), OxideSide::White), OxideBitboard(0x1400000000));
        assert_eq!(pawn_attacks::<OxidePosition>(OxideBitboard(0x8000000), OxideSide::Black), OxideBitboard(0x140000));
        assert_eq!(pawn_attacks::<OxidePosition>(OxideBitboard(0x201004400000), OxideSide::White), OxideBitboard(0x50280aa0000000));
        assert_eq!(pawn_attacks::<OxidePosition>(OxideBitboard(0x201004400000), OxideSide::Black), OxideBitboard(0x50280aa000));
        assert_eq!(pawn_attacks::<OxidePosition>(OxideBitboard(0xff00), OxideSide::White), OxideBitboard(0xff0000));
        assert_eq!(pawn_attacks::<OxidePosition>(OxideBitboard(0xff000000000000), OxideSide::Black), OxideBitboard(0xff0000000000));
    }

    #[test]
    fn pawn_pushes_works() {
        assert_eq!(pawn_pushes::<OxidePosition>(OxideBitboard::EMPTY, OxideSide::White), OxideBitboard::EMPTY);
        assert_eq!(pawn_pushes::<OxidePosition>(OxideBitboard::EMPTY, OxideSide::Black), OxideBitboard::EMPTY);
        assert_eq!(pawn_pushes::<OxidePosition>(OxideBitboard(0x8000000), OxideSide::White), OxideBitboard(0x800000000));
        assert_eq!(pawn_pushes::<OxidePosition>(OxideBitboard(0x8000000), OxideSide::Black), OxideBitboard(0x80000));
        assert_eq!(pawn_pushes::<OxidePosition>(OxideBitboard(0x201004400000), OxideSide::White), OxideBitboard(0x20100440000000));
        assert_eq!(pawn_pushes::<OxidePosition>(OxideBitboard(0x201004400000), OxideSide::Black), OxideBitboard(0x2010044000));
        assert_eq!(pawn_pushes::<OxidePosition>(OxideBitboard(0xff00), OxideSide::White), OxideBitboard(0xff0000));
        assert_eq!(pawn_pushes::<OxidePosition>(OxideBitboard(0xff000000000000), OxideSide::Black), OxideBitboard(0xff0000000000));
    }

    #[test]
    fn pawn_west_attacks_works() {
        assert_eq!(pawn_west_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideSide::White), OxideBitboard::EMPTY);
        assert_eq!(pawn_west_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideSide::Black), OxideBitboard::EMPTY);
        assert_eq!(pawn_west_attacks::<OxidePosition>(OxideBitboard(0x100000), OxideSide::White), OxideBitboard(0x8000000));
        assert_eq!(pawn_west_attacks::<OxidePosition>(OxideBitboard(0x20000), OxideSide::White), OxideBitboard(0x1000000));
        assert_eq!(pawn_west_attacks::<OxidePosition>(OxideBitboard(0x1000000), OxideSide::White), OxideBitboard::EMPTY);
        assert_eq!(pawn_west_attacks::<OxidePosition>(OxideBitboard(0x100000), OxideSide::Black), OxideBitboard(0x800));
        assert_eq!(pawn_west_attacks::<OxidePosition>(OxideBitboard(0x20000), OxideSide::Black), OxideBitboard(0x100));
        assert_eq!(pawn_west_attacks::<OxidePosition>(OxideBitboard(0x1000000), OxideSide::Black), OxideBitboard::EMPTY);
    }

    #[test]
    fn pawn_east_attacks_works() {
        assert_eq!(pawn_east_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideSide::White), OxideBitboard::EMPTY);
        assert_eq!(pawn_east_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideSide::Black), OxideBitboard::EMPTY);
        assert_eq!(pawn_east_attacks::<OxidePosition>(OxideBitboard(0x100000), OxideSide::White), OxideBitboard(0x20000000));
        assert_eq!(pawn_east_attacks::<OxidePosition>(OxideBitboard(0x20000000), OxideSide::White), OxideBitboard(0x4000000000));
        assert_eq!(pawn_east_attacks::<OxidePosition>(OxideBitboard(0x800000000000), OxideSide::White), OxideBitboard::EMPTY);
        assert_eq!(pawn_east_attacks::<OxidePosition>(OxideBitboard(0x100000), OxideSide::Black), OxideBitboard(0x2000));
        assert_eq!(pawn_east_attacks::<OxidePosition>(OxideBitboard(0x20000000), OxideSide::Black), OxideBitboard(0x400000));
        assert_eq!(pawn_east_attacks::<OxidePosition>(OxideBitboard(0x800000000000), OxideSide::Black), OxideBitboard::EMPTY);
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    extern crate test;
    use test::Bencher;
    use oxide_interface::game::OxideSquare::E3;
    use oxide_interface::engine::OxidePosition;
    use interface::game::Square;
    use oxide_interface::game::OxideSide;

    #[bench]
    fn white_pawn_attacks_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        bencher.iter(|| pawn_attacks::<OxidePosition>(from_mask, OxideSide::White));
    }

    #[bench]
    fn black_pawn_attacks_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        bencher.iter(|| pawn_attacks::<OxidePosition>(from_mask, OxideSide::Black));
    }

    #[bench]
    fn white_pawn_pushes_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        bencher.iter(|| pawn_pushes::<OxidePosition>(from_mask, OxideSide::White));
    }

    #[bench]
    fn black_pawn_pushes_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        bencher.iter(|| pawn_pushes::<OxidePosition>(from_mask, OxideSide::Black));
    }
}
