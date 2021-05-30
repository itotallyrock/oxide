
#[inline]
pub const fn pdep(value: u64, mut mask: u64) -> u64 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))] {
        return unsafe { std::arch::x86_64::_pdep_u64(value, mask) };
    }

    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))] {
        let mut res = 0;
        let mut bb = 1;
        // Loop until no bits are left in mask
        while mask != 0 {
            // Deposit the masked bits into res
            if (value & bb) != 0 {
                res |= mask & mask.wrapping_neg();
            }

            // Remove lowest bit from kas
            mask &= mask - 1;
            // Increment which bit we are on
            bb += bb;
        }

        res
    }
}
