#![allow(clippy::all)]
// generated models are shit and make clippy sad :c

pub mod credentials;
pub mod error;
pub mod file;
pub mod file_list;
pub mod file_upload_response;
pub mod instance_setup_request;
pub mod instance_setup_request_admin;
pub mod instance_setup_response;
pub mod instance_status;
pub mod instance_status_status;
pub mod list_info;
pub mod password_credentials;
pub mod session;
pub mod session_request;

use std::fmt::Display;

pub use credentials::Credentials;
pub use error::*;
pub use file::*;
pub use file_list::*;
pub use file_upload_response::*;
pub use instance_setup_request::*;
pub use instance_setup_request_admin::*;
pub use instance_setup_response::*;
pub use instance_status::*;
pub use instance_status_status::*;
pub use list_info::*;
pub use password_credentials::PasswordCredentials;
pub use session::*;
pub use session_request::*;

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Database::Postgres => "postgresql",
            Database::Sqlite => "sqlite",
        }
        .fmt(f)
    }
}

impl Display for FileStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStorage::Filesystem => "filesystem",
            FileStorage::S3 => "s3",
        }
        .fmt(f)
    }
}
