//https://github.com/ferrous-systems/espressif-trainings/blob/main/intro/http-server/src/main.rs
//https://github.com/esp-rs/esp-idf-svc
use anyhow::Result;
use core::str;
use std::{
	sync::{Arc, Mutex},
	thread::sleep,
	time::Duration,
};

use embedded_svc::{
	http::server::{registry::Registry, Request, Response},
	io::Read,
	io::Write,
};
use esp_idf_svc::http::server::{Configuration, EspHttpRequest, EspHttpServer};


pub fn start_server() -> Result<EspHttpServer> {
	let server_config = Configuration::default();
	let mut server = EspHttpServer::new(&server_config)?;
	server.handle_get("/", |_request, response| {
		let html = templated("welcome");
		let mut writer = response.into_writer()?;
		writer.write_all(html.as_bytes())?;
		Ok(())
	})?;

	server.handle_get("/nicha", move |_request, response| {
		let html = templated("❤️❤️❤️HELLO FROM NICHA!❤️❤️❤️");
		let mut writer = response.into_writer()?;
		writer.write_all(html.as_bytes())?;
		Ok(())
	})?;
	server.handle_post("/data", move |mut request, response| {
		let mut reader = request.reader();
		let mut buff: [u8; 1024] = [0; 1024];
		let len = reader.read(&mut buff)?;
		println!("\nbytes received!");
		println!("bytes: {:?}", &buff[0..len]);
		// for i in 0..len {
		// 	println!("byte: {}", buff[i]);
		// }
		let mut writer = response.into_writer()?;
		writer.write_all(b"ok")?;
		Ok(())
	})?;

	Ok(server)
}

fn templated(content: impl AsRef<str>) -> String {
	format!(
		r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>howdy</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
		content.as_ref()
	)
}
