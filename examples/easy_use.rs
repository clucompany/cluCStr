
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
	let cstr = cstr!("My CSTR!");
	//&CStr
	
	assert_eq!(cstr.to_bytes(), b"My CSTR!");
	assert_eq!(cstr.to_bytes_with_nul(), b"My CSTR!\0");
}

