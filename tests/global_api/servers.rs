use {
	color_eyre::Result,
	gokz_rs::{global_api, global_api::Server, SteamID},
	serde_json::json,
};

#[tokio::test]
async fn get_servers() -> Result<()> {
	let steam_id = SteamID::try_from(76561198282622073_u64)?;
	let servers = global_api::get_servers_owned_by(steam_id, &crate::GOKZ_CLIENT).await?;

	let expected = json!([
		{
			"id": 1561,
			"port": 27015,
			"ip": "45.85.219.81",
			"name": "Church of Schnose",
			"owner_steamid64": "76561198282622073"
		},
		{
			"id": 999,
			"port": 28078,
			"ip": "51.89.6.104",
			"name": "Hikari KZ",
			"owner_steamid64": "76561198282622073"
		},
		{
			"id": 657,
			"port": 27015,
			"ip": "45.81.234.73",
			"name": "Alpha's KZ",
			"owner_steamid64": "76561198282622073"
		}
	]);

	let expected = serde_json::from_value::<Vec<Server>>(expected)?;

	assert_eq!(servers, expected);
	Ok(())
}
