use {
	crate::{
		global_api::{self, GlobalAPI},
		prelude::*,
	},
	log::info,
};

/// Creates a list of names of maps a player hasn't completed.
pub async fn get_unfinished(
	player_identifier: &PlayerIdentifier,
	mode: &Mode,
	runtype: bool,
	tier: Option<u8>,
	client: &crate::Client,
) -> Result<Vec<String>, Error> {
	info!("[extra::get_unfinished] starting...");

	// fetch all ids of maps completed by the player
	let completed_maps =
		GlobalAPI::get_player_records(player_identifier, mode, runtype, 0, Some(9999), client)
			.await?
			.into_iter()
			.map(|rec| rec.map_id)
			.collect::<Vec<u32>>();

	// fetch filters for current mode and runtype and filter against the maps the player has
	// completed
	let uncompleted_ids = global_api::record_filters::get(
		global_api::record_filters::Params {
			stages: Some(0),
			mode_ids: Some(*mode as u8),
			tickrates: Some(128),
			has_teleports: Some(runtype),
			limit: Some(9999),
			..Default::default()
		},
		client,
	)
	.await?
	.into_iter()
	.filter_map(|record_filter| {
		if !completed_maps.contains(&record_filter.map_id) {
			return Some(record_filter.map_id);
		}
		None
	})
	.collect::<Vec<u32>>();

	// fetch all global maps and filter out the names of the ones we want
	let uncompleted_map_names = GlobalAPI::get_maps(true, Some(9999), client)
		.await?
		.into_iter()
		.filter_map(|map| {
			let tier_matches = match tier {
				Some(tier) => map.difficulty == tier,
				None => true,
			};

			let runtype_matches = match runtype {
				true => !&map.name.starts_with("kzpro_"),
				false => true,
			};

			if uncompleted_ids.contains(&map.id) && tier_matches && runtype_matches {
				return Some(map.name);
			}
			None
		})
		.collect::<Vec<String>>();

	info!("[extra::get_unfinished] completed successfully.");

	Ok(uncompleted_map_names)
}
