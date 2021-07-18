mod error_catchers;
mod product;
mod profile;
mod user;

use error_catchers::*;
use product::*;
use profile::*;
use user::*;

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
        change_password
    ]
}

pub fn product_routes() -> Vec<Route> {
    routes![
        init_product,
        get_product_digest,
        get_products,
        get_filtered_products
    ]
}

pub fn profile_routes() -> Vec<Route> {
    routes![submit_profile, get_profile,]
}

pub fn api_error_catchers() -> Vec<Catcher> {
    catchers![default_error_catcher]
}
