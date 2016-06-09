#![no_std]

extern crate byteorder;

mod util;
mod ops;

use byteorder::ByteOrder;
pub use ops::Hc256Rng;


/// ```
/// use hc256::HC256;
/// let mut output = [0; 32];
///
/// HC256::new(&[0; 32], &[0; 32]).process(&[0; 32], &mut output);
/// assert_eq!(
///     output,
///     [
///         0x5b, 0x07, 0x89, 0x85, 0xd8, 0xf6, 0xf3, 0x0d,
///         0x42, 0xc5, 0xc0, 0x2f, 0xa6, 0xb6, 0x79, 0x51,
///         0x53, 0xf0, 0x65, 0x34, 0x80, 0x1f, 0x89, 0xf2,
///         0x4e, 0x74, 0x24, 0x8b, 0x72, 0x0b, 0x48, 0x18
///     ]
/// );
///
/// let mut iv = [0; 32];
/// iv[0] = 1;
/// HC256::new(&[0; 32], &iv).process(&[0; 32], &mut output);
/// assert_eq!(
///     output,
///     [
///         0xaf, 0xe2, 0xa2, 0xbf, 0x4f, 0x17, 0xce, 0xe9,
///         0xfe, 0xc2, 0x05, 0x8b, 0xd1, 0xb1, 0x8b, 0xb1,
///         0x5f, 0xc0, 0x42, 0xee, 0x71, 0x2b, 0x31, 0x01,
///         0xdd, 0x50, 0x1f, 0xc6, 0x0b, 0x08, 0x2a, 0x50
///     ]
/// );
///
/// let mut key = [0; 32];
/// key[0] = 0x55;
/// HC256::new(&key, &[0; 32]).process(&[0; 32], &mut output);
/// assert_eq!(
///     output,
///     [
///         0x1c, 0x40, 0x4a, 0xfe, 0x4f, 0xe2, 0x5f, 0xed,
///         0x95, 0x8f, 0x9a, 0xd1, 0xae, 0x36, 0xc0, 0x6f,
///         0x88, 0xa6, 0x5a, 0x3c, 0xc0, 0xab, 0xe2, 0x23,
///         0xae, 0xb3, 0x90, 0x2f, 0x42, 0x0e, 0xd3, 0xa8
///     ]
/// );
/// ```
#[derive(Copy)]
pub struct HC256 {
    inner: Hc256Rng,
    buff: [u8; 4],
    index: usize
}

impl Clone for HC256 { fn clone(&self) -> HC256 { *self } }

impl HC256 {
    pub fn new(key: &[u8], iv: &[u8]) -> HC256 {
        let mut ukey = [0; 8];
        let mut uiv = [0; 8];
        util::u8_to_u32(key, &mut ukey);
        util::u8_to_u32(iv, &mut uiv);

        HC256 {
            inner: Hc256Rng::init(&ukey, &uiv),
            buff: [0; 4],
            index: 0
        }
    }

    pub fn process(&mut self, input: &[u8], output: &mut [u8]) {
        let mut pos = 0;

        if input.len() >= self.index {
            pos += self.index;
            for (i, b) in self.take(pos).enumerate() {
                output[i] = input[i] ^ b;
            }
        }

        while pos + 4 <= input.len() {
            let end = pos + 4;

            util::u32_to_u8(
                util::Endian::read_u32(&input[pos..end]) ^ self.inner.gen(),
                &mut output[pos..end]
            );

            pos = end;
        }

        for b in self.take(input.len() - pos) {
            output[pos] = input[pos] ^ b;
            pos += 1;
        }
    }
}

impl Iterator for HC256 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            util::u32_to_u8(self.inner.gen(), &mut self.buff);
        }
        let output = self.buff[self.index];
        self.index = (self.index + 1) % self.buff.len();
        Some(output)
    }
}
