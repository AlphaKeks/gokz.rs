/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url(replay_id: u32) -> String {
	format!("records/replay/{replay_id}")
}
