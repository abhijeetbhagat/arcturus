//use crate::common::bindingresponse::BindingResponse;
use crate::common::attributes::xormappedaddress::XorMappedAddress;
use crate::common::stunmessage::{Class, Method, StunMessage, StunMessageHeader, Type};
use crate::utils::miscutils;
use crate::utils::miscutils::Result;
use async_std::net::SocketAddr;
use async_std::net::TcpStream;
use async_std::net::ToSocketAddrs;
use async_std::prelude::*; //reading, writing over streams
use std::net::Ipv4Addr;

pub struct StunClient {
    addr: SocketAddr,
    stream: Option<TcpStream>,
}

impl StunClient {
    pub async fn new(addr: impl ToSocketAddrs) -> Result<Self> {
        Ok(StunClient {
            addr: addr.to_socket_addrs().await?.next().unwrap(),
            stream: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let stream = Some(TcpStream::connect(self.addr).await?);
        self.stream = stream;
        Ok(())
    }

    pub async fn get_reflexive_address(&self) -> Result<()> {
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
            let integral_addr: u32 =
                u32::from(xor_mapped_addr.address.left().unwrap()) ^ miscutils::MAGIC_COOKIE;
            let ip: u32 = u32::from("172.20.10.2".parse::<Ipv4Addr>().unwrap());
            assert!(integral_addr == ip);
        } else {
            //TODO abhi: assert on an ipv6 address
            //let integral_addr: u128 =
            //    u128::from(xor_mapped_addr.address.right().unwrap()) ^ miscutils::MAGIC_COOKIE;
            //let ip: u32 = u32::from("172.20.10.2".parse::<Ipv4Addr>().unwrap());
            //assert!(integral_addr == ip);
        }
        Ok(())
    }

    async fn send_binding_request(&self) -> Result<StunMessage> {
        let txn_id = miscutils::gen_txn_id();
        let header = StunMessageHeader::new(
            Type(Class::Request, Method::BindingRequest),
            0, //length excludes header length
            txn_id,
        );
        let msg = StunMessage::new(header, None);
        let (reader, writer) = &mut (&self.stream, &self.stream);
        writer.as_ref().unwrap().write(&msg.as_raw()).await?;
        let mut buf = [0; 1024];
        let size = reader.as_ref().unwrap().read(&mut buf).await?;
        let binding_response = StunMessage::from_raw(&buf[0..size]).unwrap();
        Ok(binding_response)
    }
}
