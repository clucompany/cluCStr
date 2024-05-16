//Copyright 2019-2024 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::needless_doctest_main)]

//#Ulin Project (17 1819) - 2024
//

/*!
Safe and efficient creation of "CStr" with zero-byte checking and support for concatenating multiple values.

## Note

You can use `c"wow"` since Rust 1.77.0 instead of `cstr!("wow")` from this crate. This new feature provides more concise code and faster compilation. If you are using an older Rust API (like 1.66), this crate will still be relevant for some time.

## Example
```rust
use cluCStr::cstr;
use core::ffi::CStr;

fn main() {
	let cstr = cstr!(b"How are you?");
	
	assert_eq!(cstr.to_bytes_with_nul(), b"How are you?\0");
}
```
*/

#![no_std]

extern crate proc_macro;
extern crate alloc;

use core::ffi::CStr;
use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::num::NonZeroU8;
use alloc::vec::Vec;
use quote::quote;
use proc_macro::TokenStream;
use proc_macro2::{TokenTree as TokenTree2, Literal, Span};

/// Returns tokens that generate a compilation error with the given message
/// in the specified source code range.
#[inline]
fn __make_pm_compile_error(span: Span, message: &str) -> TokenStream {
	TokenStream::from(quote::quote_spanned! {
		span =>
		compile_error! { #message }
	})
}

/// The macro creates a tree with a single compile_error macro and with
/// the corrected span and error message.
macro_rules! pm_compile_error {
	($span: expr, $e: expr) => {{
		return __make_pm_compile_error($span, $e);
	}};
}

/// Checks for null bytes in the data being processed and aborts with an error
/// message if one is detected.
macro_rules! thiserr_nullbyte {
	[
		$lit: ident, $e: expr $(,)?
	] => {{
		let e: Result<(), ErrDetectedNullByte> = $e; // only ErrDetectedNullByte
		
		if e.is_err() {
			pm_compile_error!($lit.span(), "Format convention error, null byte detected.");
		}
	}};
}

/// A marker that determines the presence of a zero byte (error) in the
/// formation of CSTR.
struct ErrDetectedNullByte;

/// The SafeCStrBuilder struct provides an interface for safely creating C-compatible
/// strings (strings that are terminated by a null byte).
/// It guarantees that all data is valid (does not contain any null bytes),
/// except for the trailing null byte.
struct SafeCStrBuilder(Vec<u8>);

impl SafeCStrBuilder {
	/// Creates an empty array without allocations.
	#[inline(always)]
	pub const fn empty() -> Self {
		SafeCStrBuilder(Vec::new())
	}
	
	/// Pushes a byte to the CSTR.
	///
	/// # Errors
	///
	/// Returns an error if the byte contains a null byte.
	#[inline(always)]
	pub fn push(&mut self, a: u8) -> Result<(), ErrDetectedNullByte> {
		match NonZeroU8::new(a) {
			Some(a) => {
				self.push_nonzero(a);
				
				Ok(())
			},
			None => Err(ErrDetectedNullByte)
		}
	}
	
	/// Pushes a non-zero byte to the CSTR.
	#[inline]
	pub fn push_nonzero(&mut self, a: NonZeroU8) {
		self.0.push(a.get())
	}
	
	/// Checks if the CSTR is empty.
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
	
	/// Returns a fragment of CSTR bytes.
	///
	/// (always without trailing null byte)
	#[inline]
	#[allow(dead_code)]
	pub fn as_slice(&self) -> &[u8] {
		&self.0
	}
	
	/// Extends the CSTR with a slice of bytes.
	///
	/// # Errors
	///
	/// Returns an error if the slice contains a null byte.
	pub fn extend_from_slice(&mut self, arr: &[u8]) -> Result<(), ErrDetectedNullByte> {
		match memchr::memchr(0, arr) {
			Some(..) => Err(ErrDetectedNullByte),
			None => {
				self.0.extend_from_slice(arr);

				Ok(())
			}
		}
	}
	
	/// Converts the CSTR into a COW slice of bytes.
	/// 
	/// !Note that if the string is empty, no allocation occurs and a single
	/// generic empty CSTR is returned.
	pub fn into(mut self) -> Cow<'static, [u8]> {
		match self.is_empty() {
			true => {
				/// Generic empty CSTR.
				static ECSSTR: &'static [u8] = &[0u8];
				
				Cow::Borrowed(ECSSTR)
			},
			false => {
				self.0.push(0);
				
				self.0.into()
			}
		}
	}
	
	/// Validates the SafeDataCSTR.
	///
	/// !Calls the valid function if the CSTR is valid (does not contain a null byte).
	/// !Calls the invalid function if the CSTR is invalid (contains a null byte).
	#[inline]
	pub fn validate_with_fns<R>(
		&self,
		
		valid: impl FnOnce() -> R,
		invalid: impl FnOnce(usize) -> R
	) -> R {
		match memchr::memchr(0, &self.0) {
			Some(a) => invalid(a),
			None => valid(),
		}
	}
	
	/// Checks if the CSTR is valid (does not contain a null byte).
	#[inline]
	pub fn is_valid(&self) -> bool {
		self.validate_with_fns(
			|| true, // valid
			|_| false, // invalid
		)
	}
}

