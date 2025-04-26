# Glfw + Rust
This repo contains `glfw-sys` crate that provides FFI bindings to [glfw](https://www.glfw.org). You are not really supposed to use this crate directly, but rather use [glfw](https://crates.io/crates/glfw) crate instead.


## Design
This library has two main purposes:
1. provide FFI bindings to glfw: pre-generated (fast compile times) and build-time generation (slower).
2. link to glfw library: system builds (using `pkg-config`), source builds (using `cmake`) and [pre-built official glfw libs](https://github.com/glfw/glfw/releases) (only for windows and mac).


For normal applications, you only need to care about 2 features:
1. `src-build` - if you want to build from source. adds around 10 seconds of build time.
2. `static-link` - if you want to link statically. On linux, this requires `src-build` too, so prefer dynamic linking during development for faster compile times. 

### Features

#### Building And Linking

> NOTE: For emscripten, none of these features apply. We just pass the necessary flags like `-sUSE_GLFW=3` to linker and simply let emscripten take care of things.

- `static-link` - statically link glfw. If disabled, we will dynamically link glfw.

We try to build glfw in this order:
- `src-build` - If enabled, build glfw from source (sources are included with crate). Ensure `cmake` is installed and any other required dependencies.
- `prebuilt-libs` (only for windows/macos. ignored on other platforms) - If enabled, we download and link pre-built glfw libs from <https://github.com/glfw/glfw/releases/>.

> NOTE: We use curl + tar (unzip on macos) to download and extract pre-built libs. mac/win10+ will have these by default.

Finally, if neither `src-build` nor `prebuilt-libs` feature is enabled, we will try to use `pkg-config` to find and link to system glfw libs.

#### Platform Backends (non-mac and non-windows only)
* `x11` and `wayland` - enables support for x11/wayland. Enable both and you can choose which one to use during initialization. `x11/wayland` are ignored on windows/macos platforms.

#### Vulkan
- `vulkan` enables some vulkan convenience functions (eg: `glfwVulkanSupported`).
- Only enable this if you need vulkan support.

#### Native Handles
These features expose native "HWND"/"NSWindow"/"X11Connection" etc.. handles.
Unless you are using `wgpu`-like libs that need raw-window-handles, these features can be ignored.
- `native-handles` - enable APIs to get platform specific window handles or display connections or monitor ids. useful for raw-window-handle support.
- `native-gl` - enable APIs for getting platform specific gl contexts (`wgl`, `egl`, `glx`, `nsgl` etc..). Most users should ignore this. 
- `native-egl` - enable egl API even for x11 builds, if you plan to use `egl` contexts with x11 windows. Most users should ignore this.

#### Miscellaneous
* `osmesa` - I have no idea. Ignore this unless you know what you are doing.
* `bindgen` - generate glfw FFI bindings at build time from headers. See [Below](#bindgen)


### Pre-Generated bindings
We generate FFI bindings at `src/sys/pregenerated.rs` and include them with the crate to keep the compile times fast. These are used when `bindgen` feature is disabled.

This contains core bindings, but skips platform specific bindings (eg: window handles or other platform specific API). Because generating them requires platform headers (eg: `windows.h`) and we can't provide headers for *all* platforms at once.

So, platform specific bindings are manually maintained by hand in `src/sys/manual.rs`.

### Bindgen
When `bindgen` feature is turned on, we generate bindings with bindgen during build time.
This is a fallback, when pre-generated bindings have any mistakes in them (eg: wrong types or missing functions). But this may add significant compile-time overhead.

These features will influence the bindings generated.
* `native-handles`, `native-egl`, `native-gl` - This generates bindings by including system headers for specific types (eg: `HWND` from `windows.h`) and may bloat compile times *a lot* (25+ seconds on windows) due to inclusion of **huge** platform-specific headers.
* `vulkan` - includes vulkan header for vk related types (eg: `vkInstance`).

### Release Check List
* When updating glfw version, make sure to checkout the submodule and commit it. 
* When updating glfw version, don't forget to change the url link in build.rs to download the pre-built libs of the correct version.
* When updating glfw version, don't forget to update the pkg-config `atleast_version` argument.
* Check that the bindings generated are the same on all platforms by checking the CI logs for the `gen_bindings.sh` step.
