use crate::traits::DarkPool;
use std::{cmp, collections::HashMap};

pub type ItemQty = (u32, u32);

pub struct PlainDarkPool {}

impl PlainDarkPool {
	pub fn new() -> Self {
		Self {}
	}
}

impl Default for PlainDarkPool {
	fn default() -> Self {
		Self::new()
	}
}

impl DarkPool<ItemQty> for PlainDarkPool {
	fn volume_matching(
		&self,
		b_orders: Vec<ItemQty>,
		s_orders: Vec<ItemQty>,
	) -> (Vec<ItemQty>, Vec<ItemQty>) {
		let aggregate_orders = |orders: &Vec<ItemQty>| -> HashMap<_, _> {
			let mut aggregate = HashMap::<u32, u32>::new();
			for (item, qty) in orders.iter() {
				aggregate.insert(
					*item,
					match aggregate.get(item) {
						Some(v) => *v + *qty,
						None => *qty,
					},
				);
			}
			aggregate
		};

		let fulfill_orders = |orders: &Vec<ItemQty>, mut transact_items: HashMap<u32, u32>| -> Vec<ItemQty> {
			let mut transacted_orders = Vec::<ItemQty>::new();
			for (item, qty) in orders {
				let reserved = *(transact_items.get(item).unwrap_or(&0));
				let transacted_qty = cmp::min(*qty, reserved);
				transacted_orders.push((*item, transacted_qty));
				transact_items.insert(*item, reserved - transacted_qty);
			}

			transacted_orders
		};

		// aggregate up on buy and sell orders
		let agg_buy = aggregate_orders(&b_orders);
		let agg_sell = aggregate_orders(&s_orders);

		// items remained for transactions
		let mut transact_items = HashMap::<u32, u32>::new();
		for key in agg_buy.keys() {
			transact_items
				.insert(*key, cmp::min(*agg_buy.get(key).unwrap(), *agg_sell.get(key).unwrap_or(&0)));
		}

		let b_fulfilled = fulfill_orders(&b_orders, transact_items.clone());
		let s_fulfilled = fulfill_orders(&s_orders, transact_items.clone());

		(b_fulfilled, s_fulfilled)
	}
}
