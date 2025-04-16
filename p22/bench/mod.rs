#![feature(test)]

extern crate test;
use test::Bencher;
use p22::calc::{fibonacci_loop, fibonacci_rec};

#[bench]
fn bench_fibonacci_20_loop(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(fibonacci_loop(20));
    });
}

fn bench_fibonacci_30_loop(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(fibonacci_loop(30));
    });
}

#[bench]
fn bench_fibonacci_20_rec(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(fibonacci_rec(20));
    });
}

#[bench]
fn bench_fibonacci_30_rec(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(fibonacci_rec(30));
    });
}

