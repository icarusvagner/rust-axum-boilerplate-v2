pub mod cid_mid;

use sea_query::Iden;

use crate::model::admin::{TblAccRemoved, TblAccRemovedHistory, TblAdmin, TblDetails};

pub trait TableModel: Iden {
    const TABLE: Self;
    const CID: Self;
    const MID: Self;
    const ID: Self;
}

impl TableModel for TblDetails {
    const TABLE: Self = TblDetails::Table;
    const CID: Self = TblDetails::Cid;
    const MID: Self = TblDetails::Mid;
    const ID: Self = TblDetails::Id;
}

impl TableModel for TblAdmin {
    const TABLE: Self = TblAdmin::Table;
    const CID: Self = TblAdmin::Cid;
    const MID: Self = TblAdmin::Mid;
    const ID: Self = TblAdmin::Id;
}

impl TableModel for TblAccRemovedHistory {
    const TABLE: Self = TblAccRemovedHistory::Table;
    const CID: Self = TblAccRemovedHistory::Cid;
    const MID: Self = TblAccRemovedHistory::Mid;
    const ID: Self = TblAccRemovedHistory::Id;
}

impl TableModel for TblAccRemoved {
    const TABLE: Self = TblAccRemoved::Table;
    const CID: Self = TblAccRemoved::Cid;
    const MID: Self = TblAccRemoved::Mid;
    const ID: Self = TblAccRemoved::Id;
}
