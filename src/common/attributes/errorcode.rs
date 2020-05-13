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
    class_number: u8,
    reason_phrase: String,
}

impl ErrorCode {
    pub fn new(class_number, reason_phrase: String) -> Self {
        ErrorCode{
            class_number: class_number,
            reason_phrase: reason_phrase
        }
    }

    pub fn as_raw(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(12);
        vec.extend_from_slice((&self.class_number as u32).to_be_bytes());
        vec.extend_from_slice(&self.reason_phrase.as_bytes());
        vec
    }

    pub fn from_raw(data: &[u8]) -> Option<Self> {
        let mut iter = data.iter();
        let class_number = bitutils::read_u32(&mut iter);
    }

    fn get_string(error_code: u8) -> &str{
        match error_code {
            300 => "Try Alternate",
            400 => "Bad Request",
            401 => "Unauthorized",
            420 => "Unknown Attribute",
            438 => "Stale Nonce",
            500 => "Server Error"
        }
    }
}
