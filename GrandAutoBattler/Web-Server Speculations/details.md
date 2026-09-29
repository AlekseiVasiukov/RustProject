# Combatant Details Card Component -- Implementation Instructions

## Overview

Create a reusable "combatant details" modal card that appears when the user clicks/taps any combatant in [`enemies.html`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.html) or [`battle-journal.html`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.html). The card shows full combatant details overlaid on a pixelated GIF animation background.

---

## 1. Data Organization: Grid vs Flexbox

**Use CSS Grid for the main card layout.** Here's why and how:

### Recommended Grid Structure

```
┌─────────────────────────────────┐
│        combatant-details-card   │
├─────────────────────────────────┤
│  .card-background (positioned)  │ ← Layer 0: GIF background
│  ┌─────────┐  ┌──────────────┐ │
│  │ .avatar │  │ .name        │ │ ← Layer 1: Stats row 1
│  │  (64px) │  │ .pretty-type │ │
│  ├─────────┤  ├──────────────┤ │
│  │ .health │  │ .hard-stats  │ │ ← Layer 2: Stats row 2
│  │  initial│  │ H A R D      │ │
│  └─────────┘  └──────────────┘ │
└─────────────────────────────────┘
```

**Grid Template:**
```scss
.combatant-details-card {
  display: grid;
  grid-template-columns: 128px 1fr;  // Avatar column + data column
  grid-template-rows: auto auto;     // Name row + stats row
  gap: 12px;
  padding: 16px;
}
```

**Why Grid over Flexbox:**
- Grid gives you explicit 2D control (columns AND rows) in one declaration
- Flexbox is 1D (either row OR column, not both simultaneously)
- Your data has a clear tabular structure: avatar + name/type on row 1, then health + HARD stats on row 2
- Flexbox would require nested flex containers to achieve the same result

---

## 2. Layering: Foreground vs Background

Use **CSS `position` + `z-index`** to separate layers:

### The Key Concept: Positioning Context

```scss
.combatant-details-card {
  position: relative;  // Creates a "stacking context"
  // All children with position:absolute are relative to THIS box
}

// LAYER 0 — Background (under everything)
.card-background {
  position: absolute;
  inset: 0;          // Shorthand for top:0; right:0; bottom:0; left:0
  z-index: 0;        // Lowest layer
  // This is where your pixelatedImage component goes
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    image-rendering: pixelated;
  }
}

// LAYER 1 — Foreground content (on top)
.card-content {
  position: relative;  // Stays in normal document flow
  z-index: 1;          // Above the background
  // All your text, stats, etc. goes here
}
```

**How to differentiate elements:**
- **`position: absolute` + `z-index: 0`** = Background layer (the GIF)
- **`position: relative` + `z-index: 1`** = Foreground layer (the text)
- Elements with higher `z-index` stack ON TOP of lower values
- The `position: relative` on the parent ensures absolute children are confined to the card bounds

---

## 3. Modal Without Controls (Click-to-Close)

### Implementation Pattern

```html
<!-- In a shared component, e.g., combatant-details/modal.html -->
<div class="modal-overlay" (click)="close()">
  <div class="modal-card" (click)="$event.stopPropagation()">
    <!-- Card content here -->
  </div>
</div>
```

**How it works:**
1. **`.modal-overlay`** covers the entire viewport (`position: fixed; inset: 0`)
2. Clicking the overlay triggers `close()` — this handles "click outside to dismiss"
3. **`(click)="$event.stopPropagation()"`** on the card prevents clicks ON the card from bubbling up to the overlay — this prevents accidental dismissal when the user taps the card itself
4. No close button, no minimize/maximize — pure click-based interaction

### Overlay Styling
```scss
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);  // Semi-transparent dark backdrop
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;  // Above everything else on the page
}
```

---

## 4. Component Lifecycle: Reuse vs Redraw

**Recommendation: Destroy and recreate each time.**

### Why Not Persist?

