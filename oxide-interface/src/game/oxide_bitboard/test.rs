use super::*;
use crate::game::OxideSquare;

#[test]
fn masks_are_valid() {
    assert_eq!(OxideBitboard::SQUARE.0, 0x1u64);
    assert_eq!(OxideBitboard::EMPTY.0, 0x0u64);
    assert_eq!(OxideBitboard::FULL.0, 0xffffffffffffffffu64);
    assert_eq!(OxideBitboard::A_FILE.0, 0x101010101010101u64);
    assert_eq!(OxideBitboard::B_FILE.0, 0x202020202020202u64);
    assert_eq!(OxideBitboard::C_FILE.0, 0x404040404040404u64);
    assert_eq!(OxideBitboard::D_FILE.0, 0x808080808080808u64);
    assert_eq!(OxideBitboard::E_FILE.0, 0x1010101010101010u64);
    assert_eq!(OxideBitboard::F_FILE.0, 0x2020202020202020u64);
    assert_eq!(OxideBitboard::G_FILE.0, 0x4040404040404040u64);
    assert_eq!(OxideBitboard::H_FILE.0, 0x8080808080808080u64);
    assert_eq!(OxideBitboard::RANK_1.0, 0xffu64);
    assert_eq!(OxideBitboard::RANK_2.0, 0xff00u64);
    assert_eq!(OxideBitboard::RANK_3.0, 0xff0000u64);
    assert_eq!(OxideBitboard::RANK_4.0, 0xff000000u64);
    assert_eq!(OxideBitboard::RANK_5.0, 0xff00000000u64);
    assert_eq!(OxideBitboard::RANK_6.0, 0xff0000000000u64);
    assert_eq!(OxideBitboard::RANK_7.0, 0xff000000000000u64);
    assert_eq!(OxideBitboard::RANK_8.0, 0xff00000000000000u64);
}

#[test]
fn fill_north_works() {
    assert_eq!(OxideBitboard::north_fill(OxideBitboard(0x3040a1024408800)), OxideBitboard(0xfffefefcecc88800));
    assert_eq!(OxideBitboard::north_fill(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xffffffffffffffff));
    assert_eq!(OxideBitboard::north_fill(OxideBitboard(0x0)), OxideBitboard(0x0));
}

#[test]
fn fill_south_works() {
    assert_eq!(OxideBitboard::south_fill(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x3070f1f3f7fffff));
    assert_eq!(OxideBitboard::south_fill(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xffffffffffffffff));
    assert_eq!(OxideBitboard::south_fill(OxideBitboard(0x0)), OxideBitboard(0x0));
}

#[test]
fn fill_east_works() {
    assert_eq!(OxideBitboard::east_fill(OxideBitboard(0x3040a1024408800)), OxideBitboard(0xfffcfef0fcc0f800));
    assert_eq!(OxideBitboard::east_fill(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xffffffffffffffff));
    assert_eq!(OxideBitboard::east_fill(OxideBitboard(0x0)), OxideBitboard(0x0));
}

#[test]
fn fill_west_works() {
    assert_eq!(OxideBitboard::west_fill(OxideBitboard(0x3040a1024408800)), OxideBitboard(0x3070f1f3f7fff00));
    assert_eq!(OxideBitboard::west_fill(OxideBitboard(0xffffffffffffffff)), OxideBitboard(0xffffffffffffffff));
    assert_eq!(OxideBitboard::west_fill(OxideBitboard(0x0)), OxideBitboard(0x0));
}

#[test]
fn fill_occluded_north_works() {
    assert_eq!(OxideBitboard::north_occluded_fill(OxideBitboard(0x100020004000800), OxideBitboard(0xfcfbf5efdbbf77ff)), OxideBitboard(0x102060c0c080800));
    assert_eq!(OxideBitboard::north_occluded_fill(OxideBitboard(0x8142242418000000), OxideBitboard(0x5abdc3dbe7ffffff)), OxideBitboard(0xc366243c18000000));
    assert_eq!(OxideBitboard::north_occluded_fill(OxideBitboard(0x0), OxideBitboard(0x0)), OxideBitboard(0x0));
    assert_eq!(OxideBitboard::north_occluded_fill(OxideBitboard(0x0), OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x0));
}

