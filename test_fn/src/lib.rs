use std::ffi::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_fn() -> *const c_char { 
    "Hello from the dynamic library!".as_ptr() as *const c_char
}