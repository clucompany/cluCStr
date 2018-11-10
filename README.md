# clucstr
[![Build Status](https://travis-ci.org/clucompany/cluCStr.svg?branch=master)](https://travis-ci.org/clucompany/cluCStr)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/clucstr)](https://crates.io/crates/clucstr)
[![Documentation](https://docs.rs/clucstr/badge.svg)](https://docs.rs/clucstr)

Creation of strings C with zero cost. A plug-in for the rust compiler.

# Features
1. The transparent creation of the C strings with zero cost.
2. Check of the C lines at the level of the compiler.
3. Convenient macro for creation of lines.
4. Plug-in for the compiler. 


# Use
```
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
	let c_str = cstr!("cluWorld!!!");
	let c_str_barr = cstr!(b"cluWorld!!!");
	let c_str_b = cstr!(b'A');
}
```

```
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
    println_str(cstr!("cluWorld!!!"));
    //CSTR "cluWorld!!!"
    //CArray [99, 108, 117, 87, 111, 114, 108, 100, 33, 33, 33, 0] 12
    
    println_str(cstr!(b"cluWorld!!!"));
    //CSTR "cluWorld!!!"
    //CArray [99, 108, 117, 87, 111, 114, 108, 100, 33, 33, 33, 0] 12
    
    println_str(cstr!(b'A'));
    //CSTR "A"
    //CArray [65, 0] 2
}


fn println_str(cstr: &CStr) {
    println!("CSTR {:?}", cstr);
    
    let cstr_array = cstr.to_bytes_with_nul();
    println!("CArray {:?} {}", cstr_array, cstr_array.len());
    println!();
}
```

# Panic
```
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
	let c_str = cstr!("\0Test_str"); 
	// PANIC! A null byte was found.
	
	let c_str = cstr!(b"\0Test_array"); 
	// PANIC! A null byte was found.
	
	let c_str = cstr!("Test_str\0"); 
	//It is allowed to write since the null byte is at the end.
	
	let c_str = cstr!(b"Test_str\0"); 
	//It is allowed to write since the null byte is at the end.
}
```

# Benchmarking
cstr_macros - old method of converting strings to cstr. Note that in CStr, there is no protection from null bytes.

cstr_plugin - new method for converting strings to cstr.
```
#![feature(plugin)]
#![plugin(clucstr)]
#![feature(test)]

extern crate test;
use std::ffi::CStr;


#[macro_export]
macro_rules! cstr_macro {
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


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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
			let _cstr0 = cstr_macro!("test");
		}
	});
    }
}
``` 
running 2 tests

test tests::cstr_macros ... bench:          90 ns/iter (+/- 14)

test tests::cstr_plugin ... bench:           0 ns/iter (+/- 0)


# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
