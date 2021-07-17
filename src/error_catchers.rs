use rocket::http::Status;
use rocket::request::Request;
use rocket::serde::json::Json;

use crate::responses::ErrorResponse;

#[catch(default)]
pub fn default_error_catcher(status: Status, _request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        success: false,
        message: status.reason().unwrap_or("服务器内部错误").to_string(),
        error_code: Some(status.code as i32),
    })
}
