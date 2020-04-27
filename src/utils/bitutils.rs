use std::convert::TryInto;
use std::slice::Iter;

struct BitStream<'a> {
    data: &'a [u8],
    current_pos: usize,
}

pub fn read_u8(slice: &mut Iter<u8>) -> u8 {
    *slice.next().unwrap()
}

pub fn read_u16(slice: &mut Iter<u8>) -> u16 {
    let result = u16::from_be_bytes(slice.as_slice().get(0..2).unwrap().try_into().unwrap());
    slice.next();
    slice.next();
    result
}

pub fn read_u32(slice: &mut Iter<u8>) -> u32 {
    let result = u32::from_be_bytes(slice.as_slice().get(0..4).unwrap().try_into().unwrap());
    slice.next();
    slice.next();
    slice.next();
    slice.next();
    result
}

pub fn read_nbytes<'a>(slice: &'a mut Iter<u8>, n: usize) -> &'a [u8] {
    let window = slice.as_slice().get(0..n).unwrap();
    for i in 0..n {
        slice.next();
    }
    window
}

pub fn to_txn_id(bytes: &[u8]) -> u128 {
    u128::from_be_bytes(bytes.try_into().unwrap())
}
/*pub fn read_n(slice: &[u8], n: u32) -> &[u8] {
    slice.get()
}*/
