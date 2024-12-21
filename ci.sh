cargo --version

cargo install cargo-nextest
cargo install --locked cargo-outdated --features vendored-openssl

cargo outdated --exit-code 1 && \
cargo fmt --check && \
cargo check  --profile=ci --all-features --tests --examples && \
cargo clippy --profile=ci --all-features --tests --examples -- -Dwarnings && \
cargo nextest run --cargo-profile=ci --workspace --all-features --hide-progress-bar --failure-output final && \
cargo test --profile=ci --doc --all-features && \
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --profile=ci --all-features --examples

# TODO no_std tests
# TODO cargo audit
