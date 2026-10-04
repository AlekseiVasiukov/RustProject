# Combatant Details Modal Card -- Complete Implementation Guide

## Status: READY FOR IMPLEMENTATION

This document provides step-by-step, copy-paste-ready instructions to complete the combatant details modal feature. You have already prepared the foundation in the `combatant-details` component and the `enemies` component. This guide sews everything together.

---

## Table of Contents

1. [Current State Assessment](#1-current-state-assessment)
2. [Step 1: Fix enemies.html - Add Click Handlers](#2-step-1-fix-enemieshtml---add-click-handlers)
3. [Step 2: Complete combatant-details.ts](#3-step-2-complete-combatant-detailsts)
4. [Step 3: Complete combatant-details.html](#4-step-3-complete-combatant-detailshhtml)
5. [Step 4: Complete combatant-details.scss](#5-step-4-complete-combatant-detailsscss)
6. [Step 5: Integrate with Battle Journal Component](#6-step-5-integrate-with-battle-journal-component)
7. [Summary of Changes](#7-summary-of-changes)

---

## 1. Current State Assessment

### What You Already Have

| File | What Exists | What's Missing |
|------|-------------|----------------|
| [`combatant-details.ts`](GrandAutoBattler_Angular_UI/src/app/components/combatant-details/combatant-details.ts) | `input.required<Combatant \| null>()`, `output<void>()` | Need to add `close()` method that emits `closed` |
| [`combatant-details.html`](GrandAutoBattler_Angular_UI/src/app/components/combatant-details/combatant-details.html) | Empty `<div class="modal-card">` skeleton | Full card content with grid layout, GIF background, stats display |
| [`combatant-details.scss`](GrandAutoBattler_Angular_UI/src/app/components/combatant-details/combatant-details.scss) | Partial media query for mobile | Modal overlay, z-index layering, glossy card styling, animations |
| [`enemies.ts`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.ts) | `selectedCombatant` signal, `onCombatantClick()`, component imported | Nothing -- ready to use |
| [`enemies.html`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.html) | Modal component in template (partial), `(closed)` commented out | Fix `(click)` binding syntax, uncomment `(closed)` handler |
| [`battle-journal.ts`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.ts) | Combatant data available | Import component, add `selectedCombatant` signal |

---

## 2. Step 1: Fix enemies.html - Add Click Handlers

### What to Fix

On line 14 of [`enemies.html`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.html:14), there is a syntax error:

```html
<!-- BROKEN - line 14 -->
<div class="unit-picture-and-text" click="onCombatantClick(combatant">
```

**Problems:**
1. `click=` should be `(click)=` (Angular event binding requires parentheses)
2. Missing closing parenthesis and quote: `combatant` should be `combatant)`

Additionally, the `(closed)` output on the modal is commented out on lines 1-3.

### Complete Replacement for enemies.html

Replace the entire file content with:

```html
@if (selectedCombatant()) {
  <app-combatant-details
    [combatant]="selectedCombatant()"
    (closed)="selectedCombatant(null)"
  >
  </app-combatant-details>
}

<div class="enemies-container">
  <div class="game-zone">
    <div class="combatants-grid">
      <!-- Our side - GOLDEN -->
      <div class="side our-side">
        <h3 class="side-title golden">Our Forces</h3>
        @for (combatant of ourCombatants(); track combatant.id) {
          <div class="unit-picture-and-text" (click)="onCombatantClick(combatant)">
            <div class="combatant-card golden">
              <div class="combatant-name">{{ getFullName(combatant) }}</div>
              <div class="combatant-type">{{ getUnitType(combatant) }}</div>
              <div class="hard-and-hp">
                <div class="combatant-stats">
                  <span>H[{{ combatant.parameters.health }}]</span>
                  <span>A[{{ combatant.parameters.attack }}]</span>
                  <span>R[{{ combatant.parameters.range }}]</span>
                  <span>D[{{ combatant.parameters.defense }}]</span>
                </div>
                <div class="combatant-hp">HP: {{ combatant.health_points }}</div>
              </div>
            </div>
            <img
              [src]="getCombatantImage(combatant, 0)"
              [alt]="combatant.unit_type_pretty_name"
              class="unit-icon"
              loading="lazy"
            />
          </div>
        }
      </div>

      <!-- Enemy side - MAGENTA -->
      <div class="side enemy-side">
        <h3 class="side-title magenta">Enemy Forces</h3>
        @for (combatant of enemyCombatants(); track combatant.id) {
          <div class="unit-picture-and-text" (click)="onCombatantClick(combatant)">
            <img
              [src]="getCombatantImage(combatant, 1)"
              [alt]="combatant.unit_type_pretty_name"
              class="unit-icon inverted"
              loading="lazy"
            />
            <div class="combatant-card magenta">
              <div class="combatant-name">{{ getFullName(combatant) }}</div>
              <div class="combatant-type">{{ getUnitType(combatant) }}</div>
              <div class="hard-and-hp">
                <div class="combatant-stats">
                  <span>H[{{ combatant.parameters.health }}]</span>
                  <span>A[{{ combatant.parameters.attack }}]</span>
                  <span>R[{{ combatant.parameters.range }}]</span>
                  <span>D[{{ combatant.parameters.defense }}]</span>
                </div>
                <div class="combatant-hp">HP: {{ combatant.health_points }}</div>
              </div>
            </div>
          </div>
        }
      </div>
    </div>
  </div>

  <button class="btn-advance-stage" (click)="goToBattleJournal()">BATTLE!!</button>
</div>
```

**What changed:**
- Lines 1-6: Uncommented the modal, added `(closed)="selectedCombatant(null)"` binding. When the user clicks the overlay to close, this signal resets to `null` and the `@if` block destroys the component.
- Line 15: Fixed `(click)="onCombatantClick(combatant)"` for our side cards
- Line 43: Added `(click)="onCombatantClick(combatant)"` for enemy side cards (was missing before)
- Removed `alt=""` duplicate attributes on `<img>` tags

---

## 3. Step 2: Complete combatant-details.ts

### What to Change

The component class needs one addition: a `close()` method that calls `this.closed.emit()`. This method is called when the user clicks the overlay background.

### Complete Replacement for combatant-details.ts

```typescript
import { Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import type { Combatant } from '../../core/models/game.model';

@Component({
  selector: 'app-combatant-details',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './combatant-details.html',
  styleUrl: './combatant-details.scss',
})
export class CombatantDetails {
  combatant = input.required<Combatant | null>();
  closed = output<void>();

  close(): void {
    this.closed.emit();
  }

  // Helper to get full display name
  getFullName(c: Combatant): string {
    const first = c.first_name ?? '';
    const last = c.last_name ?? '';
    return `${first} ${last}`.trim() || 'Unknown';
  }

  // Helper to get pretty unit type name
  getUnitType(c: Combatant): string {
    return c.unit_type_pretty_name || c.unit_type_name;
  }
}
```

**What changed:**
- Added `CommonModule` import (needed for `@if` and other template features)
- Added `close()` method that emits the `closed` output signal
- Added `getFullName()` and `getUnitType()` helper methods so the template doesn't need verbose expressions

---

## 4. Step 3: Complete combatant-details.html

### What to Change

Replace the empty skeleton with the full modal structure containing:
1. Click-to-close overlay layer
2. Card with `stopPropagation()` to prevent accidental dismissal
3. GIF background layer (Layer 0, z-index 0)
4. Grid-based stats content (Layer 1, z-index 1)

### Complete Replacement for combatant-details.html

```html
<div class="modal-overlay" (click)="close()">
  <div class="combatant-details-card" (click)="$event.stopPropagation()">
    <!-- LAYER 0: GIF Background -->
    @if (combatant()) {
      <div class="card-background">
        <img
          [src]="'/assets/images/gif/' + combatant()!.unit_type_name + '.gif'"
          [alt]="getUnitType(combatant()!)"
          class="background-gif"
        />
      </div>
    }

    <!-- LAYER 1: Foreground Content -->
    @if (combatant(); as c) {
      <div class="card-content">
        <!-- Row 1: Avatar + Name -->
        <div class="avatar">
          <img
            [src]="'/assets/images/128px/' + c.unit_type_name + '.png'"
            [alt]="getUnitType(c)"
            class="avatar-image"
          />
        </div>
        <div class="name-section">
          <div class="name">{{ getFullName(c) }}</div>
          <div class="pretty-type">{{ getUnitType(c) }}</div>
        </div>

        <!-- Row 2: Health + HARD Stats -->
        <div class="health-section">
          <span class="health-label">Health Pool:</span>
          <span class="health-value">{{ c.parameters.health }}</span>
          <span class="health-current">(Current: {{ c.current_health_points }}/{{ c.health_points }})</span>
        </div>
        <div class="hard-stats">
          <div class="stat-box">
            <span class="stat-letter">H</span>
            <span class="stat-value">{{ c.parameters.health }}</span>
          </div>
          <div class="stat-box">
            <span class="stat-letter">A</span>
            <span class="stat-value">{{ c.parameters.attack }}</span>
          </div>
          <div class="stat-box">
            <span class="stat-letter">R</span>
            <span class="stat-value">{{ c.parameters.range }}</span>
          </div>
          <div class="stat-box">
            <span class="stat-letter">D</span>
            <span class="stat-value">{{ c.parameters.defense }}</span>
          </div>
        </div>
      </div>
    }
  </div>
</div>
```

**Explanation of structure:**
- `<div class="modal-overlay">` covers the full viewport. Clicking it triggers `close()`.
- `<div class="combatant-details-card">` is centered inside the overlay. `(click)="$event.stopPropagation()"` prevents clicks on the card from bubbling to the overlay.
- `.card-background` contains the GIF animation, positioned absolutely behind everything with `z-index: 0`.
- `.card-content` contains all text data in a CSS Grid layout, positioned relatively with `z-index: 1`.
- The `@if (combatant(); as c)` block safely unwraps the nullable input and provides `c` as a non-null reference.

---

## 5. Step 4: Complete combatant-details.scss

### What to Change

Replace the partial media query with the complete styling system including:
1. Full-screen modal overlay with flexbox centering
2. Grid-based card layout
3. `z-index` layering for background GIF vs foreground content
4. Glossy card effect (from [`cssgloss.md`](Web-Server Speculations/cssgloss.md))
5. One-time flare animation for desktop GIF background
6. Mobile-responsive breakpoints

### Complete Replacement for combatant-details.scss

```scss
/* ============================================================
   LAYER 0: Modal Overlay (full viewport backdrop)
   ============================================================ */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

/* ============================================================
   Card Container (relative positioning for z-index context)
   ============================================================ */
.combatant-details-card {
  position: relative;
  display: grid;
  grid-template-columns: 128px 1fr;
  grid-template-rows: auto auto;
  gap: 12px;
  padding: 16px;
  max-width: 480px;
  min-width: 320px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 10px;
  background: linear-gradient(to bottom, #2a2a2a 0%, #1a1a1a 100%);
  box-shadow:
    inset 2px 2px 0 rgba(255, 255, 255, 0.35),
    inset -3px -3px 0 rgba(0, 0, 0, 0.45),
    inset 0 0 6px rgba(0, 0, 0, 0.25),
    0 4px 20px rgba(0, 0, 0, 0.6);
  overflow: hidden;
}

/* ============================================================
   LAYER 0: GIF Background (behind all content)
   ============================================================ */
.card-background {
  position: absolute;
  inset: 0;
  z-index: 0;

  .background-gif {
    width: 100%;
    height: 100%;
    object-fit: cover;
    image-rendering: pixelated;

    /* Desktop: One-time flare animation */
    @media (min-width: 801px) {
      animation: flareIn 1.5s ease-out forwards;
    }

    /* Mobile: Infinite pan with fade edges */
    @media (max-width: 800px) {
      animation: none;
    }
  }
}

/* Desktop flare animation - starts bright/oversaturated, settles to normal */
@keyframes flareIn {
  0% {
    filter: brightness(3.0) saturate(3.0) contrast(0.2) blur(4px);
    opacity: 0;
  }
  30% {
    filter: brightness(1.8) saturate(1.8) contrast(0.6) blur(2px);
    opacity: 0.7;
  }
  70% {
    filter: brightness(1.2) saturate(1.2) contrast(0.9) blur(0.5px);
    opacity: 0.9;
  }
  100% {
    filter: brightness(1.0) saturate(1.0) contrast(1.0) blur(0px);
    opacity: 1;
  }
}

/* ============================================================
   LAYER 1: Foreground Content (on top of background)
   ============================================================ */
.card-content {
  position: relative;
  z-index: 1;
  color: #e0e0e0;
  width: 100%;

  /* Dark semi-transparent backdrop behind text for readability */
  background: rgba(0, 0, 0, 0.45);
  border-radius: 6px;
  padding: 8px;
}

/* ============================================================
   Grid Cell: Avatar (Row 1, Column 1)
   ============================================================ */
.avatar {
  grid-row: 1;
  grid-column: 1;
  display: flex;
  align-items: center;
  justify-content: center;

  .avatar-image {
    width: 128px;
    height: 128px;
    image-rendering: pixelated;
    border: 2px solid rgba(255, 255, 255, 0.15);
    border-radius: 6px;
  }
}

/* ============================================================
   Grid Cell: Name Section (Row 1, Column 2)
   ============================================================ */
.name-section {
  grid-row: 1;
  grid-column: 2;
  display: flex;
  flex-direction: column;
  justify-content: center;

  .name {
    font-size: 1.4rem;
    font-weight: bold;
    color: #ffffff;
    text-shadow: 0 0 8px rgba(255, 255, 255, 0.2);
    margin-bottom: 4px;
  }

  .pretty-type {
    font-size: 0.9rem;
    color: #b0b0b0;
    font-style: italic;
  }
}

/* ============================================================
   Grid Cell: Health Section (Row 2, Column 1)
   ============================================================ */
.health-section {
  grid-row: 2;
  grid-column: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;

  .health-label {
    font-size: 0.8rem;
    color: #999;
  }

  .health-value {
    font-size: 1.1rem;
    font-weight: bold;
    color: #ff6b6b;
  }

  .health-current {
    font-size: 0.75rem;
    color: #777;
  }
}

/* ============================================================
   Grid Cell: HARD Stats (Row 2, Column 2)
   ============================================================ */
.hard-stats {
  grid-row: 2;
  grid-column: 2;
  display: flex;
  justify-content: space-around;
  align-items: center;
  gap: 8px;

  .stat-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    padding: 4px 8px;
    min-width: 40px;

    .stat-letter {
      font-size: 0.75rem;
      font-weight: bold;
      color: #ffcc00;
    }

    .stat-value {
      font-size: 1.1rem;
      font-weight: bold;
      color: #ffffff;
    }
  }
}

/* ============================================================
   Glossy Highlight Effect (::before pseudo-element)
   ============================================================ */
.combatant-details-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 40%;
  background: linear-gradient(
    to bottom,
    rgba(255, 255, 255, 0.15) 0%,
    rgba(255, 255, 255, 0.0) 100%
  );
  border-radius: 10px 10px 0 0;
  z-index: 2;
  pointer-events: none;
}

/* ============================================================
   MOBILE RESPONSIVE (width < 800px)
   ============================================================ */
@media (max-width: 800px) {
  .combatant-details-card {
    grid-template-columns: 1fr;
    max-width: 90vw;
    max-height: 80vh;
    overflow-y: auto;
    padding: 8px;
    gap: 8px;
  }

  /* On mobile, the background becomes a top banner instead of full card */
  .card-background {
    position: relative;
    height: 160px;
    inset: auto;

    .background-gif {
      width: 100%;
      height: 160px;
      object-fit: cover;
    }
  }

  /* Avatar spans full width and centers */
  .avatar {
    grid-column: 1 / -1;

    .avatar-image {
      width: 96px;
      height: 96px;
    }
  }

  /* Name section centers on mobile */
  .name-section {
    grid-column: 1 / -1;
    align-items: center;
    text-align: center;
  }

  /* Health and stats stack vertically */
  .health-section {
    grid-column: 1 / -1;
  }

  .hard-stats {
    grid-column: 1 / -1;
  }
}
```

**Explanation of each section:**

| CSS Block | Purpose | Key Technique |
|-----------|---------|---------------|
| `.modal-overlay` | Full-screen dark backdrop | `position: fixed; inset: 0;` + flexbox centering |
| `.combatant-details-card` | Main card container | `position: relative` creates stacking context for children |
| `.card-background` | GIF layer behind content | `position: absolute; z-index: 0` places it under everything |
| `.card-content` | Text/data layer on top | `position: relative; z-index: 1` keeps it above background |
| Grid cells (`.avatar`, `.name-section`, etc.) | 2D layout control | `grid-row` / `grid-column` explicit placement |
| `.hard-stats .stat-box` | Individual stat boxes | Flexbox inside the grid cell for internal layout |
| `::before` on card | Glossy top highlight | Semi-transparent gradient, `pointer-events: none` |
| `@keyframes flareIn` | One-time desktop animation | `brightness`, `saturate`, `contrast`, `blur` progression |
| `@media (max-width: 800px)` | Mobile layout | Single-column grid, top-banner background |

---

## 6. Step 5: Integrate with Battle Journal Component

### Overview

The [`battle-journal`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.ts:1) component already displays combatants during battle replay. Adding click-to-open details modal works the same pattern as in `enemies`.

### Step 5A: Update battle-journal.ts

Add these imports and properties near the top of the class:

```typescript
// Add to imports at top of file:
import { CombatantDetails } from '../combatant-details/combatant-details';

// Inside @Component decorator, add CombatantDetails to imports array:
@Component({
  // ...
  imports: [CommonModule, CombatantDetails],
  // ...
})

// Add new properties in the class body:
selectedCombatant = signal<Combatant | null>(null);

onCombatantClick(combatant: Combatant): void {
  this.selectedCombatant.set(combatant);
}
```

### Step 5B: Update battle-journal.html

Add the modal at the top of the template, and add click handlers to the unit cards:

```html
<!-- Add at the VERY TOP of battle-journal.html, before <div class="journal-container"> -->
@if (selectedCombatant()) {
  <app-combatant-details
    [combatant]="selectedCombatant()"
    (closed)="selectedCombatant(null)"
  >
  </app-combatant-details>
}

<div class="journal-container" (keydown)="handleKeyDown($event)" tabindex="0">
```

Then add `(click)` handlers to the unit cards inside the battle presentation:

```html
<!-- For our side cards (around line 26), add (click) to .unit-card: -->
<div class="unit-card {{ getCombatantHightlighClass(combatant) }} golden" (click)="onCombatantClick(combatant)">

<!-- For enemy side cards (around line 59), add (click) to .unit-card: -->
<div class="unit-card {{ getCombatantHightlighClass(combatant) }} magenta" (click)="onCombatantClick(combatant)">
```

---

## 7. Summary of Changes

### Files to Modify

| # | File | Action | Lines Changed |
|---|------|--------|---------------|
| 1 | [`enemies.html`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.html:14) | Fix `(click)` bindings, uncomment `(closed)` | ~10 lines |
| 2 | [`combatant-details.ts`](GrandAutoBattler_Angular_UI/src/app/components/combatant-details/combatant-details.ts:1) | Add `close()`, helper methods | ~15 lines added |
| 3 | [`combatant-details.html`](GrandAutoBattler_Angular_UI/src/app/components/combatant-details/combatant-details.html:1) | Full template rewrite | ~60 lines |
| 4 | [`combatant-details.scss`](GrandAutoBattler_Angular_UI/src/app/components/combatant-details/combatant-details.scss:1) | Full styling rewrite | ~180 lines |
| 5 | [`battle-journal.ts`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.ts:1) | Add import, signal, method | ~8 lines added |
| 6 | [`battle-journal.html`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.html:1) | Add modal + click handlers | ~8 lines added |

### Implementation Order

Follow this order to avoid confusion:

1. **Step 2** - Complete `combatant-details.ts` (the brain)
2. **Step 3** - Complete `combatant-details.html` (the template)
3. **Step 4** - Complete `combatant-details.scss` (the styling)
4. **Step 1** - Fix `enemies.html` (connect to existing prep work)
5. **Step 5** - Integrate with `battle-journal` (extend to second consumer)

After each step, save the file and verify no compile errors in your terminal (`ng serve` or `npm start`).

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                     Page Component                       │
│                  (enemies or battle-journal)             │
│                                                         │
│  selectedCombatant = signal<Combatant | null>(null)     │
│  onCombatantClick(c) { selectedCombatant.set(c) }       │
│                                                         │
│  ┌─────────────┐    click     ┌─────────────────────┐  │
│  │ Combatant   │ ──────────►  │ @if (selected) {    │  │
│  │ Card        │              │   <app-combatant-    │  │
│  │ (enemies.   │              │     details          │  │
│  │  html)      │              │     [combatant]="..." │  │
│  │             │              │     (closed)="null"   │  │
│  └─────────────┘              │   ></app-combatant-   │  │
│             │                 │     details>          │  │
│             │                 │ }                    │  │
│             │                 └─────────────────────┘  │
│             │                         │                │
│             │                   click overlay          │
│             │                   emits closed           │
│             │                   signal = null          │
│             │                   @if destroys           │
│             │                   component              │
└─────────────────────────────────────────────────────────┘
```

---

## Reference: Related Documents

- [`cssgloss.md`](Web-Server Speculations/cssgloss.md) - Glossy button/Glass card CSS techniques, flare animation parameters
- [`pixelatedImage.md`](Web-Server Speculations/images.md) - Pixel-perfect image rendering guide
- [`combatant-details.ts`](GrandAutoBattler_Angular_UI/src/app/components/combatant-details/combatant-details.ts:1) - Current component source
- [`game.model.ts`](GrandAutoBattler_Angular_UI/src/app/core/models/game.model.ts:45) - Combatant interface definition

