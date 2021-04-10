mod args;
mod chan;

use
{
	core::str::FromStr,

	args::Args,
	chan::Chan,

	crate::youtube,

	serenity::
	{
		async_trait,
		client::{Context, EventHandler},
		model::channel::Message,
	}
};

/// # Summary
///
/// The `wisdom-bot`.
pub struct WisdomBot;

impl WisdomBot
{
	/// # Summary
	///
	/// The command which activates [`WisdomBot`].
	const fn command() -> &'static str { "!wisdom" }

	/// # Summary
	///
	/// The discriminator of this bot.
	const fn discriminator() -> u16 { 3703 }

	/// # Summary
	///
	/// Return whether or not a `message` indicates that a user is asking for help.
	fn requires_help(message: &Message) -> bool
	{
		message.content == Self::command()
	}

	/// # Summary
	///
	/// The username of this bot.
	const fn username() -> &'static str { "Weiß the Wise" }
}

#[async_trait]
impl EventHandler for WisdomBot
{
	async fn message(&self, ctx: Context, received: Message)
	{
		/// # Summary
		///
		/// Write some message to the `received.channel_id`, then return from the function.
		macro_rules! say_and_return
		{
			($content: expr) =>
			{
				{
					Chan(received.channel_id).say(&ctx, $content).await;
					return;
				}
			}
		}

		if (received.author.bot && received.author.name == Self::username() && received.author.discriminator == Self::discriminator()) ||
			!received.content.split(' ').next().unwrap_or("").contains(Self::command())
		{
			return;
		}
		else if Self::requires_help(&received)
		{
			say_and_return!(Args::usage())
		}

		let args = match Args::from_str(&received.content)
		{
			Ok(a) => a,
			Err(e) => say_and_return!(e),
		};

		let channel_id = match youtube::channel_id_of(&args.username).await
		{
			Ok(Some(id)) => id,
			Err(e) => say_and_return!(e),
			_ => say_and_return!(format!("No YouTube user of name '{}' found", args.username)),
		};

		match youtube::random_video_by(&channel_id).await
		{
			Ok(video_id) => say_and_return!(format!("!{} https://www.youtube.com/watch?v={}", args.play_command, video_id)),
			Err(e) => say_and_return!(e),
		};
	}
}
