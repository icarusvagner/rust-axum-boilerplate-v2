use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use lib_auth::pass;
use time::OffsetDateTime;

use crate::{
    ctx::Ctx,
    model::{general::cid_mid::GeneralBmc, ModelManager},
};

use super::{
    AccRemovedForCreate, Admin, AdminForAuth, AdminForCreate, AdminForLogin, AdminRemoved,
    AdminUnameId, AdminWithDetailsForCreate, DetailsForCreate, DetailsForInsert, TblAccRemoved,
    TblAccRemovedHistory, TblAdmin, TblDetails,
};
use crate::model::{Error, Result};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow};

pub trait AdminBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl AdminBy for Admin {}
impl AdminBy for AdminForLogin {}
impl AdminBy for AdminForAuth {}
impl AdminBy for AdminUnameId {}
impl AdminBy for AdminRemoved {}

pub struct AdminBmc;

impl AdminBmc {
    pub async fn admin_insert(
        ctx: &Ctx,
        mm: &ModelManager,
        data: AdminWithDetailsForCreate,
    ) -> Result<i64> {
        let AdminForCreate { uname, email, pwd } = data.admin;
        let DetailsForCreate {
            first_name,
            last_name,
            birth_date,
        } = data.detail;

        if let Some(is_removed) = Self::check_removed_acc::<AdminRemoved>(
            ctx,
            mm,
            uname.clone().as_str(),
            email.clone().as_str(),
        )
        .await?
        {
            return Err(Error::AccountIsRemoved {
                uname: uname.clone(),
                email: email.clone(),
                id: is_removed.admin_id,
            });
        }

        let mm = mm.new_with_txn()?;

        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblAdmin::Table)
            .columns([TblAdmin::Uname, TblAdmin::Email])
            .values_panic([uname.clone().into(), email.into()])
            .returning_col(TblAdmin::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_admin" && constraint.contains("uname") {
                        Some(Error::UsernameAlreadyExists {
                            uname: uname.clone(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        let birth_date_parsed = NaiveDate::parse_from_str(birth_date.as_str(), "%Y-%m-%d")?;

        GeneralBmc::update_cid_mid::<TblAdmin>(ctx, &mm, id).await?;
        Self::update_pass(ctx, &mm, id, &pwd).await?;
        Self::insert_details(
            ctx,
            &mm,
            DetailsForInsert {
                admin_id: id,
                first_name,
                last_name,
                birth_date: birth_date_parsed,
            },
        )
        .await?;

        mm.dbx().commit_txn().await?;

        Ok(id)
    }

    pub async fn insert_details(
        ctx: &Ctx,
        mm: &ModelManager,
        data: DetailsForInsert,
    ) -> Result<()> {
        let DetailsForInsert {
            admin_id,
            first_name,
            last_name,
            birth_date,
        } = data;

        // Convert NaiveDate to NaiveDateTime and then to OffsetDateTime
        let birth_naive_datetime = NaiveDateTime::new(
            birth_date,
            NaiveTime::from_hms_opt(0, 0, 0).expect("Valid time"),
        );

        // Convert to OffsetDateTime and map error if necessary
        let birth_offset_datetime =
            OffsetDateTime::from_unix_timestamp(birth_naive_datetime.and_utc().timestamp())?; // Mapping the error here

        let (sql, values) = Query::insert()
            .into_table(TblDetails::Table)
            .columns([
                TblDetails::AdminId,
                TblDetails::FirstName,
                TblDetails::LastName,
                TblDetails::BirthDate,
            ])
            .values_panic([
                admin_id.into(),
                first_name.into(),
                last_name.into(),
                birth_offset_datetime.into(),
            ])
            .returning_col(TblDetails::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = mm
            .dbx()
            .fetch_one(sqlx_query)
            .await
            .map_err(|ex| Error::InsertionFailed {
                entity: TblDetails::Table.to_string(),
                cause: ex.to_string(),
            })?;

        GeneralBmc::update_cid_mid::<TblDetails>(ctx, mm, id).await?;

        Ok(())
    }

    pub async fn first_by_uname<E>(_ctx: &Ctx, mm: &ModelManager, uname: &str) -> Result<Option<E>>
    where
        E: AdminBy,
    {
        let (sql, values) = Query::select()
            .columns([
                TblAdmin::Id,
                TblAdmin::Uname,
                TblAdmin::Email,
                TblAdmin::Pwd,
                TblAdmin::PwdSalt,
                TblAdmin::TokenSalt,
            ])
            .from(TblAdmin::Table)
            .and_where(Expr::col(TblAdmin::Uname).eq(uname))
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
        let entity = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(entity)
    }

    pub async fn get<E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: AdminBy,
    {
        let (sql, values) = Query::select()
            .columns([
                TblAdmin::Id,
                TblAdmin::Uname,
                TblAdmin::Email,
                TblAdmin::Pwd,
                TblAdmin::PwdSalt,
                TblAdmin::TokenSalt,
                TblAdmin::AdminRole,
                TblAdmin::AdminStat,
                TblAdmin::Cid,
                TblAdmin::Ctime,
                TblAdmin::Mid,
                TblAdmin::Mtime,
            ])
            .from(TblAdmin::Table)
            .and_where(Expr::col(TblAdmin::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);

        let entity = mm
            .dbx()
            .fetch_optional(sqlx_query)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "tbl_admin",
                id,
            })?;

        Ok(entity)
    }

    pub async fn update_pass(ctx: &Ctx, mm: &ModelManager, id: i64, password: &str) -> Result<()> {
        let admin: AdminForLogin = Self::get(ctx, mm, id).await?;
        let pwd = pass::hash_pwd(pass::ContentToHash {
            content: password.to_string(),
            salt: admin.pwd_salt,
        })
        .await?;

        let (sql, values) = Query::update()
            .table(TblAdmin::Table)
            .values([(TblAdmin::Pwd, pwd.into())])
            .and_where(Expr::col(TblAdmin::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_with(&sql, values);

        let _count = mm.dbx().execute(sqlx_query).await?;

        Ok(())
    }

    pub async fn check_removed_acc<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        uname: &str,
        email: &str,
    ) -> Result<Option<E>>
    where
        E: AdminBy,
    {
        let (sql, values) = Query::select()
            .columns([
                TblAccRemoved::Id,
                TblAccRemoved::AdminId,
                TblAccRemoved::Uname,
                TblAccRemoved::Email,
                TblAccRemoved::Cid,
                TblAccRemoved::Ctime,
                TblAccRemoved::Mid,
                TblAccRemoved::Mtime,
            ])
            .from(TblAccRemoved::Table)
            .and_where(Expr::col(TblAccRemoved::Uname).eq(uname))
            .and_where(Expr::col(TblAccRemoved::Email).eq(email))
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
        let entity = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(entity)
    }

    pub async fn acc_to_removed(
        ctx: &Ctx,
        mm: &ModelManager,
        acc_fc: AccRemovedForCreate,
    ) -> Result<i64> {
        let AccRemovedForCreate {
            admin_id,
            uname,
            email,
        } = acc_fc;

        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let (sql, values) = Query::insert()
            .into_table(TblAccRemoved::Table)
            .columns([
                TblAccRemoved::AdminId,
                TblAccRemoved::Uname,
                TblAccRemoved::Email,
            ])
            .values_panic([admin_id.into(), uname.clone().into(), email.clone().into()])
            .returning_col(TblAccRemoved::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);

        let (id,) = mm
            .dbx()
            .fetch_one(sqlx_query)
            .await
            .map_err(|model_error| {
                Error::resolve_unique_violation(
                    Error::Dbx(model_error),
                    Some(|table: &str, constraint: &str| {
                        if table == TblAccRemoved::Table.to_string()
                            && constraint.contains("admin_id")
                        {
                            Some(Error::AdminAlreadyRemoved { admin_id })
                        } else {
                            None
                        }
                    }),
                )
            })?;

        GeneralBmc::update_cid_mid::<TblAccRemoved>(ctx, &mm, id).await?;

        mm.dbx().commit_txn().await?;

        Ok(id)
    }

    pub async fn acc_to_removed_history(
        ctx: &Ctx,
        mm: &ModelManager,
        acc_fc: AccRemovedForCreate,
    ) -> Result<i64> {
        let AccRemovedForCreate {
            admin_id,
            uname,
            email,
        } = acc_fc;

        let (sql, values) = Query::insert()
            .into_table(TblAccRemovedHistory::Table)
            .columns([
                TblAccRemovedHistory::AdminId,
                TblAccRemovedHistory::Uname,
                TblAccRemovedHistory::Email,
            ])
            .values_panic([admin_id.into(), uname.into(), email.into()])
            .returning_col(TblAccRemovedHistory::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);

        let (id,) = mm
            .dbx()
            .fetch_one(sqlx_query)
            .await
            .map_err(|model_error| {
                Error::resolve_unique_violation(
                    Error::Dbx(model_error),
                    Some(|table: &str, constraint: &str| {
                        if table == TblAccRemoved::Table.to_string()
                            && constraint.contains("admin_id")
                        {
                            Some(Error::AdminAlreadyRemoved { admin_id })
                        } else {
                            None
                        }
                    }),
                )
            })?;

        GeneralBmc::update_cid_mid::<TblAccRemovedHistory>(ctx, mm, id).await?;

        mm.dbx().commit_txn().await?;

        Ok(id)
    }
}
