use
{
	core::fmt::Display,

	serenity::
	{
		client::Context,
		model::id::ChannelId
	},
};

pub(super) struct Chan(pub ChannelId);

impl Chan
{
	/// # Summary
	///
	/// Put some `content` on the `channel`.
	pub async fn say(self, ctx: &Context, content: impl Display)
	{
		if let Err(e) = self.0.say(&ctx.http, content).await
		{
			eprintln!("{:?}", e);
		}
	}
}


