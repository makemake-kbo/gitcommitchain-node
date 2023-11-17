use alloy_primitives::{
	address,
	Address,
	U256,
};

#[derive(Debug)]
pub struct Transaction {
	pub origin: Address,
	pub to: Address,
	pub value: U256,
	pub basefee: U256,
	pub max_basefee: U256,
	pub max_priority: U256,
	pub calldata: Vec<u8>,
	pub signature: [u8; 65],
}

impl Transaction {
	pub fn new(
		origin: Address,
		to: Address,
		value: U256,
		basefee: U256,
		max_basefee: U256,
		max_priority: U256,
		calldata: Vec<u8>,
		signature: [u8; 65],
	) -> Self {
		Self {
			origin,
			to,
			value,
			basefee,
			max_basefee,
			max_priority,
			calldata,
			signature,
		}
	}

	pub fn default() -> Self {
		Self {
			origin: address!("0000000000000000000000000000000000000000"),
			to: address!("0000000000000000000000000000000000000000"),
			value: U256::from(0),
			basefee: U256::from(0),
			max_basefee: U256::from(0),
			max_priority: U256::from(0),
			calldata: Vec::new(),
			signature: [0; 65],
		}
	}
}

#[derive(Debug)]
pub struct Block {
	pub hash: [u8; 32],
	pub previous_hash: [u8; 32],
	pub coinbase: Address,
	pub transactions: Vec<Transaction>,
}

impl Block {
	pub fn new(
		hash: [u8; 32],
		previous_hash: [u8; 32],
		coinbase: Address,
		transactions: Vec<Transaction>,
	) -> Self {
		Self {
			hash,
			previous_hash,
			coinbase,
			transactions,
		}
	}

	pub fn default () -> Self {
		Self {
			hash: [0; 32],
			previous_hash: [0; 32],
			coinbase: address!("0000000000000000000000000000000000000000"),
			transactions: Vec::new(),
		}
	}
}

