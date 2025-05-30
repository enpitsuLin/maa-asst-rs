mod message;
mod raw;
pub mod task;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr::NonNull;
use std::sync::Once;
use thiserror::Error;

static INIT: Once = Once::new();

#[derive(Error, Debug)]
pub enum Error {
    #[error("资源加载失败")]
    ResourceLoadFailed,
    #[error("创建实例失败")]
    CreateFailed,
    #[error("连接失败")]
    ConnectFailed,
    #[error("任务添加失败")]
    TaskAppendFailed,
    #[error("任务参数设置失败")]
    TaskParamsSetFailed,
    #[error("启动失败")]
    StartFailed,
    #[error("停止失败")]
    StopFailed,
    #[error("返回主页失败")]
    BackToHomeFailed,
    #[error("点击失败")]
    ClickFailed,
    #[error("截图失败")]
    CaptureFailed,
    #[error("未知错误")]
    Unknown,
}

pub enum StaticOptionKey {
    /// 无效
    Invalid,
    /// 用CPU进行OCR
    CpuOCR,
    /// 用GPU进行OCR
    GpuOCR,
}

pub enum InstanceOptionKey {
    /// 已弃用
    Invalid = 0,
    /// 是否启用 minitouch
    /// 开了也不代表就一定能用，有可能设备不支持等
    /// "1" 开，"0" 关
    MinitouchEnabled = 1,
    /// 触控模式设置，默认 minitouch
    /// minitouch | maatouch | adb
    TouchMode = 2,
    /// 是否暂停下干员，同时影响抄作业、肉鸽、保全
    /// "1" | "0"
    DeploymentWithPause = 3,
    /// 是否使用 AdbLite， "0" | "1"
    AdbLiteEnabled = 4,
    /// 退出时是否杀掉 Adb 进程， "0" | "1"
    KillAdbOnExit = 5,
}

/// MAA助手的主要结构体
pub struct Assistant {
    handle: NonNull<raw::AsstExtAPI>,
    /// 连接的目标设备
    target: Option<String>,
    tasks: HashMap<i32, Box<dyn task::Task>>,
}

/// 将 Rust 的回调函数转换为 C 的回调函数
pub unsafe extern "C" fn callback_wrapper(
    msg_id: i32,
    details_json: *const ::std::os::raw::c_char,
    user_data: *mut ::std::os::raw::c_void,
) {
    let json_str = std::ffi::CStr::from_ptr(details_json).to_str().unwrap();
    let details: serde_json::Value = serde_json::from_str(json_str).unwrap();
    let processor = &mut *(user_data as *mut message::Processor);
    let asst_msg = message::AsstMsg::from(msg_id);
    (processor.callback)(asst_msg, details);
}

