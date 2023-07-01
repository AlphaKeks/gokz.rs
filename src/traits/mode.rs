/// Extension trait for any type that is a "KZ Mode".
pub trait Mode {
	/// The standard format of a [`Mode`] in the context of most popular APIs.
	fn api(&self) -> String;

	/// Abbreviation of the given mode's name. This is how players usually refer to the modes.
	fn short(&self) -> String;
}
