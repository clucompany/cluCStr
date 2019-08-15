
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

fn main() {
	println!("");
	println!("Enter it in the terminal !!");
	println!();
	println!("=====================");
	println!("cargo bench --example bench");
	println!("=====================");
}

/*
   Compiling clucstr v0.1.7 (/home/pk)
    Finished release [optimized] target(s) in 0.40s
     Running target/release/examples/bench-99f2dfd5339276ba

running 2 tests
test tests::cstr_macros ... bench:          31 ns/iter (+/- 0)
test tests::cstr_plugin ... bench:           0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
*/

