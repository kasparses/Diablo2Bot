cargo test --manifest-path src/diablo2bot/Cargo.toml
cargo test --manifest-path src/diablo2bot/Cargo.toml --release -- --nocapture

cargo fmt
cargo clippy
cargo clippy -- -W clippy::pedantic
cargo clippy -- --A warnings -D clippy::uninlined_format_args

# Linux
cargo build --release
./target/release/diablo2bot Shalan

# Linux installation
At some point I had missing libraries which prevented compilation. This was fixed by running these commands:
sudo apt install libxcb-shm0-dev
sudo apt install libxcb-randr0-dev
sudo apt install libxdo-dev

# Windows
cargo build --release
.\target/release/diablo2bot.exe Shalan
