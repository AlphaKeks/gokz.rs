use {super::SteamID, pretty_assertions::assert_eq, serde::Deserialize, serde_json::json};

#[test]
fn raw() {
	let inputs = [
		json!("STEAM_0:1:161178172"),
		json!("76561198282622073"),
		json!(76561198282622073_u64),
		json!(322356345),
		json!("[U:1:322356345]"),
		json!("U:1:322356345"),
	];

	for (i, input) in inputs.into_iter().enumerate() {
		let steam_id: SteamID =
			serde_json::from_value(input).unwrap_or_else(|err| panic!("#{i}: {err:?}"));

		assert_eq!(steam_id, SteamID(76561198282622073), "#{i}");
	}
}

#[test]
fn inside_struct() {
	#[derive(Deserialize)]
	struct Balls {
		steam_id: SteamID,
	}

	let inputs = [
		json!({ "steam_id": "STEAM_0:1:161178172" }),
		json!({ "steam_id": "76561198282622073" }),
		json!({ "steam_id": 76561198282622073_u64 }),
		json!({ "steam_id": 322356345 }),
		json!({ "steam_id": "[U:1:322356345]" }),
		json!({ "steam_id": "U:1:322356345" }),
	];

	for (i, input) in inputs.into_iter().enumerate() {
		let balls: Balls =
			serde_json::from_value(input).unwrap_or_else(|err| panic!("#{i}: {err:?}"));

		assert_eq!(balls.steam_id, SteamID(76561198282622073), "#{i}");
	}
}