#[test]
fn fill_occluded_south_works() {
    assert_eq!(OxideBitboard::south_occluded_fill(OxideBitboard(0x204081020408000), OxideBitboard(0xfcfbf5efdbbf77ff)), OxideBitboard(0x2060c1c3878f0f0));
    assert_eq!(OxideBitboard::south_occluded_fill(OxideBitboard(0x8142242418000000), OxideBitboard(0x7ebd5bdbe7bfc7fb)), OxideBitboard(0x81c367677f3f0703));
    assert_eq!(OxideBitboard::south_occluded_fill(OxideBitboard(0x0), OxideBitboard(0x0)), OxideBitboard(0x0));
    assert_eq!(OxideBitboard::south_occluded_fill(OxideBitboard(0x0), OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x0));
}

#[test]
fn fill_occluded_east_works() {
    assert_eq!(OxideBitboard::east_occluded_fill(OxideBitboard(0x100020004000800), OxideBitboard(0xfcfbf5efdbbf77ff)), OxideBitboard(0x10006001c007800));
    assert_eq!(OxideBitboard::east_occluded_fill(OxideBitboard(0x102040418000000), OxideBitboard(0x7ebddbdbe7ffffff)), OxideBitboard(0x7f3e1c1cf8000000));
    assert_eq!(OxideBitboard::east_occluded_fill(OxideBitboard(0x0), OxideBitboard(0x0)), OxideBitboard(0x0));
    assert_eq!(OxideBitboard::east_occluded_fill(OxideBitboard(0x0), OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x0));
}

#[test]
fn fill_occluded_west_works() {
    assert_eq!(OxideBitboard::west_occluded_fill(OxideBitboard(0x204081020408000), OxideBitboard(0xfcfbf5efdbbf77ff)), OxideBitboard(0x2070c1f387ff000));
    assert_eq!(OxideBitboard::west_occluded_fill(OxideBitboard(0x8040202018000000), OxideBitboard(0x7ebddbdbe7ffffff)), OxideBitboard(0xfe7c38381f000000));
    assert_eq!(OxideBitboard::west_occluded_fill(OxideBitboard(0x0), OxideBitboard(0x0)), OxideBitboard(0x0));
    assert_eq!(OxideBitboard::west_occluded_fill(OxideBitboard(0x0), OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x0));
}

#[test]
fn fill_occluded_north_east_works() {
    assert_eq!(OxideBitboard::north_east_occluded_fill(OxideBitboard(0x100020004000800), OxideBitboard(0xfcfbf5efdbbf77ff)), OxideBitboard(0x4120120804100800));
    assert_eq!(OxideBitboard::north_east_occluded_fill(OxideBitboard(0x102040418000000), OxideBitboard(0x7ebddbdbe7ffffff)), OxideBitboard(0x351a0c1418000000));
    assert_eq!(OxideBitboard::north_east_occluded_fill(OxideBitboard(0x0), OxideBitboard(0x0)), OxideBitboard(0x0));
    assert_eq!(OxideBitboard::north_east_occluded_fill(OxideBitboard(0x0), OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x0));
}

#[test]
fn fill_occluded_north_west_works() {
    assert_eq!(OxideBitboard::north_west_occluded_fill(OxideBitboard(0x20408000), OxideBitboard(0xfefbfdffdbbf77ff)), OxideBitboard(0x81020408000));
    assert_eq!(OxideBitboard::north_west_occluded_fill(OxideBitboard(0x8040202018000000), OxideBitboard(0x7ebddbdbe7ffffff)), OxideBitboard(0xac58302818000000));
    assert_eq!(OxideBitboard::north_west_occluded_fill(OxideBitboard(0x0), OxideBitboard(0x0)), OxideBitboard(0x0));
    assert_eq!(OxideBitboard::north_west_occluded_fill(OxideBitboard(0x0), OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x0));
}

