use serde::Serialize;

#[derive(Serialize)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}
