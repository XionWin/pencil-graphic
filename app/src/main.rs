pub use win32_rs::Window;



fn main() {
    println!("Hello, world!");

    let _r = unsafe { egl_rs::eglGetError() };

    let window = Window::new(800, 480, "OpenGL ES 2.0");
    window.show();
}
