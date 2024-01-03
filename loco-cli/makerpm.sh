cargo add cargo-generate-rpm
cat <<-END >> ./Cargo.toml
[package.metadata.generate-rpm]
assets = [
    { source = "target/release/loco", dest = "/usr/bin/loco", mode = "755" },
]
END
cargo test && cargo build --release
strip -s target/release/loco
cargo generate-rpm
