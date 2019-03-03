

//RUN cargo bench --example bench
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