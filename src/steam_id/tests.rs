use super::{
	AccountNumber, AccountType, AccountUniverse, SteamID, COMMUNITY_REGEX, STANDARD_REGEX,
};

mod regex {
	use super::*;

	#[test]
	fn standard_regex() {
		let valid_inputs = [
			"STEAM_0:1:161178172",
			"STEAM_1:1:161178172",
			"STEAM_0:0:79208088",
			"STEAM_1:0:79208088",
			"STEAM_0:0:448781326",
			"STEAM_1:0:448781326",
			"STEAM_0:0:102468802",
			"STEAM_1:0:102468802",
			"STEAM_0:1:117591961",
			"STEAM_1:1:117591961",
			"STEAM_0:0:135486492",
			"STEAM_1:0:135486492",
			"STEAM_0:1:66405608",
			"STEAM_1:1:66405608",
			"STEAM_0:0:165881949",
			"STEAM_1:0:165881949",
			"STEAM_0:1:96787045",
			"STEAM_1:1:96787045",
			"STEAM_0:0:46898346",
			"STEAM_1:0:46898346",
			"STEAM_0:1:152337044",
			"STEAM_1:1:152337044",
			"STEAM_0:0:182656610",
			"STEAM_1:0:182656610",
			"STEAM_0:0:237850931",
			"STEAM_1:0:237850931",
			"STEAM_0:1:31653734",
			"STEAM_1:1:31653734",
			"STEAM_0:1:236646428",
			"STEAM_1:1:236646428",
			"STEAM_0:0:18982494",
			"STEAM_1:0:18982494",
			"STEAM_0:0:246217957",
			"STEAM_1:0:246217957",
			"STEAM_0:1:65663138",
			"STEAM_1:1:65663138",
		];

		for (i, input) in valid_inputs.into_iter().enumerate() {
			assert!(STANDARD_REGEX.is_match(input), "#{i}: {input}");
		}

		let invalid_inputs = [
			"STEAM_2:0:1234567890",
			"STEAM_0:2:9876543210",
			"STEAM_1:3:2468135790",
			"STEAM_2:1:333",
			"STEAM_1:3:444",
			"STEAM_1:2:777",
			"STEAM_1:3:888",
			"STEAM_1:3:999",
			"STEAM_0:2:0000",
			"STEAM_0:2:1111",
			"STEAM_0:2:2222",
			"STEAM_0:2:3333",
			"STEAM_0:2:4444",
			"STEAM_0:2:5555",
			"STEAM_0:2:6666",
			"STEAM_0:2:7777",
			"STEAM_0:2:8888",
			"STEAM_0:2:9999",
			"STEAM_0:3:0000",
			"STEAM_0:3:1111",
			"STEAM_0:3:2222",
			"STEAM_0:3:3333",
			"STEAM_0:3:4444",
			"STEAM_0:3:5555",
			"STEAM_0:3:6666",
			"STEAM_0:3:7777",
			"STEAM_0:3:8888",
			"STEAM_0:3:9999",
			"STEAM_1:0:",
			"STEAM_0:1:",
			"STEAM_2:1:",
			"STEAM_1:3:",
			"STEAM_0:1:",
			"STEAM_1:2:",
			"STEAM_1:3:",
			"STEAM_1:0:",
			"STEAM_0:1:",
			"STEAM_0:2:",
			"STEAM_0:3:",
		];

		for (i, input) in invalid_inputs.into_iter().enumerate() {
			assert!(!STANDARD_REGEX.is_match(input), "#{i}: {input}");
		}
	}

