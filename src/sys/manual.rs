#![allow(
    unused,
    clippy::upper_case_acronyms,
    reason = "for type aliases. type aliases inside extern blocks are not allowed yet."
)]

use super::*;

type HWND = *mut std::ffi::c_void;
#[cfg(all(target_os = "windows", feature = "native-handles"))]
extern "C" {
    /** @brief Returns the adapter device name of the specified monitor.
     *
     *  @return The UTF-8 encoded adapter device name (for example `\\.\DISPLAY1`)
     *  of the specified monitor, or `NULL` if an error
     *  occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.1.
     *
     *  @ingroup native
     */
    pub fn glfwGetWin32Adapter(monitor: *mut GLFWmonitor) -> *const std::ffi::c_char;
    /** @brief Returns the display device name of the specified monitor.
     *
     *  @return The UTF-8 encoded display device name (for example
     *  `\\.\DISPLAY1\Monitor0`) of the specified monitor, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.1.
     *
     *  @ingroup native
     */
    pub fn glfwGetWin32Monitor(monitor: *mut GLFWmonitor) -> *const std::ffi::c_char;

    /** @brief Returns the `HWND` of the specified window.
     *
     *  @return The `HWND` of the specified window, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @remark The `HDC` associated with the window can be queried with the
     *  [GetDC](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
     *  function.
     *  @code
     *  HDC dc = GetDC(glfwGetWin32Window(window));
     *  @endcode
     *  This DC is private and does not need to be released.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetWin32Window(window: *mut GLFWwindow) -> HWND;
}
type HGLRC = *mut std::ffi::c_void;
#[cfg(all(target_os = "windows", feature = "native-gl"))]
extern "C" {
    /** @brief Returns the `HGLRC` of the specified window.
     *
     *  @return The `HGLRC` of the specified window, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref
     *  GLFW_PLATFORM_UNAVAILABLE and @ref GLFW_NO_WINDOW_CONTEXT.
     *
     *  @remark The `HDC` associated with the window can be queried with the
     *  [GetDC](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
     *  function.
     *  @code
     *  HDC dc = GetDC(glfwGetWin32Window(window));
     *  @endcode
     *  This DC is private and does not need to be released.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetWGLContext(window: *mut GLFWwindow) -> HGLRC;
}
type CGDirectDisplayID = u32;
type NSWindow = *mut std::ffi::c_void;
type NSView = *mut std::ffi::c_void;
#[cfg(all(target_os = "macos", feature = "native-handles"))]
extern "C" {
    /** @brief Returns the `CGDirectDisplayID` of the specified monitor.
     *
     *  @return The `CGDirectDisplayID` of the specified monitor, or
     *  `kCGNullDirectDisplay` if an error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.1.
     *
     *  @ingroup native
     */
    pub fn glfwGetCocoaMonitor(monitor: *mut GLFWmonitor) -> CGDirectDisplayID;

    /** @brief Returns the `NSWindow` of the specified window.
     *
     *  @return The `NSWindow` of the specified window, or `nil` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetCocoaWindow(window: *mut GLFWwindow) -> NSWindow;

    /** @brief Returns the `NSView` of the specified window.
     *
     *  @return The `NSView` of the specified window, or `nil` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.4.
     *
     *  @ingroup native
     */
    pub fn glfwGetCocoaView(window: *mut GLFWwindow) -> NSView;
}
type NSOpenGLContext = *mut std::ffi::c_void;
#[cfg(all(target_os = "macos", feature = "native-gl"))]
extern "C" {
    /** @brief Returns the `NSOpenGLContext` of the specified window.
     *
     *  @return The `NSOpenGLContext` of the specified window, or `nil` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref
     *  GLFW_PLATFORM_UNAVAILABLE and @ref GLFW_NO_WINDOW_CONTEXT.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetNSGLContext(window: *mut GLFWwindow) -> NSOpenGLContext;
}
type XID = usize;
type Display = std::ffi::c_void;
type Window = XID;
type RRCrtc = XID;
type RROutput = XID;
#[cfg(all(
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "emscripten"),
    feature = "x11",
    feature = "native-handles"
))]
extern "C" {
    /** @brief Returns the `Display` used by GLFW.
     *
     *  @return The `Display` used by GLFW, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetX11Display() -> *mut Display;

    /** @brief Returns the `RRCrtc` of the specified monitor.
     *
     *  @return The `RRCrtc` of the specified monitor, or `None` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.1.
     *
     *  @ingroup native
     */
    pub fn glfwGetX11Adapter(monitor: *mut GLFWmonitor) -> RRCrtc;

    /* @brief Returns the `RROutput` of the specified monitor.
     *
     *  @return The `RROutput` of the specified monitor, or `None` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.1.
     *
     *  @ingroup native
     */
    pub fn glfwGetX11Monitor(monitor: *mut GLFWmonitor) -> RROutput;

    /** @brief Returns the `Window` of the specified window.
     *
     *  @return The `Window` of the specified window, or `None` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetX11Window(window: *mut GLFWwindow) -> Window;

    /** @brief Sets the current primary selection to the specified string.
     *
     *  @param string A UTF-8 encoded string.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref
     *  GLFW_PLATFORM_UNAVAILABLE and @ref GLFW_PLATFORM_ERROR.
     *
     *  @pointer_lifetime The specified string is copied before this function
     *  returns.
     *
     *  @thread_safety This function must only be called from the main thread.
     *
     *  @sa @ref clipboard
     *  @sa glfwGetX11SelectionString
     *  @sa glfwSetClipboardString
     *
     *  @since Added in version 3.3.
     *
     *  @ingroup native
     */
    pub fn glfwSetX11SelectionString(string: *const std::ffi::c_char);

    /** @brief Returns the contents of the current primary selection as a string.
     *
     *  If the selection is empty or if its contents cannot be converted, `NULL`
     *  is returned and a @ref GLFW_FORMAT_UNAVAILABLE error is generated.
     *
     *  @return The contents of the selection as a UTF-8 encoded string, or `NULL`
     *  if an error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref
     *  GLFW_PLATFORM_UNAVAILABLE and @ref GLFW_PLATFORM_ERROR.
     *
     *  @pointer_lifetime The returned string is allocated and freed by GLFW. You
     *  should not free it yourself. It is valid until the next call to @ref
     *  glfwGetX11SelectionString or @ref glfwSetX11SelectionString, or until the
     *  library is terminated.
     *
     *  @thread_safety This function must only be called from the main thread.
     *
     *  @sa @ref clipboard
     *  @sa glfwSetX11SelectionString
     *  @sa glfwGetClipboardString
     *
     *  @since Added in version 3.3.
     *
     *  @ingroup native
     */
    pub fn glfwGetX11SelectionString() -> *const std::ffi::c_char;
}
type GLXContext = XID;
type GLXWindow = XID;
#[cfg(all(
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "emscripten"),
    feature = "x11",
    feature = "native-gl"
))]
extern "C" {
    /** @brief Returns the `GLXContext` of the specified window.
     *
     *  @return The `GLXContext` of the specified window, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref
     *  GLFW_NO_WINDOW_CONTEXT and @ref GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetGLXContext(window: *mut GLFWwindow) -> GLXContext;

    /** @brief Returns the `GLXWindow` of the specified window.
     *
     *  @return The `GLXWindow` of the specified window, or `None` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref
     *  GLFW_NO_WINDOW_CONTEXT and @ref GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.2.
     *
     *  @ingroup native
     */
    pub fn glfwGetGLXWindow(window: *mut GLFWwindow) -> GLXWindow;
}

