#![allow(dead_code, unused_imports)]

pub mod global_api;
pub mod prelude;

use global_api::maps;
use prelude::{Error, ErrorKind, MapIdentifier};

pub async fn is_global(map: MapIdentifier, map_list: &Vec<maps::Response>) -> Result<maps::Response, Error> {
	match map {
		MapIdentifier::Name(name) => {
			for map in map_list {
				if map.name.contains(&name) {
					return Ok(map.to_owned());
				}
			}
		}
		MapIdentifier::Id(id) => {
			for map in map_list {
				if map.id == id {
					return Ok(map.to_owned());
				}
			}
		}
	}

	Err(Error {
		kind: ErrorKind::InvalidInput,
		tldr: "This map is not global.",
		raw: None,
	})
}

#[cfg(test)]
mod function_tests {
	use crate::{
		global_api::functions::{check_api, get_map, get_maps, get_mode, get_modes},
		is_global,
		prelude::{MapIdentifier, Mode},
	};

	#[tokio::test]
	async fn check_api_test() {
		let client = reqwest::Client::new();

		match check_api(&client).await {
			Ok(res) => println!("Success: {:#?}", res),
			Err(err) => println!("Fail: {:#?}", err),
		}
	}

	#[tokio::test]
	async fn get_maps_test() {
		let client = reqwest::Client::new();

		match get_maps(&client).await {
			Ok(res) => println!("Success: Got {} maps.", res.len()),
			Err(err) => println!("Fail: {:#?}", err),
		}
	}

	#[tokio::test]
	async fn get_map_test() {
		let client = reqwest::Client::new();

		let lionharder_name1 = MapIdentifier::Name("kz_lionharder");
		let lionharder_name2 = MapIdentifier::Name("lionHard");
		let lionharder_id = MapIdentifier::Id(992);
		let erratum_name = MapIdentifier::Name("kz_erratum_v2");

		run(lionharder_name1, &client).await;
		run(lionharder_name2, &client).await;
		run(lionharder_id, &client).await;
		run(erratum_name, &client).await;

		async fn run(map: MapIdentifier, client: &reqwest::Client) {
			match get_map(map, client).await {
				Ok(res) => println!("Success: {:#?}", res),
				Err(err) => println!("Fail: {:#?}", err),
			}
		}
	}

	#[tokio::test]
	async fn is_global_test() {
		let client = reqwest::Client::new();

		let maps = get_maps(&client).await.unwrap();

		match is_global(MapIdentifier::Name("kz_lionharder"), &maps).await {
			Ok(map) => println!("Success: {:#?}", map),
			Err(err) => println!("Fail: {:#?}", err),
		}

		match is_global(MapIdentifier::Id(992), &maps).await {
			Ok(map) => println!("Success: {:#?}", map),
			Err(err) => println!("Fail: {:#?}", err),
		}

		match is_global(MapIdentifier::Name("kz_penisman"), &maps).await {
			Ok(map) => println!("The fuck: {:#?}", map),
			Err(err) => println!("Success (hopefully): {:#?}", err),
		}

		match is_global(MapIdentifier::Id(42069), &maps).await {
			Ok(map) => println!("The fuck: {:#?}", map),
			Err(err) => println!("Success (hopefully): {:#?}", err),
		}

		match is_global(MapIdentifier::Id(0), &maps).await {
			Ok(map) => println!("The fuck: {:#?}", map),
			Err(err) => println!("Success (hopefully): {:#?}", err),
		}

		match is_global(MapIdentifier::Id(1), &maps).await {
			Ok(map) => println!("The fuck: {:#?}", map),
			Err(err) => println!("Success (hopefully): {:#?}", err),
		}
	}

	#[tokio::test]
	async fn get_modes_test() {
		let client = reqwest::Client::new();

		match get_modes(&client).await {
			Ok(modes) => println!("Success: {:#?}", modes),
			Err(why) => println!("Fail: {:#?}", why),
		}
	}

	#[tokio::test]
	async fn get_mode_test() {
		let client = reqwest::Client::new();

		match get_mode(Mode::KZTimer, &client).await {
			Ok(mode) => {
				assert_eq!(200, mode.id);
				println!("Success: {:#?}", mode);
			}
			Err(why) => println!("Fail: {:#?}", why),
		}

		match get_mode(Mode::SimpleKZ, &client).await {
			Ok(mode) => {
				assert_eq!(201, mode.id);
				println!("Success: {:#?}", mode);
			}
			Err(why) => println!("Fail: {:#?}", why),
		}

		match get_mode(Mode::Vanilla, &client).await {
			Ok(mode) => {
				assert_eq!(202, mode.id);
				println!("Success: {:#?}", mode);
			}
			Err(why) => println!("Fail: {:#?}", why),
		}
	}
}
