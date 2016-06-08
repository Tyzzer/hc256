#![feature(test)]

extern crate test;
extern crate hc256;

use test::Bencher;
use hc256::HC256;


#[bench]
fn hc128_bench(b: &mut Bencher) {
    let key = [0; 32];
    let iv = [0; 32];
    let input = [0; 64];
    let mut output = [0; 64];
    let mut cipher = HC256::new(&key, &iv);

    b.bytes = input.len() as u64;
    b.iter(|| cipher.process(&input, &mut output))
}