For a novice, persisting the component and swapping data introduces:
- State management complexity (tracking which combatant is selected, animating transitions)
- Memory overhead for a component that's rarely visible
- Potential bugs from stale data if the combatant's health changed between battles

### Angular Approach: Simple Conditional Rendering

```html
<!-- In your parent component template -->
@if (selectedCombatant()) {
  <combatant-details-modal 
    [combatant]="selectedCombatant()" 
    (closed)="selectedCombatant(null)" />
}
```

```ts
// In your component class
selectedCombatant = signal<Combatant | null>(null);

onCombatantClick(combatant: Combatant): void {
  this.selectedCombatant.set(combatant);  // Modal appears
}
```

The `@if` block in Angular handles creation/destruction automatically. When `selectedCombatant()` is `null`, the modal is removed from the DOM entirely. When you set a new combatant, a fresh modal is drawn with that combatant's data. This is the **simplest, most predictable pattern** for your skill level.

---

## 5. Adaptive UI for Mobile (Width < 800px)

### Media Query Breakpoint

```scss
.combatant-details-card {
  // Desktop layout (default, width >= 800px)
  grid-template-columns: 128px 1fr;
  max-width: 480px;
}

@media (max-width: 799px) {
  .combatant-details-card {
    // Switch to single-column vertical layout
    grid-template-columns: 1fr;
    grid-template-rows: auto auto auto;
    max-width: 90vw;  // Use most of the screen width
    max-height: 80vh;
    overflow-y: auto;  // Scroll if content overflows
  }

  .card-background {
    // On mobile, shrink background to top portion only
    position: relative;
    height: 160px;
    inset: auto;
  }

  .avatar {
    grid-column: 1 / -1;  // Span full width
    margin: 0 auto;
  }
}
```

### Key Adaptive Principles

| Aspect | Desktop (>= 800px) | Mobile (< 800px) |
|--------|-------------------|------------------|
| Grid Columns | 2 columns (avatar + data) | 1 column (stacked) |
| Max Width | Fixed 480px | 90vw (viewport-relative) |
| Background | Full card behind content | Top banner only |
| Padding | 16px | 8px (tighter) |
| Font Size | Default | Slightly smaller if needed |

---

## 6. Component Structure Summary

Create at: `src/app/components/combatant-details/`

```
combatant-details/
├── combatant-details.ts      // Component class with input() for combatant data
├── combatant-details.html    // Template with grid layout
├── combatant-details.scss    // Styling with z-index layering + media queries
└── combatant-details.spec.ts // Tests
```

### Input Interface
```ts
export class CombatantDetailsModal {
  combatant = input.required<Combatant>();
  closed = output<void>();
}
```

### Data Fields to Display (from [`game.model.ts`](GrandAutoBattler_Angular_UI/src/app/core/models/game.model.ts:45))
| Field | Source Property | Display Label |
|-------|----------------|---------------|
| Full Name | `first_name` + `last_name` | "Name" |
| Pretty Type | `unit_type_pretty_name` | "Unit Type" |
| Initial Health | `parameters.health` | "Health Pool" |
| H.A.R.D Stats | `parameters.{health, attack, range, defense}` | "H \| A \| R \| D" |

---

## 7. Quick Reference Checklist

- [ ] Create `combatant-details` component in `src/app/components/`
- [ ] Use `display: grid` for 2-column card layout
- [ ] Use `position: absolute + z-index: 0` for GIF background layer
- [ ] Use `position: relative + z-index: 1` for foreground text layer
- [ ] Wrap in `.modal-overlay` with `(click)="close()"` and `stopPropagation()` on the card
- [ ] Use `signal<Combatant | null>` in parent to control modal visibility via `@if`
- [ ] Add `@media (max-width: 799px)` block for single-column mobile layout
- [ ] Import and declare component in a shared module or use Angular's standalone components
- [ ] Add `(click)` handler to combatant cards in enemies and battle-journal components