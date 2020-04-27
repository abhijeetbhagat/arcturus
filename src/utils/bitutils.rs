use std::slice::Iter;

struct BitStream<'a> {
    data: &'a [u8],
    current_pos: usize,
}

pub fn read_u8(slice: &mut Iter<u8>) -> u8 {
    *slice.next().unwrap()
}

pub fn read_u16(slice: &mut Iter<u8>) -> u16 {
    let first = *slice.next().unwrap();
    let second = *slice.next().unwrap();
    (0xffff & first as u16) << 8 | second as u16
}

pub fn read_u32(slice: &mut Iter<u8>) -> u32 {
    let first = *slice.next().unwrap();
    let second = *slice.next().unwrap();
    let third = *slice.next().unwrap();
    let fourth = *slice.next().unwrap();
    (0xffffffff & first as u32) << 24 | (second as u32) << 16 | (third as u32) << 8 | fourth as u32
}

pub fn read_nbytes<'a>(slice: &'a mut Iter<u8>, n: usize) -> &'a [u8] {
    slice.as_slice().get(0..n).unwrap()
}
/*pub fn read_n(slice: &[u8], n: u32) -> &[u8] {
    slice.get()
}*/