#[test]
fn fill_occluded_south_east_works() {
    assert_eq!(OxideBitboard::south_east_occluded_fill(OxideBitboard(0x284582000000000), OxideBitboard(0xfd7ba7dfffbbffff)), OxideBitboard(0x28458b060800000));
    assert_eq!(OxideBitboard::south_east_occluded_fill(OxideBitboard(0x102040418000000), OxideBitboard(0x7ebddbdbe7ffffff)), OxideBitboard(0x102040c183060c0));
    assert_eq!(OxideBitboard::south_east_occluded_fill(OxideBitboard(0x0), OxideBitboard(0x0)), OxideBitboard(0x0));
    assert_eq!(OxideBitboard::south_east_occluded_fill(OxideBitboard(0x0), OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x0));
}

#[test]
fn fill_occluded_south_west_works() {
    assert_eq!(OxideBitboard::south_west_occluded_fill(OxideBitboard(0x284582000000000), OxideBitboard(0xfd7ba7dfffb3ffff)), OxideBitboard(0x2855a2d16030100));
    assert_eq!(OxideBitboard::south_west_occluded_fill(OxideBitboard(0x8040202018000000), OxideBitboard(0x7fbfdfdfe3fffdff)), OxideBitboard(0x80402030180c0402));
    assert_eq!(OxideBitboard::south_west_occluded_fill(OxideBitboard(0x0), OxideBitboard(0x0)), OxideBitboard(0x0));
    assert_eq!(OxideBitboard::south_west_occluded_fill(OxideBitboard(0x0), OxideBitboard(0xffffffffffffffff)), OxideBitboard(0x0));
}

#[test]
fn file_fill_works() {
    assert_eq!(OxideBitboard::file_fill(OxideBitboard(0xff)), OxideBitboard(0xffffffffffffffff));
    assert_eq!(OxideBitboard::file_fill(OxideBitboard(0x55)), OxideBitboard(0x5555555555555555));
    assert_eq!(OxideBitboard::file_fill(OxideBitboard(0x4404004001041050)), OxideBitboard(0x5555555555555555));
    assert_eq!(OxideBitboard::file_fill(OxideBitboard(0x28200200000)), OxideBitboard(0xa2a2a2a2a2a2a2a2));
}

#[test]
fn rank_fill_works() {
    assert_eq!(OxideBitboard::rank_fill(OxideBitboard(0xff)), OxideBitboard(0xff));
    assert_eq!(OxideBitboard::rank_fill(OxideBitboard(0x1008)), OxideBitboard(0xffff));
    assert_eq!(OxideBitboard::rank_fill(OxideBitboard(0x4404004001041050)), OxideBitboard(0xffff00ffffffffff));
    assert_eq!(OxideBitboard::rank_fill(OxideBitboard(0x28200200000)), OxideBitboard(0xffff00ff0000));
}

// Ray attacks
#[test]
fn south_ray_attacks_works() {
    assert_eq!(OxideBitboard::south_ray_attacks(OxideBitboard(0x2000000000), OxideBitboard(0xffffffdfffffffff)), OxideBitboard(0x20202020));
    assert_eq!(OxideBitboard::south_ray_attacks(OxideBitboard(0x40020000000), OxideBitboard(0xfffffbffdfffffff)), OxideBitboard(0x404242424));
    assert_eq!(OxideBitboard::south_ray_attacks(OxideBitboard(0x2000400200000), OxideBitboard(0xfffdfdfbffdffbdf)), OxideBitboard(0x20004042420));
    assert_eq!(OxideBitboard::south_ray_attacks(OxideBitboard(0x2000400200000), OxideBitboard(0xfff9fffbfbdeffdf)), OxideBitboard(0x20206022222));
}

