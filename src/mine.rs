use crate::types::{
	Mempool,
	Block
};

use tokio::sync::broadcast::Receiver;

use alloy_primitives::Address;
use blake3::hash;

fn mine_block(
	mempool: Mempool,
	difficulty_target: u128,
	coinbase: Address,
	prev_hash: [u8; 32],
) -> Option<Block> {
	// set up block

	let mut new_block = Block::default();

	new_block.transactions = mempool.into();
	new_block.coinbase = coinbase;
	new_block.previous_hash = prev_hash;

	// serialize block to bytes and hash
	let block_bytes = bincode::serialize(&new_block).unwrap();
	let block_hash = hash(block_bytes.as_slice());

	if u128::from_le_bytes(block_hash.as_bytes()[0..16].try_into().unwrap()) < difficulty_target {
		new_block.hash = *block_hash.as_bytes();
		return Some(new_block);
	}

	None
}

pub async fn mine(mut mempool_channel: Receiver<Mempool>, difficulty_target: u128, coinbase: Address, prev_hash: [u8; 32]) -> Option<Block> {
    // Receive mempool updates through the channel
    let mut mempool = match mempool_channel.recv().await {
        Ok(mempool) => mempool,
        Err(_) => return None, // Handle the error as needed
    };

    // Perform mining until a valid block is found
    loop {
        // Attempt to mine a block
        if let Some(block) = mine_block(mempool.clone(), difficulty_target, coinbase, prev_hash) {
            return Some(block);
        }

        // If mining is unsuccessful, update mempool with the latest transactions
        match mempool_channel.recv().await {
            Ok(new_mempool) => mempool = new_mempool,
            Err(_) => return None, // Handle the error as needed
        }
    }
}
