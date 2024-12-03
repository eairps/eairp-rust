use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::naive_datetime_format;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SysUser {
    pub id: Option<i64>,                 // 对应 Long 类型
    pub name: Option<String>,            // 用户姓名
    pub user_name: Option<String>,       // 登录用户名
    pub avatar: Option<String>,          // 头像
    pub password: Option<String>,        // 登录密码
    pub leader_flag: Option<bool>,       // 是否经理
    pub position: Option<String>,        // 职位
    pub email: Option<String>,           // 电子邮件
    pub phone_number: Option<String>,    // 手机号码
    pub system_language: Option<String>, // 系统语言
    pub is_manager: Option<i32>,         // 是否为管理者 0==管理者 1==员工
    pub is_system: Option<i32>,          // 是否系统自带数据
    pub status: Option<i32>,             // 状态 0：正常，1：删除，2：封禁
    pub description: Option<String>,     // 用户描述
    pub remark: Option<String>,          // 备注
    pub wechat_open_id: Option<String>,  // 微信绑定
    pub tenant_id: Option<i64>,          // 租户id

    // NaiveDateTime添加序列化和反序列化处理
    #[serde(with = "naive_datetime_format")]
    pub create_time: Option<NaiveDateTime>, // 创建时间

    #[serde(with = "naive_datetime_format")]
    pub update_time: Option<NaiveDateTime>, // 更新时间

    pub create_by: Option<i64>,          // 创建人
    pub update_by: Option<i64>,          // 修改人
    pub delete_flag: Option<i32>,        // 删除标记
}

impl SysUser {
    pub fn new(
        id: Option<i64>,
        name: Option<String>,
        user_name: Option<String>,
        avatar: Option<String>,
        password: Option<String>,
        leader_flag: Option<bool>,
        position: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
        system_language: Option<String>,
        is_manager: Option<i32>,
        is_system: Option<i32>,
        status: Option<i32>,
        description: Option<String>,
        remark: Option<String>,
        wechat_open_id: Option<String>,
        tenant_id: Option<i64>,
        create_time: Option<NaiveDateTime>,
        update_time: Option<NaiveDateTime>,
        create_by: Option<i64>,
        update_by: Option<i64>,
        delete_flag: Option<i32>,
    ) -> Self {
        SysUser {
            id,
            name,
            user_name,
            avatar,
            password,
            leader_flag,
            position,
            email,
            phone_number,
            system_language,
            is_manager,
            is_system,
            status,
            description,
            remark,
            wechat_open_id,
            tenant_id,
            create_time,
            update_time,
            create_by,
            update_by,
            delete_flag,
        }
    }
}