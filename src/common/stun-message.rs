pub enum Class{
    Request,
    Indication,
    Success,
    Error
}

pub struct Type(Class, Method);

pub enum Method{
    //TODO abhi: there are other methods too
    Binding
}

struct StunMessageHeader{
    type: Type,
    length: u16,
    magic: u32,
    txn_id: u128 //actually 96 bits but we dont have that type
}

impl StunMessageHeader{
    pub fn new(type: Type, length: u16, txn_id: u128) -> Self {
        StunMessage{
            type: type,
            length: length,
            magic: 0x2112A442,
            txn_id: txn_id
        }
    }

    pub fn get_class(self){
        self.type.0
    }

    pub fn get_method(self){
        self.type.1
    }
}

struct StunMessage{
    header: StunMessageHeader,
    payload: [u8]
}
