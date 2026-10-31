# How React Developers Would Build GrandAutoBattler

A practical comparison of how a React (18/19) developer would approach the exact
functionality currently implemented in `GrandAutoBattler_Angular_UI`.

> Source project context (Angular 21, standalone components, signals, SCSS,
> RxJS `HttpClient`, Angular Router). This document maps each concern to the
> React-world equivalent.

---

## 1. HTML or "code everything"? — JSX (JS-in-JS)

React **does NOT use plain HTML files**. There is no separate `app.html`.
Everything is written as **JSX** (or **TSX** when using TypeScript): a
JavaScript-expression syntax that looks like HTML but is *actually JavaScript*
that gets compiled into `React.createElement(...)` calls.

- In Angular you have a separate `unit-list.html` + `unit-list.ts`.
- In React you have **one file** `UnitList.tsx` containing markup + logic.

So the answer is: **a hybrid.** It *looks* like HTML, but it is code. You write
markup *inside* the component function, and you can embed expressions directly:

```tsx
// Angular (two files)
// template:   {{ faction }}        logic: (event) => ...
// React (one file, TSX):
{factions.map((faction, i) => (
  <option key={i} value={i} selected={i === ourFactionIndex}>
    {faction}
  </option>
))}
```

Key React idioms vs the Angular `@for` syntax used in `unit-list.html`:

| Angular (current)              | React equivalent                          |
| -------------------------------- | ----------------------------------------- |
| `@for (x of list; track $index)` | `list.map((x, i) => <li key={i}>)`        |
| `{{ faction }}`                 | `{faction}`                               |
| `[value]="$index"`             | `value={i}`                               |
| `(change)="onOurFactionChange($event)"` | `onChange={(e) => setOurFactionIndex(Number(e.target.value))}` |
| `[disabled]="!isValidSelection"` | `disabled={!isValidSelection}`            |
| `[selected]="..."`             | `selected={...}`                          |

React is fully **unopinionated about markup**: any framework-agnostic HTML
attributes work as camelCased props (`className`, `onClick`, `onChange`...).

### Modern note — "no-compiler" JSX
React now ships **automatic JSX runtime**, so you no longer need `import React`
in every file. TSX files are compiled by the bundler (Vite) — the user never
sees raw HTML; they see a single `.tsx` component file.

---

## 2. What additional libraries usually come together with React?

A React project of this size and shape would typically use this stack:

### Build / scaffolding
- **Vite** (via `npm create vite@latest` with the `react-ts` template) — the
  de-facto default; replaces the Angular CLI. TypeScript is included by default.

### Routing (replaces `@angular/router` + `app.routes.ts`)
- **React Router** (`react-router-dom`). Direct equivalent of `app.routes.ts`:

  ```tsx
  const routes = [
    { path: "/new-game", element: <NewGame /> },
    { path: "/unit-list", element: <UnitList /> },
    { path: "/battle-journal", element: <BattleJournal /> },
    { path: "/results", element: <GameResult /> },
    { path: "/enemies", element: <Enemies /> },
    { path: "*", element: <Navigate to="/new-game" replace /> },
  ];
  ```
- Alternatives: **TanStack Router** (type-safe, gaining popularity).

### Server data / state (replaces `GameService` + `GameSessionService` + RxJS)
- **TanStack Query** (formerly React Query) — the dominant choice (top-rated in
  State of React 2025). Replaces `HttpClient` + `Observable` + manual
  `.subscribe()`. Async data lives in query hooks, not in the component.
- For shared client state (the current `GameSessionService` signals), the common
  picks are: **Zustand** (lightweight, closest feel to Angular signals),
  or **React Context + useReducer**, or **Redux Toolkit**.
- For forms (the faction / warrior `<select>`s): **React Hook Form** or **TanStack Form**.

### The audio libraries stay the same
- `midi-player-js` and `soundfont-player` are plain JS packages — they work
  unchanged in React; you just wire them inside `useEffect`.

### Minimal dependency set for this app
```
react, react-dom, react-router-dom,
@tanstack/react-query, zustand,
midi-player-js, soundfont-player
```

(Compare: Angular needs `@angular/*`, `rxjs`, `tslib` just to function. React's
core is just `react` + `react-dom`.)

---

## 3. How Components work in React

A React component is **just a function that returns JSX**. No decorator, no
selector, no `templateUrl`.

**Current Angular** (`new-game.ts`):
```ts
@Component({ selector: 'app-new-game', templateUrl: './new-game.html', ... })
export class NewGame {
  session = inject(GameSessionService);
  onStartNewGame() { ... }
}
```

**React equivalent** (`NewGame.tsx`):
```tsx
export default function NewGame() {
  const navigate = useNavigate();
  const startNewGame = useGameSession();   // hook from Zustand/TanStack

  function onStart() {
    startNewGame();
    setTimeout(() => navigate('/unit-list'), 100);
  }

  return (
    <div className="new-game-container">
      <PixelPicture
        src="..." alt="game concept" width="100%" height="100%" pixelSize="4px"
      />
      <button className="btn-advance-stage" onClick={onStart}>
        START NEW GAME
      </button>
    </div>
  );
}
```

