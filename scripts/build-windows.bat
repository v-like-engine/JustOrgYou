@echo off
REM Build script for Windows .exe

echo ===================================
echo Building JustOrgYou for Windows
echo ===================================
echo.

cd /d "%~dp0\.."

REM Check if Flutter is installed
flutter --version >nul 2>&1
if errorlevel 1 (
    echo Error: Flutter is not installed or not in PATH
    echo Please install Flutter from https://flutter.dev
    exit /b 1
)

REM Navigate to frontend directory
cd frontend

REM Check Windows desktop support
echo Ensuring Windows desktop support...
flutter config --enable-windows-desktop

REM Clean previous builds
echo Cleaning previous builds...
flutter clean

REM Get dependencies
echo Getting dependencies...
flutter pub get

REM Build Windows executable
echo Building Windows executable...
flutter build windows --release

echo.
echo ===================================
echo Build completed successfully!
echo ===================================
echo.
echo Executable location: build\windows\runner\Release\justorgyou.exe
echo.
echo To create installer, use Inno Setup or similar tool
echo ===================================
pause
