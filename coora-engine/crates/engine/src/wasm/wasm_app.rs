use crate::{include_wasm, Plugin};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use wasmi::*;
pub type SharedStore<T> = Arc<Mutex<Store<T>>>;
use super::*;
pub struct WasmApp<T> {
	pub store: SharedStore<T>,
	pub linker: Linker<T>,
	pub instance: Option<Instance>,
	pub module: Option<Module>,
	pub engine: Engine,
	pub memory: SharedMemory<T>,
}

impl<T> WasmApp<T> {
	pub fn recycle(self, initial_state: T) -> WasmApp<T> {
		WasmApp::new_with_engine(self.engine, initial_state)
	}

	pub fn new(initial_state: T) -> WasmApp<T> {
		Self::new_with_engine(WasmApp::<T>::default_engine(), initial_state)
	}
	pub fn new_with_engine(engine: Engine, initial_state: T) -> WasmApp<T> {
		let store = Store::new(&engine, initial_state);
		let mut linker = <Linker<T>>::new();
		let store = Arc::new(Mutex::new(store));
		let memory = WasmMem::<T>::new(&store, &mut linker);

		WasmApp {
			engine,
			store,
			linker,
			memory,
			instance: None,
			module: None,
		}
	}

	pub fn add_plugin<PluginT>(&mut self, plugin: &mut PluginT) -> Result<&mut Self>
	where
		PluginT: Plugin,
	{
		plugin.bind(self)?;
		Ok(self)
	}

	pub fn build(&mut self) -> &mut Self { self.build_with_wasm(WasmApp::<T>::default_wasm()) }

	pub fn build_with_wasm(&mut self, stream: impl Read) -> &mut Self {
		let module = Module::new(&self.engine, stream).unwrap();
		let store = Arc::clone(&self.store);
		let mut store = store.lock().unwrap();
		let instance = self
			.linker
			.instantiate(&mut *store, &module)
			.unwrap()
			.start(&mut *store)
			.unwrap();
		self.instance = Some(instance);
		self.module = Some(module);
		self
	}
	pub fn default_wasm() -> &'static [u8] { include_wasm!("../../../", "hello_led") }

	pub fn default_engine() -> Engine {
		//https://github.com/barafael/wasm-on-mcu/blob/5303133d1c8b96d64452675ee486b05f26dc6e03/src/bin/wasmi.rs#L43
		//https://github.com/rustwasm/wasm-pack/issues/479
		//IMPORTANT - also set stack size compiler flag in .cargo/config.toml
		let config = Config::default();
		// config
		// config.set_stack_limits(StackLimits::new(256, 512, 128).unwrap());
		// config.wasm_features().bulk_memory = true;
		Engine::new(&config)
	}
}
