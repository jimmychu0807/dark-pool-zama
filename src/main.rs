use tfhe::{ ConfigBuilder, generate_keys, set_server_key, FheUint8 };
use tfhe::prelude::*;

fn main() {
	let config = ConfigBuilder::default().build();

	// client-side
	let (client_key, server_key) = generate_keys(config);

	let clear_a = 27u8;
	let clear_b = 128u8;

	let enc_a = FheUint8::encrypt(clear_a, &client_key);
	let enc_b = FheUint8::encrypt(clear_b, &client_key);

	// server side
	set_server_key(server_key);
	let enc_res = enc_a + enc_b;

	// client-side
	let res: u8 = enc_res.decrypt(&client_key);

	assert_eq!(res, clear_a + clear_b);
}
