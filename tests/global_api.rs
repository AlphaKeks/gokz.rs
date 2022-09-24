#![allow(unused_imports, unused_variables, dead_code)]

use gokz_rs::global_api::*;
use gokz_rs::util::*;
use gokz_rs::*;

#[tokio::test]
pub async fn test_get_maps() {
	let result = get_maps().await.expect("test_get_maps failed.");

	assert_eq!(879, result.len());
}

#[tokio::test]
pub async fn test_get_map() {
	let lionharder = GOKZMap {
		id: 992,
		name: String::from("kz_lionharder"),
		filesize: 100007576,
		validated: true,
		difficulty: 7,
		created_on: String::from("2021-06-05T15:52:16"),
		updated_on: String::from("2021-06-05T15:52:16"),
		approved_by_steamid64: String::from("76561198143205331"),
		workshop_url: String::from(
			"https://steamcommunity.com/sharedfiles/filedetails/?id=2420807980",
		),
		download_url: None,
	};

	let result_name = get_map(GOKZMapIdentifier::Name(String::from("kz_lionharder"))).await;
	let result_id = get_map(GOKZMapIdentifier::Id(992)).await;

	assert_eq!(Ok(lionharder.clone()), result_name);
	assert_eq!(Ok(lionharder), result_id);
}

#[tokio::test]
pub async fn test_validate_map() {
	let lionharder = GOKZMap {
		id: 992,
		name: String::from("kz_lionharder"),
		filesize: 100007576,
		validated: true,
		difficulty: 7,
		created_on: String::from("2021-06-05T15:52:16"),
		updated_on: String::from("2021-06-05T15:52:16"),
		approved_by_steamid64: String::from("76561198143205331"),
		workshop_url: String::from(
			"https://steamcommunity.com/sharedfiles/filedetails/?id=2420807980",
		),
		download_url: None,
	};

	let result_name = get_map(GOKZMapIdentifier::Name(String::from("kz_lionharder"))).await;
	let result_id = get_map(GOKZMapIdentifier::Id(992)).await;

	assert_eq!(Ok(lionharder.clone()), result_name);
	assert_eq!(Ok(lionharder), result_id);
}

#[tokio::test]
pub async fn test_get_modes() {
	let kzt = GOKZMode {
		id: 200,
		name: GOKZModeName::kz_timer,
		description: String::from("KZTimerGlobal mode.  Bunch of jumps and bhops and stuff."),
		latest_version: 212,
		latest_version_description: String::from("1.102"),
		website: String::from("forum.gokz.org"),
		repo: String::from("https://bitbucket.org/kztimerglobalteam/kztimerglobal"),
		contact_steamid64: String::from("76561198165203332"),
		supported_tickrates: None,
		created_on: String::from("0001-01-01T00:00:00"),
		updated_on: String::from("2018-01-09T10:45:50"),
		updated_by_id: String::from("76561198003275951"),
	};

	let skz = GOKZMode {
		id: 201,
		name: GOKZModeName::kz_simple,
		description: String::from("SimpleKZ mode. RNG? We don't need no stinkin RNG."),
		latest_version: 16,
		latest_version_description: String::from("3.2.0"),
		website: String::from("forum.gokz.org"),
		repo: String::from("https://github.com/KZGlobalTeam/gokz"),
		contact_steamid64: String::from("76561197989817982"),
		supported_tickrates: None,
		created_on: String::from("0001-01-01T00:00:00"),
		updated_on: String::from("2018-01-09T10:45:50"),
		updated_by_id: String::from("76561198003275951"),
	};

	let vnl = GOKZMode {
		id: 202,
		name: GOKZModeName::kz_vanilla,
		description: String::from("Vanilla mode. We need RNG."),
		latest_version: 12,
		latest_version_description: String::from("3.2.0"),
		website: String::from("forum.gokz.org"),
		repo: String::from("https://github.com/KZGlobalTeam/gokz"),
		contact_steamid64: String::from("76561197989817982"),
		supported_tickrates: None,
		created_on: String::from("0001-01-01T00:00:00"),
		updated_on: String::from("2018-01-09T10:45:50"),
		updated_by_id: String::from("76561197989817982"),
	};

	let vec = vec![kzt, skz, vnl];

	let result = get_modes().await.unwrap();

	assert_eq!(vec, result);
}