	#[test]
	fn community_regex() {
		let valid_inputs = [
			"U:1:322356345",
			"[U:1:322356345]",
			"U:1:158416176",
			"[U:1:158416176]",
			"U:1:897562652",
			"[U:1:897562652]",
			"U:1:204937604",
			"[U:1:204937604]",
			"U:1:235183923",
			"[U:1:235183923]",
			"U:1:270972984",
			"[U:1:270972984]",
			"U:1:132811217",
			"[U:1:132811217]",
			"U:1:331763898",
			"[U:1:331763898]",
			"U:1:193574091",
			"[U:1:193574091]",
			"U:1:93796692",
			"[U:1:93796692]",
			"U:1:304674089",
			"[U:1:304674089]",
			"U:1:365313220",
			"[U:1:365313220]",
			"U:1:475701862",
			"[U:1:475701862]",
			"U:1:63307469",
			"[U:1:63307469]",
			"U:1:473292857",
			"[U:1:473292857]",
			"U:1:37964988",
			"[U:1:37964988]",
			"U:1:492435914",
			"[U:1:492435914]",
			"U:1:131326277",
			"[U:1:131326277]",
		];

		for (i, input) in valid_inputs.into_iter().enumerate() {
			assert!(COMMUNITY_REGEX.is_match(input), "#{i}: {input}");
		}

		let invalid_inputs = [
			"U:0:12345",
			"U:0:9876",
			"U:2:44444",
			"U:1:ABCDE",
			"U:2:0",
			"U:0:ABC",
			"U:2:222",
			"U:1:DEF",
			"U:1:",
			"U:2:",
			"U:0:",
			"U:1:1:",
			"U:1:2:",
			"U:1:333:",
			"U:1:1A",
			"U:1:4B",
			"U:1:GHIJK",
			"U:0:5555",
			"U:0:0",
			"U:2:999",
			"U:2:9999",
			"U:2:ABCDEF",
			"U:2:123456",
			"U:0:1234567",
			"U:0:12",
			"U:0:99999",
			"U:0:111",
			"U:0:555",
			"U:0:654321",
			"U:0:987654",
			"U:0:66666",
			"U:0:22222",
			"U:2:7654321",
			"U:2:44444444",
			"U:2:22222222",
			"U:2:0",
			"U:2:666",
			"U:2:999999",
			"U:2:654321",
			"U:2:987654",
			"U:2:44444",
			"U:2:111111",
			"U:1:1A",
			"U:1:4B",
			"U:1:GHIJK",
			"U:1:DEFG",
			"U:1:ZYXWV",
			"U:1:PONML",
			"U:1:QWERTY",
			"U:1:ABCDEFGHIJKLMNOPQRSTUVWXYZ",
			"U:1:Z",
			"U:1:ABCDEF",
		];

		for (i, input) in invalid_inputs.into_iter().enumerate() {
			assert!(!COMMUNITY_REGEX.is_match(input), "#{i}: {input}");
		}
	}
}

#[test]
fn from_standard() {
	let inputs = [
		("STEAM_0:1:161178172", SteamID(76561198282622073_u64)),
		("STEAM_1:1:161178172", SteamID(76561198282622073_u64)),
		("STEAM_0:0:79208088", SteamID(76561198118681904_u64)),
		("STEAM_1:0:79208088", SteamID(76561198118681904_u64)),
		("STEAM_0:0:448781326", SteamID(76561198857828380_u64)),
		("STEAM_1:0:448781326", SteamID(76561198857828380_u64)),
		("STEAM_0:0:102468802", SteamID(76561198165203332_u64)),
		("STEAM_1:0:102468802", SteamID(76561198165203332_u64)),
		("STEAM_0:1:117591961", SteamID(76561198195449651_u64)),
		("STEAM_1:1:117591961", SteamID(76561198195449651_u64)),
		("STEAM_0:0:135486492", SteamID(76561198231238712_u64)),
		("STEAM_1:0:135486492", SteamID(76561198231238712_u64)),
		("STEAM_0:1:66405608", SteamID(76561198093076945_u64)),
		("STEAM_1:1:66405608", SteamID(76561198093076945_u64)),
		("STEAM_0:0:165881949", SteamID(76561198292029626_u64)),
		("STEAM_1:0:165881949", SteamID(76561198292029626_u64)),
		("STEAM_0:1:96787045", SteamID(76561198153839819_u64)),
		("STEAM_1:1:96787045", SteamID(76561198153839819_u64)),
		("STEAM_0:0:46898346", SteamID(76561198054062420_u64)),
		("STEAM_1:0:46898346", SteamID(76561198054062420_u64)),
		("STEAM_0:1:152337044", SteamID(76561198264939817_u64)),
		("STEAM_1:1:152337044", SteamID(76561198264939817_u64)),
		("STEAM_0:0:182656610", SteamID(76561198325578948_u64)),
		("STEAM_1:0:182656610", SteamID(76561198325578948_u64)),
		("STEAM_0:0:237850931", SteamID(76561198435967590_u64)),
		("STEAM_1:0:237850931", SteamID(76561198435967590_u64)),
		("STEAM_0:1:31653734", SteamID(76561198023573197_u64)),
		("STEAM_1:1:31653734", SteamID(76561198023573197_u64)),
		("STEAM_0:1:236646428", SteamID(76561198433558585_u64)),
		("STEAM_1:1:236646428", SteamID(76561198433558585_u64)),
		("STEAM_0:0:18982494", SteamID(76561197998230716_u64)),
		("STEAM_1:0:18982494", SteamID(76561197998230716_u64)),
		("STEAM_0:0:246217957", SteamID(76561198452701642_u64)),
		("STEAM_1:0:246217957", SteamID(76561198452701642_u64)),
		("STEAM_0:1:65663138", SteamID(76561198091592005_u64)),
		("STEAM_1:1:65663138", SteamID(76561198091592005_u64)),
	];

	for (i, (input, expected)) in inputs.into_iter().enumerate() {
		let steam_id = SteamID::new(input).unwrap_or_else(|_| panic!("Failed at #{i}"));
		assert_eq!(steam_id, expected, "#{i}");
	}
}

