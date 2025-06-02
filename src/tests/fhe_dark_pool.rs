use std::time::Instant;
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

	let instant = Instant::now();

	let enc_b_orders = encrypt_orders(&b_orders, &ck);
	let enc_s_orders = encrypt_orders(&s_orders, &ck);

	let encrypted_orders = instant.elapsed();

	// server-side processing
	let dp = FheDarkPool::new(sk, max_items);
	let (enc_b_fulfilled, enc_s_fulfilled) = dp.volume_matching(enc_b_orders, enc_s_orders);

	let volume_matched = instant.elapsed();

	// client-side
	let b_fulfilled = decrypt_orders(enc_b_fulfilled, &ck);
	assert_eq!(b_fulfilled, vec![]);

	let s_fulfilled = decrypt_orders(enc_s_fulfilled, &ck);
	assert_eq!(s_fulfilled, vec![(3, 0), (3, 0), (2, 0)]);

	let decrypted = instant.elapsed();

	println!("encrypt orders: {:?}", encrypted_orders);
	println!("match volume: {:?}", volume_matched);
	println!("decryption: {:?}", decrypted);

	// encrypt orders: 11.443666ms
	// match volume: 14.611870051s
	// decryption: 14.612287779s
}

#[test]
fn handle_simple_case() {
	let config = ConfigBuilder::default().build();
	let (ck, sk) = generate_keys(config);

	let b_orders = vec![(1, 55), (2, 100), (3, 80)];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];
	let max_items = 4u32;

	let instant = Instant::now();

	let enc_b_orders = encrypt_orders(&b_orders, &ck);
	let enc_s_orders = encrypt_orders(&s_orders, &ck);

	let encrypted_orders = instant.elapsed();

	// server-side processing
	let dp = FheDarkPool::new(sk, max_items);
	let (enc_b_fulfilled, enc_s_fulfilled) = dp.volume_matching(enc_b_orders, enc_s_orders);

	let volume_matched = instant.elapsed();

	// client-side
	let b_fulfilled = decrypt_orders(enc_b_fulfilled, &ck);
	assert_eq!(b_fulfilled, vec![(1, 0), (2, 30), (3, 80)]);

	let s_fulfilled = decrypt_orders(enc_s_fulfilled, &ck);
	assert_eq!(s_fulfilled, vec![(3, 40), (3, 40), (2, 30)]);

	let decrypted = instant.elapsed();

	println!("encrypt orders: {:?}", encrypted_orders);
	println!("match volume: {:?}", volume_matched);
	println!("decryption: {:?}", decrypted);

	// In a 16-core barebone machine
	// encrypt orders: 22.824481ms
	// match volume: 28.822627314s
	// decryption: 28.823213014s

	// with cloud machine - with GPU RTX 6000 Ada
	// encrypt orders: 18.763443ms
	// match volume: 7.942746173s
	// decryption: 7.943555969s
}

#[test]
fn handle_complex_case() {
	let config = ConfigBuilder::default().build();
	let (ck, sk) = generate_keys(config);

	let max_items = 5u32;
	let ttl_buy = 10u32;
	let ttl_sell = 8u32;
	assert!(ttl_buy >= ttl_sell);

	let b_orders: Vec<(u32, u32)> = (0..ttl_buy).map(|i| (i % (max_items - 1) + 1, i % 100 + 1)).collect();
	let s_orders: Vec<(u32, u32)> = (0..ttl_sell).map(|i| (i % (max_items - 1) + 1, i % 100 + 1)).collect();

	let instant = Instant::now();

	let enc_b_orders = encrypt_orders(&b_orders, &ck);
	let enc_s_orders = encrypt_orders(&s_orders, &ck);

	let encrypted_orders = instant.elapsed();

	let expected_b_fulfilled: Vec<(u32, u32)> = [
		(0..ttl_sell).map(|i| (i % (max_items - 1) + 1, i % 100 + 1)).collect::<Vec<_>>(),
		(ttl_sell..ttl_buy).map(|i| (i % (max_items - 1) + 1, 0)).collect::<Vec<_>>(),
	]
	.concat();

	let expected_s_fulfilled = s_orders.clone();

	// server-side processing
	let dp = FheDarkPool::new(sk, max_items);
	let (enc_b_fulfilled, enc_s_fulfilled) = dp.volume_matching(enc_b_orders, enc_s_orders);

	let volume_matched = instant.elapsed();

	// client-side
	let b_fulfilled = decrypt_orders(enc_b_fulfilled, &ck);
	assert_eq!(b_fulfilled, expected_b_fulfilled);

	let s_fulfilled = decrypt_orders(enc_s_fulfilled, &ck);
	assert_eq!(s_fulfilled, expected_s_fulfilled);

	let decrypted = instant.elapsed();

	println!("encrypt orders: {:?}", encrypted_orders);
	println!("match volume: {:?}", volume_matched);
	println!("decryption: {:?}", decrypted);

	// encrypt orders: 68.483139ms
	// match volume: 106.740758245s
	// decryption: 106.74250268s
}
