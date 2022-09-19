pub fn is_even(x: isize) -> bool {
	if x % 2 == 0 {
		true
	} else {
		false
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_even_test() {
		let result1 = is_even(4);
		let result2 = is_even(5);
		let result3 = is_even(0);
		let result4 = is_even(-30);

		assert_eq!(result1, true);
		assert_eq!(result2, false);
		assert_eq!(result3, true);
		assert_eq!(result4, true);
	}
}
