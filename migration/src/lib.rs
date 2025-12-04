pub use sea_orm_migration::prelude::*;

mod m20251204_222043_name1;
mod m20251204_222257_create_trades_table;
mod m20251204_222434_create_wallets_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20251204_222043_name1::Migration),
        Box::new(m20251204_222257_create_trades_table::Migration),
        Box::new(m20251204_222434_create_wallets_table::Migration)]
    }
}
