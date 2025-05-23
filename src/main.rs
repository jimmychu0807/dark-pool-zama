use dark_pool::DarkPool;
use tfhe::prelude::*;
use tfhe::{ConfigBuilder, FheUint8, generate_keys, set_server_key};

mod utils;
use utils::{decrypt_txs, encrypt_orders};

fn main() {
	let config = ConfigBuilder::default().build();

	// client-side
	let (client_key, server_key) = generate_keys(config);

	// generate the data - small case
	let b_orders = vec![(1, 55), (2, 100), (3, 80)];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];

	let enc_b_orders = encrypt_orders(b_orders.clone(), &client_key);
	let enc_s_orders = encrypt_orders(s_orders.clone(), &client_key);

	// server-side
	let dp = DarkPool::new(server_key);
	let enc_result = dp.volume_matching(enc_b_orders, enc_s_orders);

	// client-side
	let matched_result = decrypt_txs(enc_result);
	assert_eq!(matched_result, vec![(1, 0), (2, 30), (3, 80)]);
}
