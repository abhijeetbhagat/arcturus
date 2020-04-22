use std::net::IPv4Addr;

struct  StunClient{
    ip: IPv4Addr,
    port: u16
}

impl StunClient{
    pub fn new(ip: IPv4Addr, port: u16) -> Self {
        StunClient{
            ip: ip,
            port: port
        }
    }

    pub fn connect(self){

    }

    pub fn get_reflexive_address(self){

    }
}