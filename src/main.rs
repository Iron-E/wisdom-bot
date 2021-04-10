#[allow(clippy::suspicious_else_formatting)]

mod discord;
mod youtube;

use
{
	std::env,

	discord::WisdomBot,

	serenity::{Client, Result},
};

#[tokio::main]
async fn main() -> Result<()>
{
	let token = env::var("DISCORD_TOKEN").expect("You must set your token to `$DISCORD_TOKEN`");

	Client::builder(&token).event_handler(WisdomBot).await?.start().await?;

	Ok(())
}
