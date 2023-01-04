use gokz_rs::prelude::*;

#[test]
fn steam_id_constructor() {
	let valid = SteamID::new("STEAM_1:1:161178172");
	let invalid1 = SteamID::new("wordSTEAM_1:1:161178172");
	let invalid2 = SteamID::new("STEAM_1:1:161178172word");
	let invalid3 = SteamID::new("some random text");

	assert!(valid.is_ok());
	assert!(invalid1.is_err());
	assert!(invalid2.is_err());
	assert!(invalid3.is_err());
}

#[test]
fn steam_id_from_playeridentifier() {
	let name = PlayerIdentifier::Name(String::from("AlphaKeks"));
	let steam_id = PlayerIdentifier::SteamID(SteamID::new("STEAM_1:1:161178172").unwrap());
	let steam_id64 = PlayerIdentifier::SteamID64(76561198282622073);

	assert!(SteamID::try_from(name).is_err());
	assert!(SteamID::try_from(steam_id).is_ok());
	assert!(SteamID::try_from(steam_id64).is_err());
}

#[test]
fn steam_id_from_u64() {
	let alphakeks_64 = 76561198282622073;
	let alphakeks_32 = "STEAM_1:1:161178172";
	assert_eq!(alphakeks_32, SteamID::from(alphakeks_64).to_string());

	let blacky_64 = 76561198091592005;
	let blacky_32 = "STEAM_1:1:65663138";
	assert_eq!(blacky_32, SteamID::from(blacky_64).to_string());

	let charlie_64 = 76561198054062420;
	let charlie_32 = "STEAM_1:0:46898346";
	assert_eq!(charlie_32, SteamID::from(charlie_64).to_string());

	let idot_64 = 76561198955057247;
	let idot_32 = "STEAM_1:1:497395759";
	assert_eq!(idot_32, SteamID::from(idot_64).to_string());

	let ibra_64 = 76561198264939817;
	let ibra_32 = "STEAM_1:1:152337044";
	assert_eq!(ibra_32, SteamID::from(ibra_64).to_string());

	let szwagi_64 = 76561198857828380;
	let szwagi_32 = "STEAM_1:0:448781326";
	assert_eq!(szwagi_32, SteamID::from(szwagi_64).to_string());

	let gosh_64 = 76561198292029626;
	let gosh_32 = "STEAM_1:0:165881949";
	assert_eq!(gosh_32, SteamID::from(gosh_64).to_string());
}

#[test]
fn mode_api() {
	let kzt = Mode::KZTimer;
	let skz = Mode::SimpleKZ;
	let vnl = Mode::Vanilla;

	assert_eq!("kz_timer", kzt.api());
	assert_eq!("kz_simple", skz.api());
	assert_eq!("kz_vanilla", vnl.api());
}

#[test]
fn mode_short() {
	let kzt = Mode::KZTimer;
	let skz = Mode::SimpleKZ;
	let vnl = Mode::Vanilla;

	assert_eq!("KZT", &kzt.short());
	assert_eq!("SKZ", &skz.short());
	assert_eq!("VNL", &vnl.short());
}

#[test]
fn mode_display() {
	let kzt = Mode::KZTimer;
	let skz = Mode::SimpleKZ;
	let vnl = Mode::Vanilla;

	assert_eq!("KZTimer", &kzt.to_string());
	assert_eq!("SimpleKZ", &skz.to_string());
	assert_eq!("Vanilla", &vnl.to_string());
}

#[test]
fn mode_from_str() {
	assert!("kztimer".parse::<Mode>().is_ok());
	assert!("kz_timer".parse::<Mode>().is_ok());
	assert!("kzt".parse::<Mode>().is_ok());
	assert!("simplekz".parse::<Mode>().is_ok());
	assert!("simple_kz".parse::<Mode>().is_ok());
	assert!("kz_simple".parse::<Mode>().is_ok());
	assert!("skz".parse::<Mode>().is_ok());
	assert!("vanilla".parse::<Mode>().is_ok());
	assert!("vanillakz".parse::<Mode>().is_ok());
	assert!("vanilla_kz".parse::<Mode>().is_ok());
	assert!("vnl".parse::<Mode>().is_ok());
}

#[test]
fn mode_try_from_u8() {
	for i in 0..=u8::MAX {
		match i {
			200 => assert!(Mode::try_from(200).is_ok()),
			201 => assert!(Mode::try_from(201).is_ok()),
			202 => assert!(Mode::try_from(202).is_ok()),
			n => assert!(Mode::try_from(n).is_err()),
		}
	}
}

#[test]
fn mode_into_u8() {
	assert_eq!(200, Mode::KZTimer as u8);
	assert_eq!(201, Mode::SimpleKZ as u8);
	assert_eq!(202, Mode::Vanilla as u8);

	let kzt: u8 = Mode::KZTimer.into();
	assert_eq!(200, kzt);
	let skz: u8 = Mode::SimpleKZ.into();
	assert_eq!(201, skz);
	let vnl: u8 = Mode::Vanilla.into();
	assert_eq!(202, vnl);
}
