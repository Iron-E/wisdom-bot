use
{
	std::env,
	rand::Rng,
	reqwest::Result,
	serde_json::Value,
};

/// # Summary
///
/// The YouTube API has a limit on how many search results it will return. This number is that
/// limit.
const API_MAX_RESULTS: u64 = 500;

/// # Summary
///
/// An error message to use when expecting valid JSON.
const VALID_JSON: &str = "Expected valid JSON";

/// # Summary
///
/// Get the user's api_key.
fn api_key() -> String
{
	env::var("YOUTUBE_API_KEY").expect("You must set your API key to `$YOUTUBE_API_KEY`")
}

/// # Summary
///
/// Return the YouTube channel id of `username`.
pub async fn channel_id_of(username: &str) -> Result<Option<String>>
{
	let text = reqwest::get(format!(
		"https://www.googleapis.com/youtube/v3/channels?forUsername={}&key={}&part=id",
		username,
		api_key(),
	)).await?
		.text()
		.await?
	;

	let mut channel_id = Option::<String>::None;

	let json = serde_json::from_str::<Value>(&text).expect(VALID_JSON);
	if let Some(Value::Array(items)) = json.get("items")
	{
		if let Some(Value::Object(channel)) = items.get(0)
		{
			if let Some(Value::String(id)) = channel.get("id")
			{
				channel_id = Some(id.clone());
			}
		}
	}

	Ok(channel_id)
}

/// # Summary
///
/// Get all videos posted by the `channel_id`.
pub async fn random_video_by(channel_id: &str) -> Result<String>
{
	let mut current_video = 0;
	let mut page_token = String::new();
	let mut video_number = Option::<u64>::None;

	loop
	{
		let text = reqwest::get(format!(
			"https://www.googleapis.com/youtube/v3/search?channelId={}&key={}&maxResults=50{}&type=video",
			channel_id,
			api_key(),
			page_token,
		)).await?
			.text()
			.await?
		;

		let json = serde_json::from_str::<Value>(&text).expect(VALID_JSON);

		if video_number.is_none()
		{
			if let Some(Value::Object(page_info)) = json.get("pageInfo")
			{
				if let Some(Value::Number(total_results)) = page_info.get("totalResults")
				{
					video_number = Some(rand::thread_rng().gen_range(
						0..API_MAX_RESULTS.min(total_results.as_u64().expect("Expected to be able to store `totalResults` in a u64"))
					));
				}
			}
		}

		if let Some(Value::Array(items)) = json.get("items")
		{
			if let Some(video) = items.into_iter()
				.filter_map(|search_result| {
					if let Value::Object(s) = search_result
					{
						if let Some(Value::Object(video)) = s.get("id")
						{
							if let Some(Value::String(id)) = video.get("videoId")
							{
								return Some(id);
							}
						}
					}

					None
				}).find(|_|
					if current_video == video_number.expect("`video_number` should be `Some` by now")
					{
						true
					}
					else
					{
						current_video += 1;
						false
					}
				)
			{
				return Ok(video.into());
			}
		}

		if let Some(Value::String(next_page_token)) = json.get("nextPageToken")
		{
			if !next_page_token.is_empty()
			{
				page_token = format!("&pageToken={}", next_page_token);
				continue;
			}
		}
	}
}