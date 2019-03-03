
#![feature(plugin)]
#![plugin(clucstr)]

#[allow(unused_imports)]
use std::ffi::CStr;

//ATTENTION!!!
//
//For display of panic 'RLS or Rust Check' it is required to uncomment functions.

fn main() {
	//let c_str = cstr!("cluW\0orld");
	//PANIC! trailing byte detected
	
	//let c_str2 = cstr!("cluWorld\0\0");
	//PANIC! trailing byte detected
	
	//let c_str3 = cstr!("\0clu", b"W\0orld");
	//PANIC! trailing byte detected
	
	/*let c_str4 = cstr!(
		b'c', b'l', b'u', 0u8,
		b'W', b'o', b'r', b'l', b'd',
		0
	);*/
	//PANIC! trailing byte detected
}