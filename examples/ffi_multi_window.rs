use glow::HasContext;

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
fn main() {
    use glfw_sys::*;
    use std::ffi::CStr;

    unsafe {
        let mut xpos = 0i32;
        let mut ypos = 0i32;
        let mut height = 0i32;
        let mut description: *const i8 = std::ptr::null();
        let mut windows: [(*mut GLFWwindow, Option<glow::Context>); 4] = [
            (std::ptr::null_mut(), None),
            (std::ptr::null_mut(), None),
            (std::ptr::null_mut(), None),
            (std::ptr::null_mut(), None),
        ];

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

        glfwWindowHint(GLFW_DECORATED, GLFW_FALSE);

        glfwGetMonitorWorkarea(
            glfwGetPrimaryMonitor(),
            &mut xpos,
            &mut ypos,
            std::ptr::null_mut(),
            &mut height,
        );

        for i in 0..4usize {
            let size = height / 5;
            struct Color {
                r: f32,
                g: f32,
                b: f32,
            }
            let colors = [
                Color {
                    r: 0.95,
                    g: 0.32,
                    b: 0.11,
                },
                Color {
                    r: 0.50,
                    g: 0.80,
                    b: 0.16,
                },
                Color {
                    r: 0.,
                    g: 0.68,
                    b: 0.94,
                },
                Color {
                    r: 0.98,
                    g: 0.74,
                    b: 0.04,
                },
            ];

            if i > 0 {
                glfwWindowHint(GLFW_FOCUS_ON_SHOW, GLFW_FALSE);
            }

            glfwWindowHint(GLFW_POSITION_X, xpos + size * (1 + (i & 1) as i32));
            glfwWindowHint(GLFW_POSITION_Y, ypos + size * (1 + (i >> 1) as i32));

            let win = glfwCreateWindow(
                size,
                size,
                c"Multi-Window Example".as_ptr(),
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

            glfwSetInputMode(win, GLFW_STICKY_KEYS, GLFW_TRUE);

            glfwMakeContextCurrent(win);

            let ctx = glow::Context::from_loader_function_cstr(|s| {
                glfwGetProcAddress(s.as_ptr())
                    .map(|p| p as _)
                    .unwrap_or(std::ptr::null())
            });

            ctx.clear_color(colors[i].r, colors[i].g, colors[i].b, 1.0);
            windows[i] = (win, Some(ctx));
        }
        'outer: loop {
            for (win, ctx) in windows.iter() {
                let win = *win;
                glfwMakeContextCurrent(win);
                ctx.as_ref().unwrap().clear(glow::COLOR_BUFFER_BIT);
                glfwSwapBuffers(win);

                if glfwWindowShouldClose(win) == GLFW_TRUE
                    || glfwGetKey(win, GLFW_KEY_ESCAPE) == GLFW_TRUE
                {
                    break 'outer;
                }
            }
            glfwWaitEvents();
        }
        // shutdown
        glfwMakeContextCurrent(std::ptr::null_mut());
        for (w, _) in windows.into_iter() {
            glfwDestroyWindow(w);
        }
        glfwTerminate();
    }
}
