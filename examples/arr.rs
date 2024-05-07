use cluCStr::cstr;
use core::ffi::CStr;

fn main() {
	let cstr = cstr!(b"Test Custom cstr");
	
	assert_eq!(cstr.to_bytes_with_nul(), b"Test Custom cstr\0");
}
