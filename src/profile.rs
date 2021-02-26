#[derive(serde::Serialize, serde::Deserialize, Debug)]

pub struct Profile {
    pub uuid: String,
    pub init: bool,
    pub name: String,
    pub delete: bool,
    pub gender: String,
    pub time: String,
    pub phone: String,
    pub address: String,
    pub age: i64,
    pub email: String,
    pub hospital: String,
    pub firstTime: bool,
    pub profession: String,
}
