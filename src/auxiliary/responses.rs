use diesel::result::Error as DieselError;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;

use log::info;

#[derive(Debug)]
pub enum GenericError {
    DieselError(DieselError),
    ProductDuplicateError,
    ProductReuseError,
    TokenError,
    AuthError,
    ServerInternalError,
    UserNotExistError,
    PasswordIncorrectError,
    PasswordNotSetError,
    UserAlreadyExistsError,
    PermissionDeniedError,
    InvalidInputError,
    GetWechatAccessTokenError,
    GetWechatOpenIdError,
    GetWechatUserinfoError,
    ProfileNotExistError,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub error_code: Option<i32>,
}

#[derive(Serialize)]
pub struct SuccessResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn build(data: T) -> GenericResult<T> {
        Ok(Json(SuccessResponse {
            success: true,
            data,
        }))
    }
}

pub type GenericResult<T> = Result<Json<SuccessResponse<T>>, GenericError>;

impl<'a> Responder<'a, 'static> for GenericError {
    fn respond_to(self, req: &'a Request<'_>) -> response::Result<'static> {
        info!("{:?}", self);
        let error_message = match self {
            Self::DieselError(inner_error) => match inner_error {
                DieselError::NotFound => "请求的资源不存在",
                _ => "数据库错误",
            },
            Self::ProductDuplicateError => "产品重复初始化",
            Self::ProductReuseError => "产品已被使用",
            Self::TokenError => "Token解析错误",
            Self::AuthError => "用户认证错误",
            Self::ServerInternalError => "服务器内部错误",
            Self::UserNotExistError => "用户不存在",
            Self::PasswordIncorrectError => "密码错误",
            Self::PasswordNotSetError => "密码未设置",
            Self::UserAlreadyExistsError => "用户名已被占用",
            Self::PermissionDeniedError => "权限错误",
            Self::InvalidInputError => "输入不合法",
            Self::GetWechatAccessTokenError => "微信AccessToken获取失败",
            Self::GetWechatOpenIdError => "微信OpenId获取失败",
            Self::GetWechatUserinfoError => "微信Userinfo获取失败",
            Self::ProfileNotExistError => "档案未填写",
        }
        .to_string();
        let mut json_result = Json(ErrorResponse {
            success: false,
            message: error_message,
            error_code: None,
        })
        .respond_to(&req)
        .unwrap();
        json_result.set_status(Status::InternalServerError);
        Response::build_from(json_result).ok()
    }
}

impl From<DieselError> for GenericError {
    fn from(original_error: DieselError) -> Self {
        GenericError::DieselError(original_error)
    }
}
