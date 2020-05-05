extern crate async_std;
extern crate clap;
use clap::{App, Arg, SubCommand};

mod client;
mod common;
mod server;
mod utils;

use async_std::net::Ipv4Addr;
use async_std::net::SocketAddr;
use async_std::net::{IpAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};
use client::stunclient::StunClient;
use client::tcpclient::StunTcpClient;
use client::udpclient::StunUdpClient;
use server::server::StunServer;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("arcturus")
        .version("0.1")
        .author("abhi")
        .about("A STUN (RFC5389) server and client implementation")
        .subcommand(
            SubCommand::with_name("shine")
                .about(
                    "starts shining in the sky so that sailors can ask it to find their locations",
                )
                .arg(
                    Arg::with_name("addr")
                        .help("IP:port e.g. 127.0.0.1:7969 or ::1:7969")
                        .short("a")
                        .long("address")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("transport")
                        .help("Should it listen for TCP or UDP connections?")
                        .short("t")
                        .long("transport")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("whereami")
                .arg(
                    Arg::with_name("raddr")
                        .short("h")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("transport")
                        .short("t")
                        .long("transport")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("shine") {
        let ip = matches
            .value_of("addr")
            .unwrap_or("127.0.0.1:7969")
            .to_socket_addrs()
            .await
            .unwrap()
            .next()
            .unwrap();

        let transport = matches.value_of("transport").unwrap_or("udp");

        let ip = match ip {
            SocketAddr::V4(ip) => SocketAddr::V4(SocketAddrV4::new(*ip.ip(), ip.port())),
            SocketAddr::V6(ip) => SocketAddr::V6(SocketAddrV6::new(*ip.ip(), ip.port(), 0, 0)),
        };

        //TODO abhi: start in a hybrid mode (TCP + UDP at the same time)
        if transport == "tcp" {
            StunServer::start(ip).await;
        } else {
            StunServer::start_udp(ip).await;
        }

        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("whereami") {
        let rip = matches
            .value_of("raddr")
            .unwrap()
            .to_socket_addrs()
            .await
            .unwrap()
            .next()
            .unwrap();

        let transport = matches.value_of("transport").unwrap_or("udp");

        let rip = match rip {
            SocketAddr::V4(rip) => SocketAddr::V4(SocketAddrV4::new(*rip.ip(), rip.port())),
            SocketAddr::V6(rip) => SocketAddr::V6(SocketAddrV6::new(*rip.ip(), rip.port(), 0, 0)),
        };
        let mut client = if transport == "tcp" {
            StunTcpClient::new(rip).await.unwrap()
        } else {
            StunUdpClient::new(rip).await.unwrap()
        };
        client.connect().await.unwrap();
        client.get_reflexive_address().await.unwrap();

        Ok(())
    } else {
        println!("Invalid subcommand. Please RTFM.");

        Ok(())
    }
}
