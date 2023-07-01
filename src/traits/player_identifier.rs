/// Extension trait for any type that is a "KZ Player".
pub trait PlayerIdentifier {
	/// Provides a link to the player's steam profile.
	fn steam_profile(&self) -> String;

	/// Provides a link to the players's associated
	/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) route.
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String;

	/// Provides a link to the player's associated [KZ:GO](https://kzgo.eu/) page.
	#[cfg(feature = "kzgo-api")]
	fn kzgo(&self) -> String;

	/// Provides a link to the player's associated [SchnoseAPI](https://schnose.xyz/) route.
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String;
}
