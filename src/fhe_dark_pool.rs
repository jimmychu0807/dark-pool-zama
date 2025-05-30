use tfhe::prelude::*;
use tfhe::{FheUint32, ServerKey, set_server_key};

use crate::traits::DarkPool;

pub type EncItemQty = (FheUint32, FheUint32);

pub struct FheDarkPool {
	max_items: u32
}

impl FheDarkPool {
	pub fn new(sk: ServerKey, max_items: u32) -> Self {
		set_server_key(sk);
		Self {
			max_items
		}
	}
}

impl DarkPool<EncItemQty> for FheDarkPool {
	fn volume_matching(
		&self,
		enc_b_orders: Vec<EncItemQty>,
		enc_s_orders: Vec<EncItemQty>,
	) -> (Vec<EncItemQty>, Vec<EncItemQty>) {
		// NX> loop thru the enc_b_orders here
		let aggregate_orders = | enc_orders: &Vec<EncItemQty> | -> Vec<FheUint32> {
			let mut aggregate: Vec<FheUint32> = (0..self.max_items)
				.map(|_| FheUint32::try_encrypt_trivial(0u32).unwrap())
				.collect();
			for i in 0..self.max_items {
				let enc_i = FheUint32::try_encrypt_trivial(i).unwrap();
				for (ok, oq) in enc_orders {
					aggregate[i as usize] += enc_i.eq(ok).scalar_select(oq, 0u32);
				}
			}
			aggregate
		};

		let agg_buy = aggregate_orders(&enc_b_orders);
		let agg_sell = aggregate_orders(&enc_s_orders);



		// let mut buy_orders = HashMap::new();

		// for (enc_item, enc_qty) in enc_b_orders {
		// 	let val = match buy_orders.contains_key(&enc_item) {
		// 		true => buy_orders.get(&enc_item) + enc_qty,
		// 		false => enc_qty,
		// 	};
		// 	buy_orders.insert(enc_item, val);
		// }

		(vec![], vec![])
	}
}

// impl Hash for FheUint32 {
// 	fn hash<H: Hasher>(&self, state: &mut H) {

// 	}
// }
