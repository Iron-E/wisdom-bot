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
	pub custom_url: Option<String>,

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

Usage: `'{0} ' Username ('/' CustomUrl)? PlayCommand?`

Examples:

- Markiplier video (no `CustomUrl` or `PlayCommand`): `{0} Markiplier`
- Markiplier video (w/ `CustomUrl`, no `PlayCommand`): `{0} Markiplier/markiplierGAME`
- Markiplier video (w/ `PlayCommand`, no `CustomUrl`): `{0} Markiplier play`
- Markiplier video (w/ `CustomUrl` and `PlayCommand`): `{0} Markiplier/markiplierGAME play`",
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

		let mut username_and_custom_url = match args.next()
		{
			Some(u) if !u.is_empty() => u.split('/'),
			_ => bail!("Must provide a YouTube username. Run `!wisdom` for more information"),
		};

		Ok(Self
		{
			username: username_and_custom_url.next().unwrap().into(),
			custom_url: username_and_custom_url.next().filter(|url| !url.is_empty()).map(|url| url.into()),
			play_command: args.next().map(|cmd| cmd.into()).unwrap_or("play".into()),
		})
	}
}
