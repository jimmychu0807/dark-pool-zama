use tfhe::prelude::*;
use tfhe::{ClientKey, FheUint32};

use crate::{EncItemQty, ItemQty};

pub fn encrypt_orders(orders: &[ItemQty], ck: &ClientKey) -> Vec<EncItemQty> {
	orders.iter().map(|(v1, v2)| (FheUint32::encrypt(*v1, ck), FheUint32::encrypt(*v2, ck))).collect()
}

pub fn decrypt_orders(enc_txs: Vec<EncItemQty>, ck: &ClientKey) -> Vec<ItemQty> {
	enc_txs.iter().map(|(en1, en2)| (en1.decrypt(ck), en2.decrypt(ck))).collect()
}
