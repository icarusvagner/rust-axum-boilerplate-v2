use crate::model::error::Result;
use crate::{ctx::Ctx, model::ModelManager};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;

use super::TableModel;

pub struct GeneralBmc;

impl GeneralBmc {
    pub async fn update_cid_mid<E>(_ctx: &Ctx, mm: &ModelManager, gen_id: i64) -> Result<()>
    where
        E: TableModel + 'static,
    {
        let (sql, values) = Query::update()
            .table(E::TABLE)
            .values([(E::CID, gen_id.into()), (E::MID, gen_id.into())])
            .and_where(Expr::col(E::ID).eq(gen_id))
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_with(&sql, values);

        let _ = mm.dbx().execute(sqlx_query).await?;

        Ok(())
    }
}
