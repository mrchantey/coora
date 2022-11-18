use wasmi::*;

pub struct WasmEngine {
	pub engine: Engine,
}

impl WasmEngine {
	pub fn new() -> WasmEngine {
		//https://github.com/barafael/wasm-on-mcu/blob/5303133d1c8b96d64452675ee486b05f26dc6e03/src/bin/wasmi.rs#L43
		//https://github.com/rustwasm/wasm-pack/issues/479
		//IMPORTANT - also set stack size compiler flag in .cargo/config.toml
		let mut config = Config::default();
		config.set_stack_limits(StackLimits::new(256, 512, 128).unwrap());

		WasmEngine {
			// engine: Engine::new(&config),
			engine: Engine::default(),
		}
	}

	// pub fn instantiate<T>(
	// 	&mut self,
	// 	initial_state: T,
	// ) -> WasmInstanceBuilder<T> {
	// 	WasmInstanceBuilder::new(self, initial_state)
	// }
}
