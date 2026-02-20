use alloc::string::String;

use wasm_bindgen::{JsCast, JsValue};

pub fn value_to_string(value: &JsValue) -> String {
	value.unchecked_ref::<js_sys::Object>().to_string().into()
}

pub fn num_elements(dims: &[i32]) -> usize {
	let mut size = 1usize;
	for dim in dims {
		if *dim < 0 {
			return 0;
		}
		size *= *dim as usize;
	}
	size
}
