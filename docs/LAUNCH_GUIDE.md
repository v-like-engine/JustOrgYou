# JustOrgYou Launch Guide

Complete guide to building and launching JustOrgYou on different platforms.

## Table of Contents

1. [Android APK Installation](#android-apk-installation)
2. [Windows .exe Installation](#windows-exe-installation)
3. [Desktop (Linux/macOS)](#desktop-linuxmacos)
4. [Backend Server Setup](#backend-server-setup)
5. [Security Considerations](#security-considerations)
6. [Troubleshooting](#troubleshooting)

---

## Android APK Installation

### Prerequisites

- Flutter SDK 3.16.0 or higher
- Android SDK (API 21+)
- Java JDK 11 or higher

### Step 1: Build the APK

#### Option A: Using Build Script (Recommended)

**Linux/macOS:**
```bash
cd JustOrgYou
./scripts/build-android.sh
```

**Windows:**
```bash
cd JustOrgYou
scripts\build-android.bat
```

#### Option B: Manual Build

```bash
cd JustOrgYou/frontend

# Clean previous builds
flutter clean

# Get dependencies
flutter pub get

# Build release APK
flutter build apk --release

# Or build app bundle for Play Store
flutter build appbundle --release
```

### Step 2: Install on Android Device

#### Method 1: Via USB (ADB)

```bash
# Enable USB debugging on your Android device
# Connect device via USB

# Install APK
adb install frontend/build/app/outputs/flutter-apk/app-release.apk
```

#### Method 2: Direct Install

1. Copy `app-release.apk` to your Android device
2. Navigate to the file using a file manager
3. Tap to install
4. Enable "Install from Unknown Sources" if prompted
5. Complete installation

### Step 3: Launch the App

1. Find "JustOrgYou" icon on your device
2. Tap to launch
3. Grant necessary permissions when prompted
4. Start capturing tasks!

---

## Windows .exe Installation

### Prerequisites

- Flutter SDK 3.16.0 or higher
- Visual Studio 2019 or higher (with C++ desktop development)

### Step 1: Build the .exe

#### Option A: Using Build Script (Recommended)

**PowerShell/Command Prompt:**
```cmd
cd JustOrgYou
scripts\build-windows.bat
```

**Git Bash:**
```bash
cd JustOrgYou
./scripts/build-windows.sh
```

#### Option B: Manual Build

```bash
cd JustOrgYou/frontend

# Enable Windows desktop support
flutter config --enable-windows-desktop

# Clean previous builds
flutter clean

# Get dependencies
flutter pub get

# Build Windows executable
flutter build windows --release
```

### Step 2: Locate Executable

The executable will be at:
```
frontend/build/windows/runner/Release/justorgyou.exe
```

### Step 3: Create Portable Package

1. Create a folder named "JustOrgYou"
2. Copy the entire `Release` folder contents to it
3. Create a shortcut to `justorgyou.exe`
4. (Optional) Create installer using Inno Setup

### Step 4: Launch

1. Double-click `justorgyou.exe`
2. Windows Defender might show a warning (first launch)
3. Click "More info" → "Run anyway"
4. Application will launch

---

## Desktop (Linux/macOS)

### Linux

```bash
cd JustOrgYou/frontend

# Enable Linux desktop support
flutter config --enable-linux-desktop

# Build
flutter build linux --release

# Run
./build/linux/x64/release/bundle/justorgyou
```

### macOS

```bash
cd JustOrgYou/frontend

# Enable macOS desktop support
flutter config --enable-macos-desktop

# Build
flutter build macos --release

# Run
open build/macos/Build/Products/Release/justorgyou.app
```

---

## Backend Server Setup

The backend provides AI features (semantic search, categorization learning).

### Quick Start with Docker (Recommended)

```bash
cd JustOrgYou

# Start backend and database
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f backend
```

Backend will be available at: http://localhost:8000

### Manual Setup

```bash
cd JustOrgYou/backend

# Create virtual environment
python -m venv venv

# Activate virtual environment
# Linux/macOS:
source venv/bin/activate
# Windows:
venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Create .env file
cp ../.env.example .env
# Edit .env with your settings

# Run server
uvicorn app.main:app --host 0.0.0.0 --port 8000
```

### Configure App to Use Backend

In the app settings:
1. Go to Settings
2. Enter API URL: `http://YOUR_IP:8000`
3. Save settings
4. Restart app

**Note:** Use your computer's IP address (not localhost) for mobile devices.

---

## Security Considerations

### Android Security Features

✅ **ProGuard Enabled**: Code obfuscation to prevent reverse engineering
✅ **No Cleartext Traffic**: HTTPS-only connections (except localhost for dev)
✅ **No Backup**: Sensitive data excluded from Android backups
✅ **Secure Storage**: Hive database with optional encryption
✅ **Permission Minimization**: Only essential permissions requested

### Windows Security Features

✅ **Code Signing**: (Optional) Sign executable for trusted installation
✅ **Antivirus Scanning**: Clean from malware
✅ **Secure Updates**: Manual update process for control
✅ **Local Storage**: Data stays on device by default

### Network Security

✅ **HTTPS Only**: All API communications encrypted
✅ **Certificate Pinning**: (Optional) For production servers
✅ **Token-based Auth**: JWT tokens for API access
✅ **No Sensitive Data**: Minimal data sent to server

### Data Privacy

✅ **Local-First**: All tasks stored locally
✅ **Optional Cloud**: Server features are optional
✅ **No Tracking**: No analytics or tracking code
✅ **Open Source**: Code available for audit

---

## First Launch Setup

### 1. Launch the Application

- **Android**: Tap the JustOrgYou icon
- **Windows**: Double-click justorgyou.exe
- **Linux/macOS**: Run the executable

### 2. Grant Permissions (Android Only)

The app will request:
- **Storage**: To save and load .org files
- **Internet**: For optional AI features

Both are safe and necessary for functionality.

### 3. Optional: Configure Backend

If you want AI features:

1. Set up backend server (see above)
2. Open app Settings
3. Enter backend URL
4. Test connection
5. Enter user ID (any string, used for learning)

### 4. Start Using

1. Tap "Quick Add" button
2. Enter your first task
3. Task goes to INBOX
4. Process inbox regularly
5. Let AI learn from your patterns!

---

## Using AI Features

### Predictive Categorization

After you've categorized ~10-15 tasks manually, the AI starts learning:

1. Go to INBOX
2. Tap "Smart Sort" button
3. Review auto-categorized tasks (80%+ confidence)
4. Confirm/edit suggested categorizations (<80% confidence)
5. AI learns from your corrections!

**The more you use it, the better it gets!**

### Semantic Search

Search across ALL your tasks by meaning:

1. Tap search icon
2. Enter natural language query:
   - "buy bread" finds "Visit supermarket for milk and loaf"
   - "website work" finds "Fix homepage bug"
   - "call john" finds "Phone meeting with John Smith"
3. Results sorted by relevance

---

## Troubleshooting

### Android: App Won't Install

**Problem**: "App not installed" error

**Solutions**:
1. Enable "Unknown Sources" in settings
2. Check storage space (need ~100MB)
3. Uninstall old version first
4. Check Android version (need 5.0+)

### Android: Permissions Denied

**Problem**: Can't access storage

**Solutions**:
1. Settings → Apps → JustOrgYou → Permissions
2. Enable Storage and Internet
3. Restart app

### Windows: "Windows Protected Your PC"

**Problem**: SmartScreen blocks executable

**Solution**:
1. Click "More info"
2. Click "Run anyway"
3. (For production: code-sign the executable)

### Windows: Missing DLL Files

**Problem**: "VCRUNTIME140.dll not found"

**Solution**:
1. Install Visual C++ Redistributable
2. Download from Microsoft website
3. Restart computer

### Backend: Can't Connect

**Problem**: "Connection refused" error

**Solutions**:
1. Check backend is running: `curl http://localhost:8000/api/v1/health`
2. Check firewall settings
3. Use correct IP address (not localhost on mobile)
4. Check port 8000 is open

### AI Features: Not Working

**Problem**: No predictions or poor results

**Solutions**:
1. Need 10-15 training samples first
2. Check backend connection
3. View learning stats in settings
4. Reset model and retrain if needed

### Data: Tasks Not Saving

**Problem**: Tasks disappear after restart

**Solutions**:
1. Check storage permissions (Android)
2. Check available storage space
3. Export data as backup
4. Check logs for errors

---

## Development Mode

For developers wanting to run in development:

### Frontend Development

```bash
cd frontend

# Run on connected device
flutter run

# Run on emulator
flutter emulators --launch <emulator_id>
flutter run

# Hot reload enabled automatically
```

### Backend Development

```bash
cd backend

# Activate virtual environment
source venv/bin/activate  # or venv\Scripts\activate

# Run with auto-reload
uvicorn app.main:app --reload --host 0.0.0.0 --port 8000
```

### Full Stack Development

```bash
# Terminal 1: Backend
cd backend && uvicorn app.main:app --reload

# Terminal 2: Frontend
cd frontend && flutter run

# Terminal 3: MongoDB (if not using Docker)
mongod --dbpath ./data
```

---

## Performance Tips

### Android

- Keep app updated
- Clear cache periodically (Settings → Storage)
- Limit inbox to < 100 items (process regularly)
- Export old tasks to archive files

### Windows

- Run as administrator if file access issues
- Exclude app folder from antivirus scans
- Use SSD for better performance
- Keep Windows updated

### Backend

- Use Docker for consistent performance
- Monitor MongoDB size
- Clear old ML models periodically
- Use Redis for caching (future)

---

## Updating the App

### Android

1. Download new APK
2. Install over existing app
3. Data will be preserved

### Windows

1. Close existing app
2. Replace justorgyou.exe with new version
3. Launch new version

### Backend

```bash
# Stop old version
docker-compose down

# Pull new code
git pull

# Rebuild and start
docker-compose up -d --build
```

---

## Backup and Restore

### Export Tasks

1. Settings → Export to Org Mode
2. Choose save location
3. File saved as `justorgyou-backup-YYYY-MM-DD.org`

### Import Tasks

1. Settings → Import from Org Mode
2. Select `.org` file
3. Tasks will be imported

### Full Backup

**Android:**
```bash
# Backup app data
adb backup -f backup.ab com.justorgyou.app

# Restore
adb restore backup.ab
```

**Windows:**
Copy entire `%APPDATA%\JustOrgYou` folder

---

## Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Documentation**: `/docs` folder
- **Email**: support@justorgyou.com (if available)

---

## Security Checklist

Before deploying to production:

- [ ] Sign Android APK with release keystore
- [ ] Sign Windows .exe with code signing certificate
- [ ] Configure HTTPS with valid SSL certificate
- [ ] Change default SECRET_KEY in backend
- [ ] Enable certificate pinning
- [ ] Configure proper CORS origins
- [ ] Set DEBUG=False in production
- [ ] Regular security updates
- [ ] Backup encryption keys securely
- [ ] Monitor for security vulnerabilities

---

**Congratulations!** You're ready to use JustOrgYou! 🎉

For detailed usage guide, see [GETTING_STARTED.md](GETTING_STARTED.md)
