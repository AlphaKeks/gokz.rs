/// Extension trait for any type that is a "KZ Record".
pub trait Record {
	/// Returns a link to download the replay for this record
	fn replay_download_link(&self) -> Option<String>;

	/// Returns a link to watch the replay for this record online
	///
	/// s/o [GC](https://github.com/GameChaos/GlobalReplays)
	fn replay_view_link(&self) -> Option<String>;
}
