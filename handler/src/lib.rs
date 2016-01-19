extern crate libc;

use std::ffi::CStr;
use libc::c_char;

#[no_mangle]
pub extern "C" fn handle(c_event: *const c_char,
                         c_context: *const c_char) -> i32 {
    let event = unsafe { CStr::from_ptr(c_event).to_string_lossy()
                         .into_owned() };
    let context = unsafe { CStr::from_ptr(c_context).to_string_lossy()
                           .into_owned() };

    real_handle(event, context)
}

fn real_handle(event: String, context: String) -> i32 {
    println!("Event: {}", event);
    println!("Context: {}", context);

    return 0;
}
