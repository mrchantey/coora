// #![feature(generic_arg_infer)]
pub use coora_bindings::*;
pub use wasmi;

mod wasm;
pub use wasm::*;
mod exports;
pub use exports::*;
mod imports;
pub use imports::*;
mod include_wasm;
pub use include_wasm::*;
mod factories;
pub use factories::*;

//NaN preserving, required for imports & exports
pub use wasmi::core::{F32, F64};
