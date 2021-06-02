
use interface::game::{Position, Shiftable};

#[inline]
pub fn king_attacks<P: Position>(king_mask: P::BoardMask) -> P::BoardMask {
    let side_attacks = king_mask.east_shift() | king_mask.west_shift();
    let horizontal_attacks_unshifted = side_attacks | king_mask;

    side_attacks | horizontal_attacks_unshifted.north_shift() | horizontal_attacks_unshifted.south_shift()
}

#[cfg(test)]
mod test {
    use super::*;
    use oxide_interface::game::OxideBitboard;
    use oxide_interface::engine::OxidePosition;

    #[test]
    fn king_attacks_works() {
        assert_eq!(king_attacks::<OxidePosition>(OxideBitboard(1)), OxideBitboard(0x302u64));
        assert_eq!(king_attacks::<OxidePosition>(OxideBitboard(0x2000000000u64)), OxideBitboard(0x705070000000u64));
        assert_eq!(king_attacks::<OxidePosition>(OxideBitboard(0x80000000000000u64)), OxideBitboard(0xc040c00000000000u64));
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

    #[bench]
    fn king_attack_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        bencher.iter(|| king_attacks::<OxidePosition>(from_mask));
    }
}
