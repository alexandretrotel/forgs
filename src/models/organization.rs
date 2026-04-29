use serde::Serialize;

#[derive(Serialize)]
pub struct OrganizationRank {
    pub name: String,
    pub followers: u32,
    pub organization_url: String,
}
