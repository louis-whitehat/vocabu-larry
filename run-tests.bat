@echo off
setlocal EnableExtensions EnableDelayedExpansion

set "ROOT=%~dp0"
set "TESTS_DIR=%ROOT%tests"
set "WEBDRIVER_PORT=9516"
set "WEBDRIVER_URL=http://127.0.0.1:%WEBDRIVER_PORT%"

if defined CHROMEDRIVER_BIN (
    set "CHROMEDRIVER=%CHROMEDRIVER_BIN%"
) else (
    set "CHROMEDRIVER=C:\bin\chromedriver\chromedriver.exe"
)

if exist "%CHROMEDRIVER%\NUL" (
    set "CHROMEDRIVER=%CHROMEDRIVER%\chromedriver.exe"
)

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

where powershell >nul 2>nul
if errorlevel 1 (
    echo powershell was not found in PATH.
    exit /b 1
)

if not exist "%CHROMEDRIVER%" (
    echo chromedriver was not found at "%CHROMEDRIVER%".
    echo Set CHROMEDRIVER_BIN to the full path of chromedriver.exe if needed.
    exit /b 1
)

where rustup >nul 2>nul
if not errorlevel 1 (
    rustup target add wasm32-unknown-unknown >nul 2>nul
)

for /f %%I in ('powershell -NoProfile -Command "try { (Get-NetTCPConnection -LocalPort %WEBDRIVER_PORT% -ErrorAction Stop ^| Select-Object -First 1).OwningProcess } catch { '' }"') do set "WEBDRIVER_PID=%%I"
if defined WEBDRIVER_PID (
    echo Port %WEBDRIVER_PORT% is already in use by PID %WEBDRIVER_PID%.
    echo Stop that process or change WEBDRIVER_PORT in run-tests.bat.
    exit /b 1
)

for /f %%I in ('powershell -NoProfile -Command "$process = Start-Process -FilePath '%CHROMEDRIVER%' -ArgumentList '--port=%WEBDRIVER_PORT%','--allowed-origins=*' -PassThru -WindowStyle Hidden; $process.Id"') do set "WEBDRIVER_PID=%%I"
if not defined WEBDRIVER_PID (
    echo Failed to start chromedriver.
    exit /b 1
)

set "CHROMEDRIVER_BIN=%CHROMEDRIVER%"
set "WEBDRIVER_URL=%WEBDRIVER_URL%"

pushd "%TESTS_DIR%"
call cargo run
set "TEST_EXIT=%ERRORLEVEL%"
popd

powershell -NoProfile -Command "if ('%WEBDRIVER_PID%' -match '^\d+$') { Stop-Process -Id %WEBDRIVER_PID% -ErrorAction SilentlyContinue }" >nul 2>nul

exit /b %TEST_EXIT%