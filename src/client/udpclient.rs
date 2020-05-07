use crate::client::stunclient::StunClient;
use crate::common::attributes::xormappedaddress::XorMappedAddress;
use crate::common::stunmessage::{Class, Method, StunMessage, StunMessageHeader, Type};
use crate::utils::miscutils;
use crate::utils::miscutils::Result;
use crate::utils::obfuscation;
use async_std::net::SocketAddr;
use async_std::net::ToSocketAddrs;
use async_std::net::UdpSocket;
use async_std::prelude::*; //reading, writing over streams
use async_trait::async_trait;
use std::net::Ipv4Addr;

pub struct StunUdpClient {
    addr: SocketAddr,
    socket: Option<UdpSocket>,
}

impl StunUdpClient {
    pub async fn new(addr: impl ToSocketAddrs) -> Result<Box<dyn StunClient>> {
        Ok(Box::new(StunUdpClient {
            addr: addr.to_socket_addrs().await?.next().unwrap(),
            socket: None,
        }))
    }

    async fn send_binding_request(&self) -> Result<StunMessage> {
        let txn_id = miscutils::gen_txn_id();
        let header = StunMessageHeader::new(
            Type(Class::Request, Method::BindingRequest),
            0, //length excludes header length
            txn_id,
        );
        let msg = StunMessage::new(header, None);
        self.socket.as_ref().unwrap().send(&msg.as_raw()).await?;
        let mut buf = [0; 1024];
        let size = self.socket.as_ref().unwrap().recv(&mut buf).await?;
        let binding_response = StunMessage::from_raw(&buf[0..size]).unwrap();
        Ok(binding_response)
    }
}

#[async_trait]
impl StunClient for StunUdpClient {
    async fn connect(&mut self) -> Result<()> {
        let local_ip = if self.addr.is_ipv4() {
            "127.0.0.1:0"
        } else {
            "::1:0"
        };
        let socket = Some(UdpSocket::bind(local_ip).await?);
        self.socket = socket;
        self.socket.as_ref().unwrap().connect(self.addr).await?;
        Ok(())
    }

    async fn get_reflexive_address(&self) -> Result<()> {
        let binding_response: StunMessage = self.send_binding_request().await?;
        let xor_mapped_addr = XorMappedAddress::from_raw(
            binding_response
                .payload
                .unwrap()
                .as_slice()
                //TODO abhi: we should let a TLV decoder handle this really
                .get(4..) //TLV encoded; skip the Type and Length (combined 4 bytes) and pass only the value
                .unwrap(),
        )
        .unwrap();
        if xor_mapped_addr.family == 1 {
            let integral_addr: u32 = obfuscation::unobfuscate_v4_ip(
                xor_mapped_addr.address.left().unwrap(),
                miscutils::MAGIC_COOKIE,
            );
            //let ip: u32 = u32::from("127.0.0.1".parse::<Ipv4Addr>().unwrap());
            //assert!(integral_addr == ip);
            println!("Your IPv4 addr is {:?}", integral_addr);
        } else {
            //TODO abhi: assert on an ipv6 address
            let integral_addr: u128 = obfuscation::unobfuscate_v6_ip(
                xor_mapped_addr.address.right().unwrap(),
                miscutils::MAGIC_COOKIE,
                binding_response.header.txn_id,
            );
            println!("Your IPv6 addr is {:?}", integral_addr);
        }

        Ok(())
    }
}
