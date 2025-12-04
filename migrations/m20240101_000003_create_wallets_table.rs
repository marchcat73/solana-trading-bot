use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wallets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Wallets::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Wallets::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Wallets::PublicKey).string().not_null())
                    .col(ColumnDef::new(Wallets::EncryptedPrivateKey).text().not_null())
                    .col(ColumnDef::new(Wallets::WalletType).string_len(1).not_null())
                    .col(ColumnDef::new(Wallets::Name).string().not_null())
                    .col(ColumnDef::new(Wallets::IsDefault).boolean().not_null().default(false))
                    .col(ColumnDef::new(Wallets::IsActive).boolean().not_null().default(true))
                    .col(
                        ColumnDef::new(Wallets::BalanceSol)
                            .decimal_len(20, 9)
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(Wallets::LastSyncedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Wallets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Wallets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wallets_user_id")
                            .from(Wallets::Table, Wallets::UserId)
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
                    .name("idx_wallets_user_id")
                    .table(Wallets::Table)
                    .col(Wallets::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wallets_public_key")
                    .unique()
                    .table(Wallets::Table)
                    .col(Wallets::PublicKey)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wallets_is_default")
                    .table(Wallets::Table)
                    .col(Wallets::IsDefault)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallets::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Wallets {
    Table,
    Id,
    UserId,
    PublicKey,
    EncryptedPrivateKey,
    WalletType,
    Name,
    IsDefault,
    IsActive,
    BalanceSol,
    LastSyncedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
