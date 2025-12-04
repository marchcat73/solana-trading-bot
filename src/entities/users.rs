use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use secrecy::Secret;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64, // Telegram User ID
    #[sea_orm(unique)]
    pub telegram_username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
    pub is_admin: bool,
    pub is_active: bool,
    pub daily_trade_limit: Decimal,
    pub total_trades: i32,
    pub total_volume_sol: Decimal,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub last_active_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::trades::Entity")]
    Trades,
    #[sea_orm(has_many = "super::wallets::Entity")]
    Wallets,
}

impl Related<super::trades::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Trades.def()
    }
}

impl Related<super::wallets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Wallets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
