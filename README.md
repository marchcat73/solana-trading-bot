# solana-trading-bot

solana-trading-bot/
├── Cargo.toml
├── Cargo.lock
├── .env.example
├── .gitignore
├── .cargo/
│ └── config.toml
├── config/
│ ├── default.toml
│ ├── development.toml
│ └── production.toml
├── migrations/
│ ├── m20240101_000001_create_users_table.rs
│ ├── m20240101_000002_create_trades_table.rs
│ └── m20240101_000003_create_wallets_table.rs
├── src/
│ ├── main.rs
│ ├── lib.rs
│ ├── config/
│ │ ├── mod.rs
│ │ └── settings.rs
│ ├── database/
│ │ ├── mod.rs
│ │ └── connection.rs
│ ├── entities/
│ │ ├── mod.rs
│ │ ├── prelude.rs
│ │ ├── users.rs
│ │ ├── trades.rs
│ │ └── wallets.rs
│ ├── security/
│ │ ├── mod.rs
│ │ ├── secrets_manager.rs
│ │ ├── key_manager.rs
│ │ └── encryption.rs
│ ├── solana/
│ │ ├── mod.rs
│ │ ├── client.rs
│ │ └── trader.rs
│ ├── jupiter/
│ │ ├── mod.rs
│ │ └── client.rs
│ ├── telegram/
│ │ ├── mod.rs
│ │ ├── bot.rs
│ │ ├── handlers/
│ │ │ ├── mod.rs
│ │ │ ├── auth.rs
│ │ │ ├── trade.rs
│ │ │ └── admin.rs
│ │ └── middleware/
│ │ ├── mod.rs
│ │ └── rate_limit.rs
│ ├── api/
│ │ ├── mod.rs
│ │ ├── server.rs
│ │ ├── routes/
│ │ │ ├── mod.rs
│ │ │ ├── health.rs
│ │ │ ├── metrics.rs
│ │ │ └── admin.rs
│ │ └── middleware/
│ │ ├── mod.rs
│ │ └── auth.rs
│ ├── monitoring/
│ │ ├── mod.rs
│ │ └── metrics.rs
│ └── utils/
│ ├── mod.rs
│ └── errors.rs
├── scripts/
│ ├── init_db.sh
│ └── deploy.sh
└── tests/
├── integration/
└── unit/
