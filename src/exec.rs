use crate::types::Block;
use sled::Db;
use std::sync::Arc;

pub fn execute_state_transition(
	db: Arc<Db>,
	block: Block,
) -> Result<(), Box<dyn std::error::Error>> {
	// loop over transactions in DB and update balances
	for tx in block.transactions {
		// Check sender balance
		if db.get(tx.from) < tx.value {
			Err("Invalid state transition!")
		}
	}

	Ok(())
}
