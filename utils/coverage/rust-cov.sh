# cargo clean
rm -rf -v target/coverage/raw
mkdir -v -p target/coverage/raw

CARGO_INCREMENTAL=0 \
    RUSTFLAGS='-Cinstrument-coverage' \
    LLVM_PROFILE_FILE='target/coverage/raw/cargo-test-%p-%m.profraw' \
    RUSTDOCFLAGS="-C instrument-coverage -Z unstable-options --persist-doctests target/debug/doctestbins" \
    cargo +nightly test --workspace -q

rust-profdata merge -sparse target/coverage/raw/*.profraw -o target/coverage/coverage.profdata

rust-cov export \
    --format=lcov \
    --Xdemangler=rustfilt \
    --instr-profile="target/coverage/coverage.profdata" \
    --object target/debug/doctestbins/**/rust_out \
    --object target/debug/deps/test-* \
    --object target/debug/deps/autodj-* \
    --show-instantiation-summary \
    --show-region-summary \
    --ignore-filename-regex='/.cargo/registry' \
    src/** \
    tests/** \
    >target/coverage/rust.lcov

zsh ./grcov.sh

rust-cov report \
    --color --use-color=true \
    --Xdemangler=rustfilt \
    --instr-profile="target/coverage/coverage.profdata" \
    --object target/debug/doctestbins/**/rust_out \
    --object target/debug/deps/test-* \
    --object target/debug/deps/autodj-* \
    --show-branch-summary --show-region-summary --show-branch-summary \
    --ignore-filename-regex='/.cargo/registry' \
    src/** \
    tests/**
