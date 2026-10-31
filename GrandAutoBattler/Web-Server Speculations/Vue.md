# How Vue Developers Would Build GrandAutoBattler

A practical comparison of how a Vue (3) developer would approach the exact
functionality currently implemented in `GrandAutoBattler_Angular_UI`.

> Source project context (Angular 21, standalone components, signals, SCSS,
> RxJS `HttpClient`, Angular Router). This document maps each concern to the
> Vue-world equivalent.

---

## 1. HTML or "code everything"? — Single-File Components (SFCs)

Vue keeps a **separate template**, so it is *closer to Angular's 3-file model*
than React is — but it **colocates template + script + styles in ONE `.vue`
file** (a **Single-File Component**) instead of three sibling files.

So the answer is: **yes, real HTML.** Vue uses an actual **HTML-like template**
(plus optional JSX), compiled to render functions by the build tool.

```vue
<!-- UnitList.vue  (one file = template + logic + styles) -->
<template>
  <div class="unit-list-container">
    <h2>Select Warriors</h2>
    <label>Our Faction</label>
    <select :value="ourFactionIndex" @change="onOurFactionChange">
      <option v-for="(faction, i) in availableFactions" :key="i" :value="i">
        {{ faction }}
      </option>
    </select>
    <button class="btn-advance-stage" :disabled="!isValidSelection" @click="onConfirm">
      Confirm Selection
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
// ...logic
</script>

<style scoped>
/* component styles */
</style>
```

### Vue template syntax vs the Angular templates currently in the project

| Angular (current, `unit-list.html`)          | Vue equivalent                     |
| --------------------------------------------- | ---------------------------------- |
| `@for (x of list; track $index)`             | `v-for="(item, i) in list" :key="i"` |
| `{{ faction }}`                              | `{{ faction }}` *(identical)*       |
| `[value]="$index"`                           | `:value="i"`                        |
| `[disabled]="!isValidSelection"`             | `:disabled="!isValidSelection"`     |
| `(click)="onConfirmSelection()"`             | `@click="onConfirm"`                |
| `(change)="onOurFactionChange($event)"`      | `@change="onOurFactionChange"`      |
| `@if (condition) { ... }`                    | `v-if="condition"`                  |
| `@else`                                      | `v-else`                            |

Vue's template syntax is the **most familiar of the three** to anyone coming
from Angular — same mustache interpolation, same event-on-element style — but
with `v-`/`:` directive prefixes.

---

## 2. What additional libraries usually come together with Vue?

A modern Vue project of this shape typically uses:

### Build / scaffolding
- **Vite** with the official `create-vue` helper (`npm create vue@latest`),
  which scaffolds TypeScript, ESLint, Vitest, and router/pinia options.
  (Per the official Vue 3 migration guide, **Vue CLI → Vite**.)

### Routing (replaces `@angular/router` + `app.routes.ts`)
- **Vue Router** (official, first-party). Direct equivalent of `app.routes.ts`:

  ```ts
  const routes = [
    { path: '/new-game', component: NewGame },
    { path: '/unit-list', component: UnitList },
    { path: '/battle-journal', component: BattleJournal },
    { path: '/results', component: GameResult },
    { path: '/enemies', component: Enemies },
    { path: '/', redirect: '/new-game' },
  ]
  ```

### State management (replaces `GameSessionService` + RxJS)
- **Pinia** — the official state manager (replaces Vuex; the current default
  for Vue 3). The current `GameSessionService` maps **directly and cleanly**
  to a Pinia store:

  ```ts
  // stores/game-session.ts
  export const useGameSession = defineStore('gameSession', () => {
    const gameInstanceId = ref<string | null>(null)
    const ourFactionIndex = ref(0)
    const ourWarriorTypes = ref([0, null, null, null])
    const hasGameStarted = computed(() => gameInstanceId.value !== null)
    function startNewGame() { /* ... */ }
    function confirmSelection() { /* ... */ }
    return { gameInstanceId, ourFactionIndex, ourWarriorTypes, startNewGame, confirmSelection }
  })
  ```
  This is arguably the **most 1:1 mapping** of the whole migration — Angular
  signals (`signal` / `computed`) correspond almost exactly to Vue's
  `ref` / `computed`.

