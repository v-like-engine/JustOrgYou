# JustOrgYou Architecture

## System Overview

JustOrgYou is designed as a modular, distributed system with clear separation of concerns.

## Component Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Client Layer                         │
├─────────────────────────────────────────────────────────┤
│  Flutter App (Mobile/Desktop)  │  CLI (Rust)  │  Web    │
└────────────┬────────────────────┴──────────────┴────────┘
             │
             │ HTTP/REST
             │
┌────────────▼────────────────────────────────────────────┐
│                  API Server Layer                       │
├─────────────────────────────────────────────────────────┤
│            FastAPI (Python)                             │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Auth      │  │  AI Services │  │  User Data   │  │
│  │ Endpoints   │  │  Endpoints   │  │  Endpoints   │  │
│  └─────────────┘  └──────────────┘  └──────────────┘  │
└────────────┬────────────────────────────────┬──────────┘
             │                                │
             │ ML Models                      │ MongoDB
             │                                │
┌────────────▼────────────────┐  ┌───────────▼──────────┐
│   AI/ML Services            │  │   MongoDB            │
├─────────────────────────────┤  ├──────────────────────┤
│ • Sentence Transformers     │  │ • User Credentials   │
│ • Semantic Search           │  │ • Settings           │
│ • Tag Suggestions           │  │ • Sync Data          │
│ • Auto Categorization       │  └──────────────────────┘
│ • Duplicate Detection       │
└─────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                  Local Storage Layer                    │
├─────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────────────┐    │
│  │  Org Mode Files  │  │  Local Database (Hive)   │    │
│  │  (Plain Text)    │  │  (Mobile/Desktop Cache)  │    │
│  └──────────────────┘  └──────────────────────────┘    │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                  Core Library Layer                     │
├─────────────────────────────────────────────────────────┤
│              Todo Library (Rust)                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐ │
│  │  Parser  │  │   Task   │  │ Notebook │  │ Merge  │ │
│  │  (Org)   │  │  Model   │  │  Model   │  │ Logic  │ │
│  └──────────┘  └──────────┘  └──────────┘  └────────┘ │
└─────────────────────────────────────────────────────────┘
```

## Data Flow

### 1. Task Creation Flow

```
User Input → Flutter UI → Task BLoC → Repository →
  → Storage Service (Hive) → Rust Library (FFI) → Org File
```

### 2. AI Feature Flow

```
User Request → Flutter UI → HTTP Request → FastAPI →
  → ML Service → Sentence Transformer → Response
```

### 3. Sync Flow (Future)

```
Local Changes → Rust Library → Conflict Detection →
  → Smart Merge → API Server → MongoDB → Other Devices
```

## Design Principles

### 1. Offline-First
- All core functionality works without internet
- Local storage is the source of truth
- Server features are optional enhancements

### 2. Future-Proof
- Plain text storage (Org Mode)
- Open formats and protocols
- No vendor lock-in

### 3. Performance
- Rust core for speed
- Lazy loading and pagination
- Efficient caching strategies

### 4. Privacy
- Local-first architecture
- Optional cloud features
- Encryption for sensitive data
- Minimal server-side storage

### 5. Modularity
- Clear separation of concerns
- Pluggable components
- Easy to extend and customize

## Technology Decisions

### Why Rust for Core Library?

1. **Performance**: Near C-level speed
2. **Memory Safety**: No runtime errors
3. **Cross-Platform**: Compile to any target
4. **Type Safety**: Rich type system prevents bugs
5. **FFI**: Easy integration with other languages

### Why Flutter for Frontend?

1. **Cross-Platform**: One codebase for all platforms
2. **Performance**: Native compilation
3. **UI**: Beautiful, customizable widgets
4. **Hot Reload**: Fast development cycle
5. **Community**: Large ecosystem

### Why FastAPI for Backend?

1. **Performance**: Async/await support
2. **Type Safety**: Pydantic validation
3. **Documentation**: Auto-generated OpenAPI docs
4. **Python**: Easy ML integration
5. **Modern**: Latest Python features

### Why Org Mode Format?

1. **Plain Text**: Future-proof, human-readable
2. **Rich Format**: Supports all our features
3. **Ecosystem**: Large existing tool support
4. **Portable**: Works everywhere
5. **Version Control**: Git-friendly

## State Management (Flutter)

We use BLoC (Business Logic Component) pattern:

```
┌──────────────────────────────────────┐
│            UI Layer                  │
│  (Screens, Widgets)                  │
└──────────────┬───────────────────────┘
               │ Events
               ▼
┌──────────────────────────────────────┐
│          BLoC Layer                  │
│  (TaskBloc, SettingsBloc)            │
│  • Receives events                   │
│  • Processes business logic          │
│  • Emits states                      │
└──────────────┬───────────────────────┘
               │ Data Operations
               ▼
┌──────────────────────────────────────┐
│       Repository Layer               │
│  (TaskRepository)                    │
│  • Data abstraction                  │
│  • Caching strategy                  │
└──────────────┬───────────────────────┘
               │ Storage/Network
               ▼
┌──────────────────────────────────────┐
│        Data Sources                  │
│  (StorageService, API Service)       │
└──────────────────────────────────────┘
```

## Security Architecture

### Authentication Flow

```
User → Login → FastAPI →
  → Verify Credentials → Generate JWT →
    → Return Token → Store Securely
```

### Authorization

- JWT tokens for API access
- Role-based access control (future)
- API key for third-party integrations (future)

### Data Protection

1. **At Rest**: Local encryption option
2. **In Transit**: HTTPS only
3. **Passwords**: Bcrypt hashing
4. **Tokens**: Short-lived with refresh

## Scalability Considerations

### Backend Scaling

1. **Horizontal**: Multiple FastAPI instances
2. **Load Balancing**: Nginx/HAProxy
3. **Database**: MongoDB sharding
4. **Caching**: Redis for sessions

### Client Scaling

1. **Pagination**: Limit data loaded
2. **Virtual Scrolling**: Efficient lists
3. **Lazy Loading**: Load on demand
4. **Background Sync**: Don't block UI

## Error Handling

### Client Side
- Graceful degradation
- User-friendly messages
- Offline queue for sync
- Retry mechanisms

### Server Side
- Structured logging
- Error monitoring (Sentry)
- Graceful failures
- Health checks

## Testing Strategy

### Unit Tests
- Rust: cargo test
- Python: pytest
- Flutter: flutter test

### Integration Tests
- API endpoint tests
- Database integration
- ML model integration

### E2E Tests
- Flutter integration tests
- User flow testing
- Cross-platform validation

## Deployment Architecture

### Production Setup

```
┌─────────────────────────────────────────┐
│         Load Balancer (Nginx)           │
└────────────┬────────────────────────────┘
             │
     ┌───────┴────────┐
     ▼                ▼
┌─────────┐      ┌─────────┐
│FastAPI 1│      │FastAPI 2│
└────┬────┘      └────┬────┘
     └────────┬────────┘
              ▼
         ┌─────────┐
         │ MongoDB │
         │ Cluster │
         └─────────┘
```

## Monitoring & Observability

- **Logs**: Structured JSON logging
- **Metrics**: Prometheus + Grafana
- **Tracing**: OpenTelemetry
- **Alerts**: PagerDuty/Slack integration

## Future Architecture Considerations

1. **Microservices**: Split into smaller services
2. **Event-Driven**: Use message queues
3. **GraphQL**: Alternative to REST
4. **WebSockets**: Real-time sync
5. **CDN**: Static asset delivery
