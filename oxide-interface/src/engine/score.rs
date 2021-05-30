use interface::engine::PositionalScore;
use std::ops::{Neg, Div, Mul, Add, Sub};
use interface::types::PlyCount;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default, Debug)]
pub struct OxideScore(i32);

impl Neg for OxideScore {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Mul for OxideScore {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl Div for OxideScore {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}


impl Add for OxideScore {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}


impl Sub for OxideScore {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl PositionalScore for OxideScore {
    const MATE_SCORE: Self = Self(i32::MAX);
    fn new(centipawns: i32) -> Self {
        Self(centipawns)
    }

    fn new_mate(plies_to_mate: u16) -> Self {
        Self(OxideScore::MATE_SCORE.0 - plies_to_mate as i32)
    }

    fn is_mate(&self) -> bool {
        const MAX_MATE_PLIES: PlyCount = 50;
        Self::MATE_SCORE - Self(MAX_MATE_PLIES as i32) >= *self
    }

    fn mate_in(&self) -> Option<u16> {
        if self.is_mate() {
            Some((Self::MATE_SCORE - *self).0 as u16)
        } else {
            None
        }
    }

    fn centipawns(&self) -> i32 {
        self.0
    }
}