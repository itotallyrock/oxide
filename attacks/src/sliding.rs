use interface::game::{Position, BoardMask};

#[inline]
pub fn bishop_attacks<P: Position>(from_mask: P::BoardMask, occupied_mask: P::BoardMask) -> P::BoardMask {
    from_mask.diagonal_ray_attacks(!occupied_mask)
}

#[inline]
pub fn rook_attacks<P: Position>(from_mask: P::BoardMask, occupied_mask: P::BoardMask) -> P::BoardMask {
    from_mask.cardinal_ray_attacks(!occupied_mask)
}

#[inline]
pub fn queen_attacks<P: Position>(from_mask: P::BoardMask, occupied_mask: P::BoardMask) -> P::BoardMask {
    bishop_attacks::<P>(from_mask, occupied_mask) | rook_attacks::<P>(from_mask, occupied_mask)
}

#[cfg(test)]
mod test {
    use super::*;
    use oxide_interface::game::OxideBitboard;
    use oxide_interface::engine::OxidePosition;

    #[test]
    fn bishop_attack_works() {
        // No attackers, no blockers
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideBitboard(0)), OxideBitboard::EMPTY);
        // No attackers, all blocking
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideBitboard::FULL), OxideBitboard::EMPTY);
        // 1 attacker, all blocking
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard(0x8000000u64), OxideBitboard::FULL), OxideBitboard(0x1400140000u64));
        // 1 attacker, no blocking
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard(0x200000000000u64), OxideBitboard(0x200000000000u64)), OxideBitboard(0x8850005088040201u64));
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard(0x80u64), OxideBitboard(0x80u64)), OxideBitboard(0x102040810204000u64));
        // 1 attacker multiple blocking
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard(0x4000000u64), OxideBitboard(0x20014004024004u64)), OxideBitboard(0x20110a000a1020u64));
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard(0x400000000000u64), OxideBitboard(0x4000f8000000u64)), OxideBitboard(0x10a000a010000000u64));
        // Multiple attackers no blocking
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard(0x40040000000u64), OxideBitboard(0x40040000000u64)), OxideBitboard(0x150a10aa11a05088u64));
        // Multiple attackers aligned
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard(0x400000080000u64), OxideBitboard(0x400000080000u64)), OxideBitboard(0x10a041a214081422u64));
        // Multiple attackers multiple blockers
        assert_eq!(bishop_attacks::<OxidePosition>(OxideBitboard(0x48000200000u64), OxideBitboard(0x2458004200080u64)), OxideBitboard(0x100a448a51205088u64));
    }

    #[test]
    fn rook_attack_works() {
        // No attackers, no blockers
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideBitboard(0)), OxideBitboard::EMPTY);
        // No attackers, all blocking
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideBitboard::FULL), OxideBitboard::EMPTY);
        // 1 attacker, all blocking
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard(0x8000000u64), OxideBitboard::FULL), OxideBitboard(0x814080000u64));
        // 1 attacker, no blocking
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard(0x200000000000u64), OxideBitboard(0x200000000000u64)), OxideBitboard(0x2020df2020202020u64));
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard(0x80u64), OxideBitboard(0x80u64)), OxideBitboard(0x808080808080807fu64));
        // 1 attacker multiple blocking
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard(0x4000000u64), OxideBitboard(0x425001000u64)), OxideBitboard(0x43b040404u64));
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard(0x400000000000u64), OxideBitboard(0x4000f8000000u64)), OxideBitboard(0x4040bf4040000000u64));
        // Multiple attackers no blocking
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard(0x40040000000u64), OxideBitboard(0x40040000000u64)), OxideBitboard(0x4444fb44bf444444u64));
        // Multiple attackers aligned
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard(0x440000000000u64), OxideBitboard(0x440000000000u64)), OxideBitboard(0x4444ff4444444444u64));
        // Multiple attackers multiple blockers
        assert_eq!(rook_attacks::<OxidePosition>(OxideBitboard(0x2000048000000u64), OxideBitboard(0x120040ca040800u64)), OxideBitboard(0xa1d0a4afe484840u64));
    }

    #[test]
    fn queen_attack_works() {
        // No attackers, no blockers
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideBitboard(0)), OxideBitboard::EMPTY);
        // No attackers, all blocking
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard::EMPTY, OxideBitboard::FULL), OxideBitboard::EMPTY);
        // 1 attacker, all blocking
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard(0x8000000u64), OxideBitboard::FULL), OxideBitboard(0x1c141c0000u64));
        // 1 attacker, no blocking
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard(0x8000000u64), OxideBitboard(0x200000000000u64)), OxideBitboard(0x8092a1cf71c2a49u64));
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard(0x80u64), OxideBitboard(0x80u64)), OxideBitboard(0x8182848890a0c07fu64));
        // 1 attacker multiple blocking
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard(0x200000u64), OxideBitboard(0x3800622000u64)), OxideBitboard(0xa8705e7088u64));
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard(0x400000000000u64), OxideBitboard(0x4000f8000000u64)), OxideBitboard(0x50e0bfe050000000u64));
        // Multiple attackers no blocking
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard(0x40040000000u64), OxideBitboard(0x40040000000u64)), OxideBitboard(0x554efbeebfe454ccu64));
        // Multiple attackers aligned
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard(0x440000004000u64), OxideBitboard(0x440000004000u64)), OxideBitboard(0x55eeffee55ecffe6u64));
        // Multiple attackers multiple blockers
        assert_eq!(queen_attacks::<OxidePosition>(OxideBitboard(0x42000000u64), OxideBitboard(0x1244e6801000u64)), OxideBitboard(0x12e7a5e75a52u64));
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
    fn bishop_attacks_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        let occupied_mask = test::black_box(E3.to_mask());
        bencher.iter(|| bishop_attacks::<OxidePosition>(from_mask, occupied_mask));
    }

    #[bench]
    fn rook_attacks_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        let occupied_mask = test::black_box(E3.to_mask());
        bencher.iter(|| rook_attacks::<OxidePosition>(from_mask, occupied_mask));
    }

    #[bench]
    fn queen_attacks_bench(bencher: &mut Bencher) {
        let from_mask = test::black_box(E3.to_mask());
        let occupied_mask = test::black_box(E3.to_mask());
        bencher.iter(|| queen_attacks::<OxidePosition>(from_mask, occupied_mask));
    }
}