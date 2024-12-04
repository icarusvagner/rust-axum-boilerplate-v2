#![allow(unused)] // For example code.

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For examples.

use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3999")?;

    // -- Login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "superoot",
            "pwd": "superoot"
        }),
    );
    req_login.await?.print().await?;

    // -- Create Admin
    // let req_create_admin = hc.do_post(
    //     "/api/admin/create",
    //     json!({
    //         "first_name": "john",
    //         "last_name": "doe",
    //         "birth_date": "1990-01-22",
    //         "uname": "admin01",
    //         "email": "admin01@email.com",
    //         "pwd": "admin01",
    //         "role": "2,3,4,5"
    //     }),
    // );
    // let result = req_create_admin.await?;
    // result.print().await?;

    // -- Removed admin
    // let req_remove_admin = hc.do_post(
    //     "/api/admin/remove",
    //     json!({
    //         "admin_id": 1025,
    //         "uname": "admin01",
    //         "email": "admin01@email.com",
    //     }),
    // );
    // let result = req_remove_admin.await?;
    // result.print().await?;

    Ok(())
}
