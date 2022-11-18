pub use coora_engine::*;

fn main() {
	//
	let mut a = MyFoo;
	a.do_thing();
	let b = a.as_shared();
	// let b = a.
	// let b = a.to
	bind_the_things(b);
	//
}


fn bind_the_things<T>(plugin: T)
where
	T: Plugin,
{
	// plugin.bind(builder)
}
struct MyFoo;
// impl Shared for MyFoo {}
impl Foo for MyFoo {
	fn do_thing(&mut self) {}
	fn do_other_thing(&self, a: u32) {}
	fn add(&self, a: u32) -> u32 { 2 }
}

#[coora_plugin]
trait Foo {
	fn do_thing(&mut self);
	fn do_other_thing(&self, a: u32);
	fn add(&self, a: u32) -> u32;
}
