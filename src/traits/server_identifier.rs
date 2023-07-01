/// Extension trait for any type that is a "KZ Server".
pub trait ServerIdentifier {
	/// Provides a link to the server's associated
	/// [GlobalAPI](https://kztimerglobal.com/swagger/index.html?urls.primaryName=V2) route.
	#[cfg(feature = "global-api")]
	fn global_api(&self) -> String;

	/// Provides a link to the server's associated [SchnoseAPI](https://schnose.xyz/) route.
	#[cfg(feature = "schnose-api")]
	fn schnose_api(&self) -> String;
}
