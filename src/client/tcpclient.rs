//use crate::common::bindingresponse::BindingResponse;
use crate::client::stunclient::StunClient;
use crate::common::stunmessage::{Class, Method, StunMessage, StunMessageHeader, Type};
use crate::utils::miscutils;
use crate::utils::miscutils::Result;
use async_std::net::SocketAddr;
use async_std::net::TcpStream;
use async_std::net::ToSocketAddrs;
use async_std::prelude::*; //reading, writing over streams
use async_trait::async_trait;

pub struct StunTcpClient {
    addr: SocketAddr,
    stream: Option<TcpStream>,
}

impl StunTcpClient {
    pub async fn new(addr: impl ToSocketAddrs) -> Result<Box<dyn StunClient + std::marker::Sync>> {
        Ok(Box::new(StunTcpClient {
            addr: addr.to_socket_addrs().await?.next().unwrap(),
            stream: None,
        }))
    }
}

#[async_trait]
impl StunClient for StunTcpClient {
    async fn connect(&mut self) -> Result<()> {
        let stream = Some(TcpStream::connect(self.addr).await?);
        self.stream = stream;
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
