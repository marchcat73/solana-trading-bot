//! SeaORM Entities

pub mod prelude;
pub mod users;
pub mod trades;
pub mod wallets;

pub use users::Entity as Users;
pub use trades::Entity as Trades;
pub use wallets::Entity as Wallets;
