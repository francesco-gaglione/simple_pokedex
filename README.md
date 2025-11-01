# Simple Pokedex

## Quick Start

### Prerequisites

- Rust 1.86+
- `just` command runner
- Docker & Docker Compose (optional)

### Local Development

```
# Setup environment
cp .env.example .env

# Run dev server
just run

# In another terminal, test the API
curl http://localhost:8080/pokemon/pikachu
curl http://localhost:8080/pokemon/translated/pikachu

# View interactive API docs
open http://localhost:8080/swagger-ui/
```

### Common Commands

```
# View all available commands
just help

# Run with debug logging
just run-debug

# Run tests
just test

# Code formatting and linting
just fmt
just lint

# All quality checks
just check

# Docker deployment
just docker-build
just docker-up
just docker-down
```

### Local Docker Deployment

```
# Build and run with Docker Compose
just docker-up

# API will be available at http://localhost:8080

# Stop containers
just docker-down
```

## API Endpoints

### 1. Get Pokemon Information

```
GET /pokemon/{pokemon_name}
```

Example request:

```
curl http://localhost:8080/pokemon/pikachu
```

Example response:

```
{
  "name": "pikachu",
  "description": "When several of these Pokémon gather, their electricity can build and cause lightning storms.",
  "habitat": "Forest",
  "isLegendary": false
}
```

### 2. Get Pokemon with Fun Translation

```
GET /pokemon/translated/{pokemon_name}
```

Translation rules:

- Yoda Translation if: Habitat = "cave" OR Is Legendary
- Shakespeare Translation otherwise
- Fallback to original description if translation fails (rate limit)

Example request:

```
curl http://localhost:8080/pokemon/translated/mewtwo
```

Example response (Legendary Pokemon gets Yoda translation):

```
{
  "name": "mewtwo",
  "description": "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.",
  "habitat": "Laboratory",
  "isLegendary": true
}
```

### 3. Health Check

```
GET /health
```

## Architecture

The project follows Clean Architecture principles to maximize maintainability and testability:

```
src/
├── domain/
│   └── entities/
│       └── pokemon.rs   # Pokemon domain entity
├── application/
│   ├── use_cases/       # Business workflows
│   ├── traits.rs        # Repository interfaces (abstractions)
│   └── app_error.rs     # Application-specific errors
├── adapters/
│   ├── repositories/    # Concrete repository implementations
│   ├── clients/         # External API clients
│   ├── http/            # Axum handlers & routing
│   └── openapi.rs       # Swagger documentation
└── infra/
    ├── config.rs        # Environment configuration
    ├── setup.rs         # Dependency injection & initialization
    └── app.rs           # Router & middleware setup
```

### Design Principles

Dependency Injection (DI): All dependencies are injected at startup in `infra/setup.rs`, not scattered throughout the codebase. This makes swapping implementations trivial.

Repository Pattern: External service calls (PokéAPI, FunTranslations) are abstracted behind trait interfaces (`PokemonRepository`, `TranslationRepository`). The application layer never knows about HTTP clients directly.

Error Handling: Custom `AppError` enum with discriminated variants maps precisely to HTTP status codes, avoiding surprises and providing type safety.

## Configuration

Environment variables:

| Variable         | Default | Description                                  |
| ---------------- | ------- | -------------------------------------------- |
| `SERVER_PORT`    | `8080`  | Server listening port                        |
| `RUST_LOG`       | `info`  | Tracing log level (debug, info, warn, error) |
| `ENABLE_SWAGGER` | `true`  | Enable Swagger UI at `/swagger-ui/`          |

Example `.env`:

```
SERVER_PORT=8080
RUST_LOG=debug
ENABLE_SWAGGER=true
```

## Key Design Decisions

### 1. Caching Strategy for Translations

Problem: FunTranslations API has rate limits (5 requests/hour per IP).

Solution: Implement in-memory cache (`moka` crate) at the `TranslationRepository` layer with 5-minute TTL and max 1000 entries.

Benefits:

- Same Pokemon doesn't hit the API twice
- Graceful fallback to original description if rate-limited
- Transparent to application logic (repositories handle caching internally)

Trade-offs:

- Small memory overhead
- Stale data if Pokemon descriptions change (rare in practice)

### 2. Error Handling Pattern

Instead of generic `String` errors, use a discriminated enum:

```
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    GenericError(String),
}
```

Benefits:

- Type safety: Compiler prevents missing error cases
- Auto HTTP mapping: Each variant maps to precise status code
- Structured logging: Error type included in tracing spans
- Predictable behavior for API consumers

### 3. Trait-Based Abstraction

External API clients are never used directly. Instead, repository traits are defined:

```
#[async_trait]
pub trait TranslationRepository: Send + Sync {
    async fn get_shakespeare(&self, description: &str) -> AppResult<String>;
    async fn get_yoda(&self, description: &str) -> AppResult<String>;
}
```

Benefits:

- Easy to mock in tests (using `mockall`)
- Easy to swap implementations
- Domain logic independent of infrastructure details
- Clear, testable contracts between layers

### 4. Why No Database?

The challenge doesn't require persistence. Adding a database would introduce unnecessary complexity, migration management burden, and external service dependency. Instead, caching is implemented at the API integration layer, which solves the real problem (rate limiting).

### 5. OpenAPI Documentation

Using `utoipa` macros, endpoints self-document:

```
#[utoipa::path(
    get,
    path = "/pokemon/{pokemon_name}",
    responses(
        (status = 200, description = "Pokemon retrieved successfully", body = PokemonResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
```

Swagger UI auto-generates at `/swagger-ui/`. No manual documentation to maintain.

## Known Limitations

1. FunTranslations Rate Limit: 5 requests/hour per IP without API key. After limit, translations fall back to original text.
2. Pokemon Names: Must match PokéAPI exactly (case-insensitive: "pikachu", "bulbasaur", etc.)
3. Description Extraction: Only English descriptions are used. Other languages are filtered.
4. No Persistent Cache: Cache lives only in memory. Restarting the server clears it.

## Possible Enhancements

- Add Redis caching layer for distributed deployments
- Metrics endpoint (`/metrics`) for cache hit/miss rates

## Development

### Building

```
# Development build (debug symbols, faster compilation)
just build

# Release build (optimized, no debug info)
just build-release

# Docker build
just docker-build
```

### Code Quality

```
# Format code
just fmt

# Check formatting (don't modify)
just fmt-check

# Lint with clippy
just lint

# Run all quality checks
just check

# Fix common warnings automatically
just fix
```

### Code Style

- Formatted with `rustfmt`
- Linted with `clippy`
