use crate::EncItemQty;
use tfhe::prelude::*;
use tfhe::{ClientKey, FheUint32};

pub type ItemQty = (u32, u32);

pub fn encrypt_orders(orders: Vec<ItemQty>, ck: &ClientKey) -> Vec<EncItemQty> {
	orders
		.iter()
		.map(|(v1, v2)| (FheUint32::encrypt(*v1, ck), FheUint32::encrypt(*v2, ck)))
		.collect()
}

pub fn decrypt_txs(enc_txs: Vec<EncItemQty>, ck: &ClientKey) -> Vec<ItemQty> {
	let mut decrypted: Vec<ItemQty> =
		enc_txs.iter().map(|(en1, en2)| (en1.decrypt(ck), en2.decrypt(ck))).collect();

	decrypted.sort_by(|a, b| a.0.cmp(&b.0));
	decrypted
}
