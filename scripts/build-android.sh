#!/bin/bash
# Build script for Android APK

set -e

echo "==================================="
echo "Building JustOrgYou for Android"
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

# Clean previous builds
echo "Cleaning previous builds..."
flutter clean

# Get dependencies
echo "Getting dependencies..."
flutter pub get

# Run code generation if needed
# echo "Running code generation..."
# flutter pub run build_runner build --delete-conflicting-outputs

# Build APK
echo "Building APK..."
flutter build apk --release

# Build App Bundle (for Google Play)
echo "Building App Bundle..."
flutter build appbundle --release

echo ""
echo "==================================="
echo "Build completed successfully!"
echo "==================================="
echo ""
echo "APK location: build/app/outputs/flutter-apk/app-release.apk"
echo "App Bundle location: build/app/outputs/bundle/release/app-release.aab"
echo ""
echo "Install APK on device:"
echo "  adb install build/app/outputs/flutter-apk/app-release.apk"
echo ""
echo "Or copy APK to your device and install manually"
echo "==================================="
