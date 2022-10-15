#![allow(dead_code)]

pub const BASE_URL: &'static str = "https://kztimerglobal.com/api/v2/";

pub trait ParamData {}
pub trait ResponseData {}

pub mod status {
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct ConditionResult {
		pub condition: String,
		pub success: bool,
	}

	#[derive(Debug, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct StatusResult {
		pub status: u16,
		pub hostname: String,
		pub duration: u32,
		pub condition_results: Vec<ConditionResult>,
		pub success: bool,
		pub timestamp: String,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct StatusEvent {
		pub r#type: String,
		pub timestamp: String,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct APIStatus {
		pub name: String,
		pub key: String,
		pub results: Vec<StatusResult>,
		pub events: Vec<StatusEvent>,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct APIStatusFancy {
		pub successful_responses: u8,
		pub fast_responses: u8,
	}
}

pub mod bans {
	use crate::global_api::ParamData;
	use crate::global_api::ResponseData;
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Clone, Serialize)]
	pub struct Params {
		pub ban_types: Option<String>,
		pub ban_types_list: Option<Vec<String>>,
		pub is_expired: Option<bool>,
		pub ip: Option<String>,
		pub steamid64: Option<String>,
		pub steam_id: Option<String>,
		pub notes_contains: Option<String>,
		pub stats_contains: Option<String>,
		pub server_id: Option<u32>,
		pub created_since: Option<String>,
		pub updated_since: Option<String>,
		pub offset: Option<i32>,
		pub limit: Option<u32>,
	}

	impl ParamData for Params {}

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	pub struct Response {
		pub id: u32,
		pub ban_type: String,
		pub expires_on: String,
		pub steamid64: String,
		pub player_name: String,
		pub steam_id: String,
		pub notes: String,
		pub stats: String,
		pub server_id: u16,
		pub updated_by_id: String,
		pub created_on: String,
		pub updated_on: String,
	}

	impl ResponseData for Response {}
	impl ResponseData for Vec<Response> {}

	impl Params {
		pub fn default() -> Self {
			Params {
				ban_types: None,
				ban_types_list: None,
				is_expired: None,
				ip: None,
				steamid64: None,
				steam_id: None,
				notes_contains: None,
				stats_contains: None,
				server_id: None,
				created_since: None,
				updated_since: None,
				offset: None,
				limit: Some(1),
			}
		}
	}
}

pub mod maps {
	use serde::{Deserialize, Serialize};

	use super::{ParamData, ResponseData};

	#[derive(Debug, Clone, Serialize)]
	pub struct Params {
		pub id: Option<Vec<u16>>,
		pub name: Option<String>,
		pub larger_than_filesize: Option<u32>,
		pub smaller_than_filesize: Option<u32>,
		pub is_validated: Option<bool>,
		pub difficulty: Option<u8>,
		pub created_since: Option<String>,
		pub updated_since: Option<String>,
		pub offset: Option<i32>,
		pub limit: Option<u32>,
	}

	impl ParamData for Params {}

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	pub struct Response {
		pub id: u16,
		pub name: String,
		pub filesize: u64,
		pub validated: bool,
		pub difficulty: u8,
		pub created_on: String,
		pub updated_on: String,
		pub approved_by_steamid64: String,
		pub workshop_url: String,
		pub download_url: Option<String>,
	}

	impl ResponseData for Response {}
	impl ResponseData for Vec<Response> {}

	impl Params {
		pub fn default() -> Self {
			Params {
				id: None,
				name: None,
				larger_than_filesize: None,
				smaller_than_filesize: None,
				is_validated: None,
				difficulty: None,
				created_since: None,
				updated_since: None,
				offset: None,
				limit: Some(1),
			}
		}
	}
}

pub mod modes {
	use serde::{Deserialize, Serialize};

	use super::{ParamData, ResponseData};

	#[derive(Debug, Clone, Serialize)]
	pub enum Params {
		Name(name::Params),
		Id(id::Params),
	}

	impl ParamData for Params {}

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	pub struct Response {
		pub id: u8,
		pub name: String,
		pub description: String,
		pub latest_version: u8,
		pub latest_version_description: String,
		pub website: String,
		pub repo: String,
		pub contact_steamid64: String,
		pub supported_tickrates: Option<u8>,
		pub created_on: String,
		pub updated_on: String,
		pub updated_by_id: String,
	}

	impl ResponseData for Response {}
	impl ResponseData for Vec<Response> {}

	pub mod name {
		use serde::Serialize;

		use crate::global_api::ParamData;

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {}

		impl ParamData for Params {}
	}

	pub mod id {
		use serde::Serialize;

		use crate::global_api::ParamData;

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {}

		impl ParamData for Params {}
	}
}

pub mod player_ranks {
	use serde::{Deserialize, Serialize};

