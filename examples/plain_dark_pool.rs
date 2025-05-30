use dark_pool::{PlainDarkPool, traits::DarkPool};

fn main() {
	// generate the data - small case
	let b_orders = vec![(1, 55), (2, 100), (3, 80)];
	let s_orders = vec![(3, 40), (3, 120), (2, 30)];

	// server-side processing
	let dp = PlainDarkPool::new();
	let (b_fulfilled, s_fulfilled) = dp.volume_matching(b_orders, s_orders);

	// client-side
	assert_eq!(b_fulfilled, vec![(1, 0), (2, 30), (3, 80)]);
	assert_eq!(s_fulfilled, vec![(3, 40), (3, 40), (2, 30)]);
}
