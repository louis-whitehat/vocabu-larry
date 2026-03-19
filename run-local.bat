@echo off
setlocal

set "ROOT=%~dp0"
set "CLIENT_URL=http://127.0.0.1:5173"

where cargo >nul 2>nul
if errorlevel 1 (
    echo cargo was not found in PATH.
    exit /b 1
)

where pnpm >nul 2>nul
if errorlevel 1 (
    echo pnpm was not found in PATH.
    exit /b 1
)

start "vocabu-larry backend" cmd /k "cd /d "%ROOT%src\WebApi-Rust" && cargo run"
start "vocabu-larry frontend" cmd /k "cd /d "%ROOT%src\WebUI" && pnpm run dev -- --host 127.0.0.1 --port 5173 --strictPort"

timeout /t 3 /nobreak >nul
start "" "%CLIENT_URL%"

endlocal