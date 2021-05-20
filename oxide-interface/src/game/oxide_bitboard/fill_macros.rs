
// Macros for reducing code duplications in occluded fills
#[macro_export]
macro_rules! fill_masked {
    ($mask:ident, $column_mask:ident << $coefficient:expr) => {{
        const MASK_1: u64 = $column_mask & ($column_mask << $coefficient);
        const MASK_2: u64  = MASK_1 & (MASK_1 << (2 * $coefficient));
        $mask |= $column_mask & ($mask << $coefficient);
        $mask |= MASK_1 & ($mask << (2 * $coefficient));
        $mask |= MASK_2 & ($mask << (4 * $coefficient));
    }};
    ($mask:ident, $column_mask:ident >> $coefficient:expr) => {{
        const MASK_1: u64 = $column_mask & ($column_mask >> $coefficient);
        const MASK_2: u64  = MASK_1 & (MASK_1 >> (2 * $coefficient));
        $mask |= $column_mask & ($mask >> $coefficient);
        $mask |= MASK_1 & ($mask >> (2 * $coefficient));
        $mask |= MASK_2 & ($mask >> (4 * $coefficient));
    }};
}

#[macro_export]
macro_rules! fill_occluded_mask {
    ($mask:ident, $empty:ident, $column_mask:ident << $coefficient:expr) => {{
        $empty  &= $column_mask;
        $mask   |= $empty & ($mask  << $coefficient);
        $empty  &=          ($empty << $coefficient);
        $mask   |= $empty & ($mask  << (2 * $coefficient));
        $empty  &=          ($empty << (2 * $coefficient));
        $mask   |= $empty & ($mask  << (4 * $coefficient));
    }};
    ($mask:ident, $empty:ident, $column_mask:ident >> $coefficient:expr) => {{
        $empty  &= $column_mask;
        $mask   |= $empty & ($mask  >> $coefficient);
        $empty  &=          ($empty >> $coefficient);
        $mask   |= $empty & ($mask  >> (2 * $coefficient));
        $empty  &=          ($empty >> (2 * $coefficient));
        $mask   |= $empty & ($mask  >> (4 * $coefficient));
    }};
}