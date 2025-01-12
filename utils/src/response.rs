use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub msg: String,
    pub code: String,
    pub data: Option<T>,
}

// 枚举类来定义不同的代码和消息
pub enum BaseCodeEnum {
    SUCCESS,
    ERROR,
}

impl BaseCodeEnum {
    pub fn get_code(&self) -> &str {
        match self {
            BaseCodeEnum::SUCCESS => "200",
            BaseCodeEnum::ERROR => "500",
        }
    }

    pub fn get_msg(&self) -> &str {
        match self {
            BaseCodeEnum::SUCCESS => "Success",
            BaseCodeEnum::ERROR => "Error",
        }
    }
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Response {
            msg: BaseCodeEnum::SUCCESS.get_msg().to_string(),
            code: BaseCodeEnum::SUCCESS.get_code().to_string(),
            data: Some(data),
        }
    }

    pub fn fail(msg: String) -> Self {
        Response {
            msg,
            code: BaseCodeEnum::ERROR.get_code().to_string(),
            data: None,
        }
    }
}