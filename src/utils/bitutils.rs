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

pub fn read_u128(slice: &mut Iter<u8>) -> u128 {
    let result = u128::from_be_bytes(slice.as_slice().get(0..16).unwrap().try_into().unwrap());
    for _ in 0..16 {
        slice.next();
    }
    result
}

pub fn read_nbytes<'a>(slice: &'a mut Iter<u8>, n: usize) -> &'a [u8] {
    let window = slice.as_slice().get(0..n).unwrap();
    for _ in 0..n {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_iter_movement() {
        let mut iterator = [
            0, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            3,
        ]
        .iter();
        assert!(0 == read_u8(&mut iterator));
        assert!(5 == read_u8(&mut iterator));
        assert!(0x102 == read_u16(&mut iterator));
        assert!(0x304 == read_u16(&mut iterator));
        assert!(0x5060708 == read_u32(&mut iterator));
        assert!(&[9, 10] == read_nbytes(&mut iterator, 2));
        assert!(&[1] == read_nbytes(&mut iterator, 1));
        assert!(0x2020202020202020202020202020202 == to_txn_id(read_nbytes(&mut iterator, 16)));
        assert!(0x3 == read_u8(&mut iterator));
    }
}
