use cluCStr::cstr;
use core::ffi::CStr;

fn main() {
	let cstr = cstr!(b'A', b'B', b'C', 'D', 12);

	assert_eq!(cstr.to_bytes_with_nul(), b"ABCD12\0");
}
