use crate::error::{Error, Result};
use axum::extract::State;
use axum::Json;
use lib_core::{
    ctx::Ctx,
    model::{
        admin::{
            self, AccRemovedForCreate, AdminForCreate, AdminWithDetailsForCreate, DetailsForCreate,
        },
        ModelManager,
    },
};
use serde::Deserialize;
use serde_json::{json, Value};

pub async fn api_remove_admin(
    State(mm): State<ModelManager>,
    Json(payload): Json<AccAdminForCreatePayload>,
) -> Result<Json<Value>> {
    let ctx = Ctx::root_ctx();
    let AccAdminForCreatePayload {
        admin_id,
        uname,
        email,
    } = payload;
    let acc_fc = AccRemovedForCreate {
        admin_id,
        uname: uname.to_string(),
        email: email.to_string(),
    };

    let id = admin::AdminBmc::acc_to_removed(&ctx, &mm, acc_fc)
        .await
        .map_err(|ex| Error::CannotRemovedAdmin(ex.to_string()))?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
            "removed": true,
            "id": id,
        }
    }));

    Ok(body)
}

pub async fn api_create_admin(
    State(mm): State<ModelManager>,
    Json(payload): Json<AdminForCreatePayload>,
) -> Result<Json<Value>> {
    let ctx = Ctx::root_ctx();

    let AdminForCreatePayload {
        first_name,
        last_name,
        birth_date,
        uname,
        email,
        pwd,
    } = payload;
    let admin_c = AdminForCreate {
        uname: uname.to_string(),
        email: email.to_string(),
        pwd: pwd.to_string(),
    };
    let detail_c = DetailsForCreate {
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        birth_date: birth_date.to_string(),
    };

    let admin_fc = AdminWithDetailsForCreate {
        admin: admin_c,
        detail: detail_c,
    };

    let id = admin::AdminBmc::admin_insert(&ctx, &mm, admin_fc)
        .await
        .map_err(|ex| Error::CannotCreateAdmin(ex.to_string()))?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
            "id": id,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct AdminForCreatePayload {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub uname: String,
    pub email: String,
    pub pwd: String,
}

#[derive(Debug, Deserialize)]
pub struct AccAdminForCreatePayload {
    pub admin_id: i64,
    pub uname: String,
    pub email: String,
}
