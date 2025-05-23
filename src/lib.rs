use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use tfhe::prelude::*;
use tfhe::{FheUint32, ServerKey, set_server_key};

mod traits;
pub use traits::DarkPool;

pub type EncItemQty = (FheUint32, FheUint32);

pub struct FheDarkPool {}

impl FheDarkPool {
	pub fn new(sks: ServerKey) -> Self {
		set_server_key(sks);
		Self {}
	}
}

impl DarkPool<EncItemQty> for FheDarkPool {
	fn volume_matching(
		&self,
		enc_b_orders: Vec<EncItemQty>,
		enc_s_orders: Vec<EncItemQty>,
	) -> Vec<EncItemQty> {
		// let mut buy_orders = HashMap::new();

		// for (enc_item, enc_qty) in enc_b_orders {
		// 	let val = match buy_orders.contains_key(&enc_item) {
		// 		true => buy_orders.get(&enc_item) + enc_qty,
		// 		false => enc_qty,
		// 	};
		// 	buy_orders.insert(enc_item, val);
		// }

		vec![(FheUint32::encrypt_trivial(2u32), FheUint32::encrypt_trivial(100u32))]
	}
}

// impl Hash for FheUint32 {
// 	fn hash<H: Hasher>(&self, state: &mut H) {

// 	}
// }
