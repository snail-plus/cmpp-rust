use std::{fmt, io};
use std::error::Error;
use std::fmt::Display;

// 定义一个错误类型
#[derive(Debug)]
pub struct IoError {
    pub(crate) message: String,
}

impl Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// 实现 Error trait
impl Error for IoError {}

// 实现 Display trait，以便可以格式化错误消息
impl From<io::Error> for IoError {
    fn from(err: io::Error) -> Self {
        IoError { message: err.to_string() }
    }
}
