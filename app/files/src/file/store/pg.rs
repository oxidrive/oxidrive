use async_trait::async_trait;
use oxidrive_auth::account::AccountId;
use uuid::Uuid;

use crate::file::File;

use super::{ByNameError, FileMetadata, SaveFileError};

pub struct PgFileMetadata {
    pool: sqlx::PgPool,
}

impl PgFileMetadata {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FileMetadata for PgFileMetadata {
    async fn by_name(
        &self,
        owner_id: AccountId,
        file_name: &str,
    ) -> Result<Option<File>, ByNameError> {
        let file: Option<PgFile> = sqlx::query_as(
            r#"
select
  id,
  owner_id,
  name,
  content_type
from files
where owner_id = $1
  and name = $2
"#,
        )
        .bind(owner_id.as_uuid())
        .bind(file_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(ByNameError::wrap)?;

        Ok(file.map(File::from))
    }

    async fn save(&self, file: File) -> Result<File, SaveFileError> {
        sqlx::query(
            r#"
insert into files (
  id,
  owner_id,
  name,
  content_type
) values (
  $1,
  $2,
  $3,
  $4
) on conflict (id)
do update set
  name = excluded.name,
  content_type = excluded.content_type
"#,
        )
        .bind(file.id.as_uuid())
        .bind(file.owner_id.as_uuid())
        .bind(&file.name)
        .bind(&file.content_type)
        .execute(&self.pool)
        .await
        .map_err(SaveFileError::wrap)?;

        Ok(file)
    }
}

#[derive(sqlx::FromRow)]
struct PgFile {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    content_type: String,
}

impl From<PgFile> for File {
    fn from(file: PgFile) -> Self {
        Self {
            id: file.id.into(),
            owner_id: file.owner_id.into(),
            name: file.name,
            content_type: file.content_type,
        }
    }
}
