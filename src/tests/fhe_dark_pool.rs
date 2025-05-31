use tfhe::{ConfigBuilder, generate_keys};

use crate::utils::{decrypt_orders, encrypt_orders};
use crate::{FheDarkPool, traits::DarkPool};

#[test]
fn handle_empty_buy_order() {
	let config = ConfigBuilder::default().build();
	let (ck, sk) = generate_keys(config);

	let b_orders = vec![];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];
	let max_items = 4u32;

	let enc_b_orders = encrypt_orders(&b_orders, &ck);
	let enc_s_orders = encrypt_orders(&s_orders, &ck);

	// server-side processing
	let dp = FheDarkPool::new(sk, max_items);
	let (enc_b_fulfilled, enc_s_fulfilled) = dp.volume_matching(enc_b_orders, enc_s_orders);

	// client-side
	let b_fulfilled = decrypt_orders(enc_b_fulfilled, &ck);
	assert_eq!(b_fulfilled, vec![]);

	let s_fulfilled = decrypt_orders(enc_s_fulfilled, &ck);
	assert_eq!(s_fulfilled, vec![(3, 0), (3, 0), (2, 0)]);
}

#[test]
fn handle_simple_case() {
	let config = ConfigBuilder::default().build();
	let (ck, sk) = generate_keys(config);

	let b_orders = vec![(1, 55), (2, 100), (3, 80)];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];
	let max_items = 4u32;

	let enc_b_orders = encrypt_orders(&b_orders, &ck);
	let enc_s_orders = encrypt_orders(&s_orders, &ck);

	// server-side processing
	let dp = FheDarkPool::new(sk, max_items);
	let (enc_b_fulfilled, enc_s_fulfilled) = dp.volume_matching(enc_b_orders, enc_s_orders);

	// client-side
	let b_fulfilled = decrypt_orders(enc_b_fulfilled, &ck);
	assert_eq!(b_fulfilled, vec![(1, 0), (2, 30), (3, 80)]);

	let s_fulfilled = decrypt_orders(enc_s_fulfilled, &ck);
	assert_eq!(s_fulfilled, vec![(3, 40), (3, 40), (2, 30)]);
}

#[test]
fn handle_complex_case() {
	let config = ConfigBuilder::default().build();
	let (ck, sk) = generate_keys(config);

	let b_orders: Vec<(u32, u32)> = (0..50).map(|i| (i % 10 + 1, i % 100 + 1)).collect();

	let s_orders: Vec<(u32, u32)> = (0..40).map(|i| (i % 10 + 1, i % 100 + 1)).collect();
	let max_items = 11u32;

	let enc_b_orders = encrypt_orders(&b_orders, &ck);
	let enc_s_orders = encrypt_orders(&s_orders, &ck);

	let expected_b_fulfilled: Vec<(u32, u32)> = [
		(0..40).map(|i| (i % 10 + 1, i % 100 + 1)).collect::<Vec<_>>(),
		(40..50).map(|i| (i % 10 + 1, 0)).collect::<Vec<_>>(),
	]
	.concat();

	let expected_s_fulfilled = s_orders.clone();

	// server-side processing
	let dp = FheDarkPool::new(sk, max_items);
	let (enc_b_fulfilled, enc_s_fulfilled) = dp.volume_matching(enc_b_orders, enc_s_orders);

	// client-side
	let b_fulfilled = decrypt_orders(enc_b_fulfilled, &ck);
	assert_eq!(b_fulfilled, expected_b_fulfilled);

	let s_fulfilled = decrypt_orders(enc_s_fulfilled, &ck);
	assert_eq!(s_fulfilled, expected_s_fulfilled);
}
