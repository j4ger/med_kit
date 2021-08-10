mod error_catchers;
mod product;
mod profile;
mod reports;
mod user;
mod wechat_validation;

use error_catchers::*;
use product::*;
use profile::*;
use reports::*;
use user::*;
use wechat_validation::*;

use rocket::{Catcher, Route};

pub fn user_routes() -> Vec<Route> {
    routes![
        register,
        login,
        verify_login,
        get_users,
        get_all_users,
        change_user_role,
        remove_user,
        logout,
        change_password,
        wechat_login,
        get_user_statistics
    ]
}

pub fn product_routes() -> Vec<Route> {
    routes![
        init_product,
        get_product_digest,
        get_products,
        get_filtered_products,
        get_product,
        get_profile_by_product,
        submit_sample_time,
        get_product_statistics
    ]
}

pub fn profile_routes() -> Vec<Route> {
    routes![
        submit_profile_then_update,
        get_profile,
        submit_profile,
        get_profile_by_user,
        bind_profile,
        get_profile_statistics,
        get_profiles
    ]
}

pub fn report_routes() -> Vec<Route> {
    routes![
        upload_report,
        get_reports,
        get_filtered_reports,
        remove_report,
        get_report,
        publish_report,
    ]
}

pub fn wechat_validation_routes() -> Vec<Route> {
    routes![]
}

pub fn api_error_catchers() -> Vec<Catcher> {
    catchers![default_error_catcher]
}