impl Assistant {
    fn load_resource<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        let path = path.as_ref();
        let resource_path = CString::new(path.to_string_lossy().as_ref()).unwrap();
        unsafe {
            if raw::AsstLoadResource(resource_path.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(Error::ResourceLoadFailed)
            }
        }
    }

    pub fn set_instance_option(
        &mut self,
        key: InstanceOptionKey,
        value: impl Into<String>,
    ) -> Result<(), Error> {
        let value_str = CString::new(value.into()).unwrap();
        unsafe {
            if raw::AsstSetInstanceOption(self.handle.as_ptr(), key as i32, value_str.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(Error::Unknown)
            }
        }
    }

    pub fn set_static_option(key: StaticOptionKey, value: impl Into<String>) -> Result<(), Error> {
        let value_str = CString::new(value.into()).unwrap();
        unsafe {
            if raw::AsstSetStaticOption(key as i32, value_str.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(Error::Unknown)
            }
        }
    }

    /// 创建一个新的Assistant实例
    ///
    /// # Arguments
    /// * `resource_dir` - 资源文件夹的路径
    ///
    /// # Returns
    /// * `Ok(Assistant)` - 创建成功
    /// * `Err(Error)` - 创建失败，可能是资源加载失败或实例创建失败
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        INIT.call_once(|| Self::load_resource(path).unwrap());

        let handle = unsafe { raw::AsstCreate() };
        NonNull::new(handle)
            .map(|handle| Self {
                handle,
                target: None,
                tasks: HashMap::new(),
            })
            .ok_or(Error::CreateFailed)
    }

    /// 创建一个带有回调函数的Assistant实例
    ///
    /// # Arguments
    /// * `resource_dir` - 资源文件夹的路径
    /// * `callback` - 回调函数，接收消息ID和JSON详情
    pub fn new_with_callback<P: AsRef<Path>>(
        path: P,
        callback: impl FnMut(message::AsstMsg, serde_json::Value) + Send + 'static,
    ) -> Result<Self, Error> {
        INIT.call_once(|| Self::load_resource(path).unwrap());

        let processor = message::Processor::from(callback);

        let processor_ptr = Box::into_raw(Box::new(processor));

        let handle = unsafe { raw::AsstCreateEx(Some(callback_wrapper), processor_ptr as *mut _) };
        NonNull::new(handle)
            .map(|handle| Self {
                handle,
                target: None,
                tasks: HashMap::new(),
            })
            .ok_or(Error::CreateFailed)
    }

    /// 连接到设备
    pub fn connect(&mut self, adb_path: &str, address: &str, config: Option<&str>) -> Result<(), Error> {
        let adb_path = CString::new(adb_path).unwrap();
        let address_cstr = CString::new(address).unwrap();
        let config_str = config.map(|c| CString::new(c).unwrap());

        unsafe {
            let ret = raw::AsstAsyncConnect(
                self.handle.as_ptr(),
                adb_path.as_ptr(),
                address_cstr.as_ptr(),
                config_str.as_ref().map_or(std::ptr::null(), |cs| cs.as_ptr()),
                1,
            );
            if ret != 0 {
                self.target = Some(address.to_string());
                Ok(())
            } else {
                Err(Error::ConnectFailed)
            }
        }
    }

    /// 添加任务
    ///
    /// # Arguments
    /// * `task` - 实现了Task trait的任务类型
    ///
    /// # Returns
    /// * `i32` - 任务ID
    pub fn append_task(&mut self, task: impl task::Task + 'static) -> Result<i32, Error> {
        let type_str = CString::new(task.task_type()).unwrap();
        let params_str = CString::new(task.to_json()).unwrap();

        unsafe {
            let task_id = raw::AsstAppendTask(self.handle.as_ptr(), type_str.as_ptr(), params_str.as_ptr());
            if task_id != 0 {
                self.tasks.insert(task_id, Box::from(task));
                Ok(task_id)
            } else {
                Err(Error::TaskAppendFailed)
            }
        }
    }

    pub fn set_task_params<T: task::Task + 'static>(&mut self, task_id: i32, task: T) -> Result<(), Error> {
        let params_str = CString::new(task.to_json()).unwrap();
        unsafe {
            let ret = raw::AsstSetTaskParams(
                self.handle.as_ptr(),
                task_id,
                params_str.as_ptr(),
            );
            if ret != 0 {
                if let Some(old_task) = self.tasks.get_mut(&task_id) {
                    *old_task = Box::new(task);
                }
                Ok(())
            } else {
                Err(Error::TaskParamsSetFailed)
            }
        }
    }

    /// 启动助手
    pub fn start(&mut self) -> Result<(), Error> {
        unsafe {
            if raw::AsstStart(self.handle.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(Error::StartFailed)
            }
        }
    }

    /// 停止助手
    pub fn stop(&mut self) -> Result<(), Error> {
        unsafe {
            if raw::AsstStop(self.handle.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(Error::StopFailed)
            }
        }
    }

    pub fn click(&mut self, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            if raw::AsstAsyncClick(self.handle.as_ptr(), x, y, 1) != 0 {
                Ok(())
            } else {
                Err(Error::ClickFailed)
            }
        }
    }

    pub fn capture_screenshot(&mut self) -> Result<(), Error> {
        unsafe {
            if raw::AsstAsyncScreencap(self.handle.as_ptr(), 1) != 0 {
                Ok(())
            } else {
                Err(Error::CaptureFailed)
            }
        }
    }

    /// 返回主页
    pub fn back_to_home(&mut self) -> Result<(), Error> {
        unsafe {
            if raw::AsstBackToHome(self.handle.as_ptr()) != 0 {
                Ok(())
            } else {
                Err(Error::BackToHomeFailed)
            }
        }
    }

    pub fn get_null_size() -> u64 {
        unsafe { raw::AsstGetNullSize() }
    }

    pub fn get_uuid(&self) -> Result<String, Error> {
        unsafe {
            let mut buff_size = 1024;
            loop {
                if buff_size > 1024 * 1024 {
                    return Err(Error::Unknown);
                }
                let mut buff: Vec<u8> = Vec::with_capacity(buff_size);
                let data_size = raw::AsstGetUUID(
                    self.handle.as_ptr(),
                    buff.as_mut_ptr() as *mut i8,
                    buff_size as u64,
                );
                if data_size == Self::get_null_size() {
                    buff_size = 2 * buff_size;
                    continue;
                }
                buff.set_len(data_size as usize);
                let ret = String::from_utf8_lossy(&buff).to_string();

                return Ok(ret);
            }
        }
    }

    pub fn get_tasks_list(&self) -> Result<Vec<&dyn task::Task>, Box<dyn std::error::Error>> {
        let mut list: Vec<i32> = Vec::with_capacity(1000);
        unsafe {
            let buff = list.as_mut_ptr();
            let data_size = raw::AsstGetTasksList(self.handle.as_ptr(), buff, list.capacity().try_into()?);
            list.set_len(data_size.try_into()?);
            list.shrink_to_fit();

            let ret = list
                .iter()
                .filter_map(|id| self.tasks.get(id))
                .map(|task| task.as_ref())
                .collect();

            Ok(ret)
        }
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        unsafe { raw::AsstRunning(self.handle.as_ptr()) != 0 }
    }

    /// 检查是否已连接
    pub fn is_connected(&self) -> bool {
        unsafe { raw::AsstConnected(self.handle.as_ptr()) != 0 }
    }

    /// 打印日志
    pub fn log(level: &str, message: &str) {
        let level_cstr = CString::new(level).unwrap();
        let message_cstr = CString::new(message).unwrap();
        unsafe { raw::AsstLog(level_cstr.as_ptr(), message_cstr.as_ptr()) }
    }

    /// 获取版本信息
    pub fn version() -> Result<String, Error> {
        unsafe {
            CStr::from_ptr(raw::AsstGetVersion())
                .to_str()
                .map(|s| s.to_string())
                .map_err(|_| Error::Unknown)
        }
    }
}

