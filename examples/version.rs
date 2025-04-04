/// Mainly used to test if glfw functions are properly linked.
fn main() {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;
    unsafe { glfw_sys::glfwGetVersion(&mut major, &mut minor, &mut patch) };
    println!("GLFW version {}.{}.{}", major, minor, patch);
}
