
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
	let cstr_1 = cstr!("cluWorld");
	assert_eq!(cstr_1.to_bytes_with_nul(), b"cluWorld\0");
	//"cluWorld", [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	
	let cstr_2 = cstr!("cluWorld\0");
	//"cluWorld", [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	assert_eq!(cstr_2.to_bytes_with_nul(), b"cluWorld\0");
	
	let cstr_3 = cstr!("clu", b"World");
	//"cluWorld", [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	assert_eq!(cstr_3.to_bytes_with_nul(), b"cluWorld\0");
	
	let cstr_4 = cstr!(
		b'c', b'l', b'u',
		b'W', b'o', b'r', b'l', b'd',
		0
	);
	//"cluWorld", [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	assert_eq!(cstr_4.to_bytes_with_nul(), b"cluWorld\0");
	
	let cstr_5 = cstr!(
		"clu",
		//It is possible to insert such values as: [u8], & 'static str, u8, i8, (0 without specifying the type).
		
		b'W', b'o', b'r', b'l', b"d\0"
		//The zero byte is automatically added, it is possible to write it, and it is possible not to write.
		//It is forbidden to insert zero byte in the middle or at the beginning of a line.
	);
	//"cluWorld", [99, 108, 117, 87, 111, 114, 108, 100, 0], len:9
	assert_eq!(cstr_5.to_bytes_with_nul(), b"cluWorld\0");
	
	my_function(1, cstr_1);
	my_function(2, cstr_2);
	my_function(3, cstr_3);
	my_function(4, cstr_4);
	my_function(5, cstr_5);
}

fn my_function(num: usize, a: &'static CStr) {
	//'static --> it is possible not to write.
	
	let c_arr = a.to_bytes_with_nul();
	println!("#cstr_{} {:?}, array: {:?}, len: {}", num, a, c_arr, c_arr.len());
}