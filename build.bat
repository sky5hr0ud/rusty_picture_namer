REM Builds rusty_picture_namer.exe
cargo build
cargo build --release
cargo doc --no-deps
del /s /q .\docs
rmdir /s /q .\docs
mkdir .\docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=rusty_picture_namer\">" > target\doc\index.html
robocopy .\target\doc .\docs /e