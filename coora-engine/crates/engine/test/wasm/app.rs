use crate::{plugins::TerminalLeds, *};
use coora_engine::*;

sweet! {

	let mut leds = TerminalLeds::new(2).as_shared();
	let mut time = StdTime::new().as_shared();
	let mut app = WasmApp::new(0);
	app
		.add_plugin(&mut leds).unwrap()
		.add_plugin(&mut time).unwrap()
		.build();

		let mut sketch = SketchInstance::new(&mut app);

		test "recycle" {
		let mut app = app.recycle(0);
			app
				.add_plugin(&mut leds).unwrap()
				.add_plugin(&mut time).unwrap()
				.build();
	}

}
