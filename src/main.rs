use std::ffi::{CStr, CString};
use traits::*;

mod window;
mod traits;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

unsafe extern "C" fn err(d: *mut x11::xlib::Display, e: *mut x11::xlib::XErrorEvent) -> core::ffi::c_int {
    let mut cc = [0i8; 1000].as_mut_ptr();
    x11::xlib::XGetErrorText(d, (*e).error_code as core::ffi::c_int, cc as *mut core::ffi::c_char, 1000);
    println!("Error: {}", CStr::from_ptr(cc).to_str().unwrap());
    0
}

fn main() {
    unsafe {
        x11::xlib::XSetErrorHandler(Some(err));
    }
    let window = window::Window::new(false);
    window.show();
    window.init_glx().unwrap();
    unsafe {
        loop {
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            window.swap_buffers(10);
        }
    }
}
