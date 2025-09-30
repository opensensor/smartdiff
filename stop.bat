@echo off
REM Smart Code Diff - Stop All Services (Windows)

setlocal

set "GREEN=[92m"
set "YELLOW=[93m"
set "NC=[0m"

echo %YELLOW%Stopping Smart Code Diff services...%NC%
echo.

REM Kill backend processes
echo %YELLOW%Stopping backend...%NC%
taskkill /FI "WindowTitle eq Smart Diff Backend*" /F >nul 2>nul
if %errorlevel% equ 0 (
    echo %GREEN%Backend stopped%NC%
) else (
    echo %GREEN%Backend not running%NC%
)

REM Kill frontend processes
echo %YELLOW%Stopping frontend...%NC%
taskkill /FI "WindowTitle eq Smart Diff Frontend*" /F >nul 2>nul
if %errorlevel% equ 0 (
    echo %GREEN%Frontend stopped%NC%
) else (
    echo %GREEN%Frontend not running%NC%
)

REM Kill any cargo processes running smart-diff-server
taskkill /IM smart-diff-server.exe /F >nul 2>nul

REM Kill any node processes on port 3000
for /f "tokens=5" %%a in ('netstat -ano ^| findstr ":3000" ^| findstr "LISTENING"') do (
    taskkill /PID %%a /F >nul 2>nul
)

REM Kill any processes on port 8080
for /f "tokens=5" %%a in ('netstat -ano ^| findstr ":8080" ^| findstr "LISTENING"') do (
    taskkill /PID %%a /F >nul 2>nul
)

echo.
echo %GREEN%All services stopped!%NC%
echo.
pause

