use crate::client::stunclient::StunClient;
use crate::common::stunmessage::{Class, Method, StunMessage, StunMessageHeader, Type};
use crate::utils::miscutils;
use crate::utils::miscutils::Result;
use async_std::net::SocketAddr;
use async_std::net::ToSocketAddrs;
use async_std::net::UdpSocket;
use async_trait::async_trait;

pub struct StunUdpClient {
    addr: SocketAddr,
    socket: Option<UdpSocket>,
}

impl StunUdpClient {
    pub async fn new(addr: impl ToSocketAddrs) -> Result<Box<dyn StunClient + std::marker::Sync>> {
        Ok(Box::new(StunUdpClient {
            addr: addr.to_socket_addrs().await?.next().unwrap(),
            socket: None,
        }))
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