### Data fetching
- Vue has **no single dominant HTTP lib** like React's TanStack Query. Common
  options: plain `fetch`/`axios` inside the store, or **TanStack Query**
  (`@tanstack/vue-query`) for caching. For this app, a small service + `axios`
  (or `fetch`) is the norm.

### The audio libraries stay the same
- `midi-player-js` and `soundfont-player` are plain JS packages — used
  unchanged, wired inside `onMounted` / `onUnmounted` lifecycle hooks.

### Minimal dependency set for this app
```
vue, vue-router, pinia,
midi-player-js, soundfont-player
```

(Compare: Angular needs the whole `@angular/*` package suite + `rxjs`.)

---

## 3. How Components work in Vue

A Vue component is a **`.vue` file** (or, less commonly, a JS object). The
recommended form is **`<script setup>`** with the **Composition API**.

**Current Angular** (`new-game.ts`):
```ts
@Component({ selector: 'app-new-game', templateUrl: './new-game.html', ... })
export class NewGame {
  session = inject(GameSessionService);
  onStartNewGame() { ... }
}
```

**Vue equivalent** (`NewGame.vue`):
```vue
<script setup lang="ts">
import { useGameSession } from '@/stores/game-session'
import { useRouter } from 'vue-router'
import PixelPicture from './PixelPicture.vue'

const session = useGameSession()
const router = useRouter()

function onStart() {
  session.startNewGame()
  setTimeout(() => router.push('/unit-list'), 100)
}
</script>

<template>
  <div class="new-game-container">
    <PixelPicture
      src="..." alt="game concept" width="100%" height="100%" pixel-size="4px"
    />
    <button class="btn-advance-stage" @click="onStart">START NEW GAME</button>
  </div>
</template>
```

### Core mental model
- **Composition API + `<script setup>`** is the standard (replaces the old
  Options API). `ref()`, `computed()`, `watch()`, `onMounted()`, etc. are the
  building blocks.
- **Reactivity** is the heart: `ref`/`reactive` make plain values reactive, and
  the template auto-updates — very close to Angular **signals** in spirit.
  This makes `game-session.service.ts` (signals + computed) translate almost
  verbatim.
- **Props** replace Angular `input()`; **`emit`** events replace output
  bindings — for the `PixelPicture` component:

  ```vue
  <!-- PixelPicture.vue -->
  <script setup lang="ts">
  withDefaults(defineProps<{
    src: string
    alt?: string
    width?: string
    height?: string
    pixelSize?: string
    dimmed?: boolean
  }>(), { alt: '', width: '30rem', height: '15rem', pixelSize: '2px', dimmed: true })
  </script>
  ```
- **Component registration** is simpler than Angular's `imports: [...]`:
  imported SFCs used in a template are auto-recognized (no explicit module
  list needed).
- **Lifecycle** via hooks: `onMounted` ≈ `ngOnInit`, `onUnmounted` ≈
  `ngOnDestroy` — used for the music player.
- **`<script setup>`** auto-exposes everything declared in it to the template,
  removing boilerplate.

---

## 4. TypeScript — Yes, and first-class

- **`create-vue` / Vite** scaffold with **TypeScript by default** (`.vue` files
  use `<script setup lang="ts">`).
- `vue-tsc` (the official type-checker, part of the Vue 3 migration
  recommendations) + **Volar** IDE plugin give full TS support.
- Props are typed via `defineProps<...>()`; stores are typed; and the existing
  `core/models/game.model.ts` interfaces (`FactionResponse`, `GameResult`,
  `UnitTypeSelection`) carry over with essentially no change.

So: **yes, TypeScript is the norm and a first-class citizen** in Vue 3,
comparable to React.

