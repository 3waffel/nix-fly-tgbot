#![feature(async_closure)]
use teloxide::{prelude::*, utils::command::BotCommands};

mod web;
use web::{backend::backend_runner, frontend::frontend_runner};

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().expect("Unable to read .env");

    pretty_env_logger::init();
    log::info!("Starting Bot...");
    let bot = Bot::from_env();

    tokio::join!(
        Command::repl(bot, answer),
        frontend_runner(8080, "http://127.0.0.1:8081"),
        backend_runner(8081)
    );
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
    };
    Ok(())
}
