REM Builds rusty_picture_namer.exe
cargo doc
cargo build --debug
cargo build --release