#[cfg(all(
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "emscripten"),
    feature = "wayland",
    feature = "native-handles"
))]
extern "C" {
    /** @brief Returns the `struct wl_display*` used by GLFW.
     *
     *  @return The `struct wl_display*` used by GLFW, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.2.
     *
     *  @ingroup native
     */
    pub fn glfwGetWaylandDisplay() -> *const std::ffi::c_void;

    /** @brief Returns the `struct wl_output*` of the specified monitor.
     *
     *  @return The `struct wl_output*` of the specified monitor, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.2.
     *
     *  @ingroup native
     */
    pub fn glfwGetWaylandMonitor(monitor: *mut GLFWmonitor) -> *const std::ffi::c_void;

    /** @brief Returns the main `struct wl_surface*` of the specified window.
     *
     *  @return The main `struct wl_surface*` of the specified window, or `NULL` if
     *  an error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_PLATFORM_UNAVAILABLE.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.2.
     *
     *  @ingroup native
     */
    pub fn glfwGetWaylandWindow(window: *mut GLFWwindow) -> *mut std::ffi::c_void;
}
type EGLDisplay = *mut std::ffi::c_void;
type EGLSurface = *mut std::ffi::c_void;
type EGLContext = *mut std::ffi::c_void;
#[cfg(all(
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "emscripten"),
    any(
        all(feature = "wayland", feature = "native-gl"),
        feature = "native-egl"
    )
))]
extern "C" {
    /** @brief Returns the `EGLDisplay` used by GLFW.
     *
     *  @return The `EGLDisplay` used by GLFW, or `EGL_NO_DISPLAY` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED.
     *
     *  @remark Because EGL is initialized on demand, this function will return
     *  `EGL_NO_DISPLAY` until the first context has been created via EGL.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetEGLDisplay() -> EGLDisplay;

    /** @brief Returns the `EGLContext` of the specified window.
     *
     *  @return The `EGLContext` of the specified window, or `EGL_NO_CONTEXT` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_NO_WINDOW_CONTEXT.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetEGLContext(window: *mut GLFWwindow) -> EGLContext;

    /** @brief Returns the `EGLSurface` of the specified window.
     *
     *  @return The `EGLSurface` of the specified window, or `EGL_NO_SURFACE` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_NO_WINDOW_CONTEXT.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.0.
     *
     *  @ingroup native
     */
    pub fn glfwGetEGLSurface(window: *mut GLFWwindow) -> EGLSurface;
}
type OSMesaContext = *mut std::ffi::c_void;
#[cfg(feature = "osmesa")]
extern "C" {
    /** @brief Retrieves the color buffer associated with the specified window.
     *
     *  @param[in] window The window whose color buffer to retrieve.
     *  @param[out] width Where to store the width of the color buffer, or `NULL`.
     *  @param[out] height Where to store the height of the color buffer, or `NULL`.
     *  @param[out] format Where to store the OSMesa pixel format of the color
     *  buffer, or `NULL`.
     *  @param[out] buffer Where to store the address of the color buffer, or
     *  `NULL`.
     *  @return `GLFW_TRUE` if successful, or `GLFW_FALSE` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_NO_WINDOW_CONTEXT.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.3.
     *
     *  @ingroup native
     */
    pub fn glfwGetOSMesaColorBuffer(
        window: *mut GLFWwindow,
        width: *mut std::ffi::c_int,
        height: *mut std::ffi::c_int,
        format: *mut std::ffi::c_int,
        buffer: *mut *mut std::ffi::c_void,
    ) -> std::ffi::c_int;

    /** @brief Retrieves the depth buffer associated with the specified window.
     *
     *  @param[in] window The window whose depth buffer to retrieve.
     *  @param[out] width Where to store the width of the depth buffer, or `NULL`.
     *  @param[out] height Where to store the height of the depth buffer, or `NULL`.
     *  @param[out] bytesPerValue Where to store the number of bytes per depth
     *  buffer element, or `NULL`.
     *  @param[out] buffer Where to store the address of the depth buffer, or
     *  `NULL`.
     *  @return `GLFW_TRUE` if successful, or `GLFW_FALSE` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_NO_WINDOW_CONTEXT.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.3.
     *
     *  @ingroup native
     */
    pub fn glfwGetOSMesaDepthBuffer(
        window: *mut GLFWwindow,
        width: *mut std::ffi::c_int,
        height: *mut std::ffi::c_int,
        bytesPerValue: *mut std::ffi::c_int,
        buffer: *mut *mut std::ffi::c_void,
    ) -> std::ffi::c_int;

    /** @brief Returns the `OSMesaContext` of the specified window.
     *
     *  @return The `OSMesaContext` of the specified window, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_NO_WINDOW_CONTEXT.
     *
     *  @thread_safety This function may be called from any thread.  Access is not
     *  synchronized.
     *
     *  @since Added in version 3.3.
     *
     *  @ingroup native
     */
    pub fn glfwGetOSMesaContext(window: *mut GLFWwindow) -> OSMesaContext;

}

