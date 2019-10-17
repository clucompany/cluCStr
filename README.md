# clucstr
[![Build Status](https://travis-ci.org/clucompany/cluCStr.svg?branch=master)](https://travis-ci.org/clucompany/cluCStr)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/clucstr)](https://crates.io/crates/clucstr)
[![Documentation](https://docs.rs/clucstr/badge.svg)](https://docs.rs/clucstr)

Safe creation of “CStr” with zero cost at the compilation stage with checking for zero bytes and the ability to transfer multiple values.

# Features
1. Creation of safe CStr at a compilation stage.
2. Check of zero bytes at a stage of compilation or checks of "Rls or Rust check".
3. Concatenation of several values, different types: [u8], & 'static str, u8, i8, (0 without specifying the type).
4. All actions happen at a compilation stage, processor time is not required.

# Use
```rust
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
```

# EasyUse

```rust
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
	let cstr = cstr!("My CSTR!");
	//&CStr
	
	assert_eq!(cstr.to_bytes(), b"My CSTR!");
	assert_eq!(cstr.to_bytes_with_nul(), b"My CSTR!\0");
}
```

# Panic
```rust
#![feature(plugin)]
#![plugin(clucstr)]

#[allow(unused_imports)]
use std::ffi::CStr;

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
```

# Benchmarking

```rust
#![feature(test)]

#![feature(plugin)]
#![plugin(clucstr)]


#[cfg(test)]
mod tests {
	use super::*;
	use tests::test::Bencher;
	use std::ffi::CStr;
	
	extern crate test;
	
	
	
	macro_rules! unsafe_cstr {
		($s:expr) => {
			unsafe {
		   		::std::ffi::CStr::from_ptr(
					concat!($s, "\0") 
						as *const str  
						as *const [::std::os::raw::c_char] 
						as *const ::std::os::raw::c_char
				)
			}
		};
	}
	
	#[bench]
	fn cstr_plugin(b: &mut Bencher) {
		b.iter(|| {
			for _a in 0..10 {
				let _cstr0 = cstr!(b"test");
			}
		});
	}
	#[bench]
	fn cstr_macros(b: &mut Bencher) {
		b.iter(|| {
			for _a in 0..10 {
				let _cstr0 = unsafe_cstr!("test");
			}
		});
	}
}
``` 
running 2 tests

test tests::cstr_macros ... bench:		  67 ns/iter (+/- 1) !Attention ns > 0, full unsafe, no guarantees

test tests::cstr_plugin ... bench:		   0 ns/iter (+/- 0) !Attention ns == 0, plus zero byte checking and plus concatenation

# Launch benchmark: 

cargo bench --example bench

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
