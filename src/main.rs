#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
compile_error!("Este aplicativo foi desenvolvido exclusivamente para Windows.");

#[cfg(target_os = "windows")]
mod app {
    use std::{
        ffi::c_void,
        mem::zeroed,
        ptr::{null, null_mut},
        sync::{Mutex, OnceLock},
    };

    type Bool = i32;
    type Dword = u32;
    type Hinstance = isize;
    type Hmenu = isize;
    type Hwnd = isize;
    type Lparam = isize;
    type Lresult = isize;
    type Uint = u32;
    type UintPtr = usize;
    type Wparam = usize;
    type Hbrush = isize;
    type Hcursor = isize;
    type Hrgn = isize;
    type Hdc = isize;

    const TRUE: Bool = 1;
    const FALSE: Bool = 0;

    const CS_HREDRAW: Uint = 0x0002;
    const CS_VREDRAW: Uint = 0x0001;

    const WS_OVERLAPPED: Dword = 0x0000_0000;
    const WS_CAPTION: Dword = 0x00C0_0000;
    const WS_SYSMENU: Dword = 0x0008_0000;
    const WS_MINIMIZEBOX: Dword = 0x0002_0000;
    const WS_VISIBLE: Dword = 0x1000_0000;
    const WS_CHILD: Dword = 0x4000_0000;
    const WS_POPUP: Dword = 0x8000_0000;
    const WS_CLIPCHILDREN: Dword = 0x0200_0000;
    const WS_DISABLED: Dword = 0x0800_0000;

    const WS_EX_TOPMOST: Dword = 0x0000_0008;
    const WS_EX_LAYERED: Dword = 0x0008_0000;
    const WS_EX_TOOLWINDOW: Dword = 0x0000_0080;
    const WS_EX_TRANSPARENT: Dword = 0x0000_0020;
    const WS_EX_NOACTIVATE: Dword = 0x0800_0000;

    const CW_USEDEFAULT: i32 = i32::MIN;
    const SW_SHOW: i32 = 5;
    const SW_HIDE: i32 = 0;
    const SW_SHOWNOACTIVATE: i32 = 4;

    const WM_CREATE: Uint = 0x0001;
    const WM_DESTROY: Uint = 0x0002;
    const WM_SIZE: Uint = 0x0005;
    const WM_COMMAND: Uint = 0x0111;
    const WM_TIMER: Uint = 0x0113;
    const WM_HOTKEY: Uint = 0x0312;
    const WM_NCHITTEST: Uint = 0x0084;
    const WM_CLOSE: Uint = 0x0010;
    const WM_CTLCOLORBTN: Uint = 0x0135;
    const WM_CTLCOLORSTATIC: Uint = 0x0138;

    const HTTRANSPARENT: Lresult = -1;

    const SWP_NOSIZE: Uint = 0x0001;
    const SWP_NOACTIVATE: Uint = 0x0010;

    const HWND_TOPMOST: Hwnd = -1isize;
    const HWND_NOTOPMOST: Hwnd = -2isize;

    const SM_XVIRTUALSCREEN: i32 = 76;
    const SM_YVIRTUALSCREEN: i32 = 77;
    const SM_CXVIRTUALSCREEN: i32 = 78;
    const SM_CYVIRTUALSCREEN: i32 = 79;

    const LWA_ALPHA: Dword = 0x0000_0002;
    const IDC_ARROW: usize = 32512;

    const BS_PUSHBUTTON: Dword = 0x0000_0000;
    const BS_AUTORADIOBUTTON: Dword = 0x0000_0009;
    const BS_GROUPBOX: Dword = 0x0000_0007;
    const BS_AUTOCHECKBOX: Dword = 0x0000_0003;
    const BST_CHECKED: Wparam = 1;
    const BST_UNCHECKED: Wparam = 0;
    const BM_SETCHECK: Uint = 0x00F1;

    const SS_LEFT: Dword = 0x0000_0000;
    const SS_CENTER: Dword = 0x0000_0001;

    const MOD_ALT: Uint = 0x0001;
    const MOD_CONTROL: Uint = 0x0002;
    const MOD_NOREPEAT: Uint = 0x4000;
    const VK_OEM_PLUS: Uint = 0xBB;
    const VK_OEM_MINUS: Uint = 0xBD;
    const VK_M: Uint = 0x4D;

    const HOTKEY_ZOOM_IN: i32 = 1001;
    const HOTKEY_ZOOM_OUT: i32 = 1002;
    const HOTKEY_TOGGLE: i32 = 1003;

    const ID_START_STOP: usize = 101;
    const ID_ZOOM_MINUS: usize = 102;
    const ID_ZOOM_PLUS: usize = 103;
    const ID_MODE_LENS: usize = 104;
    const ID_MODE_FIXED: usize = 105;
    const ID_SHAPE_CIRCLE: usize = 106;
    const ID_ALWAYS_ON_TOP: usize = 107;
    const ID_EXIT: usize = 108;
    const ID_AREA_MINUS: usize = 109;
    const ID_AREA_PLUS: usize = 110;

    const TIMER_ID: UintPtr = 1;
    // ~30 FPS: suficiente para a lente e bem mais leve ao mover uma janela grande.
    const TIMER_INTERVAL_MS: Uint = 33;

    const MW_FILTERMODE_EXCLUDE: Dword = 0;
    const MS_SHOWMAGNIFIEDCURSOR: Dword = 0x0001;

    const TRANSPARENT: i32 = 1;
    const DWMWA_USE_IMMERSIVE_DARK_MODE: Dword = 20;
    const DWMWA_USE_IMMERSIVE_DARK_MODE_BEFORE_20H1: Dword = 19;

