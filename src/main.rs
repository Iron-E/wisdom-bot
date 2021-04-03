mod youtube_api;

#[tokio::main]
async fn main()
{
	let channel_id = youtube_api::channel_id_of("PragerUniversity").await.unwrap();
	let videos = youtube_api::random_video_by(&channel_id.unwrap()).await.unwrap();
	println!("{:?}", videos.unwrap().len());
}
