use sqlx::{Execute, Postgres, QueryBuilder};

// Insert query using QueryBuilder
pub fn insert() -> String {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO tbl_admin");

    query.push(" (uname, email)");
    query.push(" VALUES ($1, $2)");
    query.push(" RETURNING id");

    query.build().sql().into()
}

// Get by username query using QueryBuilder
pub fn get_by_uname() -> String {
    let mut query: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT id, uname, email, pwd, pwd_salt, token_salt FROM tbl_admin");

    query.push(" WHERE uname = $1");

    query.build().sql().into()
}

// Select all query using QueryBuilder
pub fn select_all(has_id: bool) -> String {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT");

    query.push(
        " id, uname, email, pwd_salt, token_salt, admin_role, session_typ, admin_stat, cid, ctime, mid, mtime",
    );
    query.push(" FROM tbl_admin");
    if has_id {
        query.push(" WHERE id = $1");
    }

    query.build().sql().into()
}

// Update query using QueryBuilder
pub fn update() -> String {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE tbl_admin SET");

    query.push(" uname=$1, email=$2, mtime=NOW()");
    query.push(" WHERE id=$3");

    query.build().sql().into()
}

pub fn check_removed_acc() -> String {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT id, uname FROM");
    query.push(" tbl_acc_removed");
    query.push(" WHERE uname=$1, email=$2");

    query.build().sql().into()
}

// Update password query using QueryBuilder
pub fn update_pass() -> String {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE tbl_admin SET");

    query.push(" pwd=$1, mtime=NOW()");
    query.push(" WHERE id=$2");

    query.build().sql().into()
}

// Remove admin query using QueryBuilder
pub fn removed() -> String {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO tbl_acc_removed");

    query.push(" (admin_id, cid, mid) VALUES ($1, $2, $3)");
    query.push(" RETURNING id");

    query.build().sql().into()
}