#[test]
fn from_community() {
	let inputs = [
		("U:1:322356345", SteamID(76561198282622073_u64)),
		("[U:1:322356345]", SteamID(76561198282622073_u64)),
		("U:1:158416176", SteamID(76561198118681904_u64)),
		("[U:1:158416176]", SteamID(76561198118681904_u64)),
		("U:1:897562652", SteamID(76561198857828380_u64)),
		("[U:1:897562652]", SteamID(76561198857828380_u64)),
		("U:1:204937604", SteamID(76561198165203332_u64)),
		("[U:1:204937604]", SteamID(76561198165203332_u64)),
		("U:1:235183923", SteamID(76561198195449651_u64)),
		("[U:1:235183923]", SteamID(76561198195449651_u64)),
		("U:1:270972984", SteamID(76561198231238712_u64)),
		("[U:1:270972984]", SteamID(76561198231238712_u64)),
		("U:1:132811217", SteamID(76561198093076945_u64)),
		("[U:1:132811217]", SteamID(76561198093076945_u64)),
		("U:1:331763898", SteamID(76561198292029626_u64)),
		("[U:1:331763898]", SteamID(76561198292029626_u64)),
		("U:1:193574091", SteamID(76561198153839819_u64)),
		("[U:1:193574091]", SteamID(76561198153839819_u64)),
		("U:1:93796692", SteamID(76561198054062420_u64)),
		("[U:1:93796692]", SteamID(76561198054062420_u64)),
		("U:1:304674089", SteamID(76561198264939817_u64)),
		("[U:1:304674089]", SteamID(76561198264939817_u64)),
		("U:1:365313220", SteamID(76561198325578948_u64)),
		("[U:1:365313220]", SteamID(76561198325578948_u64)),
		("U:1:475701862", SteamID(76561198435967590_u64)),
		("[U:1:475701862]", SteamID(76561198435967590_u64)),
		("U:1:63307469", SteamID(76561198023573197_u64)),
		("[U:1:63307469]", SteamID(76561198023573197_u64)),
		("U:1:473292857", SteamID(76561198433558585_u64)),
		("[U:1:473292857]", SteamID(76561198433558585_u64)),
		("U:1:37964988", SteamID(76561197998230716_u64)),
		("[U:1:37964988]", SteamID(76561197998230716_u64)),
		("U:1:492435914", SteamID(76561198452701642_u64)),
		("[U:1:492435914]", SteamID(76561198452701642_u64)),
		("U:1:131326277", SteamID(76561198091592005_u64)),
		("[U:1:131326277]", SteamID(76561198091592005_u64)),
	];

	for (i, (input, expected)) in inputs.into_iter().enumerate() {
		let steam_id = SteamID::new(input).unwrap_or_else(|_| panic!("Failed at #{i}"));
		assert_eq!(steam_id, expected, "#{i}");
	}
}

