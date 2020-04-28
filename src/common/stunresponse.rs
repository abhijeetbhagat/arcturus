use std::net::Ipv4Addr;

pub struct StunResponse {}

impl StunResponse {
    pub fn new(xor_port: u16, xor_mapped_address: Ipv4Addr) -> Self {
        StunResponse {
            xor_port: xor_port,
            xor_mapped_address: xor_mapped_address,
        }
    }

    pub fn from_raw(data: &[u8]) -> Self {
        unimplemented!();
    }

    pub fn from_v4(port: u16, ip: u32) -> StunResponse {
        unimplemented!();
    }
    pub fn from_v6(port: u16, ip: u128) -> StunResponse {
        unimplemented!();
    }

    pub fn to_raw(&self) -> &[u8] {
        unimplemented!();
    }
}
