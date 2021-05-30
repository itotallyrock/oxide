
use interface::game::{BoardMask, Position};

#[inline]
pub fn knight_attacks<P: Position>(knight_mask: P::BoardMask) -> P::BoardMask {
    (((knight_mask << 15) | (knight_mask >> 17)) & !P::BoardMask::H_FILE)
        | (((knight_mask >> 15) | (knight_mask << 17)) & !P::BoardMask::A_FILE)
        | (((knight_mask << 6) | (knight_mask >> 10)) & !(P::BoardMask::G_FILE | P::BoardMask::H_FILE))
        | (((knight_mask >> 6) | (knight_mask << 10)) & !(P::BoardMask::A_FILE | P::BoardMask::B_FILE))
}

#[cfg(test)]
mod test {
    use super::*;
    use oxide_interface::game::OxideBitboard;
    use oxide_interface::engine::OxidePosition;

    #[test]
    fn knight_attacks_works() {
        assert_eq!(knight_attacks::<OxidePosition>(OxideBitboard(0x40000000000u64)), OxideBitboard(0xa1100110a000000u64));
        assert_eq!(knight_attacks::<OxidePosition>(OxideBitboard(0x2000000000u64)), OxideBitboard(0x50880088500000u64));
        assert_eq!(knight_attacks::<OxidePosition>(OxideBitboard(0x80u64)), OxideBitboard(0x402000u64));
    }
}

#[cfg(test)]
mod bench {
    // Local imports
    use super::*;

    // External test for benchmarking
    extern crate test;
    use test::Bencher;
    use oxide_interface::game::OxideSquare::E3;
    use interface::game::Square;
    use oxide_interface::engine::OxidePosition;

    #[bench]
    fn knight_attack_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        bencher.iter(|| knight_attacks::<OxidePosition>(from_mask));
    }
}
