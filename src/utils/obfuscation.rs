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

pub fn obfuscate_v4_ip_port(sock_addr: &SocketAddrV4, magic: u32) -> (u16, u32) {
    (
        obfuscate_port(sock_addr.port(), magic),
        //deref because compiler doesn't understand
        //conversion from &IpV4Addr to u32
        obfuscate_v4_ip(u32::from(*sock_addr.ip()), magic),
    )
}

pub fn obfuscate_port(port: u16, magic: u32) -> u16 {
    _xor_u16_and_u32(port, magic)
}

/*  If the IP address family is IPv6, X-Address is computed by taking the mapped IP address
    in host byte order, XOR'ing it with the concatenation of the magic
    cookie and the 96-bit transaction ID, and converting the result to
    network byte order.
*/
pub fn obfuscate_v6_ip_port(sock_addr: &SocketAddrV6, magic: u32, txn_id: u128) -> (u16, u128) {
    (
        obfuscate_port(sock_addr.port(), magic),
        obfuscate_v6_ip(u128::from(*sock_addr.ip()), magic, txn_id),
    )
}

pub fn unobfuscate_port(port: u16, magic: u32) -> u16 {
    _xor_u16_and_u32(port, magic)
}

pub fn obfuscate_v4_ip(ip: u32, magic: u32) -> u32 {
    _xor_u32(ip, magic)
}
pub fn unobfuscate_v4_ip(ip: u32, magic: u32) -> u32 {
    _xor_u32(ip, magic)
}

pub fn unobfuscate_v6_ip(ip: u128, magic: u32, txn_id: u128) -> u128 {
    _xor_u128(ip, magic, txn_id)
}
pub fn obfuscate_v6_ip(ip: u128, magic: u32, txn_id: u128) -> u128 {
    _xor_u128(ip, magic, txn_id)
}

fn _xor_u32(op1: u32, op2: u32) -> u32 {
    op1 ^ op2
}
fn _xor_u16_and_u32(op1: u16, op2: u32) -> u16 {
    let first_16bits = ((op2 & 0xffff0000) >> 16) as u16;
    op1 ^ first_16bits
}
fn _xor_u128(op1: u128, op2: u32, op3: u128) -> u128 {
    let operand = (op2 as u128) << 96 | op3;
    op1 ^ operand
}