	use super::{ParamData, ResponseData};

	#[allow(non_snake_case)]
	#[derive(Debug, Clone, Serialize)]
	pub struct Params {
		pub points_greater_than: Option<u32>,
		pub average_greater_than: Option<u32>,
		pub rating_greater_than: Option<u32>,
		pub finishes_greater_than: Option<u32>,
		pub steamid64s: Option<Vec<u64>>,
		pub record_filter_ids: Option<Vec<u32>>,
		pub map_ids: Option<Vec<u16>>,
		pub stages: Option<Vec<u8>>,
		pub mode_ids: Option<Vec<u8>>,
		pub tickrates: Option<Vec<u8>>,
		pub has_teleports: Option<bool>,
		pub mapTag: Option<String>,
		pub offset: Option<i32>,
		pub limit: Option<u32>,
	}

	impl ParamData for Params {}

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	pub struct Response {
		pub points: u32,
		pub average: u32,
		pub rating: u32,
		pub finishes: u32,
		pub steamid64: String,
		pub steamid: String,
		pub player_name: String,
	}

	impl ResponseData for Response {}
	impl ResponseData for Vec<Response> {}

	impl Params {
		pub fn default() -> Self {
			Params {
				points_greater_than: None,
				average_greater_than: None,
				rating_greater_than: None,
				finishes_greater_than: None,
				steamid64s: None,
				record_filter_ids: None,
				map_ids: None,
				stages: None,
				mode_ids: None,
				tickrates: None,
				has_teleports: None,
				mapTag: None,
				offset: None,
				limit: Some(1),
			}
		}
	}
}

pub mod players {
	use serde::{Deserialize, Serialize};

	use super::{ParamData, ResponseData};

	#[derive(Debug, Clone, Serialize)]
	pub struct Params {
		pub name: Option<String>,
		pub steam_id: Option<String>,
		pub is_banned: Option<bool>,
		pub total_records: Option<u32>,
		pub ip: Option<String>,
		pub steamid64_list: Option<Vec<u64>>,
		pub offset: Option<i32>,
		pub limit: Option<u32>,
	}

	impl ParamData for Params {}

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	pub struct Response {
		pub steamid64: String,
		pub steam_id: String,
		pub is_banned: bool,
		pub total_records: u32,
		pub name: String,
	}

	impl ResponseData for Response {}
	impl ResponseData for Vec<Response> {}

	impl Params {
		pub fn default() -> Self {
			Params {
				name: None,
				steam_id: None,
				is_banned: None,
				total_records: None,
				ip: None,
				steamid64_list: None,
				offset: None,
				limit: Some(1),
			}
		}
	}
}

pub mod record_filters {
	use serde::{Deserialize, Serialize};

	use super::{ParamData, ResponseData};

	#[derive(Debug, Clone, Serialize)]
	pub enum Params {
		Base(base::Params),
		Distributions(distributions::Params),
	}

	impl ParamData for Params {}

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	pub enum Response {
		Base(base::Response),
		Distributions(distributions::Response),
	}

	impl ResponseData for Response {}
	impl ResponseData for Vec<Response> {}

	pub mod base {
		use serde::{Deserialize, Serialize};

		use crate::global_api::{ParamData, ResponseData};

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {
			pub ids: Option<u32>,
			pub map_ids: Option<u16>,
			pub stages: Option<u8>,
			pub mode_ids: Option<u8>,
			pub tickrates: Option<u8>,
			pub has_teleports: Option<bool>,
			pub offset: Option<i32>,
			pub limit: Option<u32>,
		}

		impl ParamData for Params {}

		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		pub struct Response {
			pub id: u32,
			pub map_id: u16,
			pub stage: u8,
			pub mode_id: u8,
			pub tickrate: u8,
			pub has_teleports: bool,
			pub created_on: String,
			pub updated_by_id: String,
		}

		impl ResponseData for Response {}
		impl ResponseData for Vec<Response> {}

		impl Params {
			pub fn default() -> Self {
				Params {
					ids: None,
					map_ids: None,
					stages: None,
					mode_ids: None,
					tickrates: None,
					has_teleports: None,
					offset: None,
					limit: Some(1),
				}
			}
		}
	}

	pub mod distributions {
		use serde::{Deserialize, Serialize};

		use crate::global_api::{ParamData, ResponseData};

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {
			pub ids: Option<Vec<u32>>,
			pub map_ids: Option<Vec<u32>>,
			pub stages: Option<Vec<u8>>,
			pub mode_ids: Option<Vec<u8>>,
			pub tickrates: Option<u8>,
			pub has_teleports: Option<bool>,
			pub offset: Option<i32>,
			pub limit: Option<u32>,
		}

