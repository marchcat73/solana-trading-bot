use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "trades")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: i64,
    pub trade_type: TradeType,
    pub input_mint: String,
    pub output_mint: String,
    pub input_amount: BigDecimal,
    pub output_amount: BigDecimal,
    pub input_symbol: String,
    pub output_symbol: String,
    pub price: BigDecimal,
    pub slippage_bps: i32,
    pub transaction_signature: String,
    pub status: TradeStatus,
    pub error_message: Option<String>,
    pub jupiter_quote_id: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub completed_at: Option<DateTimeUtc>,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum TradeType {
    #[sea_orm(string_value = "BUY")]
    Buy,
    #[sea_orm(string_value = "SELL")]
    Sell,
    #[sea_orm(string_value = "SWAP")]
    Swap,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum TradeStatus {
    #[sea_orm(string_value = "PENDING")]
    Pending,
    #[sea_orm(string_value = "EXECUTING")]
    Executing,
    #[sea_orm(string_value = "COMPLETED")]
    Completed,
    #[sea_orm(string_value = "FAILED")]
    Failed,
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
