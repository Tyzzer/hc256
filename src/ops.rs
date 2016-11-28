#[derive(Copy)]
pub struct Hc256Rng {
    p: [u32; 1024],
    q: [u32; 1024],
    c: usize
}

impl Clone for Hc256Rng { fn clone(&self) -> Hc256Rng { *self } }

impl Hc256Rng {
    pub fn init(key: &[u32; 8], iv: &[u32; 8]) -> Hc256Rng {
        let mut w = [0; 2560];
        w[..8].clone_from_slice(key);
        w[8..16].clone_from_slice(iv);
        Self::with_w(&mut w)
    }

    pub fn with_w(w: &mut [u32; 2560]) -> Hc256Rng {
        let mut hc256 = Hc256Rng {
            p: [0; 1024],
            q: [0; 1024],
            c: 0
        };

        for i in 16..2560 {
            w[i] = f2(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(f1(w[i - 15]))
                .wrapping_add(w[i - 16])
                .wrapping_add(i as u32);
        }
        hc256.p.clone_from_slice(&w[512..1536]);
        hc256.q.clone_from_slice(&w[1536..]);

        for _ in 0..4096 {
            hc256.gen();
        }

        hc256
    }

    pub fn gen(&mut self) -> u32 {
        let i = self.c & 0x3ff;
        let i3 = i.wrapping_sub(3) & 0x3ff;
        let i10 = i.wrapping_sub(10) & 0x3ff;
        let i12 = i.wrapping_sub(12) & 0x3ff;
        let i1023 = i.wrapping_sub(1023) & 0x3ff;

        let output = if self.c < 1024 {
            self.p[i] = self.p[i]
                .wrapping_add(self.p[i10])
                .wrapping_add(self.p[i3].rotate_right(10) ^ self.p[i1023].rotate_right(23))
                .wrapping_add(self.q[(self.p[i3] ^ self.p[i1023]) as usize & 0x3ff]);
            h(&self.q, self.p[i12]) ^ self.p[i]
        } else {
            self.q[i] = self.q[i]
                .wrapping_add(self.q[i10])
                .wrapping_add(self.q[i3].rotate_right(10) ^ self.q[i1023].rotate_right(23))
                .wrapping_add(self.p[(self.q[i3] ^ self.q[i1023]) as usize & 0x3ff]);
            h(&self.p, self.q[i12]) ^ self.q[i]
        };

        self.c = (self.c + 1) & 0x7ff;
        output
    }
}


#[inline]
fn h(q: &[u32], u: u32) -> u32 {
    q[(u & 0xff) as usize]
        .wrapping_add(q[256 + (u >> 8 & 0xff) as usize])
        .wrapping_add(q[512 + (u >> 16 & 0xff) as usize])
        .wrapping_add(q[768 + (u >> 24 & 0xff) as usize])
}

#[inline]
fn f1(x: u32) -> u32 {
    x.rotate_right(7)
        ^ x.rotate_right(18)
        ^ x.wrapping_shr(3)
}

#[inline]
fn f2(x: u32) -> u32 {
    x.rotate_right(17)
        ^ x.rotate_right(19)
        ^ x.wrapping_shr(10)
}

#[test]
fn test() {
    assert_eq!(
        Hc256Rng::init(&[0; 8], &[0; 8]).gen(),
        2240350043
    );
}
