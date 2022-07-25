//! # raw.rs
//! 定义参阅[AsstCaller.h](https://github.com/MaaAssistantArknights/MaaAssistantArknights/blob/master/include/AsstCaller.h)

#![allow(dead_code)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate libc;
use libc::{c_char, c_int, c_ulonglong, c_void};

pub type AsstHandle = *const c_void;
/// 任务id
pub type TaskId = c_int;
/// Api回调函数
pub type AsstApiCallback =
    Option<unsafe extern "C" fn(msg: c_int, detail_json: *const c_char, custom_arg: *mut c_void)>;

#[link(name = "MeoAssistant")]
extern "C" {
    pub fn AsstLoadResource(path: *const c_char) -> bool;
    pub fn AsstCreate() -> AsstHandle;
    pub fn AsstCreateEx(callback: AsstApiCallback, custom_arg: *mut c_void) -> AsstHandle;
    pub fn AsstDestroy(handle: AsstHandle);
    pub fn AsstConnect(
        handle: AsstHandle,
        adb_path: *const c_char,
        address: *const c_char,
        config: *const c_char,
    ) -> bool;
    pub fn AsstAppendTask(
        handle: AsstHandle,
        type_: *const c_char,
        params: *const c_char,
    ) -> TaskId;
    pub fn AsstSetTaskParams(handle: AsstHandle, id: TaskId, params: *const c_char) -> bool;
    pub fn AsstStart(handle: AsstHandle) -> bool;
    pub fn AsstStop(handle: AsstHandle) -> bool;
    pub fn AsstGetImage(
        handle: AsstHandle,
        buff: *mut c_void,
        buff_size: c_ulonglong,
    ) -> c_ulonglong;
    pub fn AsstGetUUID(
        handle: AsstHandle,
        buff: *const c_char,
        buff_size: c_ulonglong,
    ) -> c_ulonglong;
    pub fn AsstGetTasksList(
        handle: AsstHandle,
        buff: *const TaskId,
        buff_size: c_ulonglong,
    ) -> c_ulonglong;
    pub fn AsstCtrlerClick(handle: AsstHandle, x: c_int, y: c_int, block: bool) -> bool;
    pub fn AsstGetVersion() -> *const c_char;
    pub fn AsstLog(level: *const c_char, message: *const c_char);

}
