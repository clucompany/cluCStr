
use syntax::source_map::Span;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
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
use syntax::ast::IntTy;
use syntax::parse::token::TokenKind;


#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_macro("cstr", cstr);
}

#[doc(hidden)]
pub fn cstr(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<dyn MacResult + 'static> {
	let mut parser = cx.new_parser_from_tts(args);
	
	let mut args_len = args.len();
	let mut array_expr = Vec::with_capacity(args_len / 2);
	if args_len > 0 {
		match parser.parse_expr() {
			Ok(a) => array_expr.push(a),
			Err(_e) => {
				cx.span_err(parser.prev_span, "incorrect data, was expected: &[u8], str, u8, i8");
				return DummyResult::any(sp);
			}
		}
		let mut count_elements = 1;
		while parser.eat(&TokenKind::Comma) {
			args_len -= 1;
			//del comma
			
			match parser.parse_expr() {
				Ok(a) => {
					count_elements += 1;
					array_expr.push(a);
				},
				Err(_e) => {
					cx.span_err(parser.prev_span, "incorrect data, was expected: &[u8], str, u8, i8");
					return DummyResult::any(sp);
				},
			}
		}
		if count_elements != args_len {
			cx.span_err(parser.prev_span, "It was expected ',' or closing of a macro.");
			return DummyResult::any(sp);
		}
	}

	let mut r_array = Vec::with_capacity(args_len * 5);
	'is_add_null: loop {
		'looper: for (unk_index, unk) in array_expr.into_iter().enumerate() {
			match unk.node {
				ExprKind::Lit(ref l) => {
					match l.node {
						LitKind::Str(ref array, _) => {
							let s_array = array.as_str();
							let array = s_array.as_bytes();
							
							for (a_index, a) in array.into_iter().enumerate() {
								match a {
									0u8 if unk_index+1 == args_len && a_index+1 == array.len() => break 'looper,
									0u8 => {
										cx.span_err(l.span, "trailing byte detected");
										return DummyResult::any(sp);
									},
									_ => {
										r_array.push(
											cx.expr_lit(sp, LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)))
										);
									},
								}
							}
						},
						LitKind::ByteStr(ref array) => {
							let array = array.as_slice();
							
							for (a_index, a) in array.into_iter().enumerate() {
								match a {
									0u8 if unk_index+1 == args_len && a_index+1 == array.len() => break 'looper,
									0u8 => {
										cx.span_err(l.span, "trailing byte detected");
										return DummyResult::any(sp);
									},
									_ => {
										r_array.push(
											cx.expr_lit(sp, LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)))
										);
									},
								}
							}
						},
						
						LitKind::Int(0, LitIntType::Unsuffixed) => {
							if unk_index+1 == args_len {
								break 'looper;
							}else {
								cx.span_err(l.span, "trailing byte detected");
								return DummyResult::any(sp);
							}
						},
						
						LitKind::Int(ref a, LitIntType::Unsigned(UintTy::U8)) 
						| LitKind::Int(ref a, LitIntType::Signed(IntTy::I8))
						=> {
							match a {
								0u128 if unk_index+1 == args_len => break 'looper,
								
								0u128 => {
									cx.span_err(l.span, "trailing byte detected");
									return DummyResult::any(sp);
								},
								_ => {
									r_array.push(
										cx.expr_lit(sp, LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)))
									);
								},
							}	
						},
						
						LitKind::Byte(ref a) => {
							match a {
								0u8 if unk_index+1 == args_len => break 'looper,
								0u8 => {
									cx.span_err(l.span, "trailing byte detected");
									return DummyResult::any(sp);
								},
								_ => {
									r_array.push(
										cx.expr_lit(sp, LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)))
									);
								},	
							}
						},
						_ => {
							cx.span_err(l.span, "incorrect data, was expected: &[u8], str, u8, i8");
							return DummyResult::any(sp);
						}
					}
				},
				_ => {
					cx.span_err(unk.span, "incorrect data, was expected: &[u8], str, u8, i8");
					return DummyResult::any(sp);
				}
			}
		}
		
		r_array.reserve(1);
		r_array.push(
			cx.expr_lit(sp, LitKind::Int(0u128, LitIntType::Unsigned(UintTy::U8)))
		);
		break 'is_add_null;
	}
	//END ARRAY
	
	let mut result = cx.expr(
		sp, 
		ExprKind::AddrOf(Mutability::Immutable,
			cx.expr(sp, ExprKind::Array(r_array)) //[u8]
			//ARRAY EXPR u8 -> EXPR [U8]
		) 
	);// & [u8]
	
	result = cx.expr_cast(
		sp, 
		result, //[u8]
		cx.ty_ptr(sp,  // -> *const [u8]
			cx.ty(sp, TyKind::Slice(
				cx.ty_ident(sp, cx.ident_of("u8"))
			)), // P<TY>
			
			Mutability::Immutable // const
		)
	);// &[u8] as *const [u8]
	
	result = cx.expr_cast(
		sp, 
		result, //[i8]
		cx.ty_ptr(sp,  // -> *const [i8]
			cx.ty_ident(sp, cx.ident_of("CStr")),
				
			Mutability::Immutable // const
		)
	); // as *const CSTR
	
	result = cx.expr_unary(sp, UnOp::Deref, result);
	//* ([u8] as *const [u8] as *const CStr)
	
	result = cx.expr(sp, ExprKind::AddrOf(Mutability::Immutable, result));
	//& *([u8] as *const [u8] as *const CStr)
	
	MacEager::expr({
		let block = P(Block {
			stmts: { //{block}
				let mut r = Vec::with_capacity(1);
				r.push(
					cx.stmt_expr(result) //ADD EXPR TO BLOCK
				);
				r
			},
			id: ast::DUMMY_NODE_ID,
			rules: BlockCheckMode::Unsafe(UnsafeSource::CompilerGenerated), //<-- UNSAFE
			span: sp, 
			
			//recovered: false,
			//FIX!, UPDATE RUST:((
		});
		cx.expr_block(block) //RESULT EXPR
	})// unsafe { &*([u8] as *const [u8] as *const CStr) }
}


