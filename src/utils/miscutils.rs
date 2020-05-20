use rand::prelude::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const MAGIC_COOKIE: u32 = 0x2112A442;

pub fn gen_txn_id() -> u128 {
    let mut rng = thread_rng();
    let id = rng.gen::<u128>();
    id
}
