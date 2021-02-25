#[derive(serde::Serialize, serde::Deserialize)]

pub struct Profile {
    pub uuid: String,
    pub init: bool,
    pub name: String,
    pub delete: bool,
}
