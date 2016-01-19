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

    while_let_demo();

    return 0;
}


fn while_let_demo() {
    println!("Start while_let demo.");

    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.

    println!("while_let demo finished!");
}
