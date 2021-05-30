use interface::game::{Square, Position};

mod pdep;
mod pext;

use pext::pext;
use pdep::pdep;

const MAX_BISHOP_VARIATIONS: usize = 512;
const MAX_ROOK_VARIATIONS: usize = 4096;

const ROOK_BLOCKER_COUNTS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];
const BISHOP_BLOCKER_COUNTS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

const BISHOP_OCCUPANCY_MASK: [u64; 64] = [
    0x40201008040200, 0x402010080400,   0x4020100A00,     0x40221400,       0x2442800,        0x204085000,      0x20408102000,    0x2040810204000,
    0x20100804020000, 0x40201008040000, 0x4020100A0000,   0x4022140000,     0x244280000,      0x20408500000,    0x2040810200000,  0x4081020400000,
    0x10080402000200, 0x20100804000400, 0x4020100A000A00, 0x402214001400,   0x24428002800,    0x2040850005000,  0x4081020002000,  0x8102040004000,
    0x8040200020400,  0x10080400040800, 0x20100A000A1000, 0x40221400142200, 0x2442800284400,  0x4085000500800,  0x8102000201000,  0x10204000402000,
    0x4020002040800,  0x8040004081000,  0x100A000A102000, 0x22140014224000, 0x44280028440200, 0x8500050080400,  0x10200020100800, 0x20400040201000,
    0x2000204081000,  0x4000408102000,  0xA000A10204000,  0x14001422400000, 0x28002844020000, 0x50005008040200, 0x20002010080400, 0x40004020100800,
    0x20408102000,    0x40810204000,    0xA1020400000,    0x142240000000,   0x284402000000,   0x500804020000,   0x201008040200,   0x402010080400,
    0x2040810204000,  0x4081020400000,  0xA102040000000,  0x14224000000000, 0x28440200000000, 0x50080402000000, 0x20100804020000, 0x40201008040200,
];

const ROOK_OCCUPANCY_MASK: [u64; 64] = [
    0x101010101017E,    0x202020202027C,    0x404040404047A,    0x8080808080876,    0x1010101010106E,   0x2020202020205E,   0x4040404040403E,   0x8080808080807E,
    0x1010101017E00,    0x2020202027C00,    0x4040404047A00,    0x8080808087600,    0x10101010106E00,   0x20202020205E00,   0x40404040403E00,   0x80808080807E00,
    0x10101017E0100,    0x20202027C0200,    0x40404047A0400,    0x8080808760800,    0x101010106E1000,   0x202020205E2000,   0x404040403E4000,   0x808080807E8000,
    0x101017E010100,    0x202027C020200,    0x404047A040400,    0x8080876080800,    0x1010106E101000,   0x2020205E202000,   0x4040403E404000,   0x8080807E808000,
    0x1017E01010100,    0x2027C02020200,    0x4047A04040400,    0x8087608080800,    0x10106E10101000,   0x20205E20202000,   0x40403E40404000,   0x80807E80808000,
    0x17E0101010100,    0x27C0202020200,    0x47A0404040400,    0x8760808080800,    0x106E1010101000,   0x205E2020202000,   0x403E4040404000,   0x807E8080808000,
    0x7E010101010100,   0x7C020202020200,   0x7A040404040400,   0x76080808080800,   0x6E101010101000,   0x5E202020202000,   0x3E404040404000,   0x7E808080808000,
    0x7E01010101010100, 0x7C02020202020200, 0x7A04040404040400, 0x7608080808080800, 0x6E10101010101000, 0x5E20202020202000, 0x3E40404040404000, 0x7E80808080808000,
];

static BISHOP_ATTACK_TABLE: [[u64; MAX_BISHOP_VARIATIONS]; 64] = {
    let mut attack_table = [[0u64; MAX_BISHOP_VARIATIONS]; 64];

    let mut offset = 0;
    while offset < 64 {
        let from_mask = 1u64 << offset;
        let occupancy_mask = BISHOP_OCCUPANCY_MASK[offset];
        let max_blockers = BISHOP_BLOCKER_COUNTS[offset];

        let mut blocker_index = 0;
        while blocker_index < 1u16 << max_blockers {
            let blocker_mask = pdep(blocker_index as u64, occupancy_mask);
            // let attack_mask = BoardMask::diagonal_ray_attacks(from_mask, !blocker_mask);
            // attack_table[offset][blocker_index as usize] = attack_mask;
            blocker_index += 1;
        }
        offset += 1;
    }

    attack_table
};
static ROOK_ATTACK_TABLE: [[u64; MAX_ROOK_VARIATIONS]; 64] = {
    let mut attack_table = [[0u64; MAX_ROOK_VARIATIONS]; 64];

    let mut offset = 0;
    while offset < 64 {
        let from_mask = 1u64 << offset;
        let occupancy_mask = ROOK_OCCUPANCY_MASK[offset];
        let max_blockers = ROOK_BLOCKER_COUNTS[offset];

        let mut blocker_index = 0;
        while blocker_index < 1u16 << max_blockers {
            let blocker_mask = pdep(blocker_index as u64, occupancy_mask);
            // let attack_mask = BoardMask::cardinal_ray_attacks(from_mask, !blocker_mask);
            // attack_table[offset][blocker_index as usize] = attack_mask;
            blocker_index += 1;
        }
        offset += 1;
    }

    attack_table
};

#[inline]
pub fn bishop_attacks_lookup<P: Position>(from_square: P::Square, occupied_mask: P::BoardMask) -> P::BoardMask {
    let square_offset = from_square.offset() as usize;
    let occupancy_mask = BISHOP_OCCUPANCY_MASK[square_offset];
    // let index = pext(occupied_mask, occupancy_mask) as usize;

    // BISHOP_ATTACK_TABLE[square_offset][index]
    todo!()
}

#[inline]
pub fn rook_attacks_lookup<P: Position>(from_square: P::Square, occupied_mask: P::BoardMask) -> P::BoardMask {
    let square_offset = from_square.offset() as usize;
    let occupancy_mask = ROOK_OCCUPANCY_MASK[square_offset];
    // let index = pext(occupied_mask, occupancy_mask) as usize;

    // ROOK_ATTACK_TABLE[square_offset][index]
    todo!()
}
