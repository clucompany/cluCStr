
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

///Safe creation of CStr with zero cost
//It is required for documentation!
#[macro_export]
macro_rules! cstr {
	($s:expr) => {};
}
//It is required for documentation!


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
			cx.span_err(sp, "cstr only supports now one parameter");
			return DummyResult::any(sp);
		}
	};
	
	let c_array = {	
		let mut add_null = true;
		
		let mut array = match expr.node {
			ExprKind::Lit(ref l) => {
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
										cx.span_err(sp, "in line not admissible characters are found.");
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
										cx.span_err(sp, "in line not admissible characters are found.");
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
						cx.span_err(sp, "not incorrect data are transferred to a macro");
						return DummyResult::any(sp);
					}
					
				}
			},
			_ => {
				cx.span_err(sp, "not incorrect data are transferred to a macro");
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
