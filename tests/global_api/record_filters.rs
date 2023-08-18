use {
	color_eyre::Result,
	gokz_rs::{global_api, global_api::RecordFilter},
	serde_json::json,
};

#[tokio::test]
async fn get_filters() -> Result<()> {
	let expected = json!([
		{
			"id": 8155,
			"map_id": 992,
			"stage": 0,
			"mode_id": 200,
			"tickrate": 128,
			"has_teleports": true,
			"created_on": "2021-06-05T15:52:16",
			"updated_on": "2021-06-05T15:52:16",
			"updated_by_id": "76561198143205331"
		},
		{
			"id": 8157,
			"map_id": 992,
			"stage": 0,
			"mode_id": 201,
			"tickrate": 128,
			"has_teleports": true,
			"created_on": "2021-06-05T15:52:16",
			"updated_on": "2021-06-05T15:52:16",
			"updated_by_id": "76561198143205331"
		},
		{
			"id": 8197,
			"map_id": 992,
			"stage": 1,
			"mode_id": 200,
			"tickrate": 128,
			"has_teleports": true,
			"created_on": "2021-06-05T15:53:23",
			"updated_on": "2021-06-05T15:53:23",
			"updated_by_id": "76561198143205331"
		},
		{
			"id": 8199,
			"map_id": 992,
			"stage": 1,
			"mode_id": 201,
			"tickrate": 128,
			"has_teleports": true,
			"created_on": "2021-06-05T15:53:23",
			"updated_on": "2021-06-05T15:53:23",
			"updated_by_id": "76561198143205331"
		}
	]);

	let expected = serde_json::from_value::<Vec<RecordFilter>>(expected)?;

	let params = global_api::filters::Params {
		map_ids: Some(vec![992]),
		stages: Some(vec![0, 1]),
		tickrate: Some(128),
		runtype: Some(true.into()),
		..Default::default()
	};

	let result = global_api::get_filters_with(&params, &crate::GOKZ_CLIENT).await?;

	assert_eq!(result, expected);

	Ok(())
}
