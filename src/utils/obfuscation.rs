use async_std::net::{SocketAddrV4, SocketAddrV6};

/*
    https://tools.ietf.org/html/rfc5389#section-15.2
    X-Port is computed by taking the mapped port in host byte order,
    XOR'ing it with the most significant 16 bits of the magic cookie, and
    then the converting the result to network byte order.  If the IP
    address family is IPv4, X-Address is computed by taking the mapped IP
    address in host byte order, XOR'ing it with the magic cookie, and
    converting the result to network byte order.
*/

pub fn obfuscate_v4(sock_addr: &SocketAddrV4, magic: u32) -> (u16, u32) {
    let first_16bits = ((magic & 0xffff0000) >> 16) as u16;
    (
        sock_addr.port() ^ first_16bits,
        //deref because compiler doesn't understand
        //conversion from &IpV4Addr to u32
        u32::from(*sock_addr.ip()) ^ magic,
    )
}

/*  If the IP address family is IPv6, X-Address is computed by taking the mapped IP address
    in host byte order, XOR'ing it with the concatenation of the magic
    cookie and the 96-bit transaction ID, and converting the result to
    network byte order.
*/
pub fn obfuscate_v6(sock_addr: &SocketAddrV6, magic: u32, txn_id: u128) -> (u16, u128) {
    unimplemented!();
}
