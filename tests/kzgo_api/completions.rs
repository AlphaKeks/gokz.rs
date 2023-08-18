use {
	color_eyre::Result,
	gokz_rs::{kzgo_api, Mode},
};

#[tokio::test]
async fn get_completions() -> Result<()> {
	kzgo_api::get_completions(Mode::SimpleKZ, &crate::GOKZ_CLIENT).await?;
	Ok(())
}
