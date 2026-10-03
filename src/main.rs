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
