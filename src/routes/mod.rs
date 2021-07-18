mod error_catchers;
mod product;
mod profile;
mod user;

use error_catchers::*;
use product::*;
use profile::*;
use user::*;

use rocket::{Catcher, Route};

pub fn api_routes() -> Vec<Route> {
    routes![
        get_all_profiles,
        init_product,
        get_product_digest,
        submit_profile,
        get_profile,
        register,
        login,
        verify_login
    ]
}

pub fn api_error_catchers() -> Vec<Catcher> {
    catchers![default_error_catcher]
}
