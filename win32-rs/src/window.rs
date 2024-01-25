use std::{ffi::c_int, ptr::null_mut};

use crate::{
    create_window, get_last_error, get_module_handle, register_class, BeginPaint, CreateSolidBrush, DefWindowProcW, DestroyWindow, DispatchMessageW, EndPaint, FillRect, GetMessageW, GetWindowLongPtrW, HBrush, HIcon, HMenu, HWnd, LPCWStr, LParam, LResult, LoadCursorW, Msg, PanitStruct, PostQuitMessage, Rect, ShowWindow, TranslateMessage, Uint, WParam, WndClassW, CW_USEDEFAULT, GWLP_USERDATA, IDC_ARROW, SW_SHOW, WM_CLOSE, WM_CREATE, WM_DESTROY, WM_PAINT, WS_CAPTION, WS_OVERLAPPED, WS_SYSMENU
};

pub struct Window {
    handle: HWnd,
    width: c_int,
    height: c_int,
    title: String,
}

impl Window {
    pub fn new(width: c_int, height: c_int, title: &str) -> Self {
        let h_instance = get_module_handle(core::ptr::null());
        println!("h_instance: {:?}", h_instance);

        let class_name_os_str = wide_os_str("MyWindowClass");

        let title_os_str = wide_os_str(title);

        let window_class = WndClassW {
            style: 0,
            lpfn_wnd_proc: Some(window_procedure),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance,
            h_icon: 0 as HIcon,
            h_cursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) } as _,
            h_br_background: 0 as HBrush,
            lpsz_menu_name: 0 as LPCWStr,
            lpsz_class_name: class_name_os_str.as_ptr(),
        };

        let r = register_class(&window_class);
        match r {
            Err(code) => panic!("error: {:?}", code),
            Ok(r) => println!("register_class: {:#x}", r),
        }

        let mut lp = 0;

        let dw_style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU;
        let mut client_rect = Rect::new(0, 0, width, height);
        if crate::adjust_window_rect(&mut client_rect, dw_style, false) == false {
            panic!("set window rect failed");
        }

        let handle = create_window(
            0,
            class_name_os_str.as_ptr(),
            title_os_str.as_ptr(),
            dw_style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            client_rect.get_width() as _,
            client_rect.get_height() as _,
            null_mut() as HWnd,
            null_mut() as HMenu,
            h_instance,
            &mut lp as *mut _ as _,
        );
        println!("create_window: {:?}", handle);

        
        if handle.is_null() {
            panic!("Failed to create a window.");
        }

        Self {
            handle,
            width,
            height,
            title: String::from(title),
        }
    }

    pub fn show(&self) {
        let _previously_visible = unsafe { ShowWindow(self.handle as _, SW_SHOW) };

        let mut msg = Msg::default();
        loop {
            let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
            if message_return == 0 {
                break;
            } else if message_return == -1 {
                let last_error = get_last_error();
                panic!("Error with `GetMessageW`: {}", last_error);
            } else {
                unsafe {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }

    pub fn get_width(&self) -> c_int {
        self.width
    }

    pub fn get_height(&self) -> c_int {
        self.height
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }
}

unsafe extern "system" fn window_procedure(
    h_wnd: HWnd,
    msg: Uint,
    w_param: WParam,
    l_param: LParam,
) -> LResult {
    match msg {
        // WM_NCCREATE => {
        //     println!("NC Create");
        //     let createstruct: *mut CreateStructW = l_param as *mut _;
        //     if createstruct.is_null() {
        //         return 0;
        //     }
        //     let boxed_i32_ptr = (*createstruct).lp_create_params;
        //     SetWindowLongPtrW(h_wnd as _, GWLP_USERDATA, boxed_i32_ptr as LongPtr);
        //     return 1;
        // }
        WM_CREATE => println!("Create"),
        WM_CLOSE => {
            // let text_null = wide_null("Really quit?");
            // let caption_null = wide_null("My Caption");
            // let mb_output = MessageBoxW(
            //     h_wnd as _,
            //     text_null.as_ptr(),
            //     caption_null.as_ptr(),
            //     MB_OKCANCEL,
            // );
            // if mb_output == IDOK {
            //     let _success = DestroyWindow(h_wnd as _);
            // }
            let _success = DestroyWindow(h_wnd as _);
        }
        WM_DESTROY => {
            let ptr = GetWindowLongPtrW(h_wnd as _, GWLP_USERDATA) as *mut i32;
            println!("Cleaned wnd: {:?}", ptr);
            PostQuitMessage(0);
        }
        WM_PAINT => {
            // let ptr = GetWindowLongPtrW(h_wnd as _, GWLP_USERDATA) as *mut i32;
            // println!("Current ptr: {}", *ptr);
            // *ptr += 1;
            let mut ps = PanitStruct::default();
            let hdc = BeginPaint(h_wnd as _, &mut ps);
            let brush = CreateSolidBrush(0x0); // 0x{blue}{green}{red}
            let _success = FillRect(hdc, &ps.rect_paint, brush);
            EndPaint(h_wnd as _, &ps);
        }
        _ => return DefWindowProcW(h_wnd as _, msg, w_param, l_param),
    }
    0
}

pub fn wide_os_str(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
