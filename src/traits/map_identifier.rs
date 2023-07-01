/// Extension trait for any type that is a "KZ Map".
pub trait MapIdentifier {
	/// Provides a link to an image of the map, assuming [`Self`] is a `Name`.
	fn image_url(&self) -> Option<String>;

	/// Provides a link to the map's associated
	/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) route.
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String;

	/// Provides a link to the map's associated [KZ:GO](https://kzgo.eu/) page, assuming [`Self`]
	/// is a `Name`.
	#[cfg(feature = "kzgo-api")]
	fn kzgo(&self) -> Option<String>;

	/// Provides a link to the map's associated [KZ:GO](https://kzgo.eu/) API route, assuming
	/// [`Self`] is a `Name`.
	#[cfg(feature = "kzgo-api")]
	fn kzgo_api(&self) -> Option<String>;

	/// Provides a link to the map's associated [SchnoseAPI](https://schnose.xyz/) route.
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String;
}