#[test]
fn e2e() {
	let inputs = [
		(["STEAM_0:1:161178172", "U:1:322356345", "322356345", "76561198282622073"], TestCase {
			display: "STEAM_1:1:161178172",
			id64: 76561198282622073,
			community_id: 322356345,
			account_universe: 1,
			account_type: 1,
			account_number: 161178172,
		}),
		(["STEAM_0:0:79208088", "U:1:158416176", "158416176", "76561198118681904"], TestCase {
			display: "STEAM_1:0:79208088",
			id64: 76561198118681904,
			community_id: 158416176,
			account_universe: 1,
			account_type: 0,
			account_number: 79208088,
		}),
		(["STEAM_0:0:448781326", "U:1:897562652", "897562652", "76561198857828380"], TestCase {
			display: "STEAM_1:0:448781326",
			id64: 76561198857828380,
			community_id: 897562652,
			account_universe: 1,
			account_type: 0,
			account_number: 448781326,
		}),
		(["STEAM_0:0:102468802", "U:1:204937604", "204937604", "76561198165203332"], TestCase {
			display: "STEAM_1:0:102468802",
			id64: 76561198165203332,
			community_id: 204937604,
			account_universe: 1,
			account_type: 0,
			account_number: 102468802,
		}),
		(["STEAM_0:1:117591961", "U:1:235183923", "235183923", "76561198195449651"], TestCase {
			display: "STEAM_1:1:117591961",
			id64: 76561198195449651,
			community_id: 235183923,
			account_universe: 1,
			account_type: 1,
			account_number: 117591961,
		}),
		(["STEAM_0:0:135486492", "U:1:270972984", "270972984", "76561198231238712"], TestCase {
			display: "STEAM_1:0:135486492",
			id64: 76561198231238712,
			community_id: 270972984,
			account_universe: 1,
			account_type: 0,
			account_number: 135486492,
		}),
		(["STEAM_0:1:66405608", "U:1:132811217", "132811217", "76561198093076945"], TestCase {
			display: "STEAM_1:1:66405608",
			id64: 76561198093076945,
			community_id: 132811217,
			account_universe: 1,
			account_type: 1,
			account_number: 66405608,
		}),
		(["STEAM_0:0:165881949", "U:1:331763898", "331763898", "76561198292029626"], TestCase {
			display: "STEAM_1:0:165881949",
			id64: 76561198292029626,
			community_id: 331763898,
			account_universe: 1,
			account_type: 0,
			account_number: 165881949,
		}),
		(["STEAM_0:1:96787045", "U:1:193574091", "193574091", "76561198153839819"], TestCase {
			display: "STEAM_1:1:96787045",
			id64: 76561198153839819,
			community_id: 193574091,
			account_universe: 1,
			account_type: 1,
			account_number: 96787045,
		}),
		(["STEAM_0:0:46898346", "U:1:93796692", "93796692", "76561198054062420"], TestCase {
			display: "STEAM_1:0:46898346",
			id64: 76561198054062420,
			community_id: 93796692,
			account_universe: 1,
			account_type: 0,
			account_number: 46898346,
		}),
		(["STEAM_0:1:152337044", "U:1:304674089", "304674089", "76561198264939817"], TestCase {
			display: "STEAM_1:1:152337044",
			id64: 76561198264939817,
			community_id: 304674089,
			account_universe: 1,
			account_type: 1,
			account_number: 152337044,
		}),
		(["STEAM_0:0:182656610", "U:1:365313220", "365313220", "76561198325578948"], TestCase {
			display: "STEAM_1:0:182656610",
			id64: 76561198325578948,
			community_id: 365313220,
			account_universe: 1,
			account_type: 0,
			account_number: 182656610,
		}),
		(["STEAM_0:0:237850931", "U:1:475701862", "475701862", "76561198435967590"], TestCase {
			display: "STEAM_1:0:237850931",
			id64: 76561198435967590,
			community_id: 475701862,
			account_universe: 1,
			account_type: 0,
			account_number: 237850931,
		}),
		(["STEAM_0:1:31653734", "U:1:63307469", "63307469", "76561198023573197"], TestCase {
			display: "STEAM_1:1:31653734",
			id64: 76561198023573197,
			community_id: 63307469,
			account_universe: 1,
			account_type: 1,
			account_number: 31653734,
		}),
		(["STEAM_0:1:236646428", "U:1:473292857", "473292857", "76561198433558585"], TestCase {
			display: "STEAM_1:1:236646428",
			id64: 76561198433558585,
			community_id: 473292857,
			account_universe: 1,
			account_type: 1,
			account_number: 236646428,
		}),
		(["STEAM_0:0:18982494", "U:1:37964988", "37964988", "76561197998230716"], TestCase {
			display: "STEAM_1:0:18982494",
			id64: 76561197998230716,
			community_id: 37964988,
			account_universe: 1,
			account_type: 0,
			account_number: 18982494,
		}),
		(["STEAM_0:0:246217957", "U:1:492435914", "492435914", "76561198452701642"], TestCase {
			display: "STEAM_1:0:246217957",
			id64: 76561198452701642,
			community_id: 492435914,
			account_universe: 1,
			account_type: 0,
			account_number: 246217957,
		}),
		(["STEAM_0:1:65663138", "U:1:131326277", "131326277", "76561198091592005"], TestCase {
			display: "STEAM_1:1:65663138",
			id64: 76561198091592005,
			community_id: 131326277,
			account_universe: 1,
			account_type: 1,
			account_number: 65663138,
		}),
	];

	for (i, (inputs, case)) in inputs.into_iter().enumerate() {
		for input in inputs {
			let steam_id = SteamID::new(input).unwrap_or_else(|_| panic!("Failed at #{i}"));
			assert_eq!(steam_id.to_string(), case.display, "#{i}");
			assert_eq!(steam_id.community_id(), case.community_id, "#{i}");
			assert_eq!(steam_id.account_universe(), case.account_universe, "#{i}");
			assert_eq!(steam_id.account_type(), case.account_type, "#{i}");
			assert_eq!(steam_id.account_number(), case.account_number, "#{i}");
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
struct TestCase {
	display: &'static str,
	id64: u64,
	community_id: u32,
	account_universe: AccountUniverse,
	account_type: AccountType,
	account_number: AccountNumber,
}
