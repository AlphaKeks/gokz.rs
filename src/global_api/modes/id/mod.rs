/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url(mode: &crate::prelude::Mode) -> String {
	format!("modes/id/{}", mode.as_id())
}
