use rocket::request::FromParam;

pub struct ProductBarcode<'a>(&'a str);

impl<'a> FromParam<'a> for ProductBarcode<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if param.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(ProductBarcode(param))
        } else {
            Err(param)
        }
    }
}

impl<'a> ProductBarcode<'a> {
    pub fn inner(self) -> &'a str {
        self.0
    }
}
