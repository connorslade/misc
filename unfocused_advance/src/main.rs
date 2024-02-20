use anyhow::{ensure, Result};
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{GetKeyState, VK_LEFT, VK_RIGHT, VK_SHIFT},
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageA, EnumWindows, GetMessageA, GetWindowTextA,
            GetWindowTextLengthA, SendMessageA, SetWindowsHookExA, UnhookWindowsHookEx,
            KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
        },
    },
};

const HOOK_KEYCODE: u32 = 0xC0;

static mut WINDOW_NAME: Option<String> = None;
static mut HANDLE: Option<HWND> = None;

fn main() -> Result<()> {
    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "oculante".to_string());

    unsafe {
        WINDOW_NAME = Some(name.to_owned());

        EnumWindows(Some(window_callback), LPARAM::default())?;
        ensure!(HANDLE.is_some(), "Window not found");

        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook), None, 0)?;

        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND::default(), 0, 0).as_bool() {
            DispatchMessageA(&message);
        }

        UnhookWindowsHookEx(hook)?;
    }

    Ok(())
}

unsafe extern "system" fn window_callback(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    let title_len = GetWindowTextLengthA(hwnd) as usize;
    let mut buf = vec![0u8; title_len + 1];
    GetWindowTextA(hwnd, &mut buf);

    let title = String::from_utf8_lossy(&buf[..title_len]);
    if title.contains(WINDOW_NAME.as_ref().unwrap_unchecked()) {
        println!("Found window: {}", title);
        HANDLE = Some(hwnd);
    }

    BOOL::from(true)
}

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let ptr = l_param.0 as *const KBDLLHOOKSTRUCT;

    if n_code >= 0 {
        if ptr.read().vkCode == HOOK_KEYCODE {
            println!("Advancing photo");
            if w_param.0 == WM_KEYDOWN as usize {
                let shift_down = GetKeyState(VK_SHIFT.0 as i32) >> 1 != 0;
                let keycode = if shift_down { VK_LEFT } else { VK_RIGHT }.0 as usize;
                for msg in [WM_KEYDOWN, WM_KEYUP] {
                    SendMessageA(HANDLE.unwrap_unchecked(), msg, WPARAM(keycode), LPARAM(0));
                }
            }
            return LRESULT(1);
        }
    }

    CallNextHookEx(None, n_code, w_param, l_param)
}
