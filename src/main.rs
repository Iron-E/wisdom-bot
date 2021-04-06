mod youtube;

#[tokio::main]
async fn main()
{
	let channel_id = youtube::channel_id_of("PragerUniversity").await.expect("Expected a valid username or quota to be avilalable");
	let video = youtube::random_video_by(&channel_id.unwrap()).await.unwrap();
	println!("{:?}", video);
}
