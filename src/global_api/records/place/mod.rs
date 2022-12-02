/// Constructs the API route for this module so it can be used in combination with the
/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s base URL.
pub fn get_url(record_id: &u32) -> String {
	format!("records/place/{record_id}")
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
/// The shape of the [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2)'s response on this route
pub struct Place(pub u32);

impl super::super::IsResponse for Place {}
impl super::super::IsResponse for Vec<Place> {}
