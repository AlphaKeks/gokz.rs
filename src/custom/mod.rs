pub mod profile;
use std::collections::HashMap;

use crate::{
	global_api::{get_maps, get_player, get_records},
	kzgo,
	prelude::*,
};

/// Will gather a bunch of data about a player from the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) and [KZGO](https://kzgo.eu/).
pub async fn get_profile(
	player_identifier: &PlayerIdentifier,
	mode: &Mode,
	client: &reqwest::Client,
) -> Result<profile::Response, Error> {
	let mut player = match get_player(player_identifier, client).await {
		Ok(data) => profile::Response {
			name: Some(data.name),
			steam_id: Some(data.steam_id),
			steam_id64: Some(data.steamid64),
			is_banned: Some(data.is_banned),
			..Default::default()
		},
		Err(why) => return Err(why),
	};

	let mut tier_maps = [HashMap::new(), HashMap::new()];

	let global_maps = match get_maps(client).await {
		Ok(maps) => maps,
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::custom::get_profile", ..why })
		},
	};

	for map in global_maps {
		tier_maps[0].insert(map.name, map.difficulty);
	}
	tier_maps[1] = tier_maps[0].clone();

	let tp = get_records(player_identifier, mode, true, 0, client)
		.await
		.unwrap_or(Vec::new());
	let pro = get_records(player_identifier, mode, false, 0, client)
		.await
		.unwrap_or(Vec::new());

	if tp.len() == 0 && pro.len() == 0 {
		return Err(Error {
			kind: ErrorKind::NoData,
			tldr: String::from("This player has 0 completions."),
			origin: String::from("gokz_rs::custom::get_profile"),
			raw: None,
		});
	}

	let x = if tp.len() > pro.len() { tp.len() } else { pro.len() };

	for i in 0..x {
		if tp.len() > i {
			if tier_maps[0].contains_key(&tp[i].map_name) {
				player.points.0 += tp[i].points as u32;
				player.completion[7].0 += 1;

				if let Some(tier) = tier_maps[0].get(&tp[i].map_name) {
					player.completion[(tier - 1) as usize].0 += 1;
				}

				if tp[i].points == 1000 {
					player.records.0 += 1;
				}

				tier_maps[0].remove(&tp[i].map_name);
			}
		}

		if pro.len() > i {
			if tier_maps[1].contains_key(&pro[i].map_name) {
				player.points.1 += pro[i].points as u32;
				player.completion[7].1 += 1;

				if let Some(tier) = tier_maps[1].get(&pro[i].map_name) {
					player.completion[(tier - 1) as usize].1 += 1;
				}

				if pro[i].points == 1000 {
					player.records.1 += 1;
				}

				tier_maps[1].remove(&pro[i].map_name);
			}
		}
	}

	let total_points = &player.points.0 + &player.points.1;
	player.rank = Some(Rank::from_points(total_points, mode));

	let doable = match kzgo::completion::get_completion_count(mode, client).await {
		Ok(completion_count) => completion_count,
		Err(why) => {
			return Err(Error { origin: why.origin + " > gokz_rs::custom::get_profile", ..why })
		},
	};

	let doable = [
		[
			doable.tp.one,
			doable.tp.two,
			doable.tp.three,
			doable.tp.four,
			doable.tp.five,
			doable.tp.six,
			doable.tp.seven,
			doable.tp.total,
		],
		[
			doable.pro.one,
			doable.pro.two,
			doable.pro.three,
			doable.pro.four,
			doable.pro.five,
			doable.pro.six,
			doable.pro.seven,
			doable.pro.total,
		],
	];

	for i in 0..8 {
		if player.completion[i].0 > 0 {
			player.completion_percentage[i].0 =
				(player.completion[i].0 as f32 / doable[0][i] as f32) * 100.0;
		}

		if player.completion[i].1 > 0 {
			player.completion_percentage[i].1 =
				(player.completion[i].1 as f32 / doable[1][i] as f32) * 100.0;
		}
	}

	return Ok(player);
}

#[cfg(test)]
#[tokio::test]
async fn get_profile_test() {
	let client = reqwest::Client::new();

	match get_profile(&PlayerIdentifier::Name(String::from("AlphaKeks")), &Mode::SimpleKZ, &client)
		.await
	{
		Err(why) => panic!("Test failed: {:#?}", why),
		Ok(player) => println!("Test successful: {:#?}", player),
	}
}
