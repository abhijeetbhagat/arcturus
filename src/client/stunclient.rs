use crate::common::attributes::xormappedaddress::XorMappedAddress;
use crate::common::stunmessage::StunMessage;
use crate::utils::miscutils;
use crate::utils::miscutils::Result;
use crate::utils::obfuscation;
use async_trait::async_trait;

#[async_trait]
pub trait StunClient {
    async fn connect(&mut self) -> Result<()>;

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

    async fn send_binding_request(&self) -> Result<StunMessage>;
}
