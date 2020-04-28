use crate::utils::bitutils;
use crate::utils::miscutils;

/*
       0                   1                   2                   3
       0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |0 0|     STUN Message Type     |         Message Length        |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |                         Magic Cookie                          |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |                                                               |
      |                     Transaction ID (96 bits)                  |
      |                                                               |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                        Format of STUN Message Header

                        0                 1
                        2  3  4 5 6 7 8 9 0 1 2 3 4 5

                       +--+--+-+-+-+-+-+-+-+-+-+-+-+-+
                       |M |M |M|M|M|C|M|M|M|C|M|M|M|M|
                       |11|10|9|8|7|1|6|5|4|0|3|2|1|0|
                       +--+--+-+-+-+-+-+-+-+-+-+-+-+-+

                      Format of STUN Message Type Field

       0                   1                   2                   3
       0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |         Type                  |            Length             |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |                         Value (variable)                ....
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                      Format of STUN Attributes

   STUN Attributes:
   Comprehension-required range (0x0000-0x7FFF):
     0x0000: (Reserved)
     0x0001: MAPPED-ADDRESS
     0x0002: (Reserved; was RESPONSE-ADDRESS)
     0x0003: (Reserved; was CHANGE-ADDRESS)
     0x0004: (Reserved; was SOURCE-ADDRESS)
     0x0005: (Reserved; was CHANGED-ADDRESS)
     0x0006: USERNAME
     0x0007: (Reserved; was PASSWORD)
     0x0008: MESSAGE-INTEGRITY
     0x0009: ERROR-CODE
     0x000A: UNKNOWN-ATTRIBUTES
     0x000B: (Reserved; was REFLECTED-FROM)
     0x0014: REALM
     0x0015: NONCE
     0x0020: XOR-MAPPED-ADDRESS

   Comprehension-optional range (0x8000-0xFFFF)
     0x8022: SOFTWARE
     0x8023: ALTERNATE-SERVER
     0x8028: FINGERPRINT
*/

pub enum Class {
    Request,
    Indication,
    Success,
    Error,
    Unknown,
}

pub struct Type(pub Class, pub Method);

#[derive(Clone)]
pub enum Method {
    //TODO abhi: there are other methods too
    BindingRequest = 0x0001,
    BindingResponse = 0x0101,
    Unknown = 0xffff,
}

pub struct StunMessageHeader {
    pub msg_type: Type,
    pub length: u16,
    pub magic: u32,
    pub txn_id: u128, //actually 96 bits but we dont have that type
}

impl StunMessageHeader {
    pub fn new(msg_type: Type, length: u16, txn_id: u128) -> Self {
        StunMessageHeader {
            msg_type: msg_type,
            length: length,
            magic: miscutils::MAGIC_COOKIE,
            txn_id: txn_id & 0xffffffffffffffffffffffff, //take 96 bits
        }
    }

    pub fn get_class(&self) -> &Class {
        &self.msg_type.0
    }

    pub fn get_method(&self) -> Method {
        self.msg_type.1.clone()
    }
}

pub struct StunMessage {
    pub header: StunMessageHeader,
    pub payload: Option<Vec<u8>>,
}

impl StunMessage {
    pub fn new(header: StunMessageHeader, payload: Option<Vec<u8>>) -> Self {
        StunMessage {
            header: header,
            payload: payload,
        }
    }

    pub fn as_raw(&self) -> Vec<u8> {
        //TODO abhi: calc the capacity
        let mut buf = Vec::with_capacity(20);
        buf.extend_from_slice(&(self.header.get_method() as u16).to_be_bytes());
        buf.extend_from_slice(&self.header.length.to_be_bytes());
        buf.extend_from_slice(&miscutils::MAGIC_COOKIE.to_be_bytes());
        buf.extend_from_slice(&self.header.txn_id.to_be_bytes().get(4..).unwrap());
        if self.payload.is_some() {
            buf.extend_from_slice(&self.payload.as_ref().unwrap());
        }
        buf
    }

    pub fn from_raw(data: &[u8]) -> Option<Self> {
        let mut iter = data.iter();
        let msg_type = bitutils::read_u16(&mut iter);
        let msg_length = bitutils::read_u16(&mut iter);
        let _magic_cookie = bitutils::read_u32(&mut iter);
        let txn_id =
            bitutils::to_txn_id(&[&[0u8, 0, 0, 0], bitutils::read_nbytes(&mut iter, 12)].concat());
        let (class, method) = StunMessage::get_class_and_method(msg_type);
        let header = StunMessageHeader::new(Type(class, method), msg_length, txn_id);
        let payload = if msg_length > 0 {
            Some(Box::new(bitutils::read_nbytes(&mut iter, msg_length as usize)).to_vec())
        } else {
            None
        };

        Some(StunMessage::new(header, payload))
    }

    fn get_class_and_method(word: u16) -> (Class, Method) {
        let class = match word & 0x110 {
            0x0 => Class::Request,
            0x10 => Class::Indication,
            0x100 => Class::Success,
            0x110 => Class::Error,
            _ => Class::Unknown,
        };

        let method = match word & 0xffff {
            0x1 => Method::BindingRequest,
            0x101 => Method::BindingResponse,
            _ => Method::Unknown,
        };

        (class, method)
    }
}

#[cfg(test)]
mod test {
    use super::{Class, Method, StunMessage, StunMessageHeader, Type};
    use crate::utils::miscutils;
    #[test]
    fn test_msg_to_raw_conversion() {
        let txn_id = miscutils::gen_txn_id();
        let header = StunMessageHeader::new(
            Type(Class::Request, Method::BindingRequest),
            20 + 0, //header is 20 bytes + payload length
            txn_id,
        );
        let msg = StunMessage::new(header, None);
        let raw_data = msg.as_raw();
        assert!(raw_data[0] == 0 && raw_data[1] == 1);
    }
}