/*
manually maintained vulkan bindings.
I didn't know about allowlist-file feature and thought enabling vulkan means
generating bindings for the entire vulkan header (too expensive).
So, I made these bindings, which are now useless. Just keeping them here *in-case* we
ever need them again.

#[cfg(all(not(feature = "bindings"), feature = "vulkan"))]
mod vulkan {
    type VkInstance = *mut std::ffi::c_void;
    type VkPhysicalDevice = *mut std::ffi::c_void;
    type VkSurfaceKHR = *mut std::ffi::c_void;
    type VkAllocationCallbacks = *mut std::ffi::c_void;
    type VkResult = std::ffi::c_int;
    /** @brief Returns the address of the specified Vulkan instance function.
     *
     *  This function returns the address of the specified Vulkan core or extension
     *  function for the specified instance.  If instance is set to `NULL` it can
     *  return any function exported from the Vulkan loader, including at least the
     *  following functions:
     *
     *  - `vkEnumerateInstanceExtensionProperties`
     *  - `vkEnumerateInstanceLayerProperties`
     *  - `vkCreateInstance`
     *  - `vkGetInstanceProcAddr`
     *
     *  If Vulkan is not available on the machine, this function returns `NULL` and
     *  generates a @ref GLFW_API_UNAVAILABLE error.  Call @ref glfwVulkanSupported
     *  to check whether Vulkan is at least minimally available.
     *
     *  This function is equivalent to calling `vkGetInstanceProcAddr` with
     *  a platform-specific query of the Vulkan loader as a fallback.
     *
     *  @param[in] instance The Vulkan instance to query, or `NULL` to retrieve
     *  functions related to instance creation.
     *  @param[in] procname The ASCII encoded name of the function.
     *  @return The address of the function, or `NULL` if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED and @ref
     *  GLFW_API_UNAVAILABLE.
     *
     *  @pointer_lifetime The returned function pointer is valid until the library
     *  is terminated.
     *
     *  @thread_safety This function may be called from any thread.
     *
     *  @sa @ref vulkan_proc
     *
     *  @since Added in version 3.2.
     *
     *  @ingroup vulkan
     */
    pub fn glfwGetInstanceProcAddress(
        instance: VkInstance,
        procname: *const std::ffi::c_char,
    ) -> GLFWvkproc;

    /** @brief Returns whether the specified queue family can present images.
     *
     *  This function returns whether the specified queue family of the specified
     *  physical device supports presentation to the platform GLFW was built for.
     *
     *  If Vulkan or the required window surface creation instance extensions are
     *  not available on the machine, or if the specified instance was not created
     *  with the required extensions, this function returns `GLFW_FALSE` and
     *  generates a @ref GLFW_API_UNAVAILABLE error.  Call @ref glfwVulkanSupported
     *  to check whether Vulkan is at least minimally available and @ref
     *  glfwGetRequiredInstanceExtensions to check what instance extensions are
     *  required.
     *
     *  @param[in] instance The instance that the physical device belongs to.
     *  @param[in] device The physical device that the queue family belongs to.
     *  @param[in] queuefamily The index of the queue family to query.
     *  @return `GLFW_TRUE` if the queue family supports presentation, or
     *  `GLFW_FALSE` otherwise.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref
     *  GLFW_API_UNAVAILABLE and @ref GLFW_PLATFORM_ERROR.
     *
     *  @remark @macos This function currently always returns `GLFW_TRUE`, as the
     *  `VK_MVK_macos_surface` and `VK_EXT_metal_surface` extensions do not provide
     *  a `vkGetPhysicalDevice*PresentationSupport` type function.
     *
     *  @thread_safety This function may be called from any thread.  For
     *  synchronization details of Vulkan objects, see the Vulkan specification.
     *
     *  @sa @ref vulkan_present
     *
     *  @since Added in version 3.2.
     *
     *  @ingroup vulkan
     */
    pub fn glfwGetPhysicalDevicePresentationSupport(
        instance: VkInstance,
        device: VkPhysicalDevice,
        queuefamily: u32,
    ) -> std::ffi::c_int;

    /** @brief Creates a Vulkan surface for the specified window.
     *
     *  This function creates a Vulkan surface for the specified window.
     *
     *  If the Vulkan loader or at least one minimally functional ICD were not found,
     *  this function returns `VK_ERROR_INITIALIZATION_FAILED` and generates a @ref
     *  GLFW_API_UNAVAILABLE error.  Call @ref glfwVulkanSupported to check whether
     *  Vulkan is at least minimally available.
     *
     *  If the required window surface creation instance extensions are not
     *  available or if the specified instance was not created with these extensions
     *  enabled, this function returns `VK_ERROR_EXTENSION_NOT_PRESENT` and
     *  generates a @ref GLFW_API_UNAVAILABLE error.  Call @ref
     *  glfwGetRequiredInstanceExtensions to check what instance extensions are
     *  required.
     *
     *  The window surface cannot be shared with another API so the window must
     *  have been created with the [client api hint](@ref GLFW_CLIENT_API_attrib)
     *  set to `GLFW_NO_API` otherwise it generates a @ref GLFW_INVALID_VALUE error
     *  and returns `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`.
     *
     *  The window surface must be destroyed before the specified Vulkan instance.
     *  It is the responsibility of the caller to destroy the window surface.  GLFW
     *  does not destroy it for you.  Call `vkDestroySurfaceKHR` to destroy the
     *  surface.
     *
     *  @param[in] instance The Vulkan instance to create the surface in.
     *  @param[in] window The window to create the surface for.
     *  @param[in] allocator The allocator to use, or `NULL` to use the default
     *  allocator.
     *  @param[out] surface Where to store the handle of the surface.  This is set
     *  to `VK_NULL_HANDLE` if an error occurred.
     *  @return `VK_SUCCESS` if successful, or a Vulkan error code if an
     *  error occurred.
     *
     *  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref
     *  GLFW_API_UNAVAILABLE, @ref GLFW_PLATFORM_ERROR and @ref GLFW_INVALID_VALUE
     *
     *  @remark If an error occurs before the creation call is made, GLFW returns
     *  the Vulkan error code most appropriate for the error.  Appropriate use of
     *  @ref glfwVulkanSupported and @ref glfwGetRequiredInstanceExtensions should
     *  eliminate almost all occurrences of these errors.
     *
     *  @remark @macos GLFW prefers the `VK_EXT_metal_surface` extension, with the
     *  `VK_MVK_macos_surface` extension as a fallback.  The name of the selected
     *  extension, if any, is included in the array returned by @ref
     *  glfwGetRequiredInstanceExtensions.
     *
     *  @remark @macos This function creates and sets a `CAMetalLayer` instance for
     *  the window content view, which is required for MoltenVK to function.
     *
     *  @remark @x11 By default GLFW prefers the `VK_KHR_xcb_surface` extension,
     *  with the `VK_KHR_xlib_surface` extension as a fallback.  You can make
     *  `VK_KHR_xlib_surface` the preferred extension by setting the
     *  [GLFW_X11_XCB_VULKAN_SURFACE](@ref GLFW_X11_XCB_VULKAN_SURFACE_hint) init
     *  hint.  The name of the selected extension, if any, is included in the array
     *  returned by @ref glfwGetRequiredInstanceExtensions.
     *
     *  @thread_safety This function may be called from any thread.  For
     *  synchronization details of Vulkan objects, see the Vulkan specification.
     *
     *  @sa @ref vulkan_surface
     *  @sa @ref glfwGetRequiredInstanceExtensions
     *
     *  @since Added in version 3.2.
     *
     *  @ingroup vulkan
     */
    pub fn glfwCreateWindowSurface(
        instance: VkInstance,
        window: *mut GlfwWindow,
        allocator: *const VkAllocationCallbacks,
        surface: *mut VkSurfaceKHR,
    ) -> VkResult;
}

#[cfg(all(not(feature = "bindings"), feature = "vulkan"))]
pub use vulkan::*;
*/
