use crate::{PlainDarkPool, traits::DarkPool};

#[test]
fn handle_empty_buy_order() {
	let b_orders = vec![];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];

	// server-side processing
	let dp = PlainDarkPool::new();
	let (b_fulfilled, s_fulfilled) = dp.volume_matching(b_orders, s_orders);

	// client-side
	assert_eq!(b_fulfilled, vec![]);
	assert_eq!(s_fulfilled, vec![(3, 0), (3, 0), (2, 0)]);
}

#[test]
fn handle_simple_case() {
	let b_orders = vec![(1, 55), (2, 100), (3, 80)];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];

	// server-side processing
	let dp = PlainDarkPool::new();
	let (b_fulfilled, s_fulfilled) = dp.volume_matching(b_orders, s_orders);

	// client-side
	assert_eq!(b_fulfilled, vec![(1, 0), (2, 30), (3, 80)]);
	assert_eq!(s_fulfilled, vec![(3, 40), (3, 40), (2, 30)]);
}

#[test]
fn handle_complex_case() {
	let b_orders: Vec<(u32, u32)> = (0..500).map(|i| (i % 50 + 1, i % 100 + 1)).collect();

	let s_orders: Vec<(u32, u32)> = (0..400).map(|i| (i % 50 + 1, i % 100 + 1)).collect();

	let expected_b_fulfilled: Vec<(u32, u32)> = [
		(0..400).map(|i| (i % 50 + 1, i % 100 + 1)).collect::<Vec<_>>(),
		(400..500).map(|i| (i % 50 + 1, 0)).collect::<Vec<_>>(),
	]
	.concat();

	let expected_s_fulfilled = s_orders.clone();

	// server-side processing
	let dp = PlainDarkPool::new();
	let (b_fulfilled, s_fulfilled) = dp.volume_matching(b_orders, s_orders);

	// client-side
	assert_eq!(b_fulfilled, expected_b_fulfilled);
	assert_eq!(s_fulfilled, expected_s_fulfilled);
}
