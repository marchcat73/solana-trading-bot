use teloxide::{
    prelude::*,
    types::{Message},
    utils::command::BotCommands,
};
use std::sync::Arc;

use crate::{
    config::settings::TelegramSettings,
    database::connection::DatabaseConnectionPool,
    security::secrets_manager::SecretsManager,
    monitoring::metrics::MetricsRegistry,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Начать работу с ботом")]
    Start,
    #[command(description = "Помощь и список команд")]
    Help,
    #[command(description = "Баланс кошелька")]
    Balance,
    #[command(description = "Купить токен")]
    Buy,
    #[command(description = "Продать токен")]
    Sell,
    #[command(description = "Поиск токена", parse_with = "split")]
    Search(String),
    #[command(description = "История сделок")]
    History,
    #[command(description = "Настройки")]
    Settings,
    #[command(description = "Добавить кошелек")]
    AddWallet,
    #[command(description = "Список кошельков")]
    Wallets,
    #[command(description = "Лимиты торговли")]
    Limits,
    #[command(description = "Статистика")]
    Stats,
}

impl Command {
    pub fn descriptions() -> String {
        let mut descriptions = String::from("Доступные команды:\n\n");

        descriptions.push_str("/start - Начать работу с ботом\n");
        descriptions.push_str("/help - Помощь и список команд\n");
        descriptions.push_str("/balance - Баланс кошелька\n");
        descriptions.push_str("/buy - Купить токен\n");
        descriptions.push_str("/sell - Продать токен\n");
        descriptions.push_str("/search <запрос> - Поиск токена\n");
        descriptions.push_str("/history - История сделок\n");
        descriptions.push_str("/settings - Настройки\n");
        descriptions.push_str("/addwallet - Добавить кошелек\n");
        descriptions.push_str("/wallets - Список кошельков\n");
        descriptions.push_str("/limits - Лимиты торговли\n");
        descriptions.push_str("/stats - Статистика\n");

        descriptions
    }
}

pub struct TelegramBot {
    bot: Bot,
    settings: TelegramSettings,
    database: DatabaseConnectionPool,
    secrets: SecretsManager,
    metrics: MetricsRegistry,
}

impl TelegramBot {
    pub async fn new(
        settings: TelegramSettings,
        database: DatabaseConnectionPool,
        secrets: SecretsManager,
        metrics: MetricsRegistry,
    ) -> Result<Self, anyhow::Error> {
        let bot_token = secrets.get_telegram_token().await;
        let bot = Bot::new(bot_token);

        Ok(Self {
            bot,
            settings,
            database,
            secrets,
            metrics,
        })
    }

    pub async fn start(self) -> Result<(), anyhow::Error> {
        tracing::info!("Starting Telegram bot...");

        let handler = Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(Self::handle_command)
            )
            .branch(
                dptree::filter(|msg: Message| msg.text().is_some())
                    .endpoint(Self::handle_text)
            );

        let bot = self.bot.clone();
        let settings = self.settings.clone();
        let database = Arc::new(self.database.clone());
        let secrets = Arc::new(self.secrets.clone());
        let metrics = Arc::new(self.metrics.clone());

        Dispatcher::builder(bot, handler)
            .dependencies(dptree::deps![
                settings,
                database,
                secrets,
                metrics
            ])
            .default_handler(|upd| async move {
                tracing::warn!("Unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "Произошла ошибка в обработчике"
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;

        Ok(())
    }

    pub async fn handle_command(
        bot: Bot,
        msg: Message,
        cmd: Command,
        settings: TelegramSettings,
        database: Arc<DatabaseConnectionPool>,
        secrets: Arc<SecretsManager>,
        metrics: Arc<MetricsRegistry>,
    ) -> Result<(), teloxide::RequestError> {
        let chat_id = msg.chat.id;

        match cmd {
            Command::Start => {
                bot.send_message(chat_id, "Добро пожаловать в Solana Trading Bot! Используйте /help для списка команд.").await?;
            }
            Command::Help => {
                bot.send_message(chat_id, Command::descriptions()).await?;
            }
            Command::Balance => {
                bot.send_message(chat_id, "Функция баланса пока не реализована").await?;
            }
            Command::Buy => {
                bot.send_message(chat_id, "Функция покупки пока не реализована").await?;
            }
            Command::Sell => {
                bot.send_message(chat_id, "Функция продажи пока не реализована").await?;
            }
            Command::Search(query) => {
                bot.send_message(chat_id, format!("Поиск токена: {}", query)).await?;
            }
            Command::History => {
                bot.send_message(chat_id, "Функция истории пока не реализована").await?;
            }
            Command::Settings => {
                bot.send_message(chat_id, "Функция настроек пока не реализована").await?;
            }
            Command::AddWallet => {
                bot.send_message(chat_id, "Функция добавления кошелька пока не реализована").await?;
            }
            Command::Wallets => {
                bot.send_message(chat_id, "Функция списка кошельков пока не реализована").await?;
            }
            Command::Limits => {
                bot.send_message(chat_id, "Функция лимитов пока не реализована").await?;
            }
            Command::Stats => {
                bot.send_message(chat_id, "Функция статистики пока не реализована").await?;
            }
        }

        Ok(())
    }

    pub async fn handle_text(
        bot: Bot,
        msg: Message,
        settings: TelegramSettings,
        database: Arc<DatabaseConnectionPool>,
        secrets: Arc<SecretsManager>,
        metrics: Arc<MetricsRegistry>,
    ) -> Result<(), teloxide::RequestError> {
        let chat_id = msg.chat.id;
        let text = msg.text().unwrap_or("");

        if text.starts_with('/') {
            // Это команда, которую не распознал макрос
            bot.send_message(chat_id, "Неизвестная команда. Используйте /help для списка команд.").await?;
        } else {
            bot.send_message(chat_id, "Я понимаю только команды. Используйте /help для списка команд.").await?;
        }

        Ok(())
    }
}
