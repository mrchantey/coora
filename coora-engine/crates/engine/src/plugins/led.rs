use crate::*;
use std::sync::{Arc, Mutex};

pub type SharedLeds = Arc<Mutex<dyn LedStrip + Send>>;
// pub type RGBA8 = RGBA<u8>;

pub trait LedStrip {
	fn as_shared(self) -> SharedLeds;

	fn set_leds(&mut self, r: u8, g: u8, b: u8, w: u8);
	fn show(&mut self);
}

pub struct Led;
impl Led {
	pub fn append_imports(builder: &mut SketchBuilder, leds: &SharedLeds) {
		let leds1 = Arc::clone(&leds);
		builder.add_import_4(
			"led",
			"setAll",
			move |_, r: u32, g: u32, b: u32, w: u32| {
				leds1
					.lock()
					.unwrap()
					.set_leds(r as u8, g as u8, b as u8, w as u8);
			},
		);
		let leds2 = Arc::clone(&leds);
		builder.add_import_0("led", "show", move |_| {
			leds2.lock().unwrap().show();
		});
	}
}
