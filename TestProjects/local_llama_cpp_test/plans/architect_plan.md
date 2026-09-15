# Local Llama.cpp Game Engine Plan

## Project Overview

Rust application combining an Axum HTTP server with a reqwest-based client for llama.cpp, using a channel-based queue system for async request processing with simulated delays and failures.

---

## Architecture Diagram

```mermaid
flowchart LR
    Client[HTTP Client] -->|POST /game/query| Axum[Axum Server]
    Axum -->|send| Channel[mpsc Channel Queue]
    Channel -->|recv| Worker[Background Worker]
    Worker -->|2s delay| Delay[Queue Delay Simulation]
    Delay -->|500ms delay| Request[Request Delay Simulation]
    Request -->|3rd request fails| Llama[llama.cpp Server]
    Llama -->|Response| Worker
    Worker -->|Result| Axum
```

---

## Dependencies

Add to [`Cargo.toml`](Cargo.toml):

- `tokio` with `full` features (async runtime)
- `axum` with `macros` features (HTTP server framework)
- `reqwest` with `json` and `rustls-tls` features (HTTP client for llama.cpp)
- `serde` with `derive` and `serde_json` (serialization)
- `tower-http` with `trace` and `cors` features (logging/middleware + cross-origin support)
- `tracing` and `tracing-subscriber` (structured logging)
- `anyhow` (error handling)
- `utoipa` with `axum_extras` (Swagger/OpenAPI generation)
- `utoipa-swagger-ui` (Swagger UI Axum integration)
- `chrono` with `serde` (timestamps in responses)

---

## Module Structure

```
src/
├── main.rs              # Entry point, wire up server + worker
├── config.rs            # Configuration (model URL, ports, delays)
├── client.rs            # reqwest client for llama.cpp
├── server.rs            # Axum server definition
├── queue.rs             # Channel-based queue worker
└── types.rs             # Shared data types
```

---

## Types Design

### Request/Response Types
- `GameQueryRequest` - wrapper for prompt data sent to llama.cpp
- `GameQueryResponse` - LLM response wrapper
- `QueueMessage` - enum for queue commands (Query, Shutdown)

### Configuration
- `Config` struct holding: llama server URL, axum bind address, simulation delays, model path for docs

---

## Simulation Behavior

The queue worker will simulate:
1. **2-second queue delay** - `tokio::time::sleep` before processing any request
2. **500ms request delay** - additional sleep before making the actual reqwest call
3. **Every 3rd request fails** - counter-based logic returning an error on requests 3, 6, 9, etc.

---

## Coding Style Guidelines

**Novice-Friendly Approach:**

1. **Extensive Comments**: Every module, function, and non-obvious line will have `//` comments explaining the "why", not just the "what"
2. **Explicit Types**: Full type annotations on function signatures and variables, avoiding implicit inference where clarity matters
3. **Named Over Clever**: Prefer readable named patterns over macros/shortcuts
4. **Step-by-Step Logic**: Break complex operations into small, named intermediate variables
5. **Error Messages**: Descriptive error messages that explain what went wrong and how to fix it
6. **No Hidden Magic**: Avoid heavy macro usage, trait wizardry, or unsafe blocks
7. **Documentation Comments**: `///` doc comments on public functions explaining parameters and return values

## Swagger/OpenAPI Integration

- **Endpoint Documentation**: All Axum endpoints annotated with `#[utoipa::path]` for auto-generated OpenAPI specs
- **Swagger UI**: Available at `/swagger-ui` for interactive API exploration
- **OpenAPI JSON**: Available at `/api-docs/openapi.json`
- **Dedicated Guide**: `SWAGGER_INTEGRATION.md` explaining:
  - How utoipa works with Axum
  - How to add new endpoints to Swagger
  - How to customize request/response schemas
  - How to version the API documentation

## Linux Startup Documentation

Based on the Windows example (`llama-server -m model.gguf -c 2048 -t 8 --port 8080`), the Linux equivalent:

```bash
llama-server -m /opt/models/qwen-0.6b.gguf -c 2048 -t 8 --port 8080
```

Documented in a `LLAMA_SERVER.md` file with notes on Qwen 0.6B specific parameters.

---

## Todo List

- [ ] Update `Cargo.toml` with required dependencies
- [ ] Create `src/types.rs` with shared data structures
- [ ] Create `src/config.rs` with configuration struct
- [ ] Create `src/client.rs` with reqwest-based llama.cpp client
- [ ] Create `src/queue.rs` with channel-based worker and simulation logic
- [ ] Create `src/server.rs` with Axum routes and state
- [ ] Update `src/main.rs` to wire everything together (clear startup sequence)
- [ ] Create `LLAMA_SERVER.md` with Linux startup documentation
- [ ] Create `SWAGGER_INTEGRATION.md` with Swagger setup guide for novice Rust devs
