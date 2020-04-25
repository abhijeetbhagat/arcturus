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
}

pub struct Type(pub Class, pub Method);

pub enum Method {
    //TODO abhi: there are other methods too
    Binding,
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

    pub fn get_class(self) -> Class {
        self.msg_type.0
    }

    pub fn get_method(self) -> Method {
        self.msg_type.1
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

    pub fn as_raw(&self) -> &[u8] {
        //TODO abhi: convert this message to raw
        unimplemented!();
    }

    pub fn from_raw(data: &[u8]) -> Option<Self> {
        let msg_type = bitutils::read_u16(data);
        unimplemented!();
    }
}

enum Attribute {
    MappedAddress,
    Username,
    MessageIntegrity,
    ErrorCode,
    UnknownAttributes,
    Realm,
    Nonce,
    XorMappedAddress,
    //TODO abhi: the following are unsupported
    Software,
    AlternateServer,
    Fingerprint,
}
