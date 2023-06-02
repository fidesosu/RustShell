use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use winapi::um::shellapi::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW};
use winapi::um::winuser::SW_HIDE;

pub fn elevate() -> Result<(), Box<dyn std::error::Error>> {
    let verb = CString::new("runas")?;
    let verb_utf16: Vec<u16> = verb.as_bytes_with_nul().iter().map(|&c| c as u16).collect();

    let exe_path = std::env::current_exe()?;
    let exe_path_utf16: Vec<u16> = exe_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();

    unsafe {
        let mut info = SHELLEXECUTEINFOW {
            cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
            lpVerb: verb_utf16.as_ptr(),
            lpFile: exe_path_utf16.as_ptr(),
            fMask: SEE_MASK_NOCLOSEPROCESS,
            nShow: SW_HIDE,
            ..Default::default()
        };

        if ShellExecuteExW(&mut info) == 0 {
            return Err("Failed to elevate process".into());
        }
    }

    Ok(())
}
