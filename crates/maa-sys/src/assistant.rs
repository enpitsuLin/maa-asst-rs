use crate::types::*;
use hashbrown::HashMap;
use std::env::consts::OS;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr::NonNull;
use std::sync::OnceLock;

use crate::{
    binding,
    protocol::{message, task},
};

static SHARED_LIBRARY: OnceLock<binding::MaaCore> = OnceLock::new();

// 一张 720p 图像，24位色深，原始大小为 1280 * 720 * 3（2.7 MB）
// 压缩后的图像数据应小于原始大小。
// 在大多数情况下，4MB 应该足够
const INIT_SIZE: usize = 1024 * 1024 * 4;
// 32MB 应该足够用于 4K 原始图像，但实际使用中可能不需要这么大
const MAX_SIZE: usize = 1024 * 1024 * 32;

/// MAA助手的主要结构体
/// 负责管理与设备的连接、任务执行和资源控制
pub struct Assistant {
    /// 指向底层C++ API的指针
    handle: NonNull<binding::AsstExtAPI>,
    /// 当前连接的设备地址，如果未连接则为None
    target: Option<String>,
    /// 存储所有已添加的任务，键为任务ID
    tasks: HashMap<i32, Box<dyn task::Task>>,
}

/// 将Rust的回调函数转换为C的回调函数
///
/// # Arguments
/// * `msg_id` - 消息ID
/// * `details_json` - JSON格式的消息详情
/// * `user_data` - 用户数据指针，指向消息处理器
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
    /// 加载 MAA 助手运行库文件
    ///
    /// # Arguments
    /// * `path` - 运行库文件的路径
    ///
    /// # Returns
    /// * `Ok(())` - 加载成功
    /// * `Err(Error::LibraryLoadFailed)` - 加载失败
    fn load_library<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        let dylib_path = path.as_ref().join(match OS {
            "macos" => "libMaaCore.dylib",
            "windows" => "MaaCore.dll",
            "linux" => "libMaaCore.so",
            _ => return Err(Error::LibraryLoadFailed),
        });
        let lib = unsafe {
            binding::MaaCore::new(dylib_path)
                .map_err(|_| Error::LibraryLoadFailed)
                .unwrap()
        };
        SHARED_LIBRARY.set(lib).map_err(|_| Error::LibraryLoadFailed)
    }

    fn try_run<F, R>(f: F) -> Result<R, Error>
    where
        F: FnOnce(&binding::MaaCore) -> R,
    {
        match SHARED_LIBRARY.get() {
            Some(lib) => Ok(f(lib)),
            None => Err(Error::LibraryLoadFailed),
        }
    }

    /// 加载MAA助手所需的资源文件
    ///
    /// # Arguments
    /// * `path` - 资源文件夹的路径
    ///
    /// # Returns
    /// * `Ok(())` - 资源加载成功
    /// * `Err(Error::ResourceLoadFailed)` - 资源加载失败
    fn load_resource<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        let path = path.as_ref();
        let resource_path = CString::new(path.to_string_lossy().as_ref()).unwrap();
        let ret = unsafe { Self::try_run(|lib| lib.AsstLoadResource(resource_path.as_ptr()))? };

        if ret != 0 {
            Ok(())
        } else {
            Err(Error::ResourceLoadFailed)
        }
    }

    /// 设置实例级别的选项
    ///
    /// # Arguments
    /// * `key` - 选项键
    /// * `value` - 选项值
    ///
    /// # Returns
    /// * `Ok(())` - 设置成功
    /// * `Err(Error::SetInstanceOptionFailed)` - 设置失败
    pub fn set_instance_option(
        &mut self,
        key: InstanceOptionKey,
        value: impl Into<String>,
    ) -> Result<(), Error> {
        let value_str = CString::new(value.into()).unwrap();
        let ret = unsafe {
            Self::try_run(|lib| {
                lib.AsstSetInstanceOption(self.handle.as_ptr(), key as i32, value_str.as_ptr())
            })?
        };
        if ret != 0 {
            Ok(())
        } else {
            Err(Error::SetInstanceOptionFailed)
        }
    }

    /// 设置全局静态选项
    ///
    /// # Arguments
    /// * `key` - 选项键
    /// * `value` - 选项值
    ///
    /// # Returns
    /// * `Ok(())` - 设置成功
    /// * `Err(Error::SetStaticOptionFailed)` - 设置失败
    pub fn set_static_option(key: StaticOptionKey, value: impl Into<String>) -> Result<(), Error> {
        let value_str = CString::new(value.into()).unwrap();
        let ret = Self::try_run(|lib| unsafe { lib.AsstSetStaticOption(key as i32, value_str.as_ptr()) })?;

        if ret != 0 {
            Ok(())
        } else {
            Err(Error::SetStaticOptionFailed)
        }
    }

    /// 创建一个新的Assistant实例
    ///
    /// # Arguments
    /// * `path` - 资源文件夹的路径
    ///
    /// # Returns
    /// * `Ok(Assistant)` - 创建成功
    /// * `Err(Error::CreateFailed)` - 创建失败
    /// * `Err(Error::ResourceLoadFailed)` - 资源加载失败
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::load_library(path.as_ref())?;
        Self::load_resource(path)?;

        let handle = unsafe { Self::try_run(|lib| lib.AsstCreate())? };
        NonNull::new(handle)
            .map(|handle: NonNull<binding::AsstExtAPI>| Self {
                handle,
                target: None,
                tasks: HashMap::new(),
            })
            .ok_or(Error::CreateFailed)
    }

    /// 创建一个带有回调函数的Assistant实例
    ///
    /// # Arguments
    /// * `path` - 资源文件夹的路径
    /// * `callback` - 回调函数，用于处理助手发送的消息
    ///
    /// # Returns
    /// * `Ok(Assistant)` - 创建成功
    /// * `Err(Error::CreateFailed)` - 创建失败
    /// * `Err(Error::ResourceLoadFailed)` - 资源加载失败
    pub fn new_with_callback<
        P: AsRef<Path>,
        F: FnMut(message::AsstMsg, serde_json::Value) + Send + 'static,
    >(
        path: P,
        callback: F,
    ) -> Result<Self, Error> {
        Self::load_library(path.as_ref())?;
        Self::load_resource(path)?;

        let processor = message::Processor::from(callback);

        let processor_ptr = Box::into_raw(Box::new(processor));

        let handle = unsafe {
            Self::try_run(|lib| lib.AsstCreateEx(Some(callback_wrapper), processor_ptr as *mut _))?
        };

        NonNull::new(handle)
            .map(|handle| Self {
                handle,
                target: None,
                tasks: HashMap::new(),
            })
            .ok_or(Error::CreateFailed)
    }

    /// 连接到指定的设备
    ///
    /// # Arguments
    /// * `adb_path` - ADB可执行文件的路径
    /// * `address` - 设备地址（如：127.0.0.1:5555）
    /// * `config` - 可选的连接配置
    ///
    /// # Returns
    /// * `Ok(())` - 连接成功
    /// * `Err(Error::ConnectFailed)` - 连接失败
    pub fn connect(&mut self, adb_path: &str, address: &str, config: Option<&str>) -> Result<(), Error> {
        let adb_path = CString::new(adb_path).unwrap();
        let address_cstr = CString::new(address).unwrap();
        let config_str = config.map(|c| CString::new(c).unwrap());

        let ret = unsafe {
            Self::try_run(|lib| {
                lib.AsstAsyncConnect(
                    self.handle.as_ptr(),
                    adb_path.as_ptr(),
                    address_cstr.as_ptr(),
                    config_str.as_ref().map_or(std::ptr::null(), |cs| cs.as_ptr()),
                    1,
                )
            })?
        };
        if ret != 0 {
            self.target = Some(address.to_string());
            Ok(())
        } else {
            Err(Error::ConnectFailed)
        }
    }

    /// 添加新的任务到任务队列
    ///
    /// # Arguments
    /// * `task` - 实现了Task trait的任务实例
    ///
    /// # Returns
    /// * `Ok(i32)` - 任务ID
    /// * `Err(Error::TaskAppendFailed)` - 任务添加失败
    pub fn append_task<T: task::Task + 'static>(&mut self, task: T) -> Result<i32, Error> {
        let type_str = CString::new(task.task_type()).unwrap();
        let params_str = CString::new(task.to_json()).unwrap();

        unsafe {
            let task_id = Self::try_run(|lib| {
                let asst_append_task = lib.AsstAppendTask.as_ref().unwrap();
                asst_append_task(self.handle.as_ptr(), type_str.as_ptr(), params_str.as_ptr())
            })?;
            if task_id != 0 {
                self.tasks.insert(task_id, Box::from(task));
                Ok(task_id)
            } else {
                Err(Error::TaskAppendFailed)
            }
        }
    }

    /// 更新已存在任务的参数
    ///
    /// # Arguments
    /// * `task_id` - 要更新的任务ID
    /// * `task` - 新的任务参数
    ///
    /// # Returns
    /// * `Ok(())` - 更新成功
    /// * `Err(Error::TaskParamsSetFailed)` - 更新失败
    pub fn set_task_params<T: task::Task + 'static>(&mut self, task_id: i32, task: T) -> Result<(), Error> {
        let params_str = CString::new(task.to_json()).unwrap();
        unsafe {
            let ret = Self::try_run(|lib| {
                lib.AsstSetTaskParams(self.handle.as_ptr(), task_id, params_str.as_ptr())
            })?;
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

    /// 启动助手开始执行任务
    ///
    /// # Returns
    /// * `Ok(())` - 启动成功
    /// * `Err(Error::StartFailed)` - 启动失败
    pub fn start(&mut self) -> Result<(), Error> {
        unsafe {
            if Self::try_run(|lib| lib.AsstStart(self.handle.as_ptr()))? != 0 {
                Ok(())
            } else {
                Err(Error::StartFailed)
            }
        }
    }

    /// 停止助手执行任务
    ///
    /// # Returns
    /// * `Ok(())` - 停止成功
    /// * `Err(Error::StopFailed)` - 停止失败
    pub fn stop(&mut self) -> Result<(), Error> {
        unsafe {
            if Self::try_run(|lib| lib.AsstStop(self.handle.as_ptr()))? != 0 {
                Ok(())
            } else {
                Err(Error::StopFailed)
            }
        }
    }

    /// 在指定坐标执行点击操作
    ///
    /// # Arguments
    /// * `x` - 点击的X坐标
    /// * `y` - 点击的Y坐标
    ///
    /// # Returns
    /// * `Ok(())` - 点击成功
    /// * `Err(Error::ClickFailed)` - 点击失败
    pub fn click(&mut self, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            if Self::try_run(|lib| lib.AsstAsyncClick(self.handle.as_ptr(), x, y, 1))? != 0 {
                Ok(())
            } else {
                Err(Error::ClickFailed)
            }
        }
    }

    /// 捕获当前屏幕截图
    ///
    /// # Returns
    /// * `Ok(())` - 截图成功
    /// * `Err(Error::CaptureFailed)` - 截图失败
    pub fn capture_screenshot(&self) -> Result<(), Error> {
        unsafe {
            if Self::try_run(|lib| {
                let asst_async_screencap = lib.AsstAsyncScreencap.as_ref().unwrap();
                asst_async_screencap(self.handle.as_ptr(), 1)
            })? != 0
            {
                Ok(())
            } else {
                Err(Error::CaptureFailed)
            }
        }
    }

    fn get_image_with_buf(&self, buf: *mut u8, size: usize) -> Result<binding::AsstSize, Error> {
        unsafe {
            let ret = Self::try_run(|lib| {
                let asst_get_image = lib.AsstGetImage.as_ref().unwrap();
                asst_get_image(
                    self.handle.as_ptr(),
                    buf as *mut std::os::raw::c_void,
                    size as binding::AsstSize,
                )
            })?;

            if ret != 0 {
                Ok(ret)
            } else {
                Err(Error::CaptureFailed)
            }
        }
    }

    pub fn get_image(&self) -> Result<Vec<u8>, Error> {
        let mut buf_size = INIT_SIZE;
        let mut buf = Vec::with_capacity(buf_size);

        loop {
            match self.get_image_with_buf(buf.as_mut_ptr(), buf_size) {
                Ok(size) => {
                    // Safety: the buffer is initialized by FFI, the size is the actual size
                    unsafe { buf.set_len(size as usize) };
                    return Ok(buf);
                },
                Err(_) => {
                    if buf_size > MAX_SIZE {
                        return Err(Error::ContentTooLarge(MAX_SIZE));
                    }
                    // Double the buffer size if it's not enough
                    buf_size *= 2;
                    buf.reserve(buf_size);
                },
            }
        }
    }

    /// 返回游戏主页
    ///
    /// # Returns
    /// * `Ok(())` - 返回成功
    /// * `Err(Error::BackToHomeFailed)` - 返回失败
    pub fn back_to_home(&mut self) -> Result<(), Error> {
        unsafe {
            if Self::try_run(|lib| {
                let asst_back_to_home = lib.AsstBackToHome.as_ref().unwrap();
                asst_back_to_home(self.handle.as_ptr())
            })? != 0
            {
                Ok(())
            } else {
                Err(Error::BackToHomeFailed)
            }
        }
    }

    /// 获取空值的大小
    pub fn get_null_size(&self) -> Result<u64, Error> {
        unsafe { Self::try_run(|lib| lib.AsstGetNullSize()) }
    }

    /// 获取当前实例的UUID
    ///
    /// # Returns
    /// * `Ok(String)` - UUID字符串
    /// * `Err(Error::Unknown)` - 获取失败
    pub fn get_uuid(&self) -> Result<String, Error> {
        unsafe {
            let mut buff_size = 1024;
            loop {
                if buff_size > 1024 * 1024 {
                    return Err(Error::Unknown);
                }
                let mut buff: Vec<u8> = Vec::with_capacity(buff_size);
                let data_size = Self::try_run(|lib| {
                    let asst_get_uuid = lib.AsstGetUUID.as_ref().unwrap();
                    asst_get_uuid(
                        self.handle.as_ptr(),
                        buff.as_mut_ptr() as *mut i8,
                        buff_size as u64,
                    )
                })?;
                if data_size == self.get_null_size()? {
                    buff_size = 2 * buff_size;
                    continue;
                }
                buff.set_len(data_size as usize);
                let ret = String::from_utf8_lossy(&buff).to_string();

                return Ok(ret);
            }
        }
    }

    /// 获取当前所有任务的列表
    ///
    /// # Returns
    /// * `Ok(Vec<&dyn task::Task>)` - 任务列表
    /// * `Err(Box<dyn std::error::Error>)` - 获取失败
    pub fn get_tasks_list(&self) -> Result<Vec<&dyn task::Task>, Box<dyn std::error::Error>> {
        let mut list: Vec<i32> = Vec::with_capacity(1000);
        unsafe {
            let buff = list.as_mut_ptr();
            let data_size = Self::try_run(|lib| {
                let asst_get_tasks_list = lib.AsstGetTasksList.as_ref().unwrap();
                asst_get_tasks_list(self.handle.as_ptr(), buff, list.capacity().try_into().unwrap())
            })?;
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

    /// 检查助手是否正在运行
    ///
    /// # Returns
    /// * `true` - 正在运行
    /// * `false` - 未运行
    pub fn is_running(&self) -> bool {
        unsafe {
            match Self::try_run(|lib| {
                let asst_running = lib.AsstRunning.as_ref().unwrap();
                asst_running(self.handle.as_ptr())
            }) {
                Ok(ret) => ret != 0,
                Err(_) => false,
            }
        }
    }

    /// 检查是否已连接到设备
    ///
    /// # Returns
    /// * `true` - 已连接
    /// * `false` - 未连接
    pub fn is_connected(&self) -> bool {
        unsafe {
            match Self::try_run(|lib| {
                let asst_connected = lib.AsstConnected.as_ref().unwrap();
                asst_connected(self.handle.as_ptr())
            }) {
                Ok(ret) => ret != 0,
                Err(_) => false,
            }
        }
    }

    /// 打印日志信息
    ///
    /// # Arguments
    /// * `level` - 日志级别
    /// * `message` - 日志消息
    pub fn log(&self, level: &str, message: &str) -> Result<(), Error> {
        let level_cstr = CString::new(level).unwrap();
        let message_cstr = CString::new(message).unwrap();
        unsafe {
            Self::try_run(|lib| {
                let asst_log = lib.AsstLog.as_ref().unwrap();
                asst_log(level_cstr.as_ptr(), message_cstr.as_ptr())
            })
        }
    }

    /// 获取MAA助手的版本信息
    ///
    /// # Returns
    /// * `Ok(String)` - 版本号
    /// * `Err(Error::Unknown)` - 获取失败
    pub fn version(&self) -> Result<String, Error> {
        unsafe {
            CStr::from_ptr(Self::try_run(|lib| {
                let asst_get_version = lib.AsstGetVersion.as_ref().unwrap();
                asst_get_version()
            })?)
            .to_str()
            .map(|s| s.to_string())
            .map_err(|_| Error::Unknown)
        }
    }
}

/// 实现Drop trait，确保资源正确释放
impl Drop for Assistant {
    fn drop(&mut self) {
        unsafe {
            Self::try_run(|lib| {
                let asst_destroy = lib.AsstDestroy.as_ref().unwrap();
                asst_destroy(self.handle.as_ptr());
            })
            .unwrap();
        }
    }
}

/// 确保Assistant可以安全地在线程间传递
unsafe impl Send for Assistant {}
unsafe impl Sync for Assistant {}
