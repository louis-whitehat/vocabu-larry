@echo off
setlocal EnableExtensions EnableDelayedExpansion

set "ROOT=%~dp0"
set "TESTS_DIR=%ROOT%tests"

where cargo >nul 2>nul
if errorlevel 1 (
    echo cargo was not found in PATH.
    exit /b 1
)

where trunk >nul 2>nul
if errorlevel 1 (
    echo trunk was not found in PATH.
    echo Install it with: cargo install trunk --locked
    exit /b 1
)

where rustup >nul 2>nul
if not errorlevel 1 (
    rustup target add wasm32-unknown-unknown >nul 2>nul
)

pushd "%TESTS_DIR%"
call cargo run
set "TEST_EXIT=%ERRORLEVEL%"
popd

exit /b %TEST_EXIT%