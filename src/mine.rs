use crate::exec::execute_state_transition;
use crate::types::{
    Block,
    Mempool,
};
use rand::Rng;
use std::sync::Arc;
use std::sync::RwLock;

use rand::thread_rng;

use alloy_primitives::Address;
use blake3::hash;

use sled::Db;

fn hash_block(
    mempool: Mempool,
    difficulty_target: u128,
    coinbase: Address,
    nonce: u64,
    prev_hash: [u8; 32],
) -> Option<Block> {
    // set up block
    let mut new_block = Block::default();

    new_block.transactions = mempool.into();
    new_block.coinbase = coinbase;
    new_block.previous_hash = prev_hash;
    new_block.nonce = nonce;

    // serialize block to bytes and hash
    let block_bytes = bincode::serialize(&new_block).unwrap();
    let block_hash = hash(block_bytes.as_slice());

    if u128::from_le_bytes(block_hash.as_bytes()[0..16].try_into().unwrap()) < difficulty_target {
        new_block.hash = *block_hash.as_bytes();
        return Some(new_block);
    }

    None
}

pub async fn mine_block(
    mempool: Arc<RwLock<Mempool>>,
    db: Arc<Db>,
    coinbase: Address,
    prev_hash: [u8; 32],
) -> Option<Block> {
    // Perform mining until a valid block is found
    loop {
        // Attempt to mine a block
        if let Some(block) = hash_block(
            mempool.read().unwrap().clone(),
            u128::MAX - u16::MAX as u128,
            coinbase,
            thread_rng().gen(),
            prev_hash,
        ) {
            let _ = execute_state_transition(db, block.clone());
            // clear mempool
            mempool.write().unwrap().transactions.clear();
            return Some(block);
        }
    }
}

pub async fn mine(
    mempool: Arc<RwLock<Mempool>>,
    db: Arc<Db>,
    coinbase: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let a = mine_block(mempool.clone(), db.clone(), coinbase, [0; 32]).await;
        if a.is_none() {
            println!("lol go next");
            continue;
        }
        // let a = a.unwrap();

        println!("Mined block!");
    }
}
