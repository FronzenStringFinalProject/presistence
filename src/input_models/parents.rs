#[derive(Debug, serde::Serialize, serde::Deserialize, typed_builder::TypedBuilder)]
pub struct NewParent {
    pub identity: String,
    pub name: String,
    pub password: String,
    pub secret: String,
}
