@echo off
setlocal EnableExtensions EnableDelayedExpansion

set "ROOT=%~dp0"
set "TESTS_DIR=%ROOT%tests"
set "WEBUI_DIR=%ROOT%src\WebUI"
set "WEBAPI_DIR=%ROOT%src\WebApi"
set "ACCEPTANCE_BACKEND_TARGET=%ROOT%tests\target\acceptance-backend"

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

pushd "%WEBUI_DIR%"
call trunk build --release
if errorlevel 1 (
    set "TEST_EXIT=%ERRORLEVEL%"
    popd
    exit /b %TEST_EXIT%
)
popd

pushd "%WEBAPI_DIR%"
set "CARGO_TARGET_DIR=%ACCEPTANCE_BACKEND_TARGET%"
call cargo build --manifest-path "%WEBAPI_DIR%\Cargo.toml"
set "TEST_EXIT=%ERRORLEVEL%"
set "CARGO_TARGET_DIR="
popd
if not "%TEST_EXIT%"=="0" exit /b %TEST_EXIT%

pushd "%TESTS_DIR%"
call cargo build --manifest-path "%TESTS_DIR%\Cargo.toml"
if errorlevel 1 (
    set "TEST_EXIT=%ERRORLEVEL%"
    popd
    exit /b %TEST_EXIT%
)

set "ACCEPTANCE_BACKEND_BIN=%ACCEPTANCE_BACKEND_TARGET%\debug\vocabu-larry-api.exe"
call target\debug\vocabu-larry-acceptance.exe
set "TEST_EXIT=%ERRORLEVEL%"
popd

exit /b %TEST_EXIT%