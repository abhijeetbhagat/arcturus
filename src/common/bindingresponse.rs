use std::net::Ipv4Addr;

pub struct BindingResponse {
    pub xor_port: u16,
    pub xor_mapped_address: Ipv4Addr,
}

impl BindingResponse {
    pub fn new(xor_port: u16, xor_mapped_address: Ipv4Addr) -> Self {
        BindingResponse {
            xor_port: xor_port,
            xor_mapped_address: xor_mapped_address,
        }
    }

    pub fn from_raw(data: &[u8]) -> Self {
        unimplemented!();
    }

    pub fn from_v4(port: u16, ip: u32) -> BindingResponse {
        unimplemented!();
    }
    pub fn from_v6(port: u16, ip: u128) -> BindingResponse {
        unimplemented!();
    }

    pub fn to_raw(&self) -> &[u8] {
        unimplemented!();
    }
}
