use std::collections::HashMap;
use std::hash::{ Hash, Hasher };

mod crate::traits;
use traits::{DarkPool};

pub type ItemQty = (u32, u32);

pub struct PlainDarkPool {}

impl DarkPool<ItemQty> for PlainDarkPool {
	fn volume_matching(
		&self,
		b_orders: Vec<ItemQty>,
		s_orders: Vec<ItemQty>,
	) -> Vec<ItemQty>
	{
		(2u32, 10u32)
	}
}
