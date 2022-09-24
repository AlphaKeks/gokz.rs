use gokz_rs::get_recent;

#[tokio::main]
async fn main() {
	let test = get_recent(gokz_rs::global_api::GOKZPlayerIdentifier::Name(
		String::from("AlphaKeks"),
	))
	.await
	.unwrap();

	println!("{:#?}", test);
}
