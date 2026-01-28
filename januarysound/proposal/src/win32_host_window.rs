use crate::windows_utils::to_wide;
use anyhow::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
    RegisterClassW, ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
    MSG, SW_SHOW, WNDCLASSW, WM_CLOSE, WM_DESTROY, WS_OVERLAPPEDWINDOW,
};

const WINDOW_CLASS_NAME: &str = "JanuarySoundHostWindow";
const WINDOW_TITLE: &str = "JanuarySound Host";

pub fn create_host_window() -> Result<HWND> {
    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(PCWSTR::null())?;
        let class_name = to_wide(WINDOW_CLASS_NAME);

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            lpszClassName: PCWSTR::from_raw(class_name.as_ptr()),
            ..Default::default()
        };

        let atom = RegisterClassW(&wc);
        if atom == 0 {
            let err = GetLastError();
            if err.0 != 1410 {
                anyhow::bail!("RegisterClassW failed: {:?}", err);
            }
        }

        let title = to_wide(WINDOW_TITLE);
        let hwnd = CreateWindowExW(
            Default::default(),
            PCWSTR::from_raw(class_name.as_ptr()),
            PCWSTR::from_raw(title.as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1024,
            768,
            HWND::default(),
            None,
            instance,
            None,
        );

        if hwnd.0.is_null() {
            anyhow::bail!("CreateWindowExW failed: {:?}", GetLastError());
        }

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        Ok(hwnd)
    }
}

pub fn run_message_loop() {
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
