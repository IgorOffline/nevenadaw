use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::UI::WindowsAndMessaging::FindWindowW;

pub fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub fn load_library(path: &str) -> anyhow::Result<HMODULE> {
    let wide_path = to_wide(path);
    unsafe {
        let handle = LoadLibraryW(PCWSTR::from_raw(wide_path.as_ptr()))?;
        if handle.is_invalid() {
            anyhow::bail!("Failed to load library (invalid handle): {}", path);
        }
        Ok(handle)
    }
}

#[allow(dead_code)]
pub fn get_function<T>(dll_handle: HMODULE, func_name: &str) -> anyhow::Result<*const T> {
    let func_name_c = std::ffi::CString::new(func_name)?;
    unsafe {
        let func_ptr = GetProcAddress(
            dll_handle,
            PCSTR::from_raw(func_name_c.as_ptr() as *const u8),
        );

        if func_ptr.is_none() {
            anyhow::bail!("Failed to get function: {}", func_name);
        }

        Ok(func_ptr.unwrap() as _)
    }
}

#[allow(dead_code)]
pub fn find_window_by_class(class_name: &str) -> anyhow::Result<Option<HWND>> {
    let wide_class = to_wide(class_name);
    unsafe {
        let hwnd = FindWindowW(PCWSTR::from_raw(wide_class.as_ptr()), PCWSTR::null())?;

        if !hwnd.0.is_null() {
            Ok(Some(hwnd))
        } else {
            Ok(None)
        }
    }
}

#[allow(dead_code)]
pub fn get_window_title(hwnd: HWND) -> anyhow::Result<String> {
    use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextLengthW, GetWindowTextW};

    unsafe {
        let len = GetWindowTextLengthW(hwnd);
        if len == 0 {
            let error = windows::Win32::Foundation::GetLastError();
            if error.is_err() {
                return Err(anyhow::anyhow!("GetWindowTextLengthW failed: {:?}", error));
            }
            return Ok(String::new());
        }

        let mut title = vec![0u16; (len + 1) as usize];
        let actual_len = GetWindowTextW(hwnd, &mut title);
        title.truncate(actual_len as usize);
        Ok(String::from_utf16(&title)?)
    }
}
