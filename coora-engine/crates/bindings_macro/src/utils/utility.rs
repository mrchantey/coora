use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
	parse::Result, spanned::Spanned, Error, FnArg, Pat, PatIdent, PathSegment, ReturnType, Type,
};


pub fn fn_result_to_typed(item: &ReturnType) -> Result<TokenStream> {
	match &item {
		ReturnType::Type(_rarrow, rtype) => {
			if let Type::Path(rtype) = &**rtype {
				let ident = rtype.path.segments.first().unwrap().clone().ident;
				Ok(quote!(#ident))
			} else {
				// Err(Error::new(item.span(), "Hmmm whaaat."))
				Ok(quote!(()))
			}
		}
		ReturnType::Default => Ok(quote!(())),
	}
}

pub fn fn_arg_to_typed(item: &FnArg) -> Result<(PatIdent, PathSegment)> {
	if let FnArg::Typed(item) = item {
		let ty = if let Type::Path(rt) = &*item.ty {
			let first = rt.path.segments.first();
			if let Some(first) = first {
				Ok(first)
			} else {
				Err(Error::new(item.pat.span(), "Not sure how we got here.."))
			}
		} else {
			Err(Error::new(
				item.pat.span(),
				"Currently only primitives are allowed, ie u32",
			))
		}?;
		let ident = if let Pat::Ident(ident) = &*item.pat {
			Ok(ident)
		} else {
			Err(Error::new(
				item.pat.span(),
				"Expected identifier, ie my_var",
			))
		}?;
		Ok((ident.clone(), ty.clone()))
	} else {
		// Ok(Ident::new("caching", item.span()))
		Err(Error::new(item.span(), "'self' is not a valid argument"))
	}
}
