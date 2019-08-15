
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
	let c_str = cstr!("cluWorld");
	//"cluWorld" <-- [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	
	let c_str2 = cstr!("cluWorld\0");
	//"cluWorld" <-- [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	
	let c_str3 = cstr!("clu", b"World");
	//"cluWorld" <-- [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	
	let c_str4 = cstr!(
		b'c', b'l', b'u',
		b'W', b'o', b'r', b'l', b'd',
		0
	);
	//"cluWorld" <-- [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	
	let c_str5 = cstr!(
		"clu",
		//It is possible to insert such values as: [u8], & 'static str, u8, i8, (0 without specifying the type).
		
		b'W', b'o', b'r', b'l', b"d\0"
		//The zero byte is automatically added, it is possible to write it, and it is possible not to write.
		//It is forbidden to insert zero byte in the middle or at the beginning of a line.
	);
	//"cluWorld" <-- [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	
	my_function(c_str);
	my_function(c_str2);
	my_function(c_str3);
	my_function(c_str4);
	my_function(c_str5);
}

fn my_function(a:  &'static CStr) {
	//'static --> it is possible not to write.
	
	let c_arr = a.to_bytes_with_nul();
	println!("{:?} <-- array: {:?}, len: {}", a, c_arr, c_arr.len());
}