#[test]
fn north_ray_attacks_works() {
    assert_eq!(OxideBitboard::north_ray_attacks(OxideBitboard(0x400), OxideBitboard(0xfffffffffffffbff)), OxideBitboard(0x404040404040000));
    assert_eq!(OxideBitboard::north_ray_attacks(OxideBitboard(0x42000), OxideBitboard(0xfffffffffffbdfff)), OxideBitboard(0x2424242424200000));
    assert_eq!(OxideBitboard::north_ray_attacks(OxideBitboard(0x4200100), OxideBitboard(0xfbffdffffbdefeff)), OxideBitboard(0x404242420010000));
}

#[test]
fn east_ray_attacks_works() {
    assert_eq!(OxideBitboard::east_ray_attacks(OxideBitboard(0x10000000000000), OxideBitboard(0xffefffffffffffff)), OxideBitboard(0xe0000000000000));
    assert_eq!(OxideBitboard::east_ray_attacks(OxideBitboard(0x4000000080000), OxideBitboard(0xfffbfffffff7ffff)), OxideBitboard(0xf8000000f00000));
    assert_eq!(OxideBitboard::east_ray_attacks(OxideBitboard(0x10080000100000), OxideBitboard(0xffefe7ffff6fffff)), OxideBitboard(0xe0100000e00000));
    assert_eq!(OxideBitboard::east_ray_attacks(OxideBitboard(0x20000800000400), OxideBitboard(0xdfdfffb7ffffebfb)), OxideBitboard(0xc0007000001800));
}

#[test]
fn west_ray_attacks_works() {
    assert_eq!(OxideBitboard::west_ray_attacks(OxideBitboard(0x200000000000), OxideBitboard(0xffffdfffffffffff)), OxideBitboard(0x1f0000000000));
    assert_eq!(OxideBitboard::west_ray_attacks(OxideBitboard(0x8000000400000), OxideBitboard(0xfff7ffffffbfffff)), OxideBitboard(0x70000003f0000));
    assert_eq!(OxideBitboard::west_ray_attacks(OxideBitboard(0x800000000202000), OxideBitboard(0xf6ffffffffcfdbff)), OxideBitboard(0x700000000101c00));
    assert_eq!(OxideBitboard::west_ray_attacks(OxideBitboard(0x20000400200000), OxideBitboard(0xffdfeffaffd7ffff)), OxideBitboard(0x1f000300180000));
}

#[test]
fn north_west_ray_attacks_works() {
    assert_eq!(OxideBitboard::north_west_ray_attacks(OxideBitboard(0x2000000000), OxideBitboard(0xffffffdfffffffff)), OxideBitboard(0x408100000000000));
    assert_eq!(OxideBitboard::north_west_ray_attacks(OxideBitboard(0x2000080000), OxideBitboard(0xffffffdffff7ffff)), OxideBitboard(0x408110204000000));
    assert_eq!(OxideBitboard::north_west_ray_attacks(OxideBitboard(0x40100000001000), OxideBitboard(0xdfbfeffdffffefff)), OxideBitboard(0x2408000204080000));
    assert_eq!(OxideBitboard::north_west_ray_attacks(OxideBitboard(0x40100000001000), OxideBitboard(0xbfbfefdffbffe7ff)), OxideBitboard(0x2408000004080000));
}

#[test]
fn north_east_ray_attacks_works() {
    assert_eq!(OxideBitboard::north_east_ray_attacks(OxideBitboard(0x10000000), OxideBitboard(0xffffffffefffffff)), OxideBitboard(0x80402000000000));
    assert_eq!(OxideBitboard::north_east_ray_attacks(OxideBitboard(0x200200000), OxideBitboard(0xfffffffdffdfffff)), OxideBitboard(0x1008048040000000));
    assert_eq!(OxideBitboard::north_east_ray_attacks(OxideBitboard(0x40000042000), OxideBitboard(0xeffffbeffffbdfff)), OxideBitboard(0x1008001088400000));
    assert_eq!(OxideBitboard::north_east_ray_attacks(OxideBitboard(0x20008000010), OxideBitboard(0xffffd9fff7bfffcf)), OxideBitboard(0x804201000402000));
}

