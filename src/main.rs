use tfhe::prelude::*;
use tfhe::{ConfigBuilder, FheUint8, generate_keys, set_server_key};

mod utils;
use utils::{decrypt_txs, encrypt_orders};

mod traits;
use traits::DarkPool;

mod plain_dark_pool;
use plain_dark_pool::PlainDarkPool;

fn main() {
	// let config = ConfigBuilder::default().build();

	// client-side
	// let (client_key, server_key) = generate_keys(config);

	// generate the data - small case
	let b_orders = vec![(1, 55), (2, 100), (3, 80)];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];

	// let enc_b_orders = encrypt_orders(b_orders.clone(), &client_key);
	// let enc_s_orders = encrypt_orders(s_orders.clone(), &client_key);

	// server-side
	let dp = PlainDarkPool::new();
	let result = dp.volume_matching(b_orders, s_orders);

	// client-side
	// let matched_result = decrypt_txs(enc_result, &client_key);
}
