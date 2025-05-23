use dark_pool::EncItemQty;
use tfhe::prelude::*;
use tfhe::{ClientKey, FheUint32};

pub type ItemQty = (u32, u32);

pub fn encrypt_orders(orders: Vec<(u32, u32)>, ck: &ClientKey) -> Vec<(FheUint32, FheUint32)> {
	return vec![(FheUint32::encrypt(2u32, ck), FheUint32::encrypt(100u32, ck))];
}

pub fn decrypt_txs(enc_txs: Vec<EncItemQty>) -> Vec<ItemQty> {
	return vec![(10u32, 20u32)];
}