		impl ParamData for Params {}

		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		pub struct Response {
			pub record_filter_id: u32,
			pub c: f32,
			pub d: f32,
			pub loc: f32,
			pub scale: f32,
			pub top_scale: f32,
			pub created_on: String,
			pub updated_on: String,
			pub updated_by_id: String,
		}

		impl ResponseData for Response {}
		impl ResponseData for Vec<Response> {}

		impl Params {
			pub fn default() -> Self {
				Params {
					ids: None,
					map_ids: None,
					stages: None,
					mode_ids: None,
					tickrates: None,
					has_teleports: None,
					offset: None,
					limit: Some(1),
				}
			}
		}
	}
}

pub mod records {
	use serde::{Deserialize, Serialize};

	use super::{ParamData, ResponseData};

	#[derive(Debug, Clone, Serialize)]
	pub enum Params {
		Place(place::Params),
		Top(top::Params),
		WorldRecords(world_records::Params),
		Recent(recent::Params),
		RecordFilter(record_filter::Params),
		ReplayList(replay_list::Params),
	}

	impl ParamData for Params {}

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	pub enum Response {
		Place(place::Response),
		Top(top::Response),
		WorldRecords(world_records::Response),
		Recent(recent::Response),
		RecordFilter(record_filter::Response),
		ReplayList(replay_list::Response),
	}

	impl ResponseData for Response {}
	impl ResponseData for Vec<Response> {}

	pub mod place {
		use serde::{Deserialize, Serialize};

		use crate::global_api::{ParamData, ResponseData};

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {}

		impl ParamData for Params {}

		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		pub struct Response(pub u16);

		impl ResponseData for Response {}
		impl ResponseData for Vec<Response> {}
	}

	pub mod top {
		use serde::{Deserialize, Serialize};

		use crate::global_api::{ParamData, ResponseData};

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {
			pub steam_id: Option<String>,
			pub server_id: Option<u32>,
			pub steamid64: Option<u64>,
			pub map_id: Option<u16>,
			pub map_name: Option<String>,
			pub tickrate: Option<u8>,
			pub overall: Option<bool>,
			pub stage: Option<u8>,
			pub modes_list_string: Option<String>,
			pub modes_list: Option<Vec<String>>,
			pub has_teleports: Option<bool>,
			pub player_name: Option<String>,
			pub offset: Option<i32>,
			pub limit: Option<u32>,
		}

		impl ParamData for Params {}

		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		pub struct Response {
			pub id: u32,
			pub steamid64: String,
			pub player_name: Option<String>,
			pub steam_id: Option<String>,
			pub server_id: u16,
			pub map_id: u16,
			pub stage: u8,
			pub mode: String,
			pub tickrate: u8,
			pub time: f32,
			pub teleports: u32,
			pub created_on: String,
			pub updated_on: String,
			pub updated_by: u64,
			pub record_filter_id: i32,
			pub server_name: Option<String>,
			pub map_name: String,
			pub points: u16,
			pub replay_id: u32,
		}

		impl ResponseData for Response {}
		impl ResponseData for Vec<Response> {}

		impl Params {
			pub fn default() -> Self {
				Params {
					steam_id: None,
					server_id: None,
					steamid64: None,
					map_id: None,
					map_name: None,
					tickrate: None,
					overall: None,
					stage: None,
					modes_list_string: None,
					modes_list: None,
					has_teleports: None,
					player_name: None,
					offset: None,
					limit: Some(1),
				}
			}
		}
	}

	pub mod world_records {
		use serde::{Deserialize, Serialize};

		use crate::global_api::{ParamData, ResponseData};

		#[allow(non_snake_case)]
		#[derive(Debug, Clone, Serialize)]
		pub struct Params {
			pub ids: Option<Vec<u32>>,
			pub map_ids: Option<Vec<u16>>,
			pub stages: Option<Vec<u8>>,
			pub mode_ids: Option<Vec<u8>>,
			pub tickrates: Option<Vec<u8>>,
			pub has_teleports: Option<bool>,
			pub mapTag: Option<String>,
			pub offset: Option<i32>,
			pub limit: Option<u32>,
		}

		impl ParamData for Params {}

		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		pub struct Response {
			steamid64: String,
			steam_id: Option<String>,
			count: u32,
			player_name: Option<String>,
		}

		impl ResponseData for Response {}
		impl ResponseData for Vec<Response> {}

		impl Params {
			pub fn default() -> Self {
				Params {
					ids: None,
					map_ids: None,
					stages: None,
					mode_ids: None,
					tickrates: None,
					has_teleports: None,
					mapTag: None,
					offset: None,
					limit: Some(1),
				}
			}
		}
	}

	pub mod recent {
		use serde::{Deserialize, Serialize};