#[test]
fn south_west_ray_attacks_works() {
    assert_eq!(OxideBitboard::south_west_ray_attacks(OxideBitboard(0x1000000000), OxideBitboard(0xffffffefffffffff)), OxideBitboard(0x8040201));
    assert_eq!(OxideBitboard::south_west_ray_attacks(OxideBitboard(0x8000000100000), OxideBitboard(0xfff7ffffffefffff)), OxideBitboard(0x40201000804));
    assert_eq!(OxideBitboard::south_west_ray_attacks(OxideBitboard(0x4002000002000), OxideBitboard(0xfffbffdffff7dfef)), OxideBitboard(0x20110080010));
    assert_eq!(OxideBitboard::south_west_ray_attacks(OxideBitboard(0x4000088000000), OxideBitboard(0xfffbfbff777fddff)), OxideBitboard(0x20100442200));
}

#[test]
fn south_east_ray_attacks_works() {
    assert_eq!(OxideBitboard::south_east_ray_attacks(OxideBitboard(0x1000000000), OxideBitboard(0xffffffefffffffff)), OxideBitboard(0x20408000));
    assert_eq!(OxideBitboard::south_east_ray_attacks(OxideBitboard(0x100200000000), OxideBitboard(0xffffeffdffffffff)), OxideBitboard(0x2044881020));
    assert_eq!(OxideBitboard::south_east_ray_attacks(OxideBitboard(0x20040004000000), OxideBitboard(0xffdffb7ffbdfffff)), OxideBitboard(0x408810281020));
    assert_eq!(OxideBitboard::south_east_ray_attacks(OxideBitboard(0x220000020000), OxideBitboard(0xffffdddfffd9dfff)), OxideBitboard(0x4488102408));
}

#[test]
fn cardinal_ray_attacks_works() {
    assert_eq!(OxideBitboard::cardinal_ray_attacks(OxideBitboard(0x200000000000), OxideBitboard(0xffffdfffffffffff)), OxideBitboard(0x2020df2020202020));
    assert_eq!(OxideBitboard::cardinal_ray_attacks(OxideBitboard(0x200000040000), OxideBitboard(0xffffdffffffbffff)), OxideBitboard(0x2424df2424fb2424));
    assert_eq!(OxideBitboard::cardinal_ray_attacks(OxideBitboard(0x200204000000), OxideBitboard(0xffdddff5ebfffbff)), OxideBitboard(0x426df2d3b262622));
    assert_eq!(OxideBitboard::cardinal_ray_attacks(OxideBitboard(0x40000200100000), OxideBitboard(0xffb7bff5fecfffff)), OxideBitboard(0x52ba521d122f1212));
}

#[test]
fn diagonal_ray_attacks_works() {
    assert_eq!(OxideBitboard::diagonal_ray_attacks(OxideBitboard(0x80000000000), OxideBitboard(0xfffff7ffffffffff)), OxideBitboard(0x2214001422418000));
    assert_eq!(OxideBitboard::diagonal_ray_attacks(OxideBitboard(0x80000400000), OxideBitboard(0xfffff7ffffbfffff)), OxideBitboard(0x22140814a241a010));
    assert_eq!(OxideBitboard::diagonal_ray_attacks(OxideBitboard(0x42000002000), OxideBitboard(0xffeffbdfeeffdfdf)), OxideBitboard(0x158b520ed9d00050));
    assert_eq!(OxideBitboard::diagonal_ray_attacks(OxideBitboard(0x10000200000080), OxideBitboard(0xdfeff3f9fe57ff7f)), OxideBitboard(0x28002d4085284000));
}

#[test]
fn bitboard_from_square_iterator_works() {
    let squares = &[OxideSquare::A2, OxideSquare::B3, OxideSquare::H4, OxideSquare::E5];
    assert_eq!(squares.iter().copied().collect::<OxideBitboard>(), OxideBitboard(0x1080020100u64));
}