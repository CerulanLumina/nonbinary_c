use core::fmt::Write;
use core::panic::PanicInfo;

pub(crate) struct ErrorHandler {
    pub function: ErrorHandlerFunc,
    pub data: *mut i8,
    pub size: usize,
    current: usize,
}

pub(crate) type ErrorHandlerFunc = unsafe extern "C" fn(*const i8);

pub(crate) static mut NONBINARY_ERROR_HANDLER: Option<ErrorHandler> = None;

impl ErrorHandler {
    pub fn new(function: ErrorHandlerFunc, data: *mut i8, size: usize) -> ErrorHandler {
        ErrorHandler {
            function,
            data,
            size,
            current: 0,
        }
    }
}
impl core::fmt::Write for ErrorHandler {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.current + s.len() < self.size {
            let dest = (self.data as usize + self.current) as *mut u8;
            unsafe { core::ptr::copy_nonoverlapping(s.as_ptr(), dest, s.len()) };
            self.current += s.len();
        }
        core::fmt::Result::Ok(())
    }
}

#[inline(never)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    if let Some(handler) = unsafe { &mut NONBINARY_ERROR_HANDLER } {
        write!(handler, "libnonbinary ").unwrap();
        write!(handler, "{}", info).unwrap();
        unsafe {
            (handler.function)(handler.data);
        }
    }
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst)
    }
}
