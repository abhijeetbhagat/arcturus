use std::net::IPv4Addr;

trait Obfuscate {
    fn obfuscate(&self, magic: u32) -> [u8];
}

impl Obfuscate for IPv4Addr { //32 bits
/* 
   https://tools.ietf.org/html/rfc5389#section-15.2
   X-Port is computed by taking the mapped port in host byte order,
   XOR'ing it with the most significant 16 bits of the magic cookie, and
   then the converting the result to network byte order.  If the IP
   address family is IPv4, X-Address is computed by taking the mapped IP
   address in host byte order, XOR'ing it with the magic cookie, and
   converting the result to network byte order.  If the IP address
   family is IPv6, X-Address is computed by taking the mapped IP address
   in host byte order, XOR'ing it with the concatenation of the magic
   cookie and the 96-bit transaction ID, and converting the result to
   network byte order.
*/
    impl obfuscate(&self, magic: u32) -> [u8] {
        u32::from(self) ^ magic
    }
}

impl Obfuscate for IPv6Addr { //128 bits
    impl obfuscate(&self, magic: u32) -> [u8] {
    }
}