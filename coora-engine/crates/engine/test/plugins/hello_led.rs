use crate::{plugins::TerminalLeds, *};
use coora_engine::*;

sweet! {
	before {

		let mut leds = TerminalLeds::new(2).as_shared();
		let mut time = StdTime::new().as_shared();
		let mut console = StdDebug::default().as_shared();
		let mut app = WasmApp::new();
		app
		.add_plugin(&mut leds)?
		.add_plugin(&mut time)?
		.add_plugin(&mut console)?
		.build()?;

		let mut sketch = SketchInstance::new(&mut app);
	}

	test "ping" {
		let a = sketch.ping();
		forky_core::utility::sleep(1);
		let b = sketch.ping();
		expect((b - a) as i32).to_be_at_least(1000)?;
	}



	test "leds"{
		sketch.update();
	}

}
