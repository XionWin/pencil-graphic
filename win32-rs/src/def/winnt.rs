use std::ffi::{c_long, c_ulong, c_ushort};
use std::ffi::{c_char, c_int, c_uint};
use crate::DECLARE_HANDLE;
use crate::DECLARE_POINTER;

DECLARE_HANDLE!{HInstance, HInstance__}
DECLARE_HANDLE!{HIcon, HIcon__}
DECLARE_HANDLE!{HCursor, HCursor__}
DECLARE_HANDLE!{HBrush, HBrush__}
DECLARE_HANDLE!{HMenu, HMenu__}
DECLARE_HANDLE!{HWnd, HWnd__}
DECLARE_POINTER!{LPVoid}
pub type Handle = LPVoid;
pub type Hdc = Handle;

pub type Char = c_char;
pub type Byte = u8;
pub type Int = c_int;
pub type Uint = c_uint;
pub type Bool = c_int;
pub type DWord = c_ulong;
pub type Long = c_long;
pub type Word = c_ushort;
pub type LongPtr = isize;
pub type ULongPtr = usize;
pub type UintPtr = usize;
pub type Wchar = u16;
pub type LPCWStr = *const Wchar;
pub type LParam = LongPtr;
pub type WParam = UintPtr;
pub type LResult = LongPtr;
pub type Atom = Word;
pub type ColorRef = DWord;


pub type WndProc = Option<
    unsafe extern "system" fn(hwnd: HWnd, uMsg: Uint, wParam: WParam, lParam: LParam) -> LResult,
>;

#[derive(Debug)]
pub struct Win32Error {
    pub code: DWord,
}