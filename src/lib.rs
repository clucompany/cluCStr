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
Secure creation of CStr with zero cost. Plugin for rust compiler.

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
#![feature(plugin_registrar, rustc_private)]
#![feature(i128_type)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::ext::quote::rt::Span;
use rustc_plugin::Registry;
use syntax::ast::ExprKind;
use syntax::ast::LitKind;
use syntax::ast::Mutability;
use syntax::ast::LitIntType;
use syntax::ast::TyKind;
use syntax::ast::UintTy;
use syntax::ast::UnOp;
use syntax::ast::BlockCheckMode;
use syntax::ast::UnsafeSource;
use syntax::ast::Block;
use syntax::ptr::P;
use syntax::ast;

///Secure creation of CStr with zero cost
//Required for docks!!!!!
#[macro_export]
macro_rules! cstr {
	($s:expr) => {};
}
//Required for docks!!!!!


#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_macro("cstr", cstr);
}

#[doc(hidden)]
pub fn cstr(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
	let mut parser = cx.new_parser_from_tts(args);

	let expr = match parser.parse_expr() {
		Ok(a) => a,
		Err(_e) => {
			cx.span_err(sp, "cstr currently only supports one argument");
			return DummyResult::any(sp);
		}
	};
	
	let c_array = {	
		let mut add_null = true;
		
		let mut array = match expr.node {
			ExprKind::Lit(ref l) => {
				//println!("{:?}", l.node);
				match l.node {
					LitKind::Str(s, _) => {
						let s = s.to_string();
						let array = s.as_bytes();
						let array_len = array.len();
						
						let mut vec_result = Vec::with_capacity(array_len);
						
						for (num, a) in array.iter().enumerate() {
							if *a == 0 {
								match num+1 == array_len {
									true => add_null = false,
									_ => {
										cx.span_err(sp, "the string has a null byte");
										return DummyResult::any(sp);
									},
								}
							}
							vec_result.push(
								cx.expr_lit(sp,
									LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)),
								)
							);
						}
						vec_result
					},
					LitKind::ByteStr(ref array) => {
						let array_len = array.len();
						let mut vec_result = Vec::with_capacity(array.len());
						
						for (num, a) in array.iter().enumerate() {
							if *a == 0 {
								match num+1 == array_len {
									true => add_null = false,
									_ => {
										cx.span_err(sp, "the string has a null byte");
										return DummyResult::any(sp);
									},
								}
							}
							vec_result.push(
								cx.expr_lit(sp,
									LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)),
								)
							);
						}
						vec_result
					},
					LitKind::Byte(ref a) => {
						if *a == 0 {
							add_null = false;
						}
						vec!(
							cx.expr_lit(sp,
								LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)),
							)
						)
					}
					_ => {
						cx.span_err(sp, "argument should be a single identifier");
						return DummyResult::any(sp);
					}
					
				}
			},
			_ => {
				cx.span_err(sp, "argument should be a single identifier");
				return DummyResult::any(sp);
			}
		};
		
		if add_null {
			array.reserve(1);
			array.push(
				cx.expr_lit(sp,
					LitKind::Int(0, LitIntType::Unsigned(UintTy::U8)),
				)
			);
		}
		
		cx.expr(
			sp, 
			ExprKind::AddrOf(Mutability::Immutable,
				cx.expr(sp, ExprKind::Array(array)) //[u8]
			) // &[u8]
		)
	};
	
	let c_array = cx.expr_cast(
		sp, 
		c_array, //[u8]
		cx.ty_ptr(sp,  // -> *const [u8]
			cx.ty(sp, TyKind::Slice(
				cx.ty_ident(sp, cx.ident_of("u8"))
			)), // P<TY>
			
			Mutability::Immutable // const
		)
	);// &[u8] as *const [u8]
	
	let c_cstr = cx.expr_cast(
		sp, 
		c_array, //[i8]
		cx.ty_ptr(sp,  // -> *const [i8]
			cx.ty_ident(sp, cx.ident_of("CStr")),
			
			Mutability::Immutable // const
		)
	); // as *const CSTR
	
	let c_cstr = cx.expr_unary(sp, UnOp::Deref, c_cstr);
	//*([u8] as *const [u8] as *const CStr)
	
	let c_cstr = cx.expr(sp, ExprKind::AddrOf(Mutability::Immutable, c_cstr));
	//&*([u8] as *const [u8] as *const CStr)
	
	let block = {
		let vec_stmts = vec!(
			cx.stmt_expr(c_cstr)
		);
		let mut block = P(Block {
			stmts: vec_stmts,
			id: ast::DUMMY_NODE_ID,
			rules: BlockCheckMode::Unsafe(UnsafeSource::CompilerGenerated),
			span: sp, 
			recovered: false,
			
		});
		cx.expr_block(block)
	};// unsafe { &*([u8] as *const [u8] as *const CStr) }
	
	
	MacEager::expr(
		block
	)
}




