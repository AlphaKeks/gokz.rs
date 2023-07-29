use {ctor::ctor, tracing_subscriber::EnvFilter};

#[ctor]
fn test_setup() {
	tracing_subscriber::fmt()
		.pretty()
		.with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "ERROR".into()))
		.init();
}