---

## 5. Styling — SCSS is a first-class, very common choice

- **SCSS/Sass is fully and officially supported** — Vite handles it natively,
  and the existing `_variables.scss` + `styles.scss` + per-component `.scss`
  structure translates directly.
- Vue's **`<style scoped>`** block is the **closest analog to Angular's
  component `styleUrl` view encapsulation** — each `.vue` file's styles apply
  only to that component. This is a near-perfect match for the current
  Angular setup.
- Global styles (the current `styles/styles.scss` + theme variables) go in a
  global stylesheet imported once in `main.ts`.
- **Tailwind CSS** is also very popular in the Vue world (and the
  create-vue scaffolder offers it), but **plain SCSS + scoped styles is the
  most faithful path** from the current Angular project.

**Verdict:** Vue is the framework where the **current SCSS approach fits most
naturally** — `<style scoped>` ≈ Angular encapsulated styles, global SCSS
theme ≈ `styles.scss`. A Vue dev could keep the entire existing styling
architecture almost intact.

---

## 6. Direct mapping table (Angular → Vue)

| Concern              | Angular (current)                          | Vue (idiomatic)                          |
| -------------------- | ------------------------------------------ | ---------------------------------------- |
| Scaffold             | Angular CLI (`ng`)                          | **create-vue** (Vite)                     |
| Markup               | separate `.html`                            | **template block in `.vue` SFC**          |
| Component            | `@Component` class + selector              | **`.vue` file + `<script setup>`**        |
| Data in view         | `signal` / template binding                | **`ref` / `computed` (reactive)**         |
| Shared state         | `GameSessionService` (Injectable, signals) | **Pinia store** (refs + computed)         |
| HTTP / async         | `HttpClient` + `Observable` + `subscribe`  | `fetch`/`axios` (or TanStack Query)       |
| Side effects / lifecycle | `ngOnInit` / `ngOnDestroy`            | **`onMounted` / `onUnmounted`**           |
| Derived values       | `computed`                                 | **`computed`** *(same name)*              |
| Routing              | `@angular/router`, `app.routes.ts`         | **Vue Router**                            |
| Input binding        | `input()`                                  | **`defineProps`**                         |
| Styles               | per-component `.scss` (encapsulated)       | **`<style scoped>` / SCSS** (very close)  |
| Types                | TS                                         | **TS (`lang="ts")` — first-class**        |
| Audio libs           | `midi-player-js`, `soundfont-player`       | **same packages**, in lifecycle hooks     |

---

## 7. Angular → Vue affinity (why this is the smallest migration)

Vue 3 is the **closest framework to the current Angular codebase**:

- **Signals ≈ refs/computed** — `game-session.service.ts` maps almost 1:1 to a
  Pinia store.
- **Component `styleUrl` (encapsulation) ≈ `<style scoped>`** — the whole SCSS
  strategy survives untouched.
- **Template syntax** (`{{ }}`, event handlers, `for`/`if`) is the most
  Angular-like of any framework.
- **TypeScript** is first-class in both.

The main conceptual differences:
- No `@Component` decorator / no `selector` — a component is just a file.
- No DI container (`inject`) — use **Pinia stores** or `provide/inject` for
  dependency sharing.
- No RxJS — replace `Observable`/`subscribe` with Promise/async functions in
  the store.

---

## 8. Bottom line

A Vue developer would build the **same app** with:
- **create-vue + Vite + TypeScript** scaffolding,
- **`.vue` Single-File Components** with real HTML templates,
- **Vue Router** for the 5 routes,
- **Pinia** store replacing `GameSessionService` (signals → refs/computed),
- **`fetch`/axios** for HTTP (no RxJS),
- **SCSS with `<style scoped>`** — keeping the current styling architecture,
- the **same** `midi-player-js` / `soundfont-player` packages for audio.

Vue is the **lowest-friction target** for this project: signals, SCSS
encapsulation, and template syntax all have near-identical counterparts.
