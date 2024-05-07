use cluCStr::cstr;
use core::ffi::CStr;

fn main() {
	let cstr = cstr!(b"test1", "test2", 0);

	assert_eq!(cstr.to_bytes_with_nul(), b"test1test20\0");
}
