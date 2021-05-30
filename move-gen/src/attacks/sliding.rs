use interface::game::Position;

#[inline]
pub fn bishop_attacks<P: Position>(from_square: P::Square, occupied_mask: P::BoardMask) -> P::BoardMask {
    #[cfg(feature = "sliding_attack_lookup")] {
        return sliding_attack_lookup::bishop_attacks_lookup::<P>(from_square, occupied_mask);
    }
    #[cfg(not(feature = "sliding_attack_lookup"))] {
        from_square.to_mask().diagonal_ray_attacks(!occupied_mask)
    }
}

#[inline]
pub fn rook_attacks<P: Position>(from_square: P::Square, occupied_mask: P::BoardMask) -> P::BoardMask {
    #[cfg(feature = "sliding_attack_lookup")] {
        return sliding_attack_lookup::rook_attacks_lookup::<P>(from_square, occupied_mask);
    }
    #[cfg(not(feature = "sliding_attack_lookup"))] {
        from_square.to_mask().cardinal_ray_attacks(!occupied_mask)
    }
}

#[inline]
pub fn queen_attacks<P: Position>(from_square: P::Square, occupied_mask: P::BoardMask) -> P::BoardMask {
    bishop_attacks::<P>(from_square, occupied_mask) | rook_attacks::<P>(from_square, occupied_mask)
}

#[cfg(feature = "sliding_attack_lookup")]
mod sliding_attack_lookup;