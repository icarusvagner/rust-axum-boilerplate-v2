mod error;

pub use self::error::{Error, Result};

// #[cfg_attr(feature = "with-rpc", derive(rpc_router::RpcResource))]
#[derive(Debug, Clone)]
pub struct Ctx {
    pub user_id: i64,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
