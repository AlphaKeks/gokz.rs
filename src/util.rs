//! Module containing utility functions that I couldn't find a better place for.

/// Returns the given `seconds` as a `hh:mm:ss.ms` formatted string.
pub fn format_time(seconds: f64) -> String {
	let hours = (seconds / 3600.0) as u8;
	let minutes = ((seconds % 3600.0) / 60.0) as u8;
	let seconds = seconds % 60.0;

	if hours == 0 {
		format!("{minutes:02}:{seconds:06.3}")
	} else {
		format!("{hours:02}:{minutes:02}:{seconds:06.3}")
	}
}
