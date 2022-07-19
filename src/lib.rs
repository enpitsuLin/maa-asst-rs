mod raw;

use libc::{c_int, c_ulonglong, c_void};
pub use raw::{Assistant, AsstApiCallback, AsstHandle, TaskId};
use std::ffi::{CStr, CString};

#[allow(temporary_cstring_as_ptr)]
pub fn load_resource<T: Into<Vec<u8>>>(path: T) -> Result<bool, Box<dyn std::error::Error>> {
    let path = CString::new(path).unwrap().as_ptr();
    unsafe { Ok(raw::AsstLoadResource(path)) }
}

pub fn create() -> AsstHandle {
    unsafe { raw::AsstCreate() }
}

pub fn create_ex(callback: AsstApiCallback, custom_arg: *mut libc::c_void) -> AsstHandle {
    unsafe { raw::AsstCreateEx(callback, custom_arg) }
}

pub fn destroy(handle: AsstHandle) {
    unsafe { raw::AsstDestroy(handle) }
}

#[allow(temporary_cstring_as_ptr)]
pub fn connect<T: Into<Vec<u8>>>(
    handle: AsstHandle,
    adb_path: T,
    address: T,
    config: T,
) -> Result<bool, Box<dyn std::error::Error>> {
    let adb_path = CString::new(adb_path)?.as_ptr();
    let address = CString::new(address)?.as_ptr();
    let config = CString::new(config)?.as_ptr();
    unsafe { Ok(raw::AsstConnect(handle, adb_path, address, config)) }
}

pub fn append_task<T: Into<Vec<u8>>>(
    handle: AsstHandle,
    type_: T,
    params: T,
) -> Result<TaskId, Box<dyn std::error::Error>> {
    let type_ = CString::new(type_)?.as_ptr();
    let params = CString::new(params)?.as_ptr();
    unsafe { Ok(raw::AsstAppendTask(handle, type_, params)) }
}

pub fn set_task_params<T: Into<Vec<u8>>>(
    handle: AsstHandle,
    id: TaskId,
    params: T,
) -> Result<bool, Box<dyn std::error::Error>> {
    let params = CString::new(params)?.as_ptr();
    unsafe { Ok(raw::AsstSetTaskParams(handle, id, params)) }
}

pub fn start(handle: AsstHandle) -> bool {
    unsafe { raw::AsstStart(handle) }
}

pub fn stop(handle: AsstHandle) -> bool {
    unsafe { raw::AsstStop(handle) }
}

pub fn get_image(handle: AsstHandle, buff: *mut c_void, buff_size: c_ulonglong) -> c_ulonglong {
    unsafe { raw::AsstGetImage(handle, buff, buff_size) }
}

pub fn controller_click(handle: AsstHandle, x: c_int, y: c_int, block: bool) -> bool {
    unsafe { raw::AsstCtrlerClick(handle, x, y, block) }
}

pub fn get_version() -> Result<String, Box<dyn std::error::Error>> {
    unsafe {
        let version = CStr::from_ptr(raw::AsstGetVersion()).to_str()?;
        Ok(String::from(version))
    }
}

pub fn log<T: Into<Vec<u8>>>(level: T, message: T) -> Result<(), Box<dyn std::error::Error>> {
    let level = CString::new(level)?.as_ptr();
    let message = CString::new(message)?.as_ptr();
    unsafe {
        raw::AsstLog(level, message);
        Ok(())
    }
}

#[test]
fn test() {}
