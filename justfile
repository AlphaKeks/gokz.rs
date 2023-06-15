# Show all available recipes
help:
	@just --list

# Format the project
format:
	cargo +nightly fmt --all

# Generate documentation
doc:
	cargo doc --lib --all-features

# Run tests
test:
	cargo test --lib --locked --all-features --all-targets

# Clippy linting
lint:
	cargo clippy --all-features --all-targets -- -D warnings
