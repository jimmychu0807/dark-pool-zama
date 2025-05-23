use tfhe::prelude::*;
use tfhe::{FheUint32, ServerKey, set_server_key};

pub type EncItemQty = (FheUint32, FheUint32);

pub struct DarkPool {}

impl DarkPool {
	pub fn new(sks: ServerKey) -> Self {
		set_server_key(sks);

		return DarkPool {};
	}

	pub fn volume_matching(
		&self,
		enc_b_orders: Vec<EncItemQty>,
		enc_s_orders: Vec<EncItemQty>,
	) -> Vec<EncItemQty> {
		return vec![(FheUint32::encrypt_trivial(2u32), FheUint32::encrypt_trivial(100u32))];
	}
}
