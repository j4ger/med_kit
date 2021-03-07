#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub pwhashed: String,
}
