@echo off
REM Smart Code Diff - Easy Start Script (Windows)
REM This script starts both the Rust backend and Next.js frontend

setlocal enabledelayedexpansion

REM Colors (using Windows 10+ ANSI support)
set "GREEN=[92m"
set "YELLOW=[93m"
set "RED=[91m"
set "BLUE=[94m"
set "NC=[0m"

REM Print banner
echo.
echo ========================================================
echo.
echo          Smart Code Diff - Easy Start
echo.
echo ========================================================
echo.

REM Check prerequisites
echo %BLUE%Checking prerequisites...%NC%

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo %RED%Error: Rust/Cargo is not installed.%NC%
    echo Please install from https://rustup.rs/
    pause
    exit /b 1
)
echo %GREEN%Rust/Cargo found%NC%

where node >nul 2>nul
if %errorlevel% neq 0 (
    echo %RED%Error: Node.js is not installed.%NC%
    echo Please install from https://nodejs.org/
    pause
    exit /b 1
)
echo %GREEN%Node.js found%NC%

where npm >nul 2>nul
if %errorlevel% neq 0 (
    echo %RED%Error: npm is not installed.%NC%
    echo Please install Node.js from https://nodejs.org/
    pause
    exit /b 1
)
echo %GREEN%npm found%NC%

REM Check if ports are available
set BACKEND_PORT=8080
set FRONTEND_PORT=3000

netstat -an | findstr ":%BACKEND_PORT%" | findstr "LISTENING" >nul 2>nul
if %errorlevel% equ 0 (
    echo %YELLOW%Warning: Port %BACKEND_PORT% is already in use%NC%
    echo Please close the application using this port and try again.
    pause
    exit /b 1
)

netstat -an | findstr ":%FRONTEND_PORT%" | findstr "LISTENING" >nul 2>nul
if %errorlevel% equ 0 (
    echo %YELLOW%Warning: Port %FRONTEND_PORT% is already in use%NC%
    echo Please close the application using this port and try again.
    pause
    exit /b 1
)

REM Install frontend dependencies if needed
if not exist "nextjs-frontend\node_modules" (
    echo %BLUE%Installing frontend dependencies (this may take a few minutes)...%NC%
    cd nextjs-frontend
    call npm install
    if %errorlevel% neq 0 (
        echo %RED%Error: Failed to install frontend dependencies%NC%
        cd ..
        pause
        exit /b 1
    )
    cd ..
    echo %GREEN%Frontend dependencies installed%NC%
) else (
    echo %GREEN%Frontend dependencies already installed%NC%
)

REM Build backend if needed
if not exist "target\release\smart-diff-server.exe" (
    echo %BLUE%Building Rust backend (this may take a few minutes)...%NC%
    cargo build --release --bin smart-diff-server
    if %errorlevel% neq 0 (
        echo %RED%Error: Failed to build backend%NC%
        pause
        exit /b 1
    )
    echo %GREEN%Backend built successfully%NC%
) else (
    echo %GREEN%Backend already built%NC%
)

REM Create log directory
if not exist "logs" mkdir logs

REM Start backend
echo %BLUE%Starting Rust backend on port %BACKEND_PORT%...%NC%
start "Smart Diff Backend" /MIN cmd /c "set RUST_LOG=info && cargo run --release --bin smart-diff-server > logs\backend.log 2>&1"

REM Wait for backend to start
echo %BLUE%Waiting for backend to be ready...%NC%
set /a count=0
:wait_backend
timeout /t 1 /nobreak >nul
curl -s http://localhost:%BACKEND_PORT%/api/health >nul 2>nul
if %errorlevel% equ 0 (
    echo %GREEN%Backend is ready!%NC%
    goto backend_ready
)
set /a count+=1
if %count% lss 30 goto wait_backend
echo %RED%Error: Backend failed to start. Check logs\backend.log for details.%NC%
type logs\backend.log
pause
exit /b 1

:backend_ready

REM Start frontend
echo %BLUE%Starting Next.js frontend on port %FRONTEND_PORT%...%NC%
cd nextjs-frontend
start "Smart Diff Frontend" /MIN cmd /c "npm run dev > ..\logs\frontend.log 2>&1"
cd ..

REM Wait for frontend to start
echo %BLUE%Waiting for frontend to be ready...%NC%
set /a count=0
:wait_frontend
timeout /t 1 /nobreak >nul
curl -s http://localhost:%FRONTEND_PORT% >nul 2>nul
if %errorlevel% equ 0 (
    echo %GREEN%Frontend is ready!%NC%
    goto frontend_ready
)
set /a count+=1
if %count% lss 30 goto wait_frontend
echo %RED%Error: Frontend failed to start. Check logs\frontend.log for details.%NC%
type logs\frontend.log
pause
exit /b 1

:frontend_ready

REM Success!
echo.
echo ========================================================
echo.
echo              Services Started Successfully!
echo.
echo ========================================================
echo.
echo %GREEN%Backend running at:  http://localhost:%BACKEND_PORT%%NC%
echo %GREEN%Frontend running at: http://localhost:%FRONTEND_PORT%%NC%
echo.
echo %BLUE%Logs are available in the 'logs' directory:%NC%
echo   - Backend:  logs\backend.log
echo   - Frontend: logs\frontend.log
echo.
echo %YELLOW%Opening frontend in your default browser...%NC%
timeout /t 2 /nobreak >nul
start http://localhost:%FRONTEND_PORT%
echo.
echo %YELLOW%Press any key to stop all services...%NC%
pause >nul

REM Cleanup
echo.
echo %BLUE%Stopping services...%NC%
taskkill /FI "WindowTitle eq Smart Diff Backend*" /F >nul 2>nul
taskkill /FI "WindowTitle eq Smart Diff Frontend*" /F >nul 2>nul
echo %GREEN%Services stopped%NC%
echo.
pause

