use {super::*, crate::error::Result, pretty_assertions::assert_eq};

#[test]
fn basic() -> Result<()> {
	let input = "STEAM_1:1:161178172";

	let steam_id = SteamID::new(input)?;
	assert_eq!("STEAM_1:1:161178172", steam_id.to_string());
	assert_eq!(76561198282622073_u64, steam_id.as_id64());
	assert_eq!(AccountUniverse::Public, steam_id.account_universe());
	assert_eq!(AccountType::Individual, steam_id.account_type());
	assert_eq!(161178172, steam_id.account_number());
	assert_eq!(322356345, steam_id.community_id());

	let from_32 = SteamID::try_from(322356345_u32)?;
	assert_eq!(steam_id, from_32);

	let from_64 = SteamID::try_from(76561198282622073_u64)?;
	assert_eq!(steam_id, from_64);

	Ok(())
}
