# Implementation Templates for the Web-Server

This document contains the code structures and patterns that should be used to implement the new Web-Server and adapt the Game Engine.

## 1. Engine Adaptations (Serde)

To allow the engine to communicate over the web, we must implement `serde` traits on all data models.

### Required Traits
Every struct that is returned to the client or received from a client must have:
`#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]`

### Example: Combatant Adaptation
```rust
// GrandAutoBattler_GameEngine/src/combatant_manager.rs

use serde::{Serialize, Deserialize}; // Add this import

#[derive(Debug, Clone, Serialize, Deserialize)] // Add Serialize and Deserialize
pub struct Combatant {
    pub id: String,
    pub faction_name: String,
    pub unit_type_name: String,
    pub unit_type_pretty_name: Option<String>,
    pub parameters: BTreeMap<String, u8>,
    pub current_health_points: usize,
    pub health_points: usize,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
```

---

## 2. Web-Server Shared State

Since Axum handles requests asynchronously, the `GameInstancesManager` must be wrapped in a thread-safe container to be shared across handlers.

### Implementation Pattern
```rust
use std::sync::{Arc, RwLock};
use GrandAutoBattler_GameEngine::game_instance_manager::GameInstancesManager;

// This is what the Axum State will hold
type SharedState = Arc<RwLock<GameInstancesManager>>;

// In your main server function:
let manager = GameInstancesManager::new();
let shared_state = Arc::new(RwLock::new(manager));
```

---

## 3. Axum Handler Templates

Handlers are the functions that execute when an HTTP endpoint is hit.

### Pattern: GET Request (Read-only)
*Use `.read()` on the RwLock.*

```rust
async fn get_factions(
    axum::extract::State(state): axum::extract::State<SharedState>,
    axum::extract::Path(game_id): axum::extract::Path<String>,
) -> Result<axum::Json<Vec<String>>, axum::http::StatusCode> {
    let manager = state.read().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let factions = manager.GetAvailableFactions(&game_id);
    Ok(axum::Json(factions))
}
```

### Pattern: POST Request (Modifying State)
*Use `.write()` on the RwLock.*

```rust
#[derive(serde::Deserialize)]
struct SimulateRequest {
    selected_factions: BTreeMap<usize, usize>,
    selected_unittypes: BTreeMap<usize, Option<usize>>,
}

async fn simulate_battle(
    axum::extract::State(state): axum::extract::State<SharedState>,
    axum::extract::Path(game_id): axum::extract::Path<String>,
    axum::Json(payload): axum::Json<SimulateRequest>,
) -> Result<axum::Json<GameResult>, axum::http::StatusCode> {
    let mut manager = state.write().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Note: We need to handle the result of the engine call carefully
    let result = manager.GetGameResultByIdAndSelection(
        &game_id, 
        &payload.selected_factions, 
        &payload.selected_unittypes
    );

    Ok(axum::Json(result))
}
```

---

## 4. Proposed API Summary

| Method | Endpoint | Description | Payload (JSON) | Response (JSON) |
|---|---|---|---|---|
| `POST` | `/game/new` | Create a new game instance | `{}` | `{ "game_id": "uuid" }` |
| `GET` | `/game/{id}/factions` | Get available factions | N/A | `["Humans", "Demons"]` |
| `GET` | `/game/{id}/units` | Get unit types for factions | N/A | `{ "0": ["Knight", "Archer"], ... }` |
| `POST` | `/game/{id}/simulate` | Run the battle simulation | `{ "factions": {...}, "units": {...} }` | `GameResult` object |

---

## 5. Project Structure Recommendation

```text
GrandAutoBattler/
├── GrandAutoBattler_Console/  <-- Legacy (becomes a web client)
├── GrandAutoBattler_Console_Runner/
├── GrandAutoBattler_GameEngine/ <-- Core logic (add Serde)
└── Web-Server/                  <-- NEW CRATE
    ├── Cargo.toml               <-- dependencies: axum, tokio, serde, engine
    └── src/
        └── main.rs             <-- Axum routing & shared state
```
