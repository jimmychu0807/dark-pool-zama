use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::traits::DarkPool;

pub type ItemQty = (u32, u32);

pub struct PlainDarkPool {}

impl PlainDarkPool {
	pub fn new() -> Self {
		Self {}
	}
}

impl DarkPool<ItemQty> for PlainDarkPool {
	fn volume_matching(&self, b_orders: Vec<ItemQty>, s_orders: Vec<ItemQty>) -> Vec<ItemQty> {
		vec![(2u32, 10u32)]
	}
}
