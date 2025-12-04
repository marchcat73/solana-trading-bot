use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::TelegramUsername)
                            .string()
                            .null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::FirstName).string().not_null())
                    .col(ColumnDef::new(Users::LastName).string().null())
                    .col(ColumnDef::new(Users::LanguageCode).string().null())
                    .col(ColumnDef::new(Users::IsPremium).boolean().null())
                    .col(ColumnDef::new(Users::IsAdmin).boolean().not_null().default(false))
                    .col(ColumnDef::new(Users::IsActive).boolean().not_null().default(true))
                    .col(
                        ColumnDef::new(Users::DailyTradeLimit)
                            .decimal_len(20, 9)
                            .not_null()
                            .default(100.0),
                    )
                    .col(ColumnDef::new(Users::TotalTrades).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Users::TotalVolumeSol)
                            .decimal_len(20, 9)
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::LastActiveAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_telegram_username")
                    .table(Users::Table)
                    .col(Users::TelegramUsername)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_is_active")
                    .table(Users::Table)
                    .col(Users::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    TelegramUsername,
    FirstName,
    LastName,
    LanguageCode,
    IsPremium,
    IsAdmin,
    IsActive,
    DailyTradeLimit,
    TotalTrades,
    TotalVolumeSol,
    CreatedAt,
    UpdatedAt,
    LastActiveAt,
}
