//Copyright 2018 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//       http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1718
//

/*!
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
*/


#![feature(plugin_registrar)]
#![feature(rustc_private)]


extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;


mod nightly;
pub use self::nightly::*;
