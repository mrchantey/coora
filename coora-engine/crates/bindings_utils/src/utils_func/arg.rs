// use anyhow::Ok;
use quote::*;
use syn::*;

#[derive(Clone)]
pub struct ReferenceArg {
	pub name: Ident,
	pub ty: Ident,
	pub name_ptr: Ident,
	pub name_len: Ident,
	pub index: usize,
}

#[derive(Clone)]
pub struct PrimitiveArg {
	pub name: Ident,
	pub ty: Ident,
	pub index: usize,
}
#[derive(Clone)]
pub struct Arg {
	pub index: usize,
	pub name: Ident,
	pub pat_ty: PatType,
	pub ty: Ident,
	// pub is_reference: bool,
}