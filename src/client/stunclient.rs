use crate::utils::miscutils::Result;
use async_trait::async_trait;

#[async_trait]
pub trait StunClient {
    async fn connect(&mut self) -> Result<()>;
    async fn get_reflexive_address(&self) -> Result<()>;
}
