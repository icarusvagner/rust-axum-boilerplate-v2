mod admin_crud;
pub mod login_logs;
pub mod roles_crud;

pub use admin_crud::*;
use chrono::NaiveDate;
use sea_query::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Iden)]
pub enum TblAdmin {
    Table,
    Id,
    Uname,
    Email,
    Pwd,
    PwdSalt,
    TokenSalt,
    AdminRole,
    AdminStat,
    Cid,
    Ctime,
    Mid,
    Mtime,
}

#[derive(Iden)]
pub enum TblDetails {
    Table,
    Id,
    AdminId,
    FirstName,
    LastName,
    BirthDate,
    Cid,
    Ctime,
    Mid,
    Mtime,
}

#[derive(Iden)]
pub enum TblAccRemoved {
    Table,
    Id,
    AdminId,
    Uname,
    Email,
    Cid,
    Ctime,
    Mid,
    Mtime,
}

#[derive(Iden)]
pub enum TblAccRemovedHistory {
    Table,
    Id,
    AdminId,
    Uname,
    Email,
    Cid,
    Ctime,
    Mid,
    Mtime,
}

#[derive(Debug, Clone, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "admin_typ")]
pub enum AdminTyp {
    Super,
    Admin,
    User,
    Sub,
}

#[derive(Debug, Clone, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "session_typ")]
pub enum SessionTyp {
    Active,
    Inactive,
}

#[derive(Debug, Clone, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "admin_stat")]
pub enum AdminStat {
    Active,
    Inactive,
    Busy,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct Admin {
    pub id: i64,
    pub uname: String,
    pub email: String,
    pub pwd_salt: String,
    pub token_salt: String,
    pub admin_role: AdminTyp,
    pub session_typ: SessionTyp,
    pub admin_stat: AdminStat,
    pub cid: i64,
    pub ctime: String,
    pub mid: i64,
    pub mtime: String,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct Details {
    pub id: i64,
    pub admin_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub cid: i64,
    pub ctime: String,
    pub mid: i64,
    pub mtime: String,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct AccRemoved {
    pub id: i64,
    pub admin_id: i64,
    pub cid: i64,
    pub ctime: String,
    pub mid: i64,
    pub mtime: String,
}

#[derive(Serialize, FromRow)]
pub struct AdminUnameId {
    pub id: i64,
    pub uname: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct AdminForInsert {
    pub uname: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct AccRemovedForCreate {
    pub admin_id: i64,
    pub uname: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct AccRemovedForInsert {
    pub admin_id: i64,
    pub uname: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct DetailsForInsert {
    pub admin_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
}

#[derive(Deserialize)]
pub struct AdminForCreate {
    pub uname: String,
    pub email: String,
    pub pwd: String,
}

#[derive(Deserialize)]
pub struct DetailsForCreate {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
}

#[derive(Deserialize)]
pub struct AdminWithDetailsForCreate {
    pub detail: DetailsForCreate,
    pub admin: AdminForCreate,
}

#[derive(Serialize)]
pub struct AdminForUnameEmailUpdate {
    pub uname: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct AdminUpdateForPwdToken {
    pub pwd: String,
    pub pwd_salt: String,
    pub token_salt: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminRemoved {
    pub admin_id: i64,
    pub uname: String,
    pub email: String,
    pub cid: String,
    pub mid: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct AdminForLogin {
    pub id: i64,
    pub uname: String,
    pub email: String,

    pub pwd: Option<String>,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Debug)]
pub struct AdminForAuth {
    pub id: i64,
    pub username: String,

    // -- token info
    pub token_salt: Uuid,
}
