#[macro_use]
extern crate curryrs;

use curryrs::types::*;

#[repr(C)]
pub struct Data {
	pub a: bool,
	pub b: u32,
	pub c: String
}

#[no_mangle]
pub extern fn create_data_c() -> *mut Data {
	Box::into_raw(Box::new(Data{
		a: true,
		b: 1,
		c: String::from("AAA")
	}))
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern fn destroy_data_c(d: *mut Data) {
	// # Safety
	//
	// Has the potential to retrieve a null pointer
	if !d.is_null() {
		drop(Box::from_raw(d))
	}
}

// Place each function you want exported into the safe_ffi! macro and it will
// export each one and place the pub extern for you!
safe_ffi! (

	fn print_data_c(d: &mut Data) -> () {
		println!("{}", d.c)
	}

	fn hello() -> () {
		println!("Hello")
	}

	fn double(x: I32) -> I32 {
		2 * x
	}

	fn square(x: U64) -> U64 {
		x * x
	}

	fn cube(x: I64) -> I64 {
		x * x * x
	}

);
