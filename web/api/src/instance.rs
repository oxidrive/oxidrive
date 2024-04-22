#[derive(Debug)]
pub struct StatusResponse {
    pub status: Status,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Status {
    pub database: String,
    pub file_storage: String,
    pub public_url: String,
    pub setup_completed: bool,
}
