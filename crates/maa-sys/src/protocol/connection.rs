#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub enum Connection {
    /// 使用 ADB 连接
    Adb(String, String),
    /// PlayCover 原生运行
    PlayCover(String),
}

impl Connection {
    /// 使用 ADB 创建连接
    pub fn adb<P, A>(path: P, address: A) -> Self 
    where
        P: Into<String>,
        A: Into<String>,
    {
        Connection::Adb(path.into(), address.into())
    }

    /// 使用 PlayCover 创建连接
    pub fn playcover<A>(address: A) -> Self 
    where
        A: Into<String>,
    {
        Connection::PlayCover(address.into())
    }

    pub fn adb_path(&self) -> Option<String> {
        match self {
            Connection::Adb(path, _) => Some(path.to_string()),
            _ => None,
        }
    }

    pub fn address(&self) -> Option<String> {
        match self {
            Connection::Adb(_, address) => Some(address.to_string()),
            Connection::PlayCover(address) => Some(address.to_string()),
        }
    }
}
