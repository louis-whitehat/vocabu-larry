@echo off
setlocal

set "ROOT=%~dp0"
set "CLIENT_URL=http://127.0.0.1:8101"

where cargo >nul 2>nul
if errorlevel 1 (
    echo cargo was not found in PATH.
    exit /b 1
)

where trunk >nul 2>nul
if errorlevel 1 (
    echo trunk was not found in PATH.
    echo Install it with: cargo install trunk
    exit /b 1
)

where rustup >nul 2>nul
if not errorlevel 1 (
    rustup target add wasm32-unknown-unknown >nul 2>nul
)

cd /d "%ROOT%src\WebUI-Yew"
call trunk build --release
if errorlevel 1 (
    echo Frontend build failed.
    exit /b 1
)

start "vocabu-larry app" cmd /k "cd /d "%ROOT%src\WebApi" && set "NODE_ENV=production" && set "VOCABULARRY_HOME=%ROOT%" && cargo run"

timeout /t 3 /nobreak >nul
start "" "%CLIENT_URL%"

endlocal