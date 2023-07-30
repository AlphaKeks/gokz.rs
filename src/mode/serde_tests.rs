use {super::Mode, pretty_assertions::assert_eq, serde::Deserialize, serde_json::json};

#[test]
fn raw() {
	let inputs = [
		(json!("KZTimer"), Mode::KZTimer),
		(json!("kz_timer"), Mode::KZTimer),
		(json!(200), Mode::KZTimer),
		(json!("201"), Mode::SimpleKZ),
		(json!("vanilla"), Mode::Vanilla),
		(json!("skz"), Mode::SimpleKZ),
	];

	for (i, (input, expected)) in inputs.into_iter().enumerate() {
		let mode: Mode =
			serde_json::from_value(input).unwrap_or_else(|err| panic!("#{i}: {err:?}"));

		assert_eq!(mode, expected, "#{i}");
	}
}

#[test]
fn inside_struct() {
	#[derive(Deserialize)]
	struct Balls {
		mode: Mode,
	}

	let inputs = [
		(json!({ "mode": "kz_timer" }), Mode::KZTimer),
		(json!({ "mode": "KZT" }), Mode::KZTimer),
		(json!({ "mode": 200 }), Mode::KZTimer),
		(json!({ "mode": "202" }), Mode::Vanilla),
		(json!({ "mode": "simple_kz" }), Mode::SimpleKZ),
		(json!({ "mode": "skz" }), Mode::SimpleKZ),
	];

	for (i, (input, expected)) in inputs.into_iter().enumerate() {
		let balls: Balls =
			serde_json::from_value(input).unwrap_or_else(|err| panic!("#{i}: {err:?}"));

		assert_eq!(balls.mode, expected, "#{i}");
	}
}
