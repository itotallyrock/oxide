
#[inline]
pub const fn pext(value: u64, mut mask: u64) -> u64 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))] {
        return unsafe { std::arch::x86_64::_pext_u64(value, mask) };
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))] {
        let mut res = 0;
        let mut bb = 1;
        // Loop until no bits left in mask
        while mask != 0 {
            // Extract a bit from the masked value
            if value & mask & (mask.wrapping_neg()) != 0 {
                res |= bb;
            }

            // Remove lowest set bit from mask
            mask &= mask - 1;
            // Increment which bit we are on
            bb += bb;
        }
        res
    }
}