@echo off
setlocal
cd /d "%~dp0"
set TARGET=x86_64-pc-windows-msvc

rustup toolchain install 1.99.0 --profile minimal
rustup target add %TARGET% --toolchain 1.99.0
cargo +1.99.0 build --release --target %TARGET%

echo.
echo Build concluido em build\%TARGET%\release\rust-magnifier.exe
pause