/// Safe and efficient creation of “CStr” with zero-byte checking and support for concatenating multiple values.
/// 
/// ```rust
/// use cluCStr::cstr;
/// use core::ffi::CStr;
/// 
/// assert_eq!(cstr!("test").to_bytes(), b"test");
/// assert_eq!(cstr!(b"test", 1).to_bytes(), b"test1");
/// assert_eq!(cstr!("test1", "test", 2).to_bytes(), b"test1test2");
/// assert_eq!(cstr!(1u8).to_bytes(), &[1u8] as &[u8]);
/// assert_eq!(cstr!(1u8, 2u8, 3u8,).to_bytes(), &[1u8, 2, 3] as &[u8]);
/// assert_eq!(cstr!(1).to_bytes(), b"1");
/// assert_eq!(cstr!(1, 2, 3, 4, 5,).to_bytes(), b"12345");
/// ```
#[proc_macro]
pub fn cstr(token: TokenStream) -> TokenStream {
	let token = proc_macro2::TokenStream::from(token);
	
	let mut cstrline = SafeCStrBuilder::empty();
	if !token.is_empty() {
		let mut iter = token.into_iter();
		let mut tree;
		'main: loop {
			tree = iter.next();
			
			'decode: loop {
				match tree {
					Some(TokenTree2::Literal(lit)) => { // 'a', "hello", 2.3
						let data = lit.to_string();
						let bytes = data.as_bytes();
						let len = bytes.len();
						
						match len {
							0 => {}, // empty
							1 => { // 1
								let a = unsafe {
									debug_assert!({
										#[allow(clippy::get_first)]
										bytes.get(0).is_some()
									});
									
									bytes.get_unchecked(0) // safety: see len == 1 and debug_assert
								};
								
								thiserr_nullbyte!(lit, cstrline.push(*a));
							},
							len => { // 2/3/4/...
								let first = unsafe { // safety: see match len, 0/1 - ignore, 2-3-4 - current
									debug_assert!({
										#[allow(clippy::get_first)]
										bytes.get(0).is_some()
									});
									
									bytes.get_unchecked(0)
								};
								let last = unsafe { // safety: see match len, 0/1 - ignore, 2-3-4 - current
									debug_assert!(bytes.get(len-1).is_some());
									
									bytes.get_unchecked(len-1)
								};
								
								match (first, last) {
									(b'"', b'"') => { // example: "test"
										let arr = unsafe {
											debug_assert!(bytes.get(1.. len-1).is_some());
											
											bytes.get_unchecked(1.. len-1) // safety: see get and debug_assert
										};
										
										thiserr_nullbyte!(lit, cstrline.extend_from_slice(arr));
									},
									(b'b', b'"') if bytes.get(1) == Some(&b'"') => { // example: b"test"
										let arr = unsafe {
											debug_assert!(bytes.get(1+1.. len-1).is_some());
											
											bytes.get_unchecked(1+1.. len-1) // safety: see get and debug_assert
										};
										
										thiserr_nullbyte!(lit, cstrline.extend_from_slice(arr));
									},
									(b'\'', b'\'') /*if len == 3*/ => { // example: '1'
										let arr = unsafe {
											debug_assert!(bytes.get(1.. len-1).is_some());
											
											bytes.get_unchecked(1.. len-1) // safety: see get and debug_assert
										};
										
										thiserr_nullbyte!(lit, cstrline.extend_from_slice(arr));
									},
									(b'b', b'\'') if /*len == 4 &&*/ bytes.get(1) == Some(&b'\'') => { // example: b'1'
										let arr = unsafe {
											debug_assert!(bytes.get(1+1.. len-1).is_some());
											
											bytes.get_unchecked(1+1.. len-1) // safety: see len == 4
										};
										
										thiserr_nullbyte!(lit, cstrline.extend_from_slice(arr));
									},
									(_, _) if bytes.ends_with(b"u8") => { // 10u8
										let bytes = unsafe {
											debug_assert!(bytes.get(.. len-b"u8".len()).is_some());
											
											bytes.get_unchecked(.. len-b"u8".len()) // safety: see end_with + debug_assert
										};
										
										let num: u8 = match String::from_utf8_lossy(bytes).parse() {
											Ok(a) => a,
											Err(..) => {
												pm_compile_error!(lit.span(), "Input Error");
											}
										};
										thiserr_nullbyte!(lit, cstrline.push(num));
									},
									(_, _) if bytes.ends_with(b"i8") => { // 10i8
										let bytes = unsafe {
											debug_assert!(bytes.get(.. len-b"i8".len()).is_some());
											
											bytes.get_unchecked(.. len-b"i8".len()) // safety: see end_with + debug_assert
										};
										
										let num: i8 = match String::from_utf8_lossy(bytes).parse() {
											Ok(a) => a,
											Err(..) => {
												pm_compile_error!(lit.span(), "Input Error");
											}
										};
										thiserr_nullbyte!(lit, cstrline.push(num as _));
									},
									(_, _) => { // len always >1!
										thiserr_nullbyte!(lit, cstrline.extend_from_slice(bytes));
									},
								}
							}
						}
						
						// Support for empty trailing comma.
						// example: (test,)
						//
						let mut is_en_fatalblock = true;
						'cparse: loop {
							tree = iter.next();
							match tree {
								None => {
									break 'main;
								},
								Some(TokenTree2::Punct(punct)) if ',' == punct.as_char() => {
									if !is_en_fatalblock {
										pm_compile_error!(punct.span(), "Unsupported.")
									}
									
									is_en_fatalblock = false;
									continue 'cparse;
								},
								
								Some(..) if !is_en_fatalblock => {
									continue 'decode;
								},
								Some(a_tree) => {
									pm_compile_error!(a_tree.span(), "It was expected ',' or closing of a macro.")
								},
							}
						}
					},
					Some(tk) => {
						pm_compile_error!(tk.span(), "incorrect data, was expected: &[u8], str, u8, i8, {integer}.");
					},
					None => {
						break 'main;
					},
				}
				
				#[allow(unreachable_code)] {
					break 'decode;
				}
			}
		}
	}
	
	debug_assert!(cstrline.is_valid()); // debug internal check
	let cstrline = cstrline.into();
	let arr = &cstrline as &[u8];
	debug_assert!( // debug internal check
		CStr::from_bytes_with_nul(arr).is_ok()
	);
	let result = Literal::byte_string(arr);
	let token = quote! {
		{
			const _H: &'static CStr = unsafe {
				&*(#result /* b"lit_array" */ as *const [u8] as *const CStr) as &'static CStr
			};
			
			_H
		}
	};
	
	TokenStream::from(token)
}