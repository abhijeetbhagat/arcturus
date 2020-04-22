use rand::prelude::*;

fn gen_txn_id() {
    let mut rng = thread_rng();
    let id = rng.gen::<u128>();
    id & 0xffffffffffffffffffffffff //take 96 bits
}