		use crate::global_api::{ParamData, ResponseData};

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {
			pub steam_id: Option<String>,
			pub steamid64: Option<u64>,
			pub map_id: Option<u16>,
			pub map_name: Option<String>,
			pub has_teleports: Option<bool>,
			pub tickrate: Option<u8>,
			pub stage: Option<u8>,
			pub modes_list_string: Option<String>,
			pub modes_list: Option<Vec<String>>,
			pub place_top_at_least: Option<u32>,
			pub place_top_overall_at_least: Option<u32>,
			pub created_since: Option<String>,
			pub offset: Option<i32>,
			pub limit: Option<u32>,
		}

		impl ParamData for Params {}

		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		pub struct Response {
			pub id: u32,
			pub steamid64: String,
			pub player_name: Option<String>,
			pub steam_id: Option<String>,
			pub server_id: u16,
			pub map_id: u16,
			pub stage: u8,
			pub mode: String,
			pub tickrate: u8,
			pub time: f32,
			pub teleports: u32,
			pub created_on: String,
			pub updated_on: String,
			pub updated_by: u64,
			pub place: u32,
			pub top_100: u8,
			pub top_100_overall: u8,
			pub server_name: Option<String>,
			pub map_name: String,
			pub points: u8,
			pub record_filter_id: u32,
			pub replay_id: u32,
		}

		impl ResponseData for Response {}
		impl ResponseData for Vec<Response> {}

		impl Params {
			pub fn default() -> Self {
				Params {
					steam_id: None,
					steamid64: None,
					map_id: None,
					map_name: None,
					has_teleports: None,
					tickrate: None,
					stage: None,
					modes_list_string: None,
					modes_list: None,
					place_top_at_least: None,
					place_top_overall_at_least: None,
					created_since: None,
					offset: None,
					limit: Some(1),
				}
			}
		}
	}

	pub mod record_filter {
		use serde::{Deserialize, Serialize};

		use crate::global_api::{ParamData, ResponseData};

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {
			pub ids: Option<Vec<u32>>,
			pub map_ids: Option<Vec<u16>>,
			pub stages: Option<Vec<u8>>,
			pub mode_ids: Option<Vec<u8>>,
			pub tickrates: Option<Vec<u8>>,
			pub has_teleports: Option<bool>,
			pub offset: Option<i32>,
			pub limit: Option<u32>,
		}

		impl ParamData for Params {}

		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		pub struct Response {
			pub record_id: u32,
			pub place: u32,
			pub top_100: u8,
			pub top_100_overall: u8,
			pub is_pb: bool,
			pub total_in_category: i32,
		}

		impl ResponseData for Response {}
		impl ResponseData for Vec<Response> {}

		impl Params {
			pub fn default() -> Self {
				Params {
					ids: None,
					map_ids: None,
					stages: None,
					mode_ids: None,
					tickrates: None,
					has_teleports: None,
					offset: None,
					limit: Some(1),
				}
			}
		}
	}

	pub mod replay_list {
		use serde::{Deserialize, Serialize};

		use crate::global_api::{ParamData, ResponseData};

		#[derive(Debug, Clone, Serialize)]
		pub struct Params {
			pub offset: Option<i32>,
			pub limit: Option<u32>,
		}

		impl ParamData for Params {}

		#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
		pub struct Response {
			pub id: u32,
			pub steamid64: String,
			pub server_id: u16,
			pub record_filter_id: u32,
			pub time: f32,
			pub teleports: u32,
			pub created_on: String,
			pub updated_on: String,
			pub updated_by: u64,
			pub points: u16,
			pub replay_id: u32,
		}

		impl ResponseData for Response {}
		impl ResponseData for Vec<Response> {}

		impl Params {
			pub fn default() -> Self {
				Params {
					offset: None,
					limit: Some(1),
				}
			}
		}
	}
}

pub mod profile {
	use crate::global_api::ParamData;
	use crate::global_api::ResponseData;
	use crate::prelude::Rank;
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Clone, Serialize)]
	pub struct Params {}

	impl ParamData for Params {}

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	pub struct Response {
		pub name: Option<String>,
		pub steam_id: Option<String>,
		pub steam_id64: Option<String>,
		pub rank: Option<Rank>,
		pub points: (u32, u32),
		pub records: (u32, u32),
		pub completion: [(u32, u32); 8],
		pub completion_percentage: [(f32, f32); 8],
		pub is_banned: Option<bool>,
	}

	impl ResponseData for Response {}
	impl ResponseData for Vec<Response> {}

	impl Response {
		pub fn default() -> Self {
			Response {
				name: None,
				steam_id: None,
				steam_id64: None,
				rank: None,
				points: (0, 0),
				records: (0, 0),
				completion: [(0, 0); 8],
				completion_percentage: [(0.0, 0.0); 8],
				is_banned: None,
			}
		}
	}
}
