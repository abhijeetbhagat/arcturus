use crate::common::attributetype::AttributeType;
use crate::utils::bitutils;

/* from https://tools.ietf.org/html/rfc5389#section-15.6

       0                   1                   2                   3
       0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |           Reserved, should be 0         |Class|     Number    |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |      Reason Phrase (variable)                                ..
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

                      Figure 7: ERROR-CODE Attribute
*/
pub struct ErrorCode {
    class_number: u16,
    reason_phrase: String,
    pub attribute_type: AttributeType,
}

impl ErrorCode {
    pub fn new(class_number: u16, reason_phrase: impl Into<String>) -> Self {
        ErrorCode {
            class_number,
            reason_phrase: reason_phrase.into(),
            attribute_type: AttributeType::ErrorCode,
        }
    }

    fn get_string(class_number: u16) -> &'static str {
        match class_number {
            0x300 => "Try Alternate",
            0x400 => "Bad Request",
            0x401 => "Unauthorized",
            0x420 => "Unknown Attribute",
            0x438 => "Stale Nonce",
            0x500 => "Server Error",
            _ => "Unknown Error",
        }
    }
}

impl From<&ErrorCode> for Vec<u8> {
    fn from(err_code: &ErrorCode) -> Vec<u8> {
        let mut vec = Vec::with_capacity(12);
        vec.extend_from_slice(&(err_code.attribute_type.clone() as u16).to_be_bytes());
        let reason_phrase = ErrorCode::get_string(err_code.class_number);
        vec.extend_from_slice(&(4 + reason_phrase.len() as u16).to_be_bytes());
        vec.extend_from_slice(&(err_code.class_number as u32).to_be_bytes());
        vec.extend_from_slice(&err_code.reason_phrase.as_bytes());
        vec
    }
}

impl From<&[u8]> for ErrorCode {
    fn from(data: &[u8]) -> ErrorCode {
        let mut iter = data.iter();
        let class_number = bitutils::read_u32(&mut iter) as u16;
        let reason_phrase = ErrorCode::get_string(class_number);
        //We do not need to read the reason phrase from the packet
        //since we already know it via get_string above
        //bitutils::read_nbytes(&mut iter, reason_phrase.len())
        ErrorCode::new(class_number, reason_phrase)
    }
}
