use serde::Serialize;

use crate::models::organization::OrganizationRank;
use crate::models::repository::Repository;

#[derive(Serialize)]
pub struct ScanResult {
    pub repository: Repository,
    pub organizations: Vec<OrganizationRank>,
}
