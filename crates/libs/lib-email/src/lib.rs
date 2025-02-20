use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

pub mod config;
pub mod error;
pub mod template;
pub mod worker;

#[derive(Serialize, Debug, PartialEq, EnumString)]
pub enum EmailProvider {
    #[strum(serialize = "hostinger")]
    Hostinger,
    #[strum(serialize = "ionos")]
    Ionos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailTemplateBody {
    pub email_add: String,
    pub email_subj: String,
    pub email_msg: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailSend {
    pub email_add: String,
    pub email_subj: String,
    pub email_msg: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailSendWrapper {
    pub email_data: EmailSend,
}
