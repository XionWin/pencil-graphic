#![allow(dead_code)]

use crate::IMPL_ZEROED;

use super::super::def::*;
use super::kernel32::*;


type LPMsg = *mut crate::Msg;
type LPPaintStruct = *mut PanitStruct;
type LPRect = *mut Rect;

#[repr(C)]
pub struct WndClassW {
    pub style: Uint,
    pub lpfn_wnd_proc: WndProc,
    pub cb_cls_extra: Int,
    pub cb_wnd_extra: Int,
    pub h_instance: HInstance,
    pub h_icon: HIcon,
    pub h_cursor: HCursor,
    pub h_br_background: HBrush,
    pub lpsz_menu_name: LPCWStr,
    pub lpsz_class_name: LPCWStr,
}
IMPL_ZEROED!(WndClassW);


#[repr(C)]
pub struct Point {
    x: Long,
    y: Long,
}
IMPL_ZEROED!(Point);

#[repr(C)]
pub struct Msg {
    hwnd: HWnd,
    message: Uint,
    w_param: WParam,
    l_param: LParam,
    time: DWord,
    pt: Point,
    l_private: DWord,
}
IMPL_ZEROED!(Msg);


#[repr(C)]
pub struct Rect {
    left: Long,
    top: Long,
    right: Long,
    bottom: Long,
}
IMPL_ZEROED!(Rect);
impl Rect {
    pub fn new(left: Long, top: Long, right: Long, bottom: Long) -> Self {
        Self {
            left,
            top,
            right,
            bottom
        }
    }

    pub fn get_width(&self) -> Long {
        self.right - self.left
    }

    pub fn get_height(&self) -> Long {
        self.bottom - self.top
    }
}

#[repr(C)]
pub struct PanitStruct {
    hdc: Hdc,
    f_erase: Bool,
    pub rect_paint: Rect,
    is_restore: Bool,
    is_inc_update: Bool,
    rgb_reserved: [Byte; 32],
}
IMPL_ZEROED!(PanitStruct);

#[repr(C)]
pub struct CreateStructW {
    pub lp_create_params: LPVoid,
    h_instance: HInstance,
    h_menu: HMenu,
    h_wnd_parent: HWnd,
    cy: Int,
    cx: Int,
    y: Int,
    x: Int,
    style: Long,
    lpsz_name: LPCWStr,
    lpsz_class: LPCWStr,
    dw_ex_style: DWord,
}
IMPL_ZEROED!(CreateStructW);

pub fn register_class(wnd_class: &WndClassW) -> Result<Atom, Win32Error> {
    unsafe {
        let r = RegisterClassW(wnd_class);
        if r != 0 {
            Ok(r)
        } else {
            Err(Win32Error {
                code: get_last_error(),
            })
        }
    }
}

pub fn create_window(
    dw_ex_style: DWord,
    lp_class_name: LPCWStr,
    lp_window_name: LPCWStr,
    dw_style: DWord,
    x: Int,
    y: Int,
    width: Int,
    height: Int,
    h_wnd_parent: HWnd,
    h_menu: HMenu,
    h_instance: HInstance,
    lp_param: LPVoid
    ) -> HWnd {
    unsafe {
        CreateWindowExW(dw_ex_style, lp_class_name, lp_window_name, dw_style, x, y, width, height, h_wnd_parent, h_menu, h_instance, lp_param)
    }
}

pub fn unregister_class(
    lp_class_name: LPCWStr,
    h_instancece: HInstance,
) -> Result<bool, Win32Error> {
    unsafe {
        if UnregisterClassW(lp_class_name, h_instancece) != 0 {
            Ok(true)
        } else {
            Err(Win32Error {
                code: get_last_error(),
            })
        }
    }
}

pub fn adjust_window_rect(rect: &mut Rect, dw_style: DWord, b_menu: bool) -> bool {
    unsafe {
        AdjustWindowRect(rect, dw_style, b_menu)
    }
}

#[link(name = "user32")]
extern "system" {
    /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    fn RegisterClassW(lpWndClass: *const WndClassW) -> Atom;

    /// [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
    fn UnregisterClassW(lpClassName: LPCWStr, hInstance: HInstance) -> Bool;

    /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(
        dw_ex_style: DWord,
        lp_class_name: LPCWStr,
        lp_window_name: LPCWStr,
        dw_style: DWord,
        x: Int,
        y: Int,
        width: Int,
        height: Int,
        h_wnd_parent: HWnd,
        h_menu: HMenu,
        h_instance: HInstance,
        lp_param: LPVoid
    ) -> HWnd;

    /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(hWnd: HWnd, Msg: Uint, wParam: WParam, lParam: LParam) -> LResult;

    /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWnd, nCmdShow: Int) -> Bool;

    /// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(lpMsg: LPMsg, hWnd: HWnd, wMsgFilterMin: Uint, wMsgFilterMax: Uint) -> Bool;

    /// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
    pub fn TranslateMessage(lpMsg: *const Msg) -> Bool;
    
    /// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
    pub fn DispatchMessageW(lpMsg: *const Msg) -> LResult;

    /// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
    pub fn DestroyWindow(hWnd: HWnd) -> Bool;

    /// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
    pub fn PostQuitMessage(nExitCode: Int);

    /// [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HInstance, lpCursorName: LPCWStr) -> HCursor;

    /// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
    pub fn BeginPaint(hWnd: HWnd, lpPaint: LPPaintStruct) -> Hdc;

    /// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
    pub fn FillRect(hDC: Hdc, lprc: *const Rect, hbr: HBrush) -> Int;

    /// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
    pub fn EndPaint(hWnd: HWnd, lpPaint: *const PanitStruct) -> Bool;

    /// [`MessageBoxW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
    pub fn MessageBoxW(hWnd: HWnd, lpText: LPCWStr, lpCaption: LPCWStr, uType: Uint) -> Int;

    /// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
    pub fn SetWindowLongPtrW(hWnd: HWnd, nIndex: Int, dwNewLong: LongPtr) -> LongPtr;

    /// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
    pub fn GetWindowLongPtrW(hWnd: HWnd, nIndex: Int) -> LongPtr;

    /// [`SetCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursor)
    pub fn SetCursor(hCursor: HCursor) -> HCursor;

    pub fn AdjustWindowRect(lpRect: LPRect, dwStyle: DWord, bMenu: bool) -> bool;
}

#[link(name = "gdi32")]
extern "system" {
    pub fn  CreateSolidBrush(color: ColorRef) -> HBrush;
}
