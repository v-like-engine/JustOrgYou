#!/bin/bash
# Build script for Windows .exe

set -e

echo "==================================="
echo "Building JustOrgYou for Windows"
echo "==================================="

cd "$(dirname "$0")/.."

# Check if Flutter is installed
if ! command -v flutter &> /dev/null; then
    echo "Error: Flutter is not installed or not in PATH"
    echo "Please install Flutter from https://flutter.dev"
    exit 1
fi

# Navigate to frontend directory
cd frontend

# Check Windows desktop support
echo "Ensuring Windows desktop support..."
flutter config --enable-windows-desktop

# Clean previous builds
echo "Cleaning previous builds..."
flutter clean

# Get dependencies
echo "Getting dependencies..."
flutter pub get

# Build Windows executable
echo "Building Windows executable..."
flutter build windows --release

echo ""
echo "==================================="
echo "Build completed successfully!"
echo "==================================="
echo ""
echo "Executable location: build/windows/runner/Release/justorgyou.exe"
echo ""
echo "To create installer, use Inno Setup or similar tool"
echo "==================================="
