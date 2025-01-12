use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::{Deserializer, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoVO {
    #[serde(serialize_with = "serialize_long")]
    pub id: Option<i64>,                // 以字符串形式序列化的 Long 类型

    pub name: Option<String>,           // 用户昵称（别名 姓名）

    pub position: Option<String>,       // 职位

    pub email: Option<String>,          // 邮箱

    pub phone_number: Option<String>,   // 电话号

    pub description: Option<String>,    // 用户个人简介

    pub user_name: Option<String>,      // 用户名（登陆的账户）

    pub avatar: Option<String>,         // 用户头像地址

    pub system_language: Option<String>, // 用户语言

    pub token: Option<String>,          // 用户token

    pub expire: Option<i64>,            // 过期时间（Unix 时间戳）
}

fn serialize_long<S>(x: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 以字符串形式序列化 Long 类型
    match *x {
        Some(value) => serializer.serialize_str(&value.to_string()),
        None => serializer.serialize_none(),
    }
}

impl UserInfoVO {
    // 你可以手动实现一个 builder 模式，或者使用一些第三方库来简化这个过程（比如 `derive_builder`）。
    pub fn builder() -> UserInfoVOBuilder {
        UserInfoVOBuilder::default()
    }
}

#[derive(Default)]
pub struct UserInfoVOBuilder {
    id: Option<i64>,
    name: Option<String>,
    position: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    description: Option<String>,
    user_name: Option<String>,
    avatar: Option<String>,
    system_language: Option<String>,
    token: Option<String>,
    expire: Option<i64>,
}

impl UserInfoVOBuilder {
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn position(mut self, position: String) -> Self {
        self.position = Some(position);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn user_name(mut self, user_name: String) -> Self {
        self.user_name = Some(user_name);
        self
    }

    pub fn avatar(mut self, avatar: String) -> Self {
        self.avatar = Some(avatar);
        self
    }

    pub fn system_language(mut self, system_language: String) -> Self {
        self.system_language = Some(system_language);
        self
    }

    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn expire(mut self, expire: i64) -> Self {
        self.expire = Some(expire);
        self
    }

    pub fn build(self) -> UserInfoVO {
        UserInfoVO {
            id: self.id,
            name: self.name,
            position: self.position,
            email: self.email,
            phone_number: self.phone_number,
            description: self.description,
            user_name: self.user_name,
            avatar: self.avatar,
            system_language: self.system_language,
            token: self.token,
            expire: self.expire,
        }
    }
}
