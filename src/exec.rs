use crate::types::Block;
use sled::{
    Batch,
    Db,
};
use std::sync::Arc;

pub fn execute_state_transition(
    db: Arc<Db>,
    block: Block,
) -> Result<(), Box<dyn std::error::Error>> {
    // create sled batch for bulk atomic writes
    let mut batch = Batch::default();

    // loop over transactions in DB and update balances
    for tx in block.transactions {
        // Check sender balance
        let res = if db.get(tx.origin).unwrap().is_none() {
            return Err("Invalid state transition!".into());
        } else {
            db.get(tx.origin).unwrap().unwrap()
        };

        let res = std::str::from_utf8(&res).unwrap();
        // convert to u128 (:
        let res = res.parse::<u128>().unwrap();

        // check if the sender has enough balance
        if res < tx.value {
            return Err("Invalid state transition!".into());
        }

        let balance = res - tx.value;
        let balance = balance.to_string();
        batch.insert::<&[u8], &[u8]>(tx.origin.as_ref(), balance.as_ref());

        // get balance of the receiver
        let res = if db.get(tx.to).unwrap().is_none() {
            return Err("Invalid state transition!".into());
        } else {
            db.get(tx.to).unwrap()
        };

        if res.is_none() {
            batch.insert::<&[u8], &[u8]>(tx.origin.as_ref(), tx.value.to_string().as_ref());
        } else {
            let res = res.unwrap();
            let res = std::str::from_utf8(&res).unwrap();
            let mut res = res.parse::<u128>().unwrap();
            res += tx.value;

            batch.insert::<&[u8], &[u8]>(tx.origin.as_ref(), res.to_string().as_ref());
        }
    }

    let _ = db.apply_batch(batch);

    Ok(())
}
