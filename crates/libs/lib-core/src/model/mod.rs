use store::{dbx::Dbx, new_db_pool};

pub mod admin;
pub mod book;
pub mod error;
pub mod general;
pub mod store;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {
    dbx: Dbx,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db_pool = new_db_pool()
            .await
            .map_err(|ex| Error::CantCreateModelManagerProvider(ex.to_string()))?;
        let dbx = Dbx::new(db_pool, false)?;

        Ok(ModelManager { dbx })
    }

    pub fn new_with_txn(&self) -> Result<ModelManager> {
        let dbx = Dbx::new(self.dbx.db().clone(), true)?;

        Ok(ModelManager { dbx })
    }

    pub fn dbx(&self) -> &Dbx {
        &self.dbx
    }
}
