use std::ffi::CStr;
use traits::*;
use objects::*;
use window::*;
use crate::math::{Camera, Vec3};

mod window;
mod traits;
mod math;
mod objects;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

unsafe extern "C" fn err(d: *mut x11::xlib::Display, e: *mut x11::xlib::XErrorEvent) -> core::ffi::c_int {
    let cc = [0i8; 1000].as_mut_ptr();
    x11::xlib::XGetErrorText(d, (*e).error_code as core::ffi::c_int, cc as *mut core::ffi::c_char, 1000);
    println!("Error: {}", CStr::from_ptr(cc).to_str().unwrap());
    0
}

fn main() {
    unsafe { unsafe_main() }
}

unsafe fn unsafe_main() {
    x11::xlib::XSetErrorHandler(Some(err));
    let window = Window::new(false);
    window.show();
    window.init_glx().unwrap();

    let camera = Camera::new(16.0 / 9.0, 70.0, 1.0, 100.0);

    let mut sphere = Sphere::new(6);
    sphere.scaled_by(2.0);
    sphere.rotate_around(45.0, Vec3::front());
    sphere.translate_by(Vec3(5.0, 0.0, -10.0));

    let mut sphere2 = Sphere::new(6);
    sphere2.scaled_by(2.0);
    sphere2.rotate_around(-45.0, Vec3::front());
    sphere2.translate_by(Vec3(-5.0, 0.0, -10.0));

    while !window.close() {
        sphere.draw(&camera);
        sphere.translate_by(Vec3(-5.0, 0.0,  10.0));
        sphere.rotate_around(1.0, Vec3::up());
        sphere.rotate_around(1.0, Vec3::front());
        sphere.translate_by(Vec3( 5.0, 0.0, -10.0));

        sphere2.draw(&camera);
        sphere2.translate_by(Vec3( 5.0, 0.0,  10.0));
        sphere2.rotate_around(-1.0, Vec3::up());
        sphere2.rotate_around(-1.0, Vec3::front());
        sphere2.translate_by(Vec3(-5.0, 0.0, -10.0));
        window.swap_buffers(60);
    }
}
