use cluCStr::cstr;
use core::ffi::CStr;

#[test]
fn cstr() {
	assert_eq!(cstr!("test").to_bytes(), b"test");
	assert_eq!(cstr!("test", 1).to_bytes(), b"test1");
	assert_eq!(cstr!("test1", "test", 2).to_bytes(), b"test1test2");
}

#[test]
fn cstr_u8() {
	assert_eq!(cstr!(1u8).to_bytes(), &[1u8] as &[u8]);
	assert_eq!(cstr!(1u8, 2u8, 3u8,).to_bytes(), &[1u8, 2, 3] as &[u8]);
}

#[test]
fn cstr_litu8() {
	assert_eq!(cstr!(1).to_bytes(), b"1");
	assert_eq!(cstr!(1, 2, 3, 4, 5,).to_bytes(), b"12345");
}
