extern crate either;
use crate::common::attributetype::AttributeType;
use crate::utils::bitutils;
use either::Either;

pub struct XorMappedAddress {
    pub family: u8,
    pub port: u16,
    pub address: Either<u32, u128>,
    pub attribute_type: AttributeType,
}

impl XorMappedAddress {
    pub fn new(family: u8, port: u16, address: Either<u32, u128>) -> Self {
        XorMappedAddress {
            family: family,
            port: port,
            address: address,
            attribute_type: AttributeType::XorMappedAddress,
        }
    }

    pub fn as_raw(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(12);
        vec.extend_from_slice(&(self.attribute_type.clone() as u16).to_be_bytes());
        if self.address.is_left() {
            //ipv4
            vec.extend_from_slice(&(4 + 4 as u16).to_be_bytes());
            vec.extend_from_slice(&(self.family as u16).to_be_bytes());
            vec.extend_from_slice(&self.port.to_be_bytes());
            vec.extend_from_slice(&self.address.left().unwrap().to_be_bytes());
        } else {
            //ipv6
            vec.extend_from_slice(&(4 + 16 as u16).to_be_bytes());
            vec.extend_from_slice(&(self.family as u16).to_be_bytes());
            vec.extend_from_slice(&self.port.to_be_bytes());
            vec.extend_from_slice(&self.address.right().unwrap().to_be_bytes());
        }
        vec
    }

    pub fn from_raw(data: &[u8]) -> Option<Self> {
        let mut iter = data.iter();
        let _ = bitutils::read_u8(&mut iter);
        let family = bitutils::read_u8(&mut iter);
        println!("family is {}", family);
        let port = bitutils::read_u16(&mut iter);
        let address = if family == 1 {
            either::Left(bitutils::read_u32(&mut iter))
        } else {
            either::Right(bitutils::read_u128(&mut iter))
        };
        Some(XorMappedAddress::new(family, port, address))
    }
}
