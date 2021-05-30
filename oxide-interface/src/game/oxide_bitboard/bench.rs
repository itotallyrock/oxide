
#![allow(soft_unstable)]
use super::*;

// External test for benchmarking
extern crate test;
use test::Bencher;

#[bench]
fn diagonal_ray_attacks_bench(bencher: &mut Bencher) {
    let bishops = test::black_box(OxideBitboard(0x8000000000000000));
    let empty = OxideBitboard(0x7fffffffffffffff);
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::diagonal_ray_attacks(bishops, empty));
}

#[bench]
fn cardinal_ray_attacks_bench(bencher: &mut Bencher) {
    let rooks = test::black_box(OxideBitboard(0x100000));
    let empty = OxideBitboard(0xffffeffffb6fdfef);
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::cardinal_ray_attacks(rooks, empty));
}

#[bench]
fn cardinal_fill_bench(bencher: &mut Bencher) {
    let rooks = test::black_box(OxideBitboard(0x100000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::cardinal_fill(rooks));
}

#[bench]
fn diagonal_fill_bench(bencher: &mut Bencher) {
    let bishops = test::black_box(OxideBitboard(0x8000000000000000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::diagonal_fill(bishops));
}

#[bench]
fn north_fill_bench(bencher: &mut Bencher) {
    let rooks = test::black_box(OxideBitboard(0x100000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::north_fill(rooks));
}

#[bench]
fn south_fill_bench(bencher: &mut Bencher) {
    let rooks = test::black_box(OxideBitboard(0x100000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::south_fill(rooks));
}

#[bench]
fn east_fill_bench(bencher: &mut Bencher) {
    let rooks = test::black_box(OxideBitboard(0x100000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::east_fill(rooks));
}

#[bench]
fn west_fill_bench(bencher: &mut Bencher) {
    let rooks = test::black_box(OxideBitboard(0x100000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::west_fill(rooks));
}

#[bench]
fn north_east_fill_bench(bencher: &mut Bencher) {
    let bishops = test::black_box(OxideBitboard(0x8000000000000000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::north_east_fill(bishops));
}

#[bench]
fn north_west_fill_bench(bencher: &mut Bencher) {
    let bishops = test::black_box(OxideBitboard(0x8000000000000000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::north_west_fill(bishops));
}

#[bench]
fn south_east_fill_bench(bencher: &mut Bencher) {
    let bishops = test::black_box(OxideBitboard(0x8000000000000000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::south_east_fill(bishops));
}

#[bench]
fn south_west_fill_bench(bencher: &mut Bencher) {
    let bishops = test::black_box(OxideBitboard(0x8000000000000000));
    bencher.iter(|| <OxideBitboard as BoardMask<OxidePosition>>::south_west_fill(bishops));
}