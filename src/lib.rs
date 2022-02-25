#![no_std]

mod panic;
use panic::{NONBINARY_ERROR_HANDLER, ErrorHandler, ErrorHandlerFunc};

#[no_mangle]
pub extern "C" fn test(a: u32, b: u32) -> u32 {
    nonbinary::add(a, b)
}

#[no_mangle]
pub extern "C" fn nonbinary_set_error_handler(
    error_handler: ErrorHandlerFunc,
    error_data: *mut i8,
    data_size: usize,
) {
    unsafe {
        NONBINARY_ERROR_HANDLER = Some(ErrorHandler::new(error_handler, error_data, data_size));
    }
}
