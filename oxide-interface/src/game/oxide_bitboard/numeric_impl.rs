
use crate::game::OxideBitboard;
use std::ops::{BitOr, BitXor, BitAnd, Not};
use std::fmt::{UpperHex, LowerHex, Formatter, Result as FormatResult};

impl const BitOr for OxideBitboard {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        OxideBitboard(self.0 | rhs.0)
    }
}

impl const BitXor for OxideBitboard {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        OxideBitboard(self.0 ^ rhs.0)
    }
}

impl const BitAnd for OxideBitboard {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        OxideBitboard(self.0 & rhs.0)
    }
}

impl const Not for OxideBitboard {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        OxideBitboard(!self.0)
    }
}

impl UpperHex for OxideBitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{:X}", self.0)
    }
}

impl LowerHex for OxideBitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{:x}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::game::OxideBitboard;
    use interface::game::BoardMask;

    #[test]
    fn bit_or_works() {
        assert_eq!(OxideBitboard::FULL | OxideBitboard::EMPTY, OxideBitboard::FULL);
        assert_eq!(OxideBitboard::A_FILE | OxideBitboard::B_FILE, OxideBitboard(0x303030303030303u64));
        assert_eq!(OxideBitboard(0x4A8E260828000449u64) | OxideBitboard(0x40004402024200u64), OxideBitboard(0x4ACE264C2A024649u64));
    }

    #[test]
    fn bit_xor_works() {
        assert_eq!(OxideBitboard::FULL ^ OxideBitboard::EMPTY, OxideBitboard::FULL);
        assert_eq!(OxideBitboard::A_FILE ^ OxideBitboard::A_FILE, OxideBitboard::EMPTY);
        assert_eq!(OxideBitboard::A_FILE ^ OxideBitboard::B_FILE, OxideBitboard(0x303030303030303u64));
        assert_eq!(OxideBitboard(0x4A8E260828000449u64) ^ OxideBitboard(0x40004c766a6201), OxideBitboard(0x4ACE26445E6A6648u64));
    }

    #[test]
    fn bit_and_works() {
        assert_eq!(OxideBitboard::FULL & OxideBitboard::EMPTY, OxideBitboard::EMPTY);
        assert_eq!(OxideBitboard::A_FILE & OxideBitboard::A_FILE, OxideBitboard::A_FILE);
        assert_eq!(OxideBitboard::A_FILE & OxideBitboard::B_FILE, OxideBitboard::EMPTY);
        assert_eq!(OxideBitboard(0x4A8E260828000449u64) & OxideBitboard(0x40004c766a6201u64), OxideBitboard(0x820000001u64));
    }

    #[test]
    fn not_works() {
        assert_eq!(!OxideBitboard::FULL, OxideBitboard::EMPTY);
        assert_eq!(!OxideBitboard(0x1u64), OxideBitboard(0xFFFFFFFFFFFFFFFEu64));
        assert_eq!(!OxideBitboard(0x4A8E260828000449u64), OxideBitboard(0xB571D9F7D7FFFBB6u64));
    }
}