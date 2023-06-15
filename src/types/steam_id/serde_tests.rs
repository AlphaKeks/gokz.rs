use {
	super::*,
	pretty_assertions::assert_eq,
	serde::{Deserialize, Serialize},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize)]
struct Player {
	steam_id: SteamID,
}

#[test]
fn ser_steam_id() -> Result<()> {
	let alphakeks = SteamID::new("STEAM_1:1:161178172")?;
	let p = Player { steam_id: alphakeks };

	let serialized = serde_json::to_string(&p.steam_id)?;
	let serialized_player = serde_json::to_string(&p)?;

	assert_eq!(serialized, "\"STEAM_1:1:161178172\"");
	assert_eq!(serialized_player, r#"{"steam_id":"STEAM_1:1:161178172"}"#);

	Ok(())
}

#[test]
fn deser_steam_id() -> Result<()> {
	let alphakeks = "\"STEAM_1:1:161178172\"";
	let p = r#"{"steam_id":"STEAM_1:1:161178172"}"#;

	let deserialized: SteamID = serde_json::from_str(alphakeks)?;
	let deserialized_player: Player = serde_json::from_str(p)?;

	assert_eq!(deserialized, SteamID(76561198282622073u64));
	assert_eq!(deserialized_player.steam_id, SteamID(76561198282622073u64));

	Ok(())
}
