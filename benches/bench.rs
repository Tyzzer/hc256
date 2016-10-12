#![feature(test)]

extern crate test;
extern crate hc256;

use test::Bencher;
use hc256::HC256;


#[bench]
fn hc256_bench(b: &mut Bencher) {
    let key = [0; 32];
    let iv = [0; 32];
    let input = [0; 1024];
    let mut output = [0; 1024];
    let mut cipher = HC256::new(&key, &iv);

    b.bytes = input.len() as u64;
    b.iter(|| cipher.process(&input, &mut output))
}

#[bench]
fn hc256_bench_once(b: &mut Bencher) {
    let key = [0; 32];
    let iv = [0; 32];
    let input = [0; 1024];
    let mut output = [0; 1024];

    b.bytes = input.len() as u64;
    b.iter(|| HC256::new(&key, &iv).process(&input, &mut output))
}
