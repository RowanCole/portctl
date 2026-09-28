use crate::i18n;

#[cfg(target_os = "linux")]
pub fn process_name(pid: u32) -> String {
    std::fs::read_to_string(format!("/proc/{pid}/comm"))
        .map(|s| s.trim().to_string())
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| i18n::UNKNOWN.to_string())
}

#[cfg(target_os = "macos")]
pub fn process_name(pid: u32) -> String {
    libproc::proc_pid::name(pid as i32).unwrap_or_else(|_| i18n::UNKNOWN.to_string())
}

#[cfg(target_os = "windows")]
pub fn process_name(pid: u32) -> String {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
    };
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle.is_null() {
            return i18n::UNKNOWN.to_string();
        }
        let mut buf = [0u16; 1024];
        let mut size = buf.len() as u32;
        let name = if QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            buf.as_mut_ptr(),
            &mut size,
        ) != 0
        {
            let full = String::from_utf16_lossy(&buf[..size as usize]);
            full.rsplit(['\\', '/'])
                .next()
                .unwrap_or(&full)
                .to_string()
        } else {
            i18n::UNKNOWN.to_string()
        };
        CloseHandle(handle);
        name
    }
}

#[cfg(unix)]
pub fn kill_process(pid: u32) -> Result<(), String> {
    let r = unsafe { libc::kill(pid as i32, libc::SIGKILL) };
    if r == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().to_string())
    }
}

#[cfg(target_os = "windows")]
pub fn kill_process(pid: u32) -> Result<(), String> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        OpenProcess, TerminateProcess, PROCESS_TERMINATE,
    };
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle.is_null() {
            return Err(std::io::Error::last_os_error().to_string());
        }
        let ok = TerminateProcess(handle, 1);
        CloseHandle(handle);
        if ok == 0 {
            Err(std::io::Error::last_os_error().to_string())
        } else {
            Ok(())
        }
    }
}
