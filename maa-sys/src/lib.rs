mod raw;
pub mod task;
 
use std::ffi::{CStr, CString};
use std::ptr::NonNull;
use std::sync::Once;

static INIT: Once = Once::new();


#[derive(Debug)]
pub enum Error {
    ResourceLoadFailed,
    CreateFailed,
}

/// MAA助手的主要结构体
pub struct Assistant {
    handle: NonNull<raw::AsstExtAPI>,
}

impl Assistant {
    /// 初始化MAA资源
    fn init_resources(resource_dir: &str) -> bool {
        let resource_path = CString::new(resource_dir).unwrap();
        unsafe { raw::AsstLoadResource(resource_path.as_ptr()) != 0 }
    }

    /// 创建一个新的Assistant实例
    ///
    /// # Arguments
    /// * `resource_dir` - 资源文件夹的路径
    ///
    /// # Returns
    /// * `Ok(Assistant)` - 创建成功
    /// * `Err(Error)` - 创建失败，可能是资源加载失败或实例创建失败
    pub fn new(resource_dir: &str) -> Result<Self, Error> {
        let mut success = true;
        INIT.call_once(|| {
            success = Self::init_resources(resource_dir);
        });

        if !success {
            return Err(Error::ResourceLoadFailed);
        }

        let handle = unsafe { raw::AsstCreate() };
        NonNull::new(handle)
            .map(|handle| Self { handle })
            .ok_or(Error::CreateFailed)
    }

    /// 创建一个带有回调函数的Assistant实例
    ///
    /// # Arguments
    /// * `resource_dir` - 资源文件夹的路径
    /// * `callback` - 回调函数
    /// * `custom_arg` - 自定义参数
    pub fn new_with_callback(
        resource_dir: &str,
        callback: raw::AsstApiCallback,
        custom_arg: *mut std::ffi::c_void,
    ) -> Result<Self, Error> {
        let mut success = true;
        INIT.call_once(|| {
            success = Self::init_resources(resource_dir);
        });

        if !success {
            return Err(Error::ResourceLoadFailed);
        }

        let handle = unsafe { raw::AsstCreateEx(callback, custom_arg) };
        NonNull::new(handle)
            .map(|handle| Self { handle })
            .ok_or(Error::CreateFailed)
    }

    /// 连接到设备
    pub fn connect(&mut self, adb_path: &str, address: &str, config: Option<&str>) -> bool {
        let adb_path = CString::new(adb_path).unwrap();
        let address = CString::new(address).unwrap();
        let config_str = config.map(|c| CString::new(c).unwrap());

        unsafe {
            raw::AsstAsyncConnect(
                self.handle.as_ptr(),
                adb_path.as_ptr(),
                address.as_ptr(),
                config_str.as_ref().map_or(std::ptr::null(), |cs| cs.as_ptr()),
                0,
            ) != 0
        }
    }

    /// 添加任务
    ///
    /// # Arguments
    /// * `task` - 实现了Task trait的任务类型
    ///
    /// # Returns
    /// * `i32` - 任务ID
    pub fn append_task(&mut self, task: impl task::Task) -> i32 {
        let type_str = CString::new(task.task_type()).unwrap();
        let params_str = CString::new(task.to_json()).unwrap();

        unsafe {
            raw::AsstAppendTask(
                self.handle.as_ptr(),
                type_str.as_ptr(),
                params_str.as_ptr(),
            )
        }
    }

    pub fn set_task_params(&mut self, task_id: i32, params: Option<&str>) -> bool {
        let params_str = params.map(|p| CString::new(p).unwrap());
        unsafe {
            raw::AsstSetTaskParams(
                self.handle.as_ptr(),
                task_id,
                params_str.as_ref().map_or(std::ptr::null(), |cs| cs.as_ptr()),
            ) != 0
        }
    }

    /// 启动助手
    pub fn start(&mut self) -> bool {
        unsafe { raw::AsstStart(self.handle.as_ptr()) != 0 }
    }

    /// 停止助手
    pub fn stop(&mut self) -> bool {
        unsafe { raw::AsstStop(self.handle.as_ptr()) != 0 }
    }

    /// 返回主页
    pub fn back_to_home(&mut self) -> bool {
        unsafe { raw::AsstBackToHome(self.handle.as_ptr()) != 0 }
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        unsafe { raw::AsstRunning(self.handle.as_ptr()) != 0 }
    }

    /// 检查是否已连接
    pub fn is_connected(&self) -> bool {
        unsafe { raw::AsstConnected(self.handle.as_ptr()) != 0 }
    }

    /// 获取版本信息
    pub fn version() -> &'static str {
        unsafe {
            CStr::from_ptr(raw::AsstGetVersion())
                .to_str()
                .unwrap_or("unknown")
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
