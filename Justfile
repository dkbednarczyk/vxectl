# Clean the project
clean:
    cargo clean

# Run linting and formatting
lint:
    cargo fmt
    cargo fix --allow-dirty
    cargo clippy --all-features --workspace -- -D warnings