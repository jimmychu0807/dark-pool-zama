use std::{cmp, collections::HashMap};
use crate::traits::DarkPool;

pub type ItemQty = (u32, u32);

pub struct PlainDarkPool {}

impl PlainDarkPool {
	pub fn new() -> Self {
		Self {}
	}
}

impl DarkPool<ItemQty> for PlainDarkPool {
	fn volume_matching(&self, b_orders: Vec<ItemQty>, s_orders: Vec<ItemQty>) -> (Vec<ItemQty>, Vec<ItemQty>) {
		// aggregate up on buy orders
		let mut agg_buy = HashMap::new();
		for (item, qty) in b_orders {
			agg_buy.insert(item, match agg_buy.get(&item) {
				Some(v) => v + qty,
				None => qty,
			});
		}

		// aggregate up on sell orders
		let mut agg_sell = HashMap::new();
		for (item, qty) in s_orders {
			agg_sell.insert(item, match agg_sell.get(&item) {
				Some(v) => v + qty,
				None => qty,
			});
		}

		// items remained for transactions
		let mut transact_items = HashMap::new();
		for key in agg_buy.keys() {
			transact_items.insert(key, cmp::min(
				agg_buy.get(key).unwrap(),
				agg_sell.get(key).unwrap_or(&0)
			));
		}

		println!("transact_items: {:?}", &transact_items);

		let fulfill_order = |orders: &Vec<ItemQty>, transact_items: HashMap<u32, u32> | -> Vec<ItemQty> {
			let transacted_orders = Vec::<ItemQty>::new();
			for (item, qty) in orders {
				let reserved = transact_items.get(item).unwrap_or(&0);
				let transacted_qty = cmp::min(qty, reserved);
				transacted_orders.push((*item, *transacted_qty));
				transact_items.insert(*item, reserved - transacted_qty);
			}

			transacted_orders
		};

		let b_transact = fulfill_order(&b_orders, transact_items.clone());
		let s_transact = fulfill_order(&s_orders, transact_items.clone());

		(b_transact, s_transact)
	}
}
