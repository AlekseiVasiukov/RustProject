# Grand Auto Battler — Angular UI Implementation Guide

> **Objective:** Connect the existing Angular UI (`GrandAutoBattler_Angular_UI`) to the Rust API (`GrandAutoBattler_API_Runner`) and implement full game flow with 5 router-based pages.

---

## Table of Contents

1. [Prerequisites and Environment Setup](#1-prerequisites-and-environment-setup)
2. [Understanding the API](#2-understanding-the-api)
3. [Architecture Overview](#3-architecture-overview)
4. [Step 1: Configure HTTP Client](#4-step-1-configure-http-client)
5. [Step 2: Define TypeScript Models](#5-step-2-define-typescript-models)
6. [Step 3: Create the Game Service](#6-step-3-create-the-game-service)
7. [Step 4: Create the Game Session Service (State Management)](#7-step-4-create-the-game-session-service-state-management)
8. [Step 5: Update Routes](#8-step-5-update-routes)
9. [Step 6: Build the Navigation Component](#9-step-6-build-the-navigation-component)
10. [Step 7: Build the New Game Page](#10-step-7-build-the-new-game-page)
11. [Step 8: Build the Select Warriors Page](#11-step-8-build-the-select-warriors-page)
12. [Step 9: Build the Your Enemies Page](#12-step-9-build-the-your-enemies-page)
13. [Step 10: Build the Battle Journal Page](#13-step-10-build-the-battle-journal-page)
14. [Step 11: Build the Results Page](#14-step-11-build-the-results-page)
15. [Testing the Full Flow](#15-testing-the-full-flow)

---

## 1. Prerequisites and Environment Setup

### What You Need
- **Rust API Server** running on `http://localhost:3000`
  ```powershell
  cd d:\REPOS\Rust\FinalRustProject\GrandAutoBattler\GrandAutoBattler_API_Runner
  cargo run
  ```
- **Angular dev server** running on `http://localhost:4200`
  ```powershell
  cd d:\REPOS\Rust\FinalRustProject\GrandAutoBattler\GrandAutoBattler_Angular_UI
  ng serve
  ```

### Key Concept: Why `provideHttpClient(withFetch())`
Angular's `HttpClient` is how you talk to the Rust API. When the API returns JSON, Angular automatically deserializes it into TypeScript objects. Think of it like this:

- **Rust `serde::Serialize`** produces JSON string on the server
- **Angular `HttpClient`** receives JSON string and converts it to TypeScript interface
- **TypeScript interfaces** define the shape of the expected data (compile-time only, no runtime cost)

You mentioned `provideHttpClient(withFetch(), withJsonpSupport())`. Here is what each does:
- `withFetch()` — Uses the modern browser `fetch()` API instead of XMLHttpRequest. This is recommended for new projects.
- `withJsonpSupport()` — Only needed for legacy JSONP callbacks. **You do NOT need this** for your API.

---

## 2. Understanding the API

Your Rust API exposes these 4 endpoints:

| Method | Endpoint | Purpose | Request Body | Response Key Field |
|--------|----------|---------|--------------|-------------------|
| `POST` | `/new_game_id` | Create new game session | None | `game_id: string` |
| `GET` | `/games/{game_id}/factions` | Get available factions | None | `factions: string[]` |
| `GET` | `/games/{game_id}/unittypes` | Get available unit types | None | `unittypes: { [key: number]: string[] }` |
| `POST` | `/games/{game_id}/result` | Generate game result | Selection object | `result: GameResult` |

### Example Request/Response

**POST `/new_game_id`**
```json
// Response
{
  "game_id": "abc-123-uuid",
  "success": true,
  "timestamp": "2025-01-01T00:00:00Z",
  "error": null
}
```

**GET `/games/abc-123-uuid/factions`**
```json
// Response
{
  "factions": ["Humans", "Demons"],
  "success": true,
  "timestamp": "2025-01-01T00:00:00Z",
  "error": null
}
```

**GET `/games/abc-123-uuid/unittypes`**
```json
// Response
{
  "unittypes": {
    "0": ["Knight", "Archer", "Mage"],
    "1": ["Succubus", "Demon Lord", "Imp"]
  },
  "success": true,
  "timestamp": "2025-01-01T00:00:00Z",
  "error": null
}
```

**POST `/games/abc-123-uuid/result`**
```json
// Request
{
  "our_faction_index": 0,
  "opponent_faction_index": 1,
  "our_unittype_indices": [0, null, 1, null]
}

// Response
{
  "result": {
    "generated_combatants": { ... },
    "battle_journal": {
      "winner_side": 0,
      "events": [ ... ]
    }
  },
  "success": true,
  "timestamp": "2025-01-01T00:00:00Z",
  "error": null
}
```

---

## 3. Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      App Component                          │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Navigation Component                     │  │
│  │  (uses GameSessionService to enable/disable links)    │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                   Router Outlet                       │  │
│  │  Shows one of 5 pages based on route + session state  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                  GameSessionService (root)                   │
│  - gameInstanceId: signal<string | null>                    │
│  - availableFactions: signal<string[]>                      │
│  - availableUnitTypes: signal<{[key: number]: string[]}>    │
│  - gameResult: signal<GameResult | null>                    │
│  Signals are reactive: when they change, all consumers     │
│  (navigation, pages) automatically update                   │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    GameService (root)                        │
│  - Wraps HttpClient                                          │
│  - startNewGame() → POST /new_game_id                       │
│  - getFactions(id) → GET /games/{id}/factions               │
│  - getUnitTypes(id) → GET /games/{id}/unittypes             │
│  - getGameResult(id, selection) → POST /games/{id}/result   │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              Rust API (localhost:3000)                       │
└─────────────────────────────────────────────────────────────┘
```

### Signal Concept Explained
Angular **Signals** are reactive primitives. Think of them as "observable boxes" that hold a value. When you call `.set(newValue)`, every component that reads that signal automatically re-renders. This is how the navigation knows when `gameInstanceId` becomes non-null and enables new links.

---

## 4. Step 1: Configure HTTP Client

### What to Change
Open [`GrandAutoBattler_Angular_UI/src/app/app.config.ts`](GrandAutoBattler_Angular_UI/src/app/app.config.ts) and add HTTP client with fetch support.

### Code

```typescript
import { ApplicationConfig, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter } from '@angular/router';
import { provideHttpClient, withFetch } from '@angular/common/http';
import { routes } from './app.routes';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    provideHttpClient(withFetch())
  ]
};
```

### Explanation
- `provideHttpClient(withFetch())` registers `HttpClient` in Angular's dependency injection system
- `withFetch()` tells Angular to use the browser's `fetch()` API under the hood
- This enables any service to `inject(HttpClient)` and make HTTP requests

---

## 5. Step 2: Define TypeScript Models

### What to Create
Replace the content of [`GrandAutoBattler_Angular_UI/src/app/core/models/unit.model.ts`](GrandAutoBattler_Angular_UI/src/app/core/models/unit.model.ts) with complete API type definitions.

### Why Interfaces?
TypeScript interfaces describe the **shape** of JSON objects. They exist only at compile time and do NOT generate JavaScript code. When `HttpClient` receives JSON, it creates plain JavaScript objects that match your interface shapes.

### Code

```typescript
// --- API Response Wrappers ---

/** Response from POST /new_game_id */
export interface NewGameIdResponse {
  game_id: string;
  success: boolean;
  timestamp: string;
  error: string | null;
}

/** Response from GET /games/{id}/factions */
export interface FactionsResponse {
  factions: string[];
  success: boolean;
  timestamp: string;
  error: string | null;
}

/** Response from GET /games/{id}/unittypes */
export interface UnitTypesResponse {
  unittypes: { [key: number]: string[] };
  success: boolean;
  timestamp: string;
  error: string | null;
}

/** Request body for POST /games/{id}/result */
export interface GameResultRequest {
  our_faction_index: number;
  opponent_faction_index: number;
  our_unittype_indices: (number | null)[];
}

/** Response from POST /games/{id}/result */
export interface GameResultApiResponse {
  result: GameResult;
  success: boolean;
  timestamp: string;
  error: string | null;
}

// --- Game Domain Models ---

/** Individual combatant unit from generated_combatants */
export interface Combatant {
  id: string;
  unit_type_name: string;
  unit_type_pretty_name: string | null;
  first_name: string | null;
  last_name: string | null;
  parameters: {
    health: number;
    attack: number;
    range: number;
    defense: number;
  };
  health_points: number;
  current_health_points: number;
}

/** Battle journal event */
export interface BattleJournalEvent {
  round: number;
  turn: number;
  cmd: BattleCommand;
}

/** Attack command inside battle journal */
export interface BattleCommand {
  type: 'Attack';
  src: string;
  dst: string;
  dmg: number;
}

/** Battle journal container — note: winner_side is INSIDE the journal, not on GameResult */
export interface BattleJournal {
  winner_side: number; // 0 = our side won, 1 = opponent won
  events: BattleJournalEvent[];
}

/** Full game result object from the API */
export interface GameResult {
  generated_combatants: { [side: number]: Combatant[] };
  battle_journal: BattleJournal;
}

/** Unit type selection for our warriors (4 slots) */
export type UnitTypeSelection = (number | null)[];
```

---

## 6. Step 3: Create the Game Service

### What to Create
Create a new file: [`GrandAutoBattler_Angular_UI/src/app/core/services/game.service.ts`](GrandAutoBattler_Angular_UI/src/app/core/services/game.service.ts)

### Purpose
This service wraps all HTTP calls to the Rust API. It uses `HttpClient` to make requests and returns `Observable` objects that components subscribe to.

### Code

```typescript
import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import {
  NewGameIdResponse,
  FactionsResponse,
  UnitTypesResponse,
  GameResultRequest,
  GameResultApiResponse,
} from '../models/unit.model';

@Injectable({
  providedIn: 'root'
})
export class GameService {
  private http = inject(HttpClient);
  private baseUrl = environment.apiUrl;

  /** POST /new_game_id — Creates a new game instance */
  startNewGame(): Observable<NewGameIdResponse> {
    return this.http.post<NewGameIdResponse>(`${this.baseUrl}/new_game_id`, {});
  }

  /** GET /games/{gameId}/factions — Lists available factions */
  getFactions(gameId: string): Observable<FactionsResponse> {
    return this.http.get<FactionsResponse>(`${this.baseUrl}/games/${gameId}/factions`);
  }

  /** GET /games/{gameId}/unittypes — Lists available unit types per faction */
  getUnitTypes(gameId: string): Observable<UnitTypesResponse> {
    return this.http.get<UnitTypesResponse>(`${this.baseUrl}/games/${gameId}/unittypes`);
  }

  /** POST /games/{gameId}/result — Submits selections and returns full game result */
  getGameResult(gameId: string, request: GameResultRequest): Observable<GameResultApiResponse> {
    return this.http.post<GameResultApiResponse>(
      `${this.baseUrl}/games/${gameId}/result`,
      request
    );
  }
}
```

### Explanation of Key Concepts

1. **`inject(HttpClient)`** — Angular's modern way to get a dependency. Equivalent to passing it in the constructor, but cleaner.

2. **`Observable<T>`** — RxJS's way of representing an asynchronous value. Think of it as a "promise that you can subscribe to." When the HTTP response arrives, the component that subscribed receives the typed data.

3. **`environment.apiUrl`** — The base URL (`http://localhost:3000`) is centralized so you can change it for production.

### Update the Environment File

Update [`GrandAutoBattler_Angular_UI/src/environments/environment.development.ts`](GrandAutoBattler_Angular_UI/src/environments/environment.development.ts):

```typescript
export const environment = {
  production: false,
  apiUrl: 'http://localhost:3000'
};
```

Update [`GrandAutoBattler_Angular_UI/src/environments/environment.ts`](GrandAutoBattler_Angular_UI/src/environments/environment.ts):

```typescript
export const environment = {
  production: true,
  apiUrl: 'http://localhost:3000' // Change to production URL later
};
```

---

## 7. Step 4: Create the Game Session Service (State Management)

### What to Create
Create a new file: [`GrandAutoBattler_Angular_UI/src/app/core/services/game-session.service.ts`](GrandAutoBattler_Angular_UI/src/app/core/services/game-session.service.ts)

### Purpose
This is the **single source of truth** for game state. All components read from and write to this service using Signals. When a signal changes, all components using it automatically update.

### Code

```typescript
import { Injectable, inject } from '@angular/core';
import { signal, computed } from '@angular/core';
import { GameService } from './game.service';
import type {
  GameResult,
  UnitTypeSelection,
} from '../models/unit.model';

@Injectable({
  providedIn: 'root'
})
export class GameSessionService {
  private gameService = inject(GameService);

  // --- Core Game State (Signals) ---

  /** The unique game instance ID. Null until user starts a new game. */
  gameInstanceId = signal<string | null>(null);

  /** List of available faction names, e.g., ["Humans", "Demons"] */
  availableFactions = signal<string[]>([]);

  /** Map of faction index → unit type names */
  availableUnitTypes = signal<{ [key: number]: string[] }>({});

  /** The full game result. Null until user confirms warrior selection. */
  gameResult = signal<GameResult | null>(null);

  // --- User Selections ---

  /** Index of our chosen faction (default 0) */
  ourFactionIndex = signal<number>(0);

  /** Index of opponent's chosen faction (default 0) */
  opponentFactionIndex = signal<number>(0);

  /** Our 4 warrior slots. Each slot is a unit type index or null. */
  ourWarriorTypes = signal<UnitTypeSelection>([0, null, null, null]);

  // --- Computed Navigation States ---

  /** True when a game has been started (gameInstanceId is not null) */
  hasGameStarted = computed(() => this.gameInstanceId() !== null);

  /** True when game result is available (user confirmed selections) */
  hasGameResult = computed(() => this.gameResult() !== null);

  // --- Actions ---

  /**
   * Step 1: Start a new game.
   * Fetches gameInstanceId, factions, and unit types in parallel.
   */
  startNewGame() {
    // Reset state
    this.gameResult.set(null);
    this.ourFactionIndex.set(0);
    this.opponentFactionIndex.set(0);
    this.ourWarriorTypes.set([0, null, null, null]);

    // Fetch new game ID
    this.gameService.startNewGame().subscribe((response) => {
      const gameId = response.game_id;
      this.gameInstanceId.set(gameId);

      // Fetch factions and unit types in parallel
      this.gameService.getFactions(gameId).subscribe((res) => {
        this.availableFactions.set(res.factions);
      });

      this.gameService.getUnitTypes(gameId).subscribe((res) => {
        // Convert numeric keys from API to match TypeScript shape
        this.availableUnitTypes.set(res.unittypes);
      });
    });
  }

  /**
   * Step 2: Confirm warrior selection and fetch game result.
   */
  confirmSelection() {
    const gameId = this.gameInstanceId();
    if (!gameId) return;

    const request = {
      our_faction_index: this.ourFactionIndex(),
      opponent_faction_index: this.opponentFactionIndex(),
      our_unittype_indices: this.ourWarriorTypes(),
    };

    this.gameService.getGameResult(gameId, request).subscribe((response) => {
      this.gameResult.set(response.result);
    });
  }

  /**
   * Validation: At least one warrior must be of a non-None type.
   */
  isValidSelection(): boolean {
    const warriors = this.ourWarriorTypes();
    return warriors.some((type) => type !== null);
  }
}
```

### Explanation

| Concept | Purpose |
|---------|---------|
| `signal<T>` | Holds a reactive value. Call `.set(newValue)` to update, `.value` or `()` to read. |
| `computed(() => ...)` | Derives a value from other signals. Automatically recalculates when dependencies change. |
| `hasGameStarted` | Used by navigation to enable/disable the "Select Warriors" link. |
| `hasGameResult` | Used by navigation to enable/disable "Your Enemies", "Battle Journal", "Results". |

---

## 8. Step 5: Update Routes

### What to Change
Update [`GrandAutoBattler_Angular_UI/src/app/app.routes.ts`](GrandAutoBattler_Angular_UI/src/app/app.routes.ts) to include all 5 pages.

### Code

```typescript
import { Routes } from '@angular/router';
import { NewGame } from './components/new-game/new-game';
import { UnitList } from './components/unit-list/unit-list';
import { Enemies } from './components/enemies/enemies';
import { BattleJournal } from './components/battle-journal/battle-journal';
import { GameResult } from './components/game-result/game-result';

export const routes: Routes = [
  { path: 'new-game', component: NewGame },
  { path: 'select-warriors', component: UnitList },
  { path: 'enemies', component: Enemies },
  { path: 'battle-journal', component: BattleJournal },
  { path: 'results', component: GameResult },
  { path: '', redirectTo: '/new-game', pathMatch: 'full' },
];
```

### Explanation
- 5 routes match the 5 pages described in the requirements
- Default redirect goes to `/new-game`
- **Note:** We do NOT use route guards here. The navigation component handles enabling/disabling links based on session state signals. This is simpler for learning purposes.

---

## 9. Step 6: Build the Navigation Component

### What to Change
Update [`GrandAutoBattler_Angular_UI/src/app/components/navigation/navigation.ts`](GrandAutoBattler_Angular_UI/src/app/components/navigation/navigation.ts) and its template.

### TypeScript Code

```typescript
import { Component, inject } from '@angular/core';
import { RouterLink, RouterLinkActive } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';

@Component({
  selector: 'app-navigation',
  standalone: true,
  imports: [RouterLink, RouterLinkActive],
  templateUrl: './navigation.html',
  styleUrl: './navigation.scss'
})
export class NavigationComponent {
  session = inject(GameSessionService);
}
```

### HTML Template

```html
<nav class="nav-container card">
  <ul class="nav-links">
    <li>
      <a routerLink="/new-game" routerLinkActive="active" [routerLinkActiveOptions]="{exact: true}">
        New Game
      </a>
    </li>
    <li [class.disabled]="!session.hasGameStarted()">
      <a [routerLink]="session.hasGameStarted() ? '/select-warriors' : null"
         routerLinkActive="active">
        Select Warriors
      </a>
    </li>
    <li [class.disabled]="!session.hasGameResult()">
      <a [routerLink]="session.hasGameResult() ? '/enemies' : null"
         routerLinkActive="active">
        Your Enemies
      </a>
    </li>
    <li [class.disabled]="!session.hasGameResult()">
      <a [routerLink]="session.hasGameResult() ? '/battle-journal' : null"
         routerLinkActive="active">
        Battle Journal
      </a>
    </li>
    <li [class.disabled]="!session.hasGameResult()">
      <a [routerLink]="session.hasGameResult() ? '/results' : null"
         routerLinkActive="active">
        Results
      </a>
    </li>
  </ul>
</nav>
```

### Explanation
- `session.hasGameStarted()` — Returns `true` when `gameInstanceId` is non-null. Enables "Select Warriors".
- `session.hasGameResult()` — Returns `true` when `gameResult` is non-null. Enables "Your Enemies", "Battle Journal", "Results".
- Disabled links have `[routerLink]="null"` which prevents navigation and the `.disabled` CSS class for visual feedback.

### Update SCSS ([`navigation.scss`](GrandAutoBattler_Angular_UI/src/app/components/navigation/navigation.scss))

```scss
.nav-links li.disabled a {
  color: gray;
  pointer-events: none;
  cursor: not-allowed;
}
```

---

## 10. Step 7: Build the New Game Page

### What to Change
Update [`GrandAutoBattler_Angular_UI/src/app/components/new-game/new-game.ts`](GrandAutoBattler_Angular_UI/src/app/components/new-game/new-game.ts) and its template.

### TypeScript Code

```typescript
import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';

@Component({
  selector: 'app-new-game',
  standalone: true,
  imports: [],
  templateUrl: './new-game.html',
  styleUrl: './new-game.scss'
})
export class NewGame {
  session = inject(GameSessionService);
  router = inject(Router);

  onStartNewGame() {
    this.session.startNewGame();
    // Navigate after triggering the API call
    // The navigation will enable "Select Warriors" once data arrives
    setTimeout(() => {
      this.router.navigate(['/select-warriors']);
    }, 100);
  }
}
```

### HTML Template

```html
<div class="new-game-container">
  <h1 class="game-title">Grand Auto Battler</h1>
  <button class="start-button" (click)="onStartNewGame()">
    Start New Game
  </button>
</div>
```

### Explanation
1. User clicks "Start New Game"
2. `startNewGame()` is called on the session service
3. The service makes 3 API calls: `new_game_id`, `factions`, `unittypes`
4. Signals update reactively when responses arrive
5. Component navigates to "Select Warriors" page
6. Navigation component sees `hasGameStarted()` become `true` and enables the link

---

## 11. Step 8: Build the Select Warriors Page

### What to Change
Completely rewrite [`GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.ts`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.ts) and its template.

### TypeScript Code

```typescript
import { Component, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';

@Component({
  selector: 'app-unit-list',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './unit-list.html',
  styleUrl: './unit-list.scss'
})
export class UnitList {
  session = inject(GameSessionService);
  router = inject(Router);

  // Expose signals for template readability
  availableFactions = session.availableFactions;
  availableUnitTypes = session.availableUnitTypes;
  ourFactionIndex = session.ourFactionIndex;
  opponentFactionIndex = session.opponentFactionIndex;
  ourWarriorTypes = session.ourWarriorTypes;

  // Validation: at least one warrior must be non-null
  isValidSelection = session.isValidSelection;

  /** Get unit type name for a given faction index and unit type index */
  getUnitTypeName(factionIndex: number, unitTypeIndex: number | null): string {
    if (unitTypeIndex === null) return 'None';
    const types = this.availableUnitTypes()[factionIndex];
    return types?.[unitTypeIndex] ?? 'None';
  }

  /** Cycle our faction selection */
  onOurFactionChange(event: Event) {
    const index = parseInt((event.target as HTMLSelectElement).value, 10);
    this.session.ourFactionIndex.set(index);
  }

  /** Cycle opponent faction selection */
  onOpponentFactionChange(event: Event) {
    const index = parseInt((event.target as HTMLSelectElement).value, 10);
    this.session.opponentFactionIndex.set(index);
  }

  /** Cycle unit type for a specific warrior slot */
  onWarriorTypeChange(slotIndex: number, event: Event) {
    const value = (event.target as HTMLSelectElement).value;
    const newIndex = value === 'null' ? null : parseInt(value, 10);

    const current = [...this.session.ourWarriorTypes()];
    current[slotIndex] = newIndex;

    // Validation: prevent setting all to null
    const allOthersNull = current.every((v, i) => i !== slotIndex && v === null);
    if (newIndex === null && allOthersNull) {
      return; // Block this change — at least one warrior must remain
    }

    this.session.ourWarriorTypes.set(current);
  }

  onConfirmSelection() {
    if (!this.session.isValidSelection()) return;
    this.session.confirmSelection();
    this.router.navigate(['/enemies']);
  }
}
```

### HTML Template

```html
<div class="unit-list-container">
  <h2>Select Warriors</h2>

  <!-- Faction Selection -->
  <div class="faction-selection">
    <div class="faction-group">
      <label for="our-faction">Our Faction:</label>
      <select id="our-faction" (change)="onOurFactionChange($event)">
        @for (faction of availableFactions(); track $index) {
          <option [value]="$index" [selected]="$index === ourFactionIndex()">
            {{ faction }}
          </option>
        }
      </select>
    </div>

    <div class="faction-group">
      <label for="opponent-faction">Opponent's Faction:</label>
      <select id="opponent-faction" (change)="onOpponentFactionChange($event)">
        @for (faction of availableFactions(); track $index) {
          <option [value]="$index" [selected]="$index === opponentFactionIndex()">
            {{ faction }}
          </option>
        }
      </select>
    </div>
  </div>

  <!-- Warrior Selection -->
  <div class="warrior-selection">
    <h3>Our Warriors</h3>
    @for (slot of [0, 1, 2, 3]; track slot) {
      <div class="warrior-slot">
        <label>Warrior {{ slot + 1 }}:</label>
        <select (change)="onWarriorTypeChange(slot, $event)">
          <option value="null" [selected]="ourWarriorTypes()[slot] === null">
            None
          </option>
          @for (typeName of availableUnitTypes()[ourFactionIndex()] ?? []; track $index) {
            <option [value]="$index" [selected]="ourWarriorTypes()[slot] === $index">
              {{ typeName }}
            </option>
          }
        </select>
      </div>
    }
  </div>

  <!-- Confirm Button -->
  <button class="confirm-button"
          [disabled]="!isValidSelection()"
          (click)="onConfirmSelection()">
    Confirm Selection
  </button>

  @if (!isValidSelection()) {
    <p class="validation-message">Select at least one warrior type.</p>
  }
</div>
```

### Explanation
- **Faction dropdowns** let the user pick our and opponent factions from available options
- **4 warrior slots** each have a dropdown with "None" + all unit types from the selected faction
- **Validation** prevents all 4 warriors from being `None` simultaneously
- **"Confirm Selection"** calls `session.confirmSelection()` which POSTs to `/games/{id}/result`
- After result arrives, navigation goes to "Your Enemies" page

---

## 12. Step 9: Build the Your Enemies Page

### What to Create
Create a NEW component:
- [`GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.ts`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.ts)
- [`GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.html`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.html)
- [`GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.scss`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.scss)

You can use Angular CLI:
```powershell
ng generate component components/enemies
```

### TypeScript Code

```typescript
import { Component, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { GameSessionService } from '../../core/services/game-session.service';
import type { Combatant } from '../../core/models/unit.model';

@Component({
  selector: 'app-enemies',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './enemies.html',
  styleUrl: './enemies.scss'
})
export class Enemies {
  session = inject(GameSessionService);

  /** Our combatants (side 0) */
  ourCombatants = computed(() => {
    const result = this.session.gameResult();
    return result?.generated_combatants?.[0] ?? [];
  });

  /** Opponent combatants (side 1) */
  enemyCombatants = computed(() => {
    const result = this.session.gameResult();
    return result?.generated_combatants?.[1] ?? [];
  });

  /** Get full display name of a combatant */
  getFullName(combatant: Combatant): string {
    const first = combatant.first_name ?? '';
    const last = combatant.last_name ?? '';
    return `${first} ${last}`.trim() || 'Unknown';
  }

  /** Get pretty unit type name */
  getUnitType(combatant: Combatant): string {
    return combatant.unit_type_pretty_name || combatant.unit_type_name;
  }
}
```

### HTML Template

```html
<div class="enemies-container">
  <h2>Your Enemies</h2>

  <div class="combatants-grid">
    <!-- Our Side (Golden) -->
    <div class="side our-side">
      <h3 class="side-title golden">Our Forces</h3>
      @for (combatant of ourCombatants(); track combatant.id) {
        <div class="combatant-card golden">
          <div class="combatant-name">{{ getFullName(combatant) }}</div>
          <div class="combatant-type">{{ getUnitType(combatant) }}</div>
          <div class="combatant-stats">
            <span>H[{{ combatant.parameters.health }}]</span>
            <span>A[{{ combatant.parameters.attack }}]</span>
            <span>R[{{ combatant.parameters.range }}]</span>
            <span>D[{{ combatant.parameters.defense }}]</span>
          </div>
          <div class="combatant-hp">HP: {{ combatant.health_points }}</div>
        </div>
      }
    </div>

    <!-- Opponent Side (Magenta) -->
    <div class="side enemy-side">
      <h3 class="side-title magenta">Enemy Forces</h3>
      @for (combatant of enemyCombatants(); track combatant.id) {
        <div class="combatant-card magenta">
          <div class="combatant-name">{{ getFullName(combatant) }}</div>
          <div class="combatant-type">{{ getUnitType(combatant) }}</div>
          <div class="combatant-stats">
            <span>H[{{ combatant.parameters.health }}]</span>
            <span>A[{{ combatant.parameters.attack }}]</span>
            <span>R[{{ combatant.parameters.range }}]</span>
            <span>D[{{ combatant.parameters.defense }}]</span>
          </div>
          <div class="combatant-hp">HP: {{ combatant.health_points }}</div>
        </div>
      }
    </div>
  </div>
</div>
```

### SCSS

```scss
.enemies-container {
  padding: 20px;
}

.combatants-grid {
  display: flex;
  gap: 40px;
}

.side {
  flex: 1;
}

.side-title {
  &.golden {
    color: gold;
  }
  &.magenta {
    color: magenta;
  }
}

.combatant-card {
  border: 1px solid #ccc;
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 10px;

  &.golden {
    border-left: 4px solid gold;
  }
  &.magenta {
    border-left: 4px solid magenta;
  }
}

.combatant-stats {
  display: flex;
  gap: 10px;
  margin: 8px 0;
}
```

---

## 13. Step 10: Build the Battle Journal Page

### What to Create
Create a NEW component:
- [`GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.ts`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.ts)
- [`GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.html`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.html)
- [`GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.scss`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.scss)

### TypeScript Code

```typescript
import { Component, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';
import type { Combatant, BattleJournalEvent } from '../../core/models/unit.model';

@Component({
  selector: 'app-battle-journal',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './battle-journal.html',
  styleUrl: './battle-journal.scss'
})
export class BattleJournal {
  session = inject(GameSessionService);
  router = inject(Router);

  /** Current page index in the battle journal */
  currentPage = 0;

  /** All battle events */
  events = computed(() => {
    return this.session.gameResult()?.battle_journal?.events ?? [];
  });

  /** Current event being displayed */
  currentEvent = computed(() => {
    return this.events()[this.currentPage];
  });

  /** Total number of events */
  totalEvents = computed(() => this.events().length);

  /** Our combatants (side 0) */
  ourCombatants = computed(() => {
    return this.session.gameResult()?.generated_combatants?.[0] ?? [];
  });

  /** Enemy combatants (side 1) */
  enemyCombatants = computed(() => {
    return this.session.gameResult()?.generated_combatants?.[1] ?? [];
  });

  /** ID of the attacker in the current event */
  attackerId = computed(() => {
    const evt = this.currentEvent();
    return evt?.cmd?.type === 'Attack' ? evt.cmd.src : '';
  });

  /** ID of the victim in the current event */
  victimId = computed(() => {
    const evt = this.currentEvent();
    return evt?.cmd?.type === 'Attack' ? evt.cmd.dst : '';
  });

  /** Navigate to Results page */
  goToResults() {
    this.router.navigate(['/results']);
  }

  // --- Navigation Controls ---

  previousPage() {
    if (this.currentPage > 0) {
      this.currentPage--;
    }
  }

  nextPage() {
    if (this.currentPage < this.totalEvents() - 1) {
      this.currentPage++;
    }
  }

  goToStart() {
    this.currentPage = 0;
  }

  goToEnd() {
    this.currentPage = this.totalEvents() - 1;
  }

  // --- Keyboard Navigation ---

  /** Listen for keyboard events to enable arrow key navigation */
  handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'ArrowLeft') {
      this.previousPage();
    } else if (event.key === 'ArrowRight') {
      this.nextPage();
    }
  }

  /** Get display name for a combatant */
  getFullName(c: Combatant): string {
    const first = c.first_name ?? '';
    const last = c.last_name ?? '';
    return `${first} ${last}`.trim() || 'Unknown';
  }

  /** Get CSS class for a combatant based on current event */
  getCombatantHighlightClass(c: Combatant): string {
    if (c.current_health_points <= 0) return 'dead';
    if (c.id === this.attackerId()) return 'attacker';
    if (c.id === this.victimId()) return 'victim';
    return 'alive';
  }
}
```

### HTML Template

```html
<div class="journal-container" (keydown)="handleKeyDown($event)" tabindex="0">
  <h2>Battle Journal</h2>

  @if (totalEvents() > 0) {
    <!-- Event Header -->
    <div class="event-header">
      <span>Round: {{ currentEvent()?.round }}</span>
      <span>Turn: {{ currentEvent()?.turn }}</span>
      <span>Page: {{ currentPage + 1 }} / {{ totalEvents() }}</span>
    </div>

    <!-- Battle Presentation -->
    <div class="battle-presentation">
      <!-- Our Side (Golden) -->
      <div class="side our-side">
        @for (combatant of ourCombatants(); track combatant.id) {
          <div class="unit-card golden {{ getCombatantHighlightClass(combatant) }}">
            <div class="unit-name">{{ getFullName(combatant) }}</div>
            <div class="unit-stats">
              H[{{ combatant.parameters.health }}]
              A[{{ combatant.parameters.attack }}]
              R[{{ combatant.parameters.range }}]
              D[{{ combatant.parameters.defense }}]
            </div>
            <div class="unit-hp">HP: {{ combatant.current_health_points }} / {{ combatant.health_points }}</div>
          </div>
        }
      </div>

      <!-- Enemy Side (Magenta) -->
      <div class="side enemy-side">
        @for (combatant of enemyCombatants(); track combatant.id) {
          <div class="unit-card magenta {{ getCombatantHighlightClass(combatant) }}">
            <div class="unit-name">{{ getFullName(combatant) }}</div>
            <div class="unit-stats">
              H[{{ combatant.parameters.health }}]
              A[{{ combatant.parameters.attack }}]
              R[{{ combatant.parameters.range }}]
              D[{{ combatant.parameters.defense }}]
            </div>
            <div class="unit-hp">HP: {{ combatant.current_health_points }} / {{ combatant.health_points }}</div>
          </div>
        }
      </div>
    </div>

    <!-- Event Log -->
    @if (currentEvent(); as event) {
      @if (event.cmd.type === 'Attack') {
        <div class="event-log">
          <p>Unit {{ event.cmd.src }} attacks {{ event.cmd.dst }} for {{ event.cmd.dmg }} damage.</p>
        </div>
      }
    }

    <!-- Navigation Controls -->
    <div class="navigation-controls">
      <button (click)="goToStart()" [disabled]="currentPage === 0" title="Go to first event">
        &#9664;&#9664;
      </button>
      <button (click)="previousPage()" [disabled]="currentPage === 0" title="Previous event">
        &#9664;
      </button>
      <button (click)="nextPage()" [disabled]="currentPage >= totalEvents() - 1" title="Next event">
        &#9654;
      </button>
      <button (click)="goToEnd()" [disabled]="currentPage >= totalEvents() - 1" title="Go to last event">
        &#9654;&#9654;
      </button>
    </div>

    <!-- Go to Results Button -->
    <button class="results-button" (click)="goToResults()">
      To Results
    </button>
  } @else {
    <p>No battle events recorded.</p>
  }
</div>
```

### SCSS

```scss
.journal-container {
  padding: 20px;
  outline: none; // For tabindex="0" focus

  .battle-presentation {
    display: flex;
    gap: 40px;
    margin: 20px 0;
  }

  .side {
    flex: 1;
  }

  .unit-card {
    border: 1px solid #ccc;
    border-radius: 8px;
    padding: 10px;
    margin-bottom: 10px;
    transition: background-color 0.3s;

    &.golden {
      border-left: 4px solid gold;
    }

    &.magenta {
      border-left: 4px solid magenta;
    }

    /* Highlight states */
    &.attacker {
      background-color: white; // White background for attacker
    }

    &.victim {
      background-color: red; // Red background for victim
      color: white;
    }

    &.dead {
      background-color: gray;
      color: white;
    }

    &.alive {
      background-color: #f0f0f0;
    }
  }

  .navigation-controls {
    display: flex;
    gap: 10px;
    margin: 20px 0;
    justify-content: center;

    button {
      padding: 8px 16px;
      font-size: 16px;
    }
  }

  .results-button {
    display: block;
    margin: 20px auto;
    padding: 10px 20px;
    font-size: 16px;
  }
}
```

### Important Note About Health Points
The API returns `current_health_points` for each combatant at the **start** of battle (full HP). As you navigate through the journal, you need to **recalculate** the HP based on events up to the current page. For a first working version, you can display the static HP. As a future enhancement, apply cumulative damage:

```typescript
// Future enhancement: Calculate HP at current page
function getHealthAtPage(combatantId: string, pageIndex: number): number {
  const events = this.events().slice(0, pageIndex + 1);
  let totalDamage = 0;
  for (const event of events) {
    if (event.cmd.type === 'Attack' && event.cmd.dst === combatantId) {
      totalDamage += event.cmd.dmg;
    }
  }
  const baseHp = this.findCombatant(combatantId)?.health_points ?? 0;
  return Math.max(0, baseHp - totalDamage);
}
```

---

## 14. Step 11: Build the Results Page

### What to Change
Completely rewrite [`GrandAutoBattler_Angular_UI/src/app/components/game-result/game-result.ts`](GrandAutoBattler_Angular_UI/src/app/components/game-result/game-result.ts) and its template.

### TypeScript Code

```typescript
import { Component, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';

@Component({
  selector: 'app-game-result',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './game-result.html',
  styleUrl: './game-result.scss'
})
export class GameResult {
  session = inject(GameSessionService);
  router = inject(Router);

  /** Determine if we won (winner_side === 0 means our side) */
  weWon = computed(() => {
    const result = this.session.gameResult();
    return result?.battle_journal?.winner_side === 0;
  });

  /** Result message text */
  resultMessage = computed(() => {
    return this.weWon() ? 'You won!' : 'You lose!';
  });

  /** Navigate to New Game page and reset */
  onStartNewGame() {
    this.session.startNewGame();
    this.router.navigate(['/new-game']);
  }
}
```

### HTML Template

```html
<div class="result-container">
  @if (session.gameResult()) {
    <h1 class="result-message">
      {{ resultMessage() }}
    </h1>
    <button class="new-game-button" (click)="onStartNewGame()">
      Start New Game
    </button>
  } @else {
    <p>No game result available.</p>
  }
</div>
```

### SCSS

```scss
.result-container {
  text-align: center;
  padding: 60px 20px;

  .result-message {
    color: white;
    background-color: #333;
    display: inline-block;
    padding: 30px 60px;
    border-radius: 12px;
    font-size: 48px;
    margin-bottom: 30px;
  }

  .new-game-button {
    padding: 12px 24px;
    font-size: 18px;
    cursor: pointer;
  }
}
```

### Explanation
- The "Start New Game" button on the Results page calls `session.startNewGame()` which resets state and fetches a new game ID
- Then navigates to `/new-game`
- This is a **full reset** — all signals return to initial state

---

## 15. Testing the Full Flow

### End-to-End Test Checklist

1. **Open `http://localhost:4200`** in your browser
   - You should see the "New Game" page
   - Navigation shows: "New Game" (enabled), others (disabled/gray)

2. **Click "Start New Game"**
   - API call to `POST /new_game_id` fires
   - `gameInstanceId` signal updates
   - Navigation: "Select Warriors" becomes enabled
   - Auto-navigate to "Select Warriors" page

3. **On "Select Warriors" page**
   - Factions dropdowns are populated from API
   - 4 warrior dropdowns show unit types from selected faction
   - Default: Faction [0], First warrior type [0], rest are None
   - "Confirm Selection" is enabled (at least one warrior is selected)
   - Try setting all warriors to None — last one should be blocked

4. **Click "Confirm Selection"**
   - API call to `POST /games/{id}/result` fires
   - `gameResult` signal populates
   - Navigation: "Your Enemies", "Battle Journal", "Results" become enabled
   - Auto-navigate to "Your Enemies" page

5. **On "Your Enemies" page**
   - Golden cards on left (our forces)
   - Magenta cards on right (enemy forces)
   - Stats (H, A, R, D) and HP displayed for each warrior

6. **Navigate to "Battle Journal"**
   - Combatants displayed in two columns
   - Use arrow buttons or keyboard arrows to step through events
   - Attacker highlighted in white, victim in red
   - Double arrows jump to first/last event
   - "To Results" button at bottom

7. **Navigate to "Results"**
   - Big white message: "You won!" or "You lose!"
   - "Start New Game" button navigates back to "New Game" page

### Debugging Tips

| Problem | Check |
|---------|-------|
| No data loads | Open browser DevTools → Network tab → verify API calls to `localhost:3000` |
| CORS error | Ensure API has `CorsLayer::permissive()` (it does in your routes) |
| Navigation doesn't enable | Check browser console for TypeScript errors; verify signals are updating |
| Wrong unit types | Check that faction index matches between faction dropdown and unit type dropdown |

---

## Summary of Files to Create/Modify

| File | Action |
|------|--------|
| `src/app/app.config.ts` | **Modify** — Add `provideHttpClient(withFetch())` |
| `src/app/app.routes.ts` | **Modify** — 5 new routes |
| `src/app/core/models/unit.model.ts` | **Replace** — Full API type definitions |
| `src/app/core/services/game.service.ts` | **Create** — HTTP API wrapper |
| `src/app/core/services/game-session.service.ts` | **Create** — Signal-based state management |
| `src/app/components/navigation/navigation.ts` | **Modify** — Inject session service |
| `src/app/components/navigation/navigation.html` | **Modify** — 5 links with signal-based enable/disable |
| `src/app/components/new-game/new-game.ts` | **Modify** — Start new game logic |
| `src/app/components/new-game/new-game.html` | **Modify** — Title + button |
| `src/app/components/unit-list/unit-list.ts` | **Replace** — Warrior selection logic |
| `src/app/components/unit-list/unit-list.html` | **Replace** — Faction/warrior dropdowns |
| `src/app/components/enemies/enemies.ts` | **Create** — Enemies display |
| `src/app/components/enemies/enemies.html` | **Create** — Two-column combatant cards |
| `src/app/components/enemies/enemies.scss` | **Create** — Golden/magenta styling |
| `src/app/components/battle-journal/battle-journal.ts` | **Create** — Journal navigation + keyboard |
| `src/app/components/battle-journal/battle-journal.html` | **Create** — Event presentation |
| `src/app/components/battle-journal/battle-journal.scss` | **Create** — Highlight styling |
| `src/app/components/game-result/game-result.ts` | **Replace** — Win/lose display |
| `src/app/components/game-result/game-result.html` | **Replace** — Result message |
| `src/environments/environment.ts` | **Modify** — Correct API URL |
| `src/environments/environment.development.ts` | **Modify** — Correct API URL |

---

## Key Angular Concepts to Learn

1. **Signals** — Reactive values (`signal`, `computed`). When a signal changes, everything reading it updates automatically.
2. **Dependency Injection** — `inject(Service)` gets a service instance. Services with `providedIn: 'root'` are singletons.
3. **HttpClient** — Makes HTTP requests. `get<T>()` and `post<T>()` return `Observable<T>`.
4. **Router** — `routerLink` in templates for declarative navigation. `router.navigate()` in TypeScript for programmatic.
5. **Template Syntax** — `@for`, `@if` are Angular's modern control flow (replacing `*ngFor`, `*ngIf`).
6. **Standalone Components** — Each component imports its own dependencies. No NgModules needed.

Good luck with your Angular learning journey!