impl Drop for Assistant {
    fn drop(&mut self) {
        unsafe {
            raw::AsstDestroy(self.handle.as_ptr());
        }
    }
}

// 确保Assistant可以安全地在线程间传递
unsafe impl Send for Assistant {}
unsafe impl Sync for Assistant {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = Assistant::version().unwrap();
        assert_ne!(version, "");
    }

    #[test]
    fn test_get_uuid() {
        let assistant = Assistant::new(env!("MAA_RESOURCE_PATH")).unwrap();
        let uuid = assistant.get_uuid().unwrap();
        assert_ne!(uuid, "");
    }

    #[test]
    fn test_get_tasks_list() {
        let mut assistant = Assistant::new(env!("MAA_RESOURCE_PATH")).unwrap();
        assistant
            .append_task(
                task::StartUpTask::builder()
                    .enable(true)
                    .client_type("Official")
                    .start_game_enabled(true)
                    .account_name("123****4567")
                    .build(),
            )
            .unwrap();

        let tasks = assistant.get_tasks_list().unwrap();
        assert!(!tasks.is_empty());
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].task_type(), "StartUp");
    }

    #[test]
    fn test_connect_device() {
        let mut assistant = Assistant::new_with_callback(env!("MAA_RESOURCE_PATH"), |msg_id, details| {
            let details_json = details.as_object().unwrap();
            println!("收到回调: msg_id={:?}\n details={:?}", msg_id, details_json);
        })
        .unwrap();

        assistant
            .set_instance_option(InstanceOptionKey::TouchMode, "adb")
            .unwrap();
        assistant.connect("adb", "192.168.20.29:33767", None).unwrap();

        if !assistant.is_connected() {
            println!("connect failed");
            return;
        }

        assistant
            .append_task(
                task::StartUpTask::builder()
                    .enable(true)
                    .client_type("Official")
                    .start_game_enabled(true)
                    .build(),
            )
            .unwrap();

        assistant.start().unwrap();

        println!("should be running");
        std::thread::sleep(std::time::Duration::from_secs(60)); // 等待60秒

        assistant.stop().unwrap();
    }
}
