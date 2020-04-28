#[macro_use]
extern crate clap;
extern crate async_std;
use clap::{App, Arg, SubCommand};

mod client;
mod common;
mod server;
mod utils;

use async_std::net::Ipv4Addr;
use async_std::net::SocketAddrV4;
use async_std::prelude::*;
use async_std::task;
use client::client::StunClient;
use server::server::StunServer;
use utils::miscutils::Result;

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
                .arg(Arg::with_name("ip").short("h").long("ip").takes_value(true))
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("whereami")
                .arg(
                    Arg::with_name("rip")
                        .long("rh")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("rport")
                        .long("rp")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("shine") {
        let ip = matches
            .value_of("ip")
            .unwrap_or("127.0.0.1")
            .parse::<Ipv4Addr>()
            .unwrap();
        let port = matches
            .value_of("port")
            .unwrap_or("7969")
            .parse::<u16>()
            .unwrap();
        let fut = StunServer::start(SocketAddrV4::new(ip, port));
        task::block_on(fut);
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("whereami") {
        let rip = matches
            .value_of("rip")
            .unwrap()
            .parse::<Ipv4Addr>()
            .unwrap();
        let rport = matches.value_of("rport").unwrap().parse::<u16>().unwrap();
        let mut client = StunClient::new(SocketAddrV4::new(rip, rport))
            .await
            .unwrap();
        client.connect().await.unwrap();
        client.get_reflexive_address().await.unwrap();
        Ok(())
    } else {
        println!("Invalid subcommand. Please RTFM.");
        Ok(())
    }
}
