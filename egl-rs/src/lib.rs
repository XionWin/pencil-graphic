mod def;

#[link(name = "resources/Mali_OpenGL_ES_Emulator/libEGL")]
extern "C" {
    pub fn eglGetError() -> crate::def::ErrorCode;
}