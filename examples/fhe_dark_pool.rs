use tfhe::{ConfigBuilder, generate_keys};

use dark_pool::utils::{decrypt_orders, encrypt_orders};
use dark_pool::{FheDarkPool, traits::DarkPool};

const ITEMS: u32 = 10;

fn main() {
	let config = ConfigBuilder::default().build();

	let (ck, sk) = generate_keys(config);

	// generate the data - small case
	let b_orders = vec![(1, 55), (2, 100), (3, 80)];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];

	// encrypt the b_orders, s_orders
	let enc_b_orders = encrypt_orders(&b_orders, &ck);
	let enc_s_orders = encrypt_orders(&s_orders, &ck);

	// server-side processing
	let darkpool = FheDarkPool::new(sk, ITEMS);
	let (enc_b_fulfilled, enc_s_fulfilled) = darkpool.volume_matching(enc_b_orders, enc_s_orders);

	let b_fulfilled = decrypt_orders(enc_b_fulfilled, &ck);
	println!("b_fulfilled: {:?}", b_fulfilled);
	assert_eq!(b_fulfilled, vec![(1, 0), (2, 30), (3, 80)]);

	let s_fulfilled = decrypt_orders(enc_s_fulfilled, &ck);
	println!("s_fulfilled: {:?}", s_fulfilled);
	assert_eq!(s_fulfilled, vec![(3, 40), (3, 40), (2, 30)]);
}
