// ignore   - %s/\/\/ \#\[/\#\[
// unignore - %s/\#\[ign/\/\/ \#\[ign

use gokz_rs::prelude::*;
use gokz_rs::GlobalAPI;
use log::info;

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_bans() -> anyhow::Result<()> {
	let steam_id = SteamID::new("STEAM_1:1:161178172")?;
	let client = gokz_rs::Client::new();

	let bans = GlobalAPI::get_bans(&steam_id, 5, &client).await;
	assert!(matches!(bans, Ok(_)));
	info!("{:#?}", bans);

	let steam_id = SteamID::new("STEAM_1:0:165881949")?;
	let bans = GlobalAPI::get_bans(&steam_id, 5, &client).await;
	assert!(matches!(bans, Err(_)));
	info!("{:#?}", bans);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_maps() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let maps = GlobalAPI::get_maps(true, None, &client).await;
	assert!(matches!(maps, Ok(_)));
	info!("{} maps", maps.unwrap().len());

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_map_() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let map_name = MapIdentifier::Name(String::from("kz_lionharder"));
	let map_id = MapIdentifier::ID(992);

	let map_name = GlobalAPI::get_map(&map_name, &client).await;
	assert!(matches!(map_name, Ok(_)));
	info!("{:#?}", map_name);

	let map_id = GlobalAPI::get_map(&map_id, &client).await;
	assert!(matches!(map_id, Ok(_)));
	info!("{:#?}", map_id);

	assert_eq!(map_name.as_ref().unwrap().name, map_id.as_ref().unwrap().name);
	assert_eq!(map_name.as_ref().unwrap().id, map_id.as_ref().unwrap().id);
	assert_eq!(map_name.as_ref().unwrap().filesize, map_id.as_ref().unwrap().filesize);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_mapcycle() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let mapcycle = GlobalAPI::get_mapcycle(None, &client).await?;
	let mapcycle_tier3 = GlobalAPI::get_mapcycle(Some(Tier::Medium), &client).await?;

	info!("{:#?}\n{} maps", mapcycle, mapcycle.len());
	info!("{:#?}\n{} tier 3 maps", mapcycle_tier3, mapcycle_tier3.len());

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn is_global() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	assert_eq!(
		Some(String::from("kz_lionharder")),
		GlobalAPI::is_global("kz_lionharder", &client).await
	);
	assert_eq!(
		Some(String::from("kz_lionharder")),
		GlobalAPI::is_global("lionharder", &client).await
	);
	assert_eq!(
		Some(String::from("kz_lionharder")),
		GlobalAPI::is_global("lionHARDER", &client).await
	);
	assert_eq!(
		Some(String::from("kz_micropenis")),
		GlobalAPI::is_global("penis", &client).await
	);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_modes() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let modes = GlobalAPI::get_modes(&client).await;
	assert!(matches!(modes, Ok(_)));
	info!("{:#?}", modes);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_mode_() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let kzt = Mode::KZTimer;
	let skz = Mode::SimpleKZ;
	let vnl = Mode::Vanilla;

	let kzt = GlobalAPI::get_mode(kzt, &client).await;
	assert!(matches!(kzt, Ok(_)));
	info!("{:#?}", kzt);

	let skz = GlobalAPI::get_mode(skz, &client).await;
	assert!(matches!(skz, Ok(_)));
	info!("{:#?}", skz);

	let vnl = GlobalAPI::get_mode(vnl, &client).await;
	assert!(matches!(vnl, Ok(_)));
	info!("{:#?}", vnl);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_players() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let players = GlobalAPI::get_players(Some(10), &client).await?;
	info!("{:#?}", players);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_player_t() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let name = PlayerIdentifier::Name(String::from("AlphaKeks"));
	let steam_id = PlayerIdentifier::SteamID(SteamID::new("STEAM_1:1:161178172")?);
	let steam_id64 = PlayerIdentifier::SteamID64(76561198282622073);

	let name = GlobalAPI::get_player(&name, &client).await?;
	let steam_id = GlobalAPI::get_player(&steam_id, &client).await?;
	let steam_id64 = GlobalAPI::get_player(&steam_id64, &client).await?;

	assert_eq!("AlphaKeks", name.name);
	assert_eq!("AlphaKeks", steam_id.name);
	assert_eq!("AlphaKeks", steam_id64.name);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_player_alts() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let llnuke = 76561198304022023.into();

	let alts = GlobalAPI::get_player_alts(&llnuke, &client).await?;
	info!("{:#?}", alts);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_filters() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let filters = GlobalAPI::get_filters(992, &client).await?;
	info!("{:#?}", filters);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_place() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let place = GlobalAPI::get_place(16557384, &client).await?;
	info!("place: {}", place);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_record() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let alphakeks_spacemario = GlobalAPI::get_record(16557384, &client).await?;

	assert_eq!(
		"AlphaKeks",
		alphakeks_spacemario
			.player_name
			.unwrap()
	);
	assert_eq!("STEAM_1:1:161178172", alphakeks_spacemario.steam_id.unwrap());

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_recent_lossy() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();

	let recent = GlobalAPI::get_recent_lossy(Mode::Vanilla, Some(5), &client).await?;
	assert_eq!(5, recent.len());

	info!("{:#?}", recent);

	Ok(())
}

// #[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_recent_t() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let alphakeks = PlayerIdentifier::SteamID64(76561198282622073);
	let jak = PlayerIdentifier::SteamID(SteamID::new("STEAM_0:1:45421221").unwrap());

	let _recent = GlobalAPI::get_recent(&alphakeks, Some(5), &client).await?;
	let recent = GlobalAPI::get_recent(&jak, Some(5), &client).await?;
	info!("{:#?}", recent);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_wr() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let lionharder = MapIdentifier::Name(String::from("kz_lionharder"));

	let lionharder_pro = GlobalAPI::get_wr(&lionharder, Mode::KZTimer, false, 0, &client).await?;
	info!("{:#?}", lionharder_pro);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_pb() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let alphakeks = PlayerIdentifier::Name(String::from("AlphaKeks"));
	let lionharder = MapIdentifier::Name(String::from("kz_lionharder"));

	let lionharder_tp =
		GlobalAPI::get_pb(&alphakeks, &lionharder, Mode::SimpleKZ, true, 0, &client).await?;
	info!("{:#?}", lionharder_tp);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_maptop() -> anyhow::Result<()> {
	let client = gokz_rs::Client::new();
	let lionharder = MapIdentifier::Name(String::from("kz_lionharder"));

	let lionharder_pro =
		GlobalAPI::get_maptop(&lionharder, Mode::SimpleKZ, false, 0, &client).await?;
	info!("{:#?}", lionharder_pro);

	let beginnerblock = MapIdentifier::Name(String::from("kz_beginnerblock_go"));

	let beginnerblock_pro =
		GlobalAPI::get_maptop(&beginnerblock, Mode::KZTimer, false, 0, &client).await?;
	assert_eq!(100, beginnerblock_pro.len());

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_replay_by_id() -> anyhow::Result<()> {
	let replay_link = GlobalAPI::get_replay_by_id(60107);
	info!("{}", replay_link);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn get_replay_by_record_id() -> anyhow::Result<()> {
	let replay_link = GlobalAPI::get_replay_by_record_id(16557384);
	info!("{}", replay_link);

	Ok(())
}

#[ignore = "expensive"]
#[test_log::test(tokio::test)]
async fn checkhealth() -> anyhow::Result<()> {
	let health = GlobalAPI::checkhealth(&gokz_rs::Client::new()).await?;
	info!("{:#?}", health);

	Ok(())
}