#[tokio::test]
pub async fn test_get_mode() {
	let kzt = GOKZMode {
		id: 200,
		name: GOKZModeName::kz_timer,
		description: String::from("KZTimerGlobal mode.  Bunch of jumps and bhops and stuff."),
		latest_version: 212,
		latest_version_description: String::from("1.102"),
		website: String::from("forum.gokz.org"),
		repo: String::from("https://bitbucket.org/kztimerglobalteam/kztimerglobal"),
		contact_steamid64: String::from("76561198165203332"),
		supported_tickrates: None,
		created_on: String::from("0001-01-01T00:00:00"),
		updated_on: String::from("2018-01-09T10:45:50"),
		updated_by_id: String::from("76561198003275951"),
	};

	let skz = GOKZMode {
		id: 201,
		name: GOKZModeName::kz_simple,
		description: String::from("SimpleKZ mode. RNG? We don't need no stinkin RNG."),
		latest_version: 16,
		latest_version_description: String::from("3.2.0"),
		website: String::from("forum.gokz.org"),
		repo: String::from("https://github.com/KZGlobalTeam/gokz"),
		contact_steamid64: String::from("76561197989817982"),
		supported_tickrates: None,
		created_on: String::from("0001-01-01T00:00:00"),
		updated_on: String::from("2018-01-09T10:45:50"),
		updated_by_id: String::from("76561198003275951"),
	};

	let vnl = GOKZMode {
		id: 202,
		name: GOKZModeName::kz_vanilla,
		description: String::from("Vanilla mode. We need RNG."),
		latest_version: 12,
		latest_version_description: String::from("3.2.0"),
		website: String::from("forum.gokz.org"),
		repo: String::from("https://github.com/KZGlobalTeam/gokz"),
		contact_steamid64: String::from("76561197989817982"),
		supported_tickrates: None,
		created_on: String::from("0001-01-01T00:00:00"),
		updated_on: String::from("2018-01-09T10:45:50"),
		updated_by_id: String::from("76561197989817982"),
	};

	let result1 = get_mode(GOKZModeIdentifier::Name(GOKZModeName::kz_timer))
		.await
		.unwrap();
	let result2 = get_mode(GOKZModeIdentifier::Id(201)).await.unwrap();
	let result3 = get_mode(GOKZModeIdentifier::Name(GOKZModeName::kz_vanilla))
		.await
		.unwrap();

	assert_eq!(kzt, result1);
	assert_eq!(skz, result2);
	assert_eq!(vnl, result3);
}

#[tokio::test]
pub async fn test_get_player() {
	let alphakeks = GOKZPlayer {
		steamid64: String::from("76561198282622073"),
		steam_id: String::from("STEAM_1:1:161178172"),
		is_banned: false,
		total_records: 0,
		name: String::from("AlphaKeks"),
	};

	let result1 = get_player(GOKZPlayerIdentifier::Name(String::from("AlphaKeks")))
		.await
		.unwrap();
	let result2 = get_player(GOKZPlayerIdentifier::SteamID(String::from(
		"STEAM_1:1:161178172",
	)))
	.await
	.unwrap();

	assert_eq!(alphakeks, result1);
	assert_eq!(alphakeks, result2);
}

#[tokio::test]
pub async fn test_get_wr() {
	let lionharder_tp = GOKZRecord {
		id: 17779399,
		steamid64: String::from("76561198091592005"),
		player_name: String::from("Blacky"),
		steam_id: String::from("STEAM_1:1:65663138"),
		server_id: 1406,
		map_id: 992,
		stage: 0,
		mode: GOKZModeName::kz_simple,
		tickrate: 128,
		time: 532.602,
		teleports: 114,
		created_on: String::from("2022-08-22T17:06:54"),
		updated_on: String::from("2022-08-22T17:06:54"),
		updated_by: 0,
		record_filter_id: 0,
		server_name: Some(String::from("Esterata.com | KZ - Legends")),
		map_name: String::from("kz_lionharder"),
		points: 1000,
		replay_id: 168353,
	};

	let lionharder_pro = GOKZRecord {
		id: 17779658,
		steamid64: String::from("76561198091592005"),
		player_name: String::from("Blacky"),
		steam_id: String::from("STEAM_1:1:65663138"),
		server_id: 1406,
		map_id: 992,
		stage: 0,
		mode: GOKZModeName::kz_simple,
		tickrate: 128,
		time: 1107.227,
		teleports: 0,
		created_on: String::from("2022-08-22T17:29:56"),
		updated_on: String::from("2022-08-22T17:29:56"),
		updated_by: 0,
		record_filter_id: 0,
		server_name: Some(String::from("Esterata.com | KZ - Legends")),
		map_name: String::from("kz_lionharder"),
		points: 1000,
		replay_id: 168378,
	};

	let result1 = get_wr(
		GOKZMapIdentifier::Name(String::from("kz_lionharder")),
		0,
		GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
		true,
	)
	.await
	.unwrap();

	let result2 = get_wr(
		GOKZMapIdentifier::Name(String::from("kz_lionharder")),
		0,
		GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
		false,
	)
	.await
	.unwrap();

	assert_eq!(lionharder_tp, result1);
	assert_eq!(lionharder_pro, result2);
}

#[tokio::test]
pub async fn test_maptop() {
	let lionharder_pro_len = 4;
	let beginnerblock_tp_len = 100;
	let kiwiterror_tp_len = 6;

	let result1 = get_maptop(
		GOKZMapIdentifier::Name(String::from("kz_lionharder")),
		0,
		GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
		false,
	)
	.await
	.unwrap();
	let result2 = get_maptop(
		GOKZMapIdentifier::Name(String::from("kz_beginnerblock_go")),
		0,
		GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
		true,
	)
	.await
	.unwrap();
	let result3 = get_maptop(
		GOKZMapIdentifier::Name(String::from("kz_kiwiterror")),
		0,
		GOKZModeIdentifier::Name(GOKZModeName::kz_timer),
		true,
	)
	.await
	.unwrap();

	assert_eq!(lionharder_pro_len, result1.len());
	assert_eq!(beginnerblock_tp_len, result2.len());
	assert_eq!(kiwiterror_tp_len, result3.len());
}

#[tokio::test]
pub async fn test_get_pb() {
	let blacky_lionharder_tp = GOKZRecord {
		id: 17779399,
		steamid64: String::from("76561198091592005"),
		player_name: String::from("Blacky"),
		steam_id: String::from("STEAM_1:1:65663138"),
		server_id: 1406,
		map_id: 992,
		stage: 0,
		mode: GOKZModeName::kz_simple,
		tickrate: 128,
		time: 532.602,
		teleports: 114,
		created_on: String::from("2022-08-22T17:06:54"),
		updated_on: String::from("2022-08-22T17:06:54"),
		updated_by: 0,
		record_filter_id: 0,
		server_name: Some(String::from("Esterata.com | KZ - Legends")),
		map_name: String::from("kz_lionharder"),
		points: 1000,
		replay_id: 168353,
	};

	let charlie_fuckthis_pro = GOKZRecord {
		id: 18072510,
		steamid64: String::from("76561198054062420"),
		player_name: String::from("charlieeilrahc"),
		steam_id: String::from("STEAM_1:0:46898346"),
		server_id: 1075,
		map_id: 1146,
		stage: 0,
		mode: GOKZModeName::kz_simple,
		tickrate: 128,
		time: 768.336,
		teleports: 0,
		created_on: String::from("2022-09-12T03:03:55"),
		updated_on: String::from("2022-09-12T03:03:55"),
		updated_by: 0,
		record_filter_id: 0,
		server_name: Some(String::from("_(:???)_")),
		map_name: String::from("kz_auuughh"),
		points: 333,
		replay_id: 195852,
	};

	let result1 = get_pb(
		GOKZPlayerIdentifier::SteamID(String::from("STEAM_1:1:65663138")),
		GOKZMapIdentifier::Name(String::from("kz_lionharder")),
		0,
		GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
		true,
	)
	.await
	.unwrap();

	let result2 = get_pb(
		GOKZPlayerIdentifier::SteamID(String::from("STEAM_1:0:46898346")),
		GOKZMapIdentifier::Name(String::from("kz_auuughh")),
		0,
		GOKZModeIdentifier::Name(GOKZModeName::kz_simple),
		false,
	)
	.await
	.unwrap();

	assert_eq!(blacky_lionharder_tp, result1);
	assert_eq!(charlie_fuckthis_pro, result2);
}

