use tfhe::prelude::*;
use tfhe::{FheUint32, ServerKey, set_server_key};

use crate::traits::DarkPool;

pub type EncItemQty = (FheUint32, FheUint32);

pub struct FheDarkPool {
	max_items: u32,
}

impl FheDarkPool {
	pub fn new(sk: ServerKey, max_items: u32) -> Self {
		set_server_key(sk);
		Self { max_items }
	}
}

impl DarkPool<EncItemQty> for FheDarkPool {
	fn volume_matching(
		&self,
		enc_b_orders: Vec<EncItemQty>,
		enc_s_orders: Vec<EncItemQty>,
	) -> (Vec<EncItemQty>, Vec<EncItemQty>) {
		// NX> loop thru the enc_b_orders here
		let aggregate_orders = |enc_orders: &Vec<EncItemQty>| -> Vec<FheUint32> {
			let mut aggregate: Vec<FheUint32> =
				(0..self.max_items).map(|_| FheUint32::try_encrypt_trivial(0u32).unwrap()).collect();
			for i in 0..self.max_items {
				let enc_i = FheUint32::try_encrypt_trivial(i).unwrap();
				for (ok, oq) in enc_orders {
					aggregate[i as usize] += enc_i.eq(ok).scalar_select(oq, 0u32);
				}
			}
			aggregate
		};

		let fulfill_orders =
			|orders: &Vec<EncItemQty>, mut transact_items: Vec<FheUint32>| -> Vec<EncItemQty> {
				let mut transacted_orders = Vec::<EncItemQty>::new();

				let enc_zero = FheUint32::try_encrypt_trivial(0u32).unwrap();

				for order in orders {
					let mut tx_qty = enc_zero.clone();

					for j in 0..transact_items.len() {
						let enc_j = FheUint32::try_encrypt_trivial(j as u32).unwrap();

						// Is the current `transact_items[j]` matches with `orders[i].0`?
						let b_item = enc_j.eq(&order.0);
						tx_qty = tx_qty.max(&b_item.select(&order.1.min(&transact_items[j]), &enc_zero));

						// update transact_items how much qty left after this order is fulfilled
						transact_items[j] -= b_item.select(&tx_qty, &enc_zero);
					}
					transacted_orders.push((order.0.clone(), tx_qty));
				}
				transacted_orders
			};

		let agg_buy = aggregate_orders(&enc_b_orders);
		let agg_sell = aggregate_orders(&enc_s_orders);

		let mut transact_items = Vec::new();
		for i in 0..agg_buy.len() {
			transact_items.push(agg_buy[i].min(&agg_sell[i]));
		}

		let b_fulfilled = fulfill_orders(&enc_b_orders, transact_items.clone());
		let s_fulfilled = fulfill_orders(&enc_s_orders, transact_items.clone());

		(b_fulfilled, s_fulfilled)
	}
}
