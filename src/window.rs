use std::ffi::CStr;
use x11::xlib::*;
use crate::gl;

pub struct Window {
    x11d: *mut Display,
    x11w: core::ffi::c_ulong,
    wm_delete_message: core::ffi::c_ulong
}

impl Window {
    pub fn new(desktop: bool) -> Self {
        unsafe {
            let x11d = XOpenDisplay(core::ptr::null());
            let x11w = XCreateSimpleWindow(
                x11d,
                XDefaultRootWindow(x11d),
                0,
                0,
                1920,
                1080,
                0,
                0,
                XWhitePixel(x11d, XDefaultScreen(x11d)),
            );

            let cstring = CStr::from_bytes_with_nul(b"WM_DELETE_WINDOW\0").unwrap();
            let mut wm_delete_message = XInternAtom(x11d, cstring.as_ptr(), False);
            XSetWMProtocols(x11d, x11w, &mut wm_delete_message, 1);

            if desktop {
                let cstring = CStr::from_bytes_with_nul(b"_NET_WM_WINDOW_TYPE\0").unwrap();

                let window_type = XInternAtom(
                    x11d,
                    cstring.as_ptr(),
                    False,
                );

                let cstring2 = CStr::from_bytes_with_nul(b"_NET_WM_WINDOW_TYPE_DESKTOP\0").unwrap();

                let desktop = XInternAtom(
                    x11d,
                    cstring2.as_ptr(),
                    False,
                );

                XChangeProperty(
                    x11d,
                    x11w,
                    window_type,
                    XA_ATOM,
                    32,
                    PropModeReplace,
                    std::mem::transmute(&desktop),
                    1,
                );
            }
            XClearWindow(x11d, x11w);

            Window { x11d, x11w, wm_delete_message }
        }
    }

    pub fn show(&self) {
        unsafe {
            XMapWindow(self.x11d, self.x11w);
        }
    }

    pub fn init_glx(&self) -> Result<(), String> {
        unsafe {
            let mut glx_attributes = [
                //x11::glx::GLX_X_RENDERABLE, gl::TRUE as c_int,
                x11::glx::GLX_DRAWABLE_TYPE, x11::glx::GLX_WINDOW_BIT,
                x11::glx::GLX_RENDER_TYPE, x11::glx::GLX_RGBA_BIT,
                x11::glx::GLX_X_VISUAL_TYPE, x11::glx::GLX_TRUE_COLOR,
                x11::glx::GLX_RED_SIZE, 8,
                x11::glx::GLX_GREEN_SIZE, 8,
                x11::glx::GLX_BLUE_SIZE, 8,
                x11::glx::GLX_ALPHA_SIZE, 8,
                x11::glx::GLX_DEPTH_SIZE, 24,
                x11::glx::GLX_STENCIL_SIZE, 8,
                x11::glx::GLX_DOUBLEBUFFER, gl::TRUE as core::ffi::c_int,
                //GLX_SAMPLE_BUFFERS  , 1,
                //GLX_SAMPLES         , 4,
                gl::NONE as core::ffi::c_int
            ];
            let glx_p = glx_attributes.as_mut_ptr();
            let vis = x11::glx::glXChooseVisual(self.x11d, 0, glx_p);
            if vis.is_null() {
                return Err("Couldn't choose visual info.".to_string());
            }
            let ctx = x11::glx::glXCreateContext(self.x11d, vis, 0 as x11::glx::GLXContext, 1);
            x11::glx::glXMakeCurrent(self.x11d, self.x11w, ctx);

            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::CULL_FACE);
        }
        Ok(())
    }

    pub fn swap_buffers(&self, fps: u64) {
        unsafe {
            std::thread::sleep(core::time::Duration::from_micros(1_000_000 / fps));
            x11::glx::glXSwapBuffers(self.x11d, self.x11w);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn close(&self) -> bool {
        unsafe {
            let mut event = std::mem::zeroed();
            if XPending(self.x11d) == 0 { return false; }
            XNextEvent(self.x11d, &mut event);
            #[allow(non_upper_case_globals)]
            match event.get_type() {
                ClientMessage => {
                    if event.client_message.data.get_long(0) == self.wm_delete_message as i64 {
                        return true;
                    }
                }
                _ => {}
            }
            false
        }
    }
}