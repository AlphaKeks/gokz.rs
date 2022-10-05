use gokz_rs::*;

#[tokio::main]
async fn main() {
	let client = reqwest::Client::new();

	/* let test = get_recent(gokz_rs::global_api::GOKZPlayerIdentifier::SteamID(
		String::from("STEAM_1:0:50615596"),
	)) */
	/* let test = get_unfinished(
		global_api::GOKZPlayerIdentifier::SteamID(String::from("STEAM_1:0:46898346")),
		Some(6),
		global_api::GOKZModeIdentifier::Name(global_api::GOKZModeName::kz_simple),
		true,
	) */
	let test = get_profile(
		global_api::GOKZPlayerIdentifier::Name(String::from("AlphaKeks")),
		global_api::GOKZModeIdentifier::Name(global_api::GOKZModeName::kz_simple),
		&client,
	)
	.await
	.unwrap();

	println!("{:#?}", test);
}
