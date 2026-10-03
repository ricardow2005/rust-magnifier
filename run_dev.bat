@echo off
setlocal
cd /d "%~dp0"

set TARGET=x86_64-pc-windows-msvc
rustup toolchain install 1.99.0 --profile minimal
if errorlevel 1 goto :error
rustup target add %TARGET% --toolchain 1.99.0
if errorlevel 1 goto :error
cargo +1.99.0 run --target %TARGET%
if errorlevel 1 goto :error
exit /b 0

:error
echo.
echo Falha ao executar o projeto.
pause
exit /b 1
