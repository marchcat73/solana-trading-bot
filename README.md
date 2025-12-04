# solana-trading-bot

```bash
# Install dependencies
cargo build --release

# Run migrations
cargo run -- migrate

# Start the bot
cargo run --release
```

## Database Migrations

```bash
# Create new migration
sea-orm-cli migrate generate <migration_name>

# Run migrations
sea-orm-cli migrate up

# Rollback migration
sea-orm-cli migrate down
```
