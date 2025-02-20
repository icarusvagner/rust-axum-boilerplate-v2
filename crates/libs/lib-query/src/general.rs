use sqlx::{Execute, Postgres, QueryBuilder};

pub fn general_insert_cid_mid(table: &str) -> String {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE ");
    query.push(table);
    query.push(" SET cid=$1, mid=$2");
    query.push(" WHERE id = $3");

    query.build().sql().into()
}
