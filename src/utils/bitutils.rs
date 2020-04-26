use std::slice::Iter;

struct BitStream<'a>{
    data: &'a[u8],
    current_pos: 0usize
}

pub fn read_u8(slice: &[u8]) -> u8 {
    slice[0]
}

pub fn read_u16(slice: &[u8]) -> u16 {
    (0xffff & slice[0] as u16) << 8 | slice[1] as u16
}

/*pub fn read_n(slice: &[u8], n: u32) -> &[u8] {
    slice.get()
}*/
