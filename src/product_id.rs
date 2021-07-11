use rocket::request::FromParam;

pub struct ProductId<'a>(&'a str);

impl<'a> FromParam<'a> for ProductId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if param.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(ProductId(param))
        } else {
            Err(param)
        }
    }
}
