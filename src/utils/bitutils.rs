use std::cell::RefCell;
use std::convert::TryInto;
use std::slice::Iter;

/// A unidirectional stream reader
struct BitStream<'a> {
    data: &'a [u8],
    current_pos: RefCell<usize>,
}

impl<'a> BitStream<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            current_pos: RefCell::new(0),
        }
    }

    pub fn read_u8(&self) -> Option<u8> {
        let result = self.data.get(*self.current_pos.borrow())?;
        *self.current_pos.borrow_mut() += 1;
        //*pos += 1;
        Some(*result)
    }

    pub fn read_u16(&self) -> Option<u16> {
        //println!("{}"*self.current_pos.borrow())
        let result = u16::from_be_bytes({
            let pos = self.get_pos();
            self.data.get(pos..pos + 2)?.try_into().unwrap()
        });
        *self.current_pos.borrow_mut() += 2;
        //*pos += 1;
        Some(result)
    }

    pub fn read_u32(&self) -> Option<u32> {
        //println!("{}"*self.current_pos.borrow())
        let result = u32::from_be_bytes({
            let pos = self.get_pos();
            self.data.get(pos..pos + 4)?.try_into().unwrap()
        });
        *self.current_pos.borrow_mut() += 4;
        //*pos += 1;
        Some(result)
    }

    pub fn read_u128(&self) -> Option<u128> {
        //println!("{}"*self.current_pos.borrow())
        let result = u128::from_be_bytes({
            let pos = self.get_pos();
            self.data.get(pos..pos + 16)?.try_into().unwrap()
        });
        *self.current_pos.borrow_mut() += 16;
        //*pos += 1;
        Some(result)
    }

    pub fn read_nbytes(&self, n: u32) -> Option<&'a [u8]> {
        //println!("{}"*self.current_pos.borrow())
        let result = {
            let pos = self.get_pos();
            self.data.get(pos..pos + n as usize)?
        };
        *self.current_pos.borrow_mut() += n as usize;
        //*pos += 1;
        Some(result)
    }

    pub fn get_pos(&self) -> usize {
        *self.current_pos.borrow()
    }
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
        assert!(0x5_060_708 == read_u32(&mut iterator));
        assert!([9, 10] == read_nbytes(&mut iterator, 2));
        assert!([1] == read_nbytes(&mut iterator, 1));
        assert!(
            0x0202_0202_0202_0202_0202_0202_0202_0202 == to_txn_id(read_nbytes(&mut iterator, 16))
        );
        assert!(0x3 == read_u8(&mut iterator));
    }

    #[test]
    fn test_bitstream_reader() {
        let reader = BitStream::new(&[
            0, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            3,
        ]);
        assert_eq!(reader.read_u8(), Some(0));
        assert_eq!(reader.read_u8(), Some(5));
        assert_eq!(reader.get_pos(), 2);
        assert_eq!(reader.read_u16(), Some(0x102));
        assert_eq!(reader.read_u16(), Some(0x304));
        assert_eq!(reader.read_u32(), Some(0x5_060_708));
        assert_eq!(Some(&[9, 10][..]), reader.read_nbytes(2));
    }
}
