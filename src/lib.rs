use std::ffi::{CStr};
use std::mem;
use std::os::raw::{c_char, c_void};
use serde::{Deserialize, Serialize};

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[derive(Serialize, Deserialize)]
struct InputPerson {
    id: String,
}

#[derive(Serialize, Deserialize)]
struct Person {
    id: String,
    name: String,
    age: u8,
}

#[no_mangle]
pub extern fn invoke(input: *mut c_char) -> *const u8 {
    let data = unsafe { CStr::from_ptr(input).to_bytes().to_vec() };
    let p1: InputPerson = serde_json::from_slice(data.as_slice()).unwrap();
    let p2 = Person {
        id: p1.id,
        name: "Jens".to_owned(),
        age: 31,
    };

    serde_json::to_string(&p2).unwrap().as_ptr()
}