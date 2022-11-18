use coora_bindings::coora_plugin;
use coora_engine::{WasmEngine, WasmInstanceBuilder};
use sweet::*;


#[coora_plugin]
pub trait MathPlugin {
	fn add(&self, a: i32, b: i32) -> i32;
	// fn scale(a: i32);
	fn foo(&self);
}

pub struct MyMathImpl;
impl MathPlugin for MyMathImpl {
	fn add(&self, a: i32, _b: i32) -> i32 { a }
	fn foo(&self) {}
}

sweet! {
	it "works" {
		let mut engine = WasmEngine::new();
		let mut _builder = WasmInstanceBuilder::new(&mut engine, 0);

		// let plugin = MyMathImpl;

		// builder.

		// let a = Arc::new(Mutex::new(2));

		// let a1 = Arc::clone(&a);
		// type StoreT = u32;

	}
}
