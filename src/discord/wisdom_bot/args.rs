use
{
	core::str::FromStr,

	super::WisdomBot,

	anyhow::{bail, Error, Result},
	const_format::formatcp,
};

/// # Summary
///
/// The arguments which [`WisdomBot`] accepts.
pub(super) struct Args
{
	/// # Summary
	///
	/// The YouTube username to get a random video for.
	pub username: String,

	/// # Summary
	///
	/// Optional. The `curstomUrl` of the YouTube user, in case there are multiple users with the
	/// same `username`.
	pub play_command: String,
}

impl Args
{
	/// # Summary
	///
	/// Return how to use the bot.
	pub const fn usage() -> &'static str
	{
		formatcp!(
"wisdom-bot selects random videos from YouTube channels.

Usage: `'{0} ' Username PlayCommand?`

Examples:

- Markiplier video (no `PlayCommand`): `{0} markiplierGAME`
- Markiplier video (w/ `PlayCommand`): `{0} markiplierGAME play`",
			WisdomBot::command()
		)
	}
}

impl FromStr for Args
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		let mut args = s.split(' ').skip(1);

		Ok(Self
		{
			username: match args.next()
			{
				Some(u) => u.into(),
				_ => bail!("Must provide a YouTube username. Run `!wisdom` for more information"),
			},
			play_command: args.next().map(|cmd| cmd.into()).unwrap_or("play".into()),
		})
	}
}
