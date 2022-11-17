use std::fmt::format;

use crate::*;
// use anyhow::anyhow;
use proc_macro2::{Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
	spanned::Spanned,
	token::{RArrow, Trait},
	Error, FnArg, ItemTrait, Pat, ReturnType, Token, TraitItem, Type,
};


pub fn rust_pat_to_str(pat: &Pat) -> Result<String> {
	if let Pat::Ident(pat) = pat {
		Ok(pat.ident.to_string())
	} else {
		Err(Error::new(pat.span(), "Expected identifier"))
	}
}

pub fn rust_type_to_ts(rtype: &Type) -> Result<String> {
	if let Type::Path(rt) = rtype {
		Ok(rt.path.segments.first().unwrap().ident.to_string())
	} else {
		Err(Error::new(
			rtype.span(),
			"Currently only primitives are allowd",
		))
	}
}

pub fn rust_method_to_ts(item: &TraitItem) -> Result<String> {
	if let TraitItem::Method(item) = item {
		// let return =
		// item.sig.output

		let return_type =
			if let ReturnType::Type(rarrow, rtype) = &item.sig.output {
				rust_type_to_ts(&**rtype)
			} else {
				Ok(format!("void"))
			}?;

		let args = item.sig.inputs.iter();
		for arg in args {}


		#[rustfmt::skip]
		let args:std::result::Result<Vec<_>, _> = item.sig.inputs.iter().map(|item| {
			if let FnArg::Typed(item) = item {
				// item.attrs[0].
				let ty = rust_type_to_ts(&*item.ty)?;
				let ident = rust_pat_to_str(&*item.pat)?;
				return Ok(format!("{}: {}",ident,ty))
			} else {
				Err(Error::new(item.span(), "'self' is not a valid argument"))
			}
		}).collect();
		let args = args?.join(", ");

		Ok(format!(
			"\nexport declare function {}({}): {};",
			item.sig.ident, args, return_type
		))
	} else {
		Err(Error::new(
			item.span(),
			"Currently only functions can be used as plugin imports",
		))
	}
}


const PREFIX: &str = r"//auto generated bindings";
const SUFFIX: &str = r"";

pub fn generate_typescript_bindings(plugin: &ItemTrait) -> Result<String> {
	let mut out = String::from(PREFIX);
	let body = plugin.items.iter();
	for item in body {
		let ts_func = rust_method_to_ts(item)?;
		out.push_str(ts_func.as_str());
	}


	out.push_str(SUFFIX);
	Ok(out)
}