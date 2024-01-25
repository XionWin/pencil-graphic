#[macro_export]
macro_rules! DECLARE_HANDLE {
    ($name: ident, $inner: ident) => {
        pub enum $inner {}
        pub type $name = *const $inner;
    };
}

#[macro_export]
macro_rules! DECLARE_POINTER {
    ($name: ident) => {
        pub type $name = *mut std::ffi::c_void;
    };
}

#[macro_export]
macro_rules! IMPL_ZEROED {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}
