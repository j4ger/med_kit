use rocket::request::FromParam;

use std::convert::Into;

use uuid::Uuid;

impl<'a> FromParam<'a> for UuidWrapper {
    type Error = uuid::Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(UuidWrapper(Uuid::parse_str(param)?))
    }
}

pub struct UuidWrapper(Uuid);

impl Into<Uuid> for UuidWrapper {
    fn into(self) -> Uuid {
        self.0
    }
}
