# JustOrgYou

A modern, GTD-based task management system with AI-powered features.

## Overview

JustOrgYou is a comprehensive task management application designed to solve the pain points of traditional GTD (Getting Things Done) systems. Built with performance, privacy, and extensibility in mind.

## Key Features

- **Future-Proof**: Plain text storage using Org Mode format
- **AI-Powered**: Semantic search, auto-categorization, tag suggestions, and duplicate detection
- **Cross-Platform**: Flutter apps for mobile and desktop, Rust CLI
- **Offline-First**: Full functionality without internet connection
- **Smart Merging**: Intelligent conflict resolution for multi-device sync
- **Fast & Responsive**: Rust core library for optimal performance
- **Privacy-Focused**: Local-first with optional cloud sync

## Architecture

JustOrgYou consists of four main components:

### Local Components

1. **Todo Library (Rust)**
   - Core business logic
   - Org Mode parsing and serialization
   - Smart task merging
   - File: `todo-lib/`

2. **Frontend Clients (Flutter)**
   - Mobile and desktop applications
   - Clean, distraction-free UI
   - Offline data persistence with Hive
   - File: `frontend/`

### Server Components

3. **API Server (Python/FastAPI)**
   - RESTful API for AI features
   - Authentication and authorization
   - MongoDB for user data
   - File: `backend/`

4. **AI/ML Services (Python)**
   - Semantic search using sentence transformers
   - Auto-categorization
   - Tag suggestions
   - Duplicate detection
   - File: `backend/app/services/ml.py`

## Tech Stack

- **Rust**: Core todo library (performance & type safety)
- **Flutter**: Cross-platform UI (mobile, desktop, web)
- **Python/FastAPI**: Backend API server
- **MongoDB**: User data and credentials
- **Hive**: Local NoSQL storage for Flutter
- **Docker**: Containerized deployment
- **GitHub Actions**: CI/CD pipeline

## Getting Started

### Prerequisites

- Rust 1.70+ (for todo library)
- Python 3.11+ (for backend)
- Flutter 3.16+ (for frontend)
- Docker & Docker Compose (for deployment)
- MongoDB (optional, for backend features)

### Quick Start with Docker

```bash
# Clone the repository
git clone https://github.com/yourusername/JustOrgYou.git
cd JustOrgYou

# Start all services
docker-compose up -d

# Access the API
curl http://localhost:8000/api/v1/health
```

### Development Setup

#### Rust Library

```bash
cd todo-lib
cargo build
cargo test
```

#### Python Backend

```bash
cd backend
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
uvicorn app.main:app --reload
```

#### Flutter Frontend

```bash
cd frontend
flutter pub get
flutter run
```

## Project Structure

```
JustOrgYou/
├── todo-lib/           # Rust core library
│   ├── src/
│   │   ├── task.rs     # Task data model
│   │   ├── notebook.rs # Notebook management
│   │   ├── parser.rs   # Org Mode parser
│   │   ├── merge.rs    # Smart merging logic
│   │   └── types.rs    # Supporting types
│   └── Cargo.toml
├── backend/            # Python FastAPI server
│   ├── app/
│   │   ├── api/        # API routes
│   │   ├── services/   # Business logic & ML
│   │   ├── main.py     # App entry point
│   │   └── config.py   # Configuration
│   ├── requirements.txt
│   └── Dockerfile
├── frontend/           # Flutter application
│   ├── lib/
│   │   ├── models/     # Data models
│   │   ├── screens/    # UI screens
│   │   ├── widgets/    # Reusable widgets
│   │   ├── blocs/      # State management
│   │   ├── repositories/ # Data layer
│   │   └── services/   # Services
│   └── pubspec.yaml
├── .github/
│   └── workflows/      # CI/CD pipelines
├── docker-compose.yml
└── README.md
```

## GTD Workflow

JustOrgYou implements the GTD methodology:

1. **Capture** - Quick add to Inbox
2. **Clarify** - Process inbox items
3. **Organize** - Categorize as TODO, WAITING, SOMEDAY, etc.
4. **Review** - Regular review of all categories
5. **Engage** - Do the tasks

### Task Categories

- **INBOX**: Quick captures, unprocessed items
- **TODO**: Actionable tasks for this week
- **WAITING**: Blocked or waiting on others
- **SOMEDAY**: Future possibilities
- **DONE**: Completed tasks

## API Documentation

Once the backend is running, visit:
- API Docs: http://localhost:8000/docs
- Health Check: http://localhost:8000/api/v1/health

### Key Endpoints

- `POST /api/v1/ai/search/semantic` - Semantic task search
- `POST /api/v1/ai/tags/suggest` - Get tag suggestions
- `POST /api/v1/ai/categorize` - Auto-categorize tasks
- `POST /api/v1/ai/duplicates/detect` - Find duplicate tasks

## Data Format

Tasks are stored in Org Mode format for future-proofing:

```org
#+TITLE: My Tasks
#+AUTHOR: John Doe

* TODO [#A] Important task :work:urgent:
SCHEDULED: <2024-01-15>
DEADLINE: <2024-01-20>
:PROPERTIES:
:ID: 123e4567-e89b-12d3-a456-426614174000
:CONTEXT: office
:END:

This is the task body with details.

** TODO Subtask 1
** DONE Subtask 2
```

## Contributing

Contributions are welcome! Please read our contributing guidelines.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## Development Workflow

### Running Tests

```bash
# Rust library
cd todo-lib && cargo test

# Python backend
cd backend && pytest

# Flutter frontend
cd frontend && flutter test
```

### Code Quality

```bash
# Rust
cargo clippy
cargo fmt

# Python
black backend/app/
isort backend/app/
flake8 backend/app/
mypy backend/app/

# Flutter
flutter analyze
flutter format .
```

## Deployment

### Docker Deployment

```bash
docker-compose up -d
```

### Environment Variables

Create a `.env` file in the backend directory:

```env
MONGODB_URL=mongodb://localhost:27017
MONGODB_DB_NAME=justorgyou
SECRET_KEY=your-secret-key-change-in-production
DEBUG=False
```

## Security

- HTTPS-only connections in production
- Password hashing with bcrypt
- JWT token authentication
- No storage of sensitive user data
- SSH key authentication for servers
- Optional server synchronization

## Roadmap

- [ ] CLI client (Rust)
- [ ] Real-time sync between devices
- [ ] Plugin system for extensibility
- [ ] Natural language task parsing
- [ ] Calendar integration
- [ ] Voice input support
- [ ] Advanced ML features
- [ ] Desktop apps (Windows, macOS, Linux)

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Inspired by GTD methodology by David Allen
- Org Mode format from Emacs
- Community feedback and contributions

## Contact

- GitHub Issues: For bug reports and feature requests
- Discussions: For questions and community support

---

**Note**: This is an open-source project built with privacy and future-proofing in mind. Your data stays on your device by default.