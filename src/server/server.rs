use crate::common::attributes::xormappedaddress::XorMappedAddress;
use either::{Left, Right};
//use crate::common::bindingresponse::BindingResponse;
use crate::common::stunmessage::{Class, Method, StunMessage, StunMessageHeader, Type};
use crate::utils::miscutils::Result;
use crate::utils::obfuscation;
use async_std::io::Read;
use async_std::net::SocketAddr;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::net::ToSocketAddrs;
use async_std::prelude::*;
use async_std::task;

pub struct StunServer {}

impl StunServer {
    pub async fn start(addr: impl ToSocketAddrs) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("Started shining at {:?}", listener.local_addr()?);
        let mut incoming_stream = listener.incoming();
        while let Some(stream) = incoming_stream.next().await {
            let stream = stream?;
            task::spawn(async move {
                StunServer::handle(stream).await;
            });
        }
        Ok(())
    }

    async fn handle(stream: TcpStream) -> Result<()> {
        let source_transport_sock_addr = stream.peer_addr().unwrap();
        println!("Got request from {:?}", source_transport_sock_addr);
        let (reader, writer) = &mut (&stream, &stream);
        let mut buf = vec![0; 1024];
        while let size = reader.read(&mut buf).await? {
            //Check if this is a STUN request since other protocols
            //can be multiplexed according to https://tools.ietf.org/html/rfc5389#section-8
            if size > 0 {
                if let Some(stun_message) = StunMessage::from_raw(&buf[0..size]) {
                    let response = match stun_message.header.msg_type.0 {
                        Class::Request => {
                            match stun_message.header.msg_type.1 {
                                Method::BindingRequest => {
                                    //This is a binding request
                                    let response = match source_transport_sock_addr {
                                        SocketAddr::V4(v4_sock_addr) => {
                                            let (xor_port, xor_mapped_addr) =
                                                obfuscation::obfuscate_v4(
                                                    &v4_sock_addr,
                                                    stun_message.header.magic,
                                                );
                                            let header = StunMessageHeader::new(
                                                Type(Class::Success, Method::BindingResponse),
                                                12,
                                                stun_message.header.txn_id,
                                            );
                                            let attribute = XorMappedAddress::new(
                                                1,
                                                xor_port,
                                                Left(xor_mapped_addr),
                                            )
                                            .as_raw();
                                            StunMessage::new(header, Some(attribute))
                                        }

                                        SocketAddr::V6(v6_sock_addr) => {
                                            let (xor_port, xor_mapped_addr) =
                                                obfuscation::obfuscate_v6(
                                                    &v6_sock_addr,
                                                    stun_message.header.magic,
                                                    stun_message.header.txn_id,
                                                );
                                            let header = StunMessageHeader::new(
                                                Type(Class::Success, Method::BindingResponse),
                                                24,
                                                stun_message.header.txn_id,
                                            );
                                            let attribute = XorMappedAddress::new(
                                                2,
                                                xor_port,
                                                Right(xor_mapped_addr),
                                            )
                                            .as_raw();
                                            StunMessage::new(header, Some(attribute))
                                        }
                                    };

                                    Some(response)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    };
                    if let Some(response) = response {
                        println!("Sending STUN response");
                        writer.write(&response.as_raw()).await?;
                        writer.flush();
                        buf.clear();
                    }
                } else {
                    continue;
                }
            }
        }
        Ok(())
    }
}