#[tokio::test]
pub async fn test_get_times() {
	let test = get_times(
		GOKZPlayerIdentifier::SteamID(String::from("STEAM_1:1:161178172")),
		GOKZModeIdentifier::Name(GOKZModeName::kz_vanilla),
		false,
	)
	.await
	.unwrap();

	assert_eq!(17, test.len());
}

#[tokio::test]
pub async fn test_get_recent() {
	let recent1 = GOKZRecord {
		id: 16937780,
		steamid64: String::from("76561198282622073"),
		player_name: String::from("AlphaKeks"),
		steam_id: String::from("STEAM_1:1:161178172"),
		server_id: 1561,
		map_id: 585,
		stage: 0,
		mode: GOKZModeName::kz_simple,
		tickrate: 128,
		time: 51.547,
		teleports: 0,
		created_on: String::from("2022-06-21T19:51:16"),
		updated_on: String::from("2022-06-21T19:51:16"),
		updated_by: 0,
		record_filter_id: 0,
		server_name: Some(String::from("Church of Schnose")),
		map_name: String::from("kz_eventide"),
		points: 953,
		replay_id: 83617,
	};

	let recent2 = GOKZRecord {
		id: 18100900,
		steamid64: String::from("76561198054062420"),
		player_name: String::from("charlieeilrahc"),
		steam_id: String::from("STEAM_1:0:46898346"),
		server_id: 553,
		map_id: 836,
		stage: 0,
		mode: GOKZModeName::kz_simple,
		tickrate: 128,
		time: 90.289,
		teleports: 0,
		created_on: String::from("2022-09-14T01:58:45"),
		updated_on: String::from("2022-09-14T01:58:45"),
		updated_by: 0,
		record_filter_id: 0,
		server_name: Some(String::from("Pixel GOKZ")),
		map_name: String::from("kz_skybridge"),
		points: 800,
		replay_id: 198919,
	};

	let recent3 = GOKZRecord {
		id: 18248937,
		steamid64: String::from("76561198264939817"),
		player_name: String::from("racist75"),
		steam_id: String::from("STEAM_1:1:152337044"),
		server_id: 1523,
		map_id: 500,
		stage: 0,
		mode: GOKZModeName::kz_simple,
		tickrate: 128,
		time: 429.438,
		teleports: 57,
		created_on: String::from("2022-09-23T23:15:23"),
		updated_on: String::from("2022-09-23T23:15:23"),
		updated_by: 0,
		record_filter_id: 0,
		server_name: Some(String::from("bladee enthusiasts")),
		map_name: String::from("kz_talltreeforest_v3"),
		points: 1000,
		replay_id: 214817,
	};

	let test1 = get_recent(GOKZPlayerIdentifier::SteamID(String::from(
		"STEAM_1:1:161178172",
	)))
	.await
	.unwrap();

	let test2 = get_recent(GOKZPlayerIdentifier::SteamID(String::from(
		"STEAM_1:0:46898346",
	)))
	.await
	.unwrap();

	let test3 = get_recent(GOKZPlayerIdentifier::SteamID(String::from(
		"STEAM_1:1:152337044",
	)))
	.await
	.unwrap();

	assert_eq!(recent1, test1);
	assert_eq!(recent2, test2);
	assert_eq!(recent3, test3);
}
