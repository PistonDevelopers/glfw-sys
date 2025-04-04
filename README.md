# Glfw + Rust
This repo contains `glfw-sys` crate that provides FFI bindings to [glfw](https://www.glfw.org). You are not really supposed to use this crate directly, but rather use [glfw](https://crates.io/crates/glfw) crate instead.


## Design
This library has two main purposes:
1. provide FFI bindings to glfw: pre-generated (fast compile times) and build-time generation (slower).
2. link to glfw library: source builds (using cmake) and [pre-built official glfw libs](https://github.com/glfw/glfw/releases).


For normal applications, you only need to care about 2 features:
1. `src_build` - if you want to build from source. adds around 10 seconds of build time.
2. `static_link` - if you want to link statically. On linux, this requires `src_build` too, so prefer dynamic linking during development for faster compile times. 

### Features
* `static_link` - statically link glfw. not available for linux if `src_build` is disabled.
* `src_build` - build glfw from source. If disabled, we download pre-built glfw libs using system's curl + unzip/tar and link them.
* `x11` and `wayland` - enables support for x11/wayland. Enable both and you can choose which one to use during initialization.
* `vulkan` - enables some vulkan convenience functions (eg: surface creation).
* `native_handles` - enable APIs to get platform specific window handles or display connections. useful for raw-window-handle support.
* `native_gl` - enable APIs for getting platform specific gl contexts (wgl, egl, glx, nsgl etc..)
* `native_egl` - enable egl API even for x11 builds
* `osmesa` - I have no idea. Ignore this unless you know what you are doing.
* `bindgen` - generate glfw FFI bindings at build time from headers. See [Below](#bindgen)

### Source Builds
if `src_build` feature is enabled, we will build glfw from scratch.
This requires `cmake` to be installed on your system and any other required dependencies.

### Prebuilt-libs builds
If `src_build` feature is disabled, we will link with prebuilt glfw libraries.
On windows and mac, we download the official libraries from <https://github.com/glfw/glfw/releases/>
On linux, we expect user to have installed glfw library for linking (eg: `libglfw3-dev` on ubuntu). For static linking, you may probably require src_build, as most distros don't provide static libs.

> NOTE: We use curl + tar (unzip on unix) to download and extract pre-built libs. mac/win10+/linux will have these by default.


### Pre-Generated bindings
We generate FFI bindings at `src/sys/pregenerated.rs` and include them with the crate to keep the compile times fast. These are used when `bindgen` feature is disabled.

This generates core bindings, but skips platform specific bindings (eg: window handles or other platform specific API). Because generating them requires platform headers (eg: `windows.h`) and we can't provide headers for *all* platforms at once.

So, platform specific bindings are manually maintained by hand in `src/sys/manual.rs`.

### Bindgen
When `bindgen` feature is turned on, we generate bindings with bindgen during build time.
This is a fallback, when pre-generated bindings have any mistakes in them (eg: wrong types or missing functions). But this may add significant compile-time overhead.

These features will influence the bindings generated.
* `native_handles`, `native_egl`, `native_gl` - This generates bindings by including system headers for specific types (eg: `HWND` from `windows.h`) and may bloat compile times *a lot* (25+ seconds on windows) due to inclusion of **huge** platform-specific headers.
* `vulkan` - includes vulkan header for vk related types (eg: `vkInstance`).

### Release Check List
* When updating glfw version, make sure to checkout the submodule and commit it. 
* When updating glfw version, don't forget to change the url link build.rs to download the pre-built libs of the correct version.
* Check that the bindings generated are the same on all platforms by checking the CI logs for the `gen_bindings.sh` step.
