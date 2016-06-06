use byteorder::{ LittleEndian, ByteOrder };

pub type Endian = LittleEndian;


#[inline]
pub fn u8_to_u32(input: &[u8], output: &mut [u32]) {
    for (i, b) in output.iter_mut().enumerate() {
        *b = Endian::read_u32(&input[(i * 4)..((i + 1) * 4)]);
    }
}

#[inline]
pub fn u32_to_u8(input: u32, output: &mut [u8]) {
    Endian::write_u32(output, input);
}
