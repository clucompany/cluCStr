
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;

fn main() {
	let c_str = cstr!(12u8); 
	
     my_function(c_str);
}

fn my_function<A: AsRef<CStr>>(a: A) {
     println!("{:?}", a.as_ref());
}