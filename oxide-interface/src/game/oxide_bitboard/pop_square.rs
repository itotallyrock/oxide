use interface::game::{Square, Position};

#[inline(always)]
fn blsr(mask: u64) -> u64 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi1"))] {
        return std::arch::x86_64::blsr_u64(mask);
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi1")))] {
        mask & mask.saturating_sub(1)
    }
}

#[inline(always)]
pub fn pop_square<P: Position>(mask: &mut u64) -> Option<P::Square> {
    if *mask == 0 {
        None
    } else {
        let offset = mask.trailing_zeros();
        let square = P::Square::from_offset(offset as u8);
        *mask = blsr(*mask);

        square
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::square::OxideSquare::*;
    use crate::engine::OxidePosition;

    #[test]
    fn blsr_works() {
        assert_eq!(blsr(0), 0);
        assert_eq!(blsr(1), 0);
        assert_eq!(blsr(2), 0);
        assert_eq!(blsr(3), 2);
        assert_eq!(blsr(4), 0);
        assert_eq!(blsr(5), 4);
        assert_eq!(blsr(6), 4);
        assert_eq!(blsr(0xb659ac122ea95e2a), 0xb659ac122ea95e28);
        assert_eq!(blsr(0xb659ac122ea95e28), 0xb659ac122ea95e20);
        assert_eq!(blsr(0xb659ac122ea95e20), 0xb659ac122ea95e00);
        assert_eq!(blsr(0xb659ac122ea95e00), 0xb659ac122ea95c00);
        assert_eq!(blsr(0xb659ac122ea95c00), 0xb659ac122ea95800);
        assert_eq!(blsr(0xb659ac122ea95800), 0xb659ac122ea95000);
        assert_eq!(blsr(0xb659ac122ea95000), 0xb659ac122ea94000);
        assert_eq!(blsr(0xb659ac122ea94000), 0xb659ac122ea90000);
    }

    #[test]
    fn pop_square_works() {
        let mut mask = 0;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), None);
        assert_eq!(mask, 0);
        let mut mask = 0xb659ac122ea95e2a;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(B1));
        assert_eq!(mask, 0xb659ac122ea95e28);
        let mut mask = 0xb659ac122ea95e28;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(D1));
        assert_eq!(mask, 0xb659ac122ea95e20);
        let mut mask = 0xb659ac122ea95e20;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(F1));
        assert_eq!(mask, 0xb659ac122ea95e00);
        let mut mask = 0xb659ac122ea95e00;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(B2));
        assert_eq!(mask, 0xb659ac122ea95c00);
        let mut mask = 0xb659ac122ea95c00;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(C2));
        assert_eq!(mask, 0xb659ac122ea95800);
        let mut mask = 0xb659ac122ea95800;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(D2));
        assert_eq!(mask, 0xb659ac122ea95000);
        let mut mask = 0xb659ac122ea95000;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(E2));
        assert_eq!(mask, 0xb659ac122ea94000);
        let mut mask = 0xb659ac122ea94000;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(G2));
        assert_eq!(mask, 0xb659ac122ea90000);
        let mut mask = 0x8000000000000001;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(A1));
        assert_eq!(mask, 0x8000000000000000);
        let mut mask = 0x8000000000000000;
        assert_eq!(pop_square::<OxidePosition>(&mut mask), Some(H8));
        assert_eq!(mask, 0);
    }
}