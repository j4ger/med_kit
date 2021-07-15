use diesel::result::Error as DieselError;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug)]
pub enum GenericError {
    DieselError(DieselError),
    ProductDuplicateError,
    ProductReuseError,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
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
        let error_message = match self {
            GenericError::DieselError(inner_error) => match inner_error {
                DieselError::NotFound => "请求的资源不存在".to_string(),
                _ => inner_error.to_string(),
            },
            GenericError::ProductDuplicateError => "产品重复初始化".to_string(),
            GenericError::ProductReuseError => "产品已被使用".to_string(),
        };
        Response::build_from(
            Json(ErrorResponse {
                success: false,
                message: error_message,
            })
            .respond_to(&req)
            .unwrap(),
        )
        .ok()
    }
}

impl From<DieselError> for GenericError {
    fn from(original_error: DieselError) -> Self {
        GenericError::DieselError(original_error)
    }
}
