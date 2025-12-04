use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use secrecy::{Secret, Serialize as SecretSerialize};
use bigdecimal::BigDecimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "wallets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: i64,
    pub public_key: String,
    #[sea_orm(column_type = "Text")]
    pub encrypted_private_key: String,
    pub wallet_type: WalletType,
    pub name: String,
    pub is_default: bool,
    pub is_active: bool,
    pub balance_sol: BigDecimal,
    pub last_synced_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum WalletType {
    #[sea_orm(string_value = "HOT")]
    Hot,
    #[sea_orm(string_value = "LEDGER")]
    Ledger,
    #[sea_orm(string_value = "PHANTOM")]
    Phantom,
    #[sea_orm(string_value = "SOLFLARE")]
    Solflare,
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
