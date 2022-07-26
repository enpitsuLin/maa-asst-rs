extern crate libc;
mod message;
mod raw;

use libc::{c_int, c_ulonglong, c_void};
pub use message::AsstMsg;
pub use raw::{AsstApiCallback, AsstHandle, TaskId};
use std::ffi::{CStr, CString};

/// 加载助手系统资源
pub fn load_resource<T: Into<Vec<u8>>>(path: T) -> Result<bool, Box<dyn std::error::Error>> {
    let path = CString::new(path)?.into_raw();
    unsafe { Ok(raw::AsstLoadResource(path)) }
}

/// 创建实例
pub fn create() -> AsstHandle {
    unsafe { raw::AsstCreate() }
}

/// 创建带回调的实例
pub fn create_ex(callback: AsstApiCallback, custom_arg: *mut libc::c_void) -> AsstHandle {
    unsafe { raw::AsstCreateEx(callback, custom_arg) }
}

/// 摧毁实例
#[allow(unused_assignments)]
pub fn destroy(mut ptr: AsstHandle) {
    unsafe {
        raw::AsstDestroy(ptr);
        ptr = 0 as *mut libc::c_void;
    }
}

/// 连接设备
pub fn connect<T: Into<Vec<u8>>>(
    handle: AsstHandle,
    adb_path: T,
    address: T,
    config: Option<T>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let adb_path = CString::new(adb_path)?.into_raw();
    let address = CString::new(address)?.into_raw();
    let config = match config {
        Some(value) => CString::new(value)?.into_raw(),
        None => std::ptr::null(),
    };
    unsafe { Ok(raw::AsstConnect(handle, adb_path, address, config)) }
}

/// 新增任务
pub fn append_task<T: Into<Vec<u8>>>(
    handle: AsstHandle,
    type_: T,
    params: T,
) -> Result<TaskId, Box<dyn std::error::Error>> {
    let type_ = CString::new(type_)?.into_raw();
    let params = CString::new(params)?.into_raw();
    unsafe { Ok(raw::AsstAppendTask(handle, type_, params)) }
}

/// 设置任务参数
pub fn set_task_params<T: Into<Vec<u8>>>(
    handle: AsstHandle,
    id: TaskId,
    params: T,
) -> Result<bool, Box<dyn std::error::Error>> {
    let params = CString::new(params)?.into_raw();
    unsafe { Ok(raw::AsstSetTaskParams(handle, id, params)) }
}

/// 开始任务
pub fn start(handle: AsstHandle) -> bool {
    unsafe { raw::AsstStart(handle) }
}

/// 停止任务
pub fn stop(handle: AsstHandle) -> bool {
    unsafe { raw::AsstStop(handle) }
}

/// 获取截图(?)
pub fn get_image(handle: AsstHandle, buff: *mut c_void, buff_size: c_ulonglong) -> c_ulonglong {
    unsafe { raw::AsstGetImage(handle, buff, buff_size) }
}

/// 获取uuid
pub fn get_uuid(handle: AsstHandle) -> Result<String, Box<dyn std::error::Error>> {
    let buff = CString::new("")?.as_ptr();
    unsafe {
        raw::AsstGetUUID(handle, buff, u64::max_value());
        let uuid = CStr::from_ptr(buff).to_str()?;
        Ok(uuid.to_owned())
    }
}

/// 获取当前任务列表
pub fn get_tasks_list(handle: AsstHandle) -> Result<Vec<TaskId>, Box<dyn std::error::Error>> {
    let mut list: Vec<TaskId> = Vec::with_capacity(1000);
    unsafe {
        let buff = list.as_ptr();
        println!("{:?} {:?}", buff, *buff);
        let data_size = raw::AsstGetTasksList(handle, buff, list.capacity().try_into()?);
        list.set_len(data_size.try_into()?);
        list.shrink_to_fit();
        Ok(list)
    }
}

/// 使用controller模拟点击
pub fn controller_click(handle: AsstHandle, x: c_int, y: c_int, block: bool) -> bool {
    unsafe { raw::AsstCtrlerClick(handle, x, y, block) }
}

/// 获取版本
pub fn get_version() -> Result<String, Box<dyn std::error::Error>> {
    unsafe {
        let version = CStr::from_ptr(raw::AsstGetVersion()).to_str()?;
        Ok(String::from(version))
    }
}

/// 日志
pub fn log<T: Into<Vec<u8>>>(level: T, message: T) -> Result<(), Box<dyn std::error::Error>> {
    let level = CString::new(level)?.into_raw();
    let message = CString::new(message)?.into_raw();
    unsafe {
        raw::AsstLog(level, message);
        Ok(())
    }
}

#[test]
fn test() {}
