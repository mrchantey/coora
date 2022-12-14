use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{ItemTrait, LitStr};



pub fn generate_rust_bindings(plugin: &ItemTrait) -> LitStr {
	let name = &plugin.ident;

	let name_str = name.to_string();
	let body = plugin.items.iter();
	let mut stream = TokenStream::new();
	stream.append_all(body);
	
	let out = quote! {
		#[link(wasm_import_module = #name_str)]
		extern "C" {
			#stream
			}
	}
	.to_string();
	let out = String::from("//AUTOGENERATED\n") + out.as_str();
	LitStr::new(out.as_str(), Span::call_site())
}
