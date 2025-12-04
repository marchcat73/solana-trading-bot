use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Trades::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Trades::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Trades::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Trades::TradeType).string_len(1).not_null())
                    .col(ColumnDef::new(Trades::InputMint).string().not_null())
                    .col(ColumnDef::new(Trades::OutputMint).string().not_null())
                    .col(
                        ColumnDef::new(Trades::InputAmount)
                            .decimal_len(30, 9)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Trades::OutputAmount)
                            .decimal_len(30, 9)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Trades::InputSymbol).string().not_null())
                    .col(ColumnDef::new(Trades::OutputSymbol).string().not_null())
                    .col(ColumnDef::new(Trades::Price).decimal_len(30, 9).not_null())
                    .col(ColumnDef::new(Trades::SlippageBps).integer().not_null())
                    .col(
                        ColumnDef::new(Trades::TransactionSignature)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Trades::Status).string_len(1).not_null())
                    .col(ColumnDef::new(Trades::ErrorMessage).text().null())
                    .col(ColumnDef::new(Trades::JupiterQuoteId).string().null())
                    .col(
                        ColumnDef::new(Trades::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Trades::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Trades::CompletedAt).timestamp_with_time_zone().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_trades_user_id")
                            .from(Trades::Table, Trades::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_trades_user_id")
                    .table(Trades::Table)
                    .col(Trades::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_trades_created_at")
                    .table(Trades::Table)
                    .col(Trades::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_trades_transaction_signature")
                    .unique()
                    .table(Trades::Table)
                    .col(Trades::TransactionSignature)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_trades_status")
                    .table(Trades::Table)
                    .col(Trades::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Trades::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Trades {
    Table,
    Id,
    UserId,
    TradeType,
    InputMint,
    OutputMint,
    InputAmount,
    OutputAmount,
    InputSymbol,
    OutputSymbol,
    Price,
    SlippageBps,
    TransactionSignature,
    Status,
    ErrorMessage,
    JupiterQuoteId,
    CreatedAt,
    UpdatedAt,
    CompletedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