### Core mental model
- **No class-based components** (legacy). Everything is a **function component**.
- **State & side effects live in Hooks** — the heart of React:
  - `useState` — local state (replaces component-local signals).
  - `useEffect` — side effects, e.g. the audio player, cleanup on unmount
    (replaces Angular `ngOnInit` / `ngOnDestroy`).
  - `useMemo` / `useCallback` — memoization (replaces `computed`).
  - **Custom hooks** — the idiomatic way to extract shared logic. A
    `useGameSession()` hook is the exact React analog of the
    `GameSessionService` (state + `startNewGame` + `confirmSelection` +
    `isValidSelection` in one reusable module).
- **Props** replace Angular `input()` — data flows parent → child:
  `PixelPicture`'s `src/alt/width/height/pixelSize/pixelSize` inputs become
  plain function parameters in a `Props` TypeScript interface.
- **Composition over inheritance** — you nest components in JSX; there is no
  `imports: [PixelPicture]` registration. Imported components are simply used.
- **Re-renders** are the update mechanism: changing a `useState` value
  re-runs the component. This is the conceptual flip from Angular's signal
  change detection, but the developer experience is similar.

---

## 4. TypeScript — Yes, by default

React and TypeScript are essentially a **locked pair** in new projects.
- The official Vite `react-ts` template is TS out of the box.
- Practically every production React app is written in **TSX**.
- Types shine for **Props interfaces**, API responses (directly replacing
  `game.model.ts` types like `FactionResponse`, `GameResult`), and hook
  return types.

```ts
// game.model.ts stays almost identical — plain TS interfaces
export interface FactionResponse { factions: string[]; }
export interface PixelPictureProps {
  src: string; alt?: string; width?: string; height?: string; pixelSize?: string;
}
```

So: **yes, TypeScript is the norm**, and the existing `core/models/game.model.ts`
transports over with almost no change.

---

## 5. Styling — SCSS is fine, but the React world leans elsewhere

- **SCSS / Sass is fully supported** (via `sass` package + Vite) — nothing stops
  a React dev from keeping `_variables.scss`, `styles.scss`, component `.scss`.
- However, the **dominant modern choices** in React are:
  - **Tailwind CSS** — the most common; utility-first, no separate SCSS files.
  - **CSS Modules** (`*.module.css`) — co-located, auto-scoped, closest to
    Angular's `styleUrl` per-component styles.
  - **CSS-in-JS / styled-components** or **Vanilla Extract** — less common now.
- Angular's **component-level `styleUrl` (view encapsulation)** maps most
  naturally to **CSS Modules** in React (global styles in `index.css`, component
  styles in `Component.module.css`).

**Verdict:** SCSS remains a perfectly valid React choice, but a React team is
more likely to reach for **Tailwind** or **CSS Modules** than a global SCSS
theme.

---

## 6. Direct mapping table (Angular → React)

| Concern              | Angular (current)                          | React (idiomatic)                          |
| -------------------- | ------------------------------------------ | ------------------------------------------ |
| Scaffold             | Angular CLI (`ng`)                          | **Vite** (`npm create vite@latest`)         |
| Markup               | separate `.html`                            | **JSX/TSX** in the component file           |
| Component            | `@Component` class + selector              | **function component** returning TSX        |
| Data in view         | `signal` / template binding                | **hooks** (`useState`), props               |
| Shared state         | `GameSessionService` (Injectable)          | **Zustand store** or **Context+useReducer** |
| HTTP / async         | `HttpClient` + `Observable` + `subscribe`  | **TanStack Query** hooks                    |
| Side effects / lifecycle | `ngOnInit` / `ngOnDestroy`            | **`useEffect`**                             |
| Derived values       | `computed`                                 | `useMemo`                                   |
| Routing              | `@angular/router`, `app.routes.ts`         | **React Router** `createBrowserRouter`      |
| Input binding        | `input()`                                  | **Props** (TS interface)                    |
| Styles               | per-component `.scss` (encapsulated)       | **CSS Modules** / **Tailwind** / SCSS       |
| Types                | TS                                         | **TS (TSX) — default**                     |
| Audio libs           | `midi-player-js`, `soundfont-player`       | **same packages**, wired in `useEffect`     |

---

## 7. Bottom line

A React developer would build the **same app** with:
- **Vite + TypeScript** scaffolding,
- **TSX** components (no separate HTML),
- **React Router** for the 5 routes,
- **TanStack Query + Zustand** to replace `GameService` / `GameSessionService`
  (no RxJS needed),
- **Tailwind or CSS Modules** for styling (SCSS still acceptable),
- the **same** `midi-player-js` / `soundfont-player` packages for audio.

The biggest conceptual shift vs Angular: **no decorators, no selectors, no
`imports` arrays, no DI container.** Logic is extracted into **custom hooks**
and state is **owned by the component or a store**, with JSX as the single
source of truth for the view.
