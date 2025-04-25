//========================================================================
// Simple multi-window example
// Copyright (c) Camilla Löwy <elmindreda@glfw.org>
//
// This software is provided 'as-is', without any express or implied
// warranty. In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would
//    be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such, and must not
//    be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source
//    distribution.
//
//========================================================================

// This is a modified version of `ffi_multi_window.rs` to use for wasm testing.
use glow::HasContext;
/// Just compile this with `cargo build --example=basic --target=wasm32-unknown-emscripten`
///
/// The basic.js + basic.wasm should be in target/wasm32-unknown-emscripten/{debug,release}/examples/basic.*
///
/// Put the files index.html + basic.js + basic.wasm in the same directory and run a http server.
///
/// For example,
///
/// ```bash
/// cargo build --example=basic --target=wasm32-unknown-emscripten
/// cp examples/index.html target/wasm32-unknown-emscripten/debug/examples
/// python -m http.server --directory target/wasm32-unknown-emscripten/debug/examples
/// ```

fn main() {
    use glfw_sys::*;
    use std::ffi::CStr;

    unsafe {
        let mut description: *const i8 = std::ptr::null();

        if glfwInit() != GLFW_TRUE {
            glfwGetError(&mut description);

            panic!(
                "Error: {:?}\n",
                description
                    .is_null()
                    .then_some(c"")
                    .unwrap_or_else(|| CStr::from_ptr(description))
            );
        }
        println!("Initialized GLFW");
        let win = glfwCreateWindow(
            600,
            400,
            c"Single-Window Example".as_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if win.is_null() {
            glfwGetError(&mut description);

            println!(
                "Error: {:?}\n",
                description
                    .is_null()
                    .then_some(c"")
                    .unwrap_or_else(|| CStr::from_ptr(description))
            );
            glfwTerminate();
            panic!();
        }
        println!("Created GLFW window with handle: {:?}", win);
        glfwMakeContextCurrent(win);

        let ctx = glow::Context::from_loader_function_cstr(|s| {
            glfwGetProcAddress(s.as_ptr())
                .map(|p| p as _)
                .unwrap_or(std::ptr::null())
        });
        println!("Created OpenGL context with handle: {:?}\n", ctx);

        // browser will call this closure every frame
        // Try to never block inside this and always return control flow to browser.
        set_main_loop_callback(move || {
            // lets animate clear color based on time
            let time = glfwGetTime() as f32;
            ctx.clear_color(time.sin(), time.cos(), time.tan(), 1.0);
            ctx.clear(glow::COLOR_BUFFER_BIT);

            glfwSwapBuffers(win);
            // don't use waitEvents as that might block event-loop inside wasm
            // and never return control flow to browser
            glfwPollEvents();

            if glfwGetKey(win, GLFW_KEY_ESCAPE) == GLFW_TRUE {
                println!("Terminating main loop");
                glfwMakeContextCurrent(std::ptr::null_mut());
                glfwDestroyWindow(win);
                glfwTerminate();
                emscripten_cancel_main_loop();
            }
        });
    }
}

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn();

#[allow(unused)]
const CANVAS_ELEMENT_NAME: *const std::ffi::c_char = "#canvas\0".as_ptr() as _;
extern "C" {
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_set_main_loop(
        func: em_callback_func,
        fps: std::ffi::c_int,
        simulate_infinite_loop: std::ffi::c_int,
    );

}

thread_local!(static MAIN_LOOP_CALLBACK: std::cell::RefCell<Option<Box<dyn FnMut()>>>  = std::cell::RefCell::new(None));

pub fn set_main_loop_callback<F: 'static>(callback: F)
where
    F: FnMut(),
{
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = Some(Box::new(callback));
    });

    unsafe {
        emscripten_set_main_loop(wrapper::<F>, 0, 1);
    }
    #[allow(clippy::extra_unused_type_parameters)]
    extern "C" fn wrapper<F>()
    where
        F: FnMut(),
    {
        MAIN_LOOP_CALLBACK.with(|z| {
            if let Some(ref mut callback) = *z.borrow_mut() {
                callback();
            }
        });
    }
}
