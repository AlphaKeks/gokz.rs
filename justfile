# Show all available recipes
help:
	@just --list

# Format the project
fmt:
	cargo +nightly fmt --all

# Generate documentation
doc:
	cargo doc --lib --all-features

# Run tests
test:
	cargo test --lib --all-features

# Clippy linting
check:
	cargo clippy --all-features -- -D warnings
