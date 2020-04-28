#[derive(Clone)]
pub enum AttributeType {
    MappedAddress = 0x0001,
    Username = 0x0006,
    MessageIntegrity = 0x0008,
    ErrorCode = 0x0009,
    UnknownAttributes = 0x000A,
    Realm = 0x0014,
    Nonce = 0x0015,
    XorMappedAddress = 0x0020,

    //TODO abhi: unsupported at the moment:
    Software = 0x8022,
    AlternateServer = 0x8023,
    Fingerprint = 0x8028,
}