    // Paleta dark inspirada no visual mostrado: fundo bem escuro, painéis azulados
    // e texto claro. COLORREF no Win32 usa a ordem 0x00BBGGRR.
    const COLOR_BG: Dword = 0x0020_100B;       // #0B1020
    const COLOR_PANEL: Dword = 0x0036_231A;    // #1A2336
    const COLOR_TEXT: Dword = 0x00EB_E7E5;     // #E5E7EB

    #[repr(C)]
    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct Rect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[repr(C)]
    struct Msg {
        hwnd: Hwnd,
        message: Uint,
        w_param: Wparam,
        l_param: Lparam,
        time: Dword,
        pt: Point,
        l_private: Dword,
    }

    #[repr(C)]
    struct WndClassW {
        style: Uint,
        wnd_proc: Option<unsafe extern "system" fn(Hwnd, Uint, Wparam, Lparam) -> Lresult>,
        cls_extra: i32,
        wnd_extra: i32,
        instance: Hinstance,
        icon: isize,
        cursor: Hcursor,
        background: Hbrush,
        menu_name: *const u16,
        class_name: *const u16,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct MagTransform {
        matrix: [[f32; 3]; 3],
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        fn RegisterClassW(class: *const WndClassW) -> u16;
        fn CreateWindowExW(
            ex_style: Dword,
            class_name: *const u16,
            window_name: *const u16,
            style: Dword,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            parent: Hwnd,
            menu: Hmenu,
            instance: Hinstance,
            param: *mut c_void,
        ) -> Hwnd;
        fn DefWindowProcW(hwnd: Hwnd, msg: Uint, w_param: Wparam, l_param: Lparam) -> Lresult;
        fn ShowWindow(hwnd: Hwnd, cmd_show: i32) -> Bool;
        fn UpdateWindow(hwnd: Hwnd) -> Bool;
        fn GetMessageW(msg: *mut Msg, hwnd: Hwnd, min: Uint, max: Uint) -> Bool;
        fn TranslateMessage(msg: *const Msg) -> Bool;
        fn DispatchMessageW(msg: *const Msg) -> Lresult;
        fn PostQuitMessage(exit_code: i32);
        fn DestroyWindow(hwnd: Hwnd) -> Bool;
        fn GetCursorPos(point: *mut Point) -> Bool;
        fn GetSystemMetrics(index: i32) -> i32;
        fn SetWindowPos(
            hwnd: Hwnd,
            insert_after: Hwnd,
            x: i32,
            y: i32,
            cx: i32,
            cy: i32,
            flags: Uint,
        ) -> Bool;
        fn GetClientRect(hwnd: Hwnd, rect: *mut Rect) -> Bool;
        fn MoveWindow(hwnd: Hwnd, x: i32, y: i32, width: i32, height: i32, repaint: Bool) -> Bool;
        fn SetTimer(hwnd: Hwnd, id: UintPtr, elapse: Uint, timer_proc: *const c_void) -> UintPtr;
        fn KillTimer(hwnd: Hwnd, id: UintPtr) -> Bool;
        fn SetWindowTextW(hwnd: Hwnd, text: *const u16) -> Bool;
        fn SetLayeredWindowAttributes(hwnd: Hwnd, color_key: Dword, alpha: u8, flags: Dword) -> Bool;
        fn SendMessageW(hwnd: Hwnd, msg: Uint, w_param: Wparam, l_param: Lparam) -> Lresult;
        fn RegisterHotKey(hwnd: Hwnd, id: i32, modifiers: Uint, vk: Uint) -> Bool;
        fn UnregisterHotKey(hwnd: Hwnd, id: i32) -> Bool;
        fn LoadCursorW(instance: Hinstance, cursor_name: *const u16) -> Hcursor;
        fn SetWindowRgn(hwnd: Hwnd, region: Hrgn, redraw: Bool) -> i32;
        fn InvalidateRect(hwnd: Hwnd, rect: *const Rect, erase: Bool) -> Bool;
    }

    #[link(name = "uxtheme")]
    unsafe extern "system" {
        fn SetWindowTheme(hwnd: Hwnd, sub_app_name: *const u16, sub_id_list: *const u16) -> i32;
    }

    #[link(name = "dwmapi")]
    unsafe extern "system" {
        fn DwmSetWindowAttribute(
            hwnd: Hwnd,
            attribute: Dword,
            value: *const c_void,
            value_size: Dword,
        ) -> i32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleW(module_name: *const u16) -> Hinstance;
    }

    #[link(name = "gdi32")]
    unsafe extern "system" {
        fn CreateEllipticRgn(left: i32, top: i32, right: i32, bottom: i32) -> Hrgn;
        fn CreateRectRgn(left: i32, top: i32, right: i32, bottom: i32) -> Hrgn;
        fn CreateSolidBrush(color: Dword) -> Hbrush;
        fn SetTextColor(hdc: Hdc, color: Dword) -> Dword;
        fn SetBkColor(hdc: Hdc, color: Dword) -> Dword;
        fn SetBkMode(hdc: Hdc, mode: i32) -> i32;
    }

    #[link(name = "Magnification")]
    unsafe extern "system" {
        fn MagInitialize() -> Bool;
        fn MagUninitialize() -> Bool;
        fn MagSetWindowSource(hwnd: Hwnd, rect: Rect) -> Bool;
        fn MagSetWindowTransform(hwnd: Hwnd, transform: *const MagTransform) -> Bool;
        fn MagSetWindowFilterList(hwnd: Hwnd, filter_mode: Dword, count: i32, hwnds: *const Hwnd) -> Bool;
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
