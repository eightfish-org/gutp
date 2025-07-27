# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GUTP (General User's Text Persistence) is a backend service implementing a content management protocol designed to support multiple frontend applications like forums, blogs, and social platforms. It's built with Rust and WebAssembly using the Spin framework, with PostgreSQL for data storage and Redis for messaging.

## Common Development Commands

### Build Commands
```bash
# Build the WebAssembly application
cd gutp && spin build

# Check for compilation errors
cargo check

# Run clippy for linting
cargo clippy

# Format code
cargo fmt
```

### Running the Application
```bash
# Start the development environment (PostgreSQL, Redis, and services)
POSTGRES_DBNAME=gutp docker compose -f devmode-1node_shared.yml up

# The application will be available at http://127.0.0.1:3000
```

### Testing
```bash
# Run API tests using Hurl
hurl apitests.hurl

# Run Rust tests
cargo test
```

### Database Management
```bash
# If database schema changes, clean up containers:
docker compose -f devmode-1node_shared.yml stop
docker rm gutp-db_1-1
docker rm gutp-subnode_1-1
```

## Architecture Overview

### Component Structure
- **gutp/**: Main Spin/WebAssembly application
  - `lib.rs`: Entry point with Redis trigger handler
  - `user.rs`, `post.rs`, `comment.rs`, `subspace.rs`: Entity modules
  - Each module follows the EightFish framework pattern with model definitions and CRUD operations
  
- **gutp-types/**: Shared type definitions
  - Contains all domain models with `EightFishModel` derive macros
  - Defines serializable structures for API communication

### Database Schema
All tables follow a consistent pattern with:
- `id`: Primary key (VARCHAR)
- `status`: Entity state management
- `created_time`/`updated_time`: Timestamps
- `data_source`: Multi-tenancy support
- `is_public`: Privacy control (where applicable)

Key entities:
- `gutpuser`: User accounts with OAuth integration
- `gutpsubspace`: Content spaces/categories
- `gutppost`: Main content items
- `gutpcomment`: Nested comments
- `gutptag`, `gutpmoderator`, `gutpextobj`: Supporting features

### Request Flow
1. HTTP requests arrive at the Spin application (port 3000)
2. Redis triggers invoke the WebAssembly handler
3. Business logic in entity modules processes requests
4. Database operations use SQL builder pattern
5. Responses are serialized to JSON

### API Patterns
- All endpoints follow `/gutp/v1/{entity}/{action}` pattern
- POST requests use form-encoded data
- Responses return JSON arrays/objects
- Standard CRUD operations: create, update, delete, list, get

## Key Development Considerations

1. **WebAssembly Target**: Code must compile to `wasm32-wasip1`
2. **No async/await**: Spin SDK doesn't support async in the traditional sense
3. **SQL Builder**: Use the sql-builder crate for database queries
4. **Form Data**: API expects form-encoded data, not JSON bodies
5. **Multi-tenancy**: Always include `data_source` field in operations
6. **Status Codes**: Use defined status constants (0-7) for entity states
7. **Privacy**: Respect `is_public` flags for content encryption requirements

## Environment Variables
- `REDIS_HOST`: Redis connection string
- `PROTO_ID`: Protocol identifier for Redis channels
- `POSTGRES_DBNAME`: Database name (default: gutp)

## Dependencies Note
The project uses local path dependencies for the EightFish framework components. These must be available at:
- `../../eightfish/sdk`
- `../../eightfish/eightfish-derive`
- `../../eightfish/spin_worker`