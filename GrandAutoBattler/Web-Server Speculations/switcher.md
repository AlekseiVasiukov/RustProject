# Warrior Switcher Implementation Guide

## Overview

Replace the existing dropdown (`<select>`) selectors for factions and warrior types with custom button-based cyclers that display `< Option >` with navigation arrows and unit type images.

## Visual Design

```
Our Faction
< Humans >

Our Warriors
< † Knight >
< † Archer >
<   None >
< † Witch >

Opponent Faction
< Demons >
```

### Key Elements
- **Left/Right arrows** (`&#9664;` and &#9654;) cycle through available options
- **Unit type icon** (†) is a `<img>` tag pointing to a 64px image
- **None** slots display no image, only whitespace padding
- **Unknown unit types** (not in the image map) skip the image entirely

## Image Mapping System

### Image Location
```
src/app/assets/images/64px/
```

### Known Image Files
| File | Faction | Pretty Unit Type |
|------|---------|-----------------|
| `humans-knight.png` | humans | knight |
| `humans-archer.png` | humans | archer |
| `humans-paladin.png` | humans | paladin |
| `humans-witch.png` | humans | witch |
| `demons-abomination.png` | demons | abomination |
| `demons-demonologist.png` | demons | demonologist |
| `demons-imp.png` | demons | imp |
| `demons-succubus.png` | demons | succubus |

### Image Key Construction
The image filename follows this pattern:
```
{factionNameLowercase}-{prettyUnitTypeNameLowercase}.png
```

For example:
- Faction: "Humans", Unit Type: "Knight" → `humans-knight.png`
- Faction: "Demons", Unit Type: "Succubus" → `demons-succubus.png`

---

## Step-by-Step Implementation

### Step 1: Create the Image Map Helper in the Component

Add a method to [`unit-list.ts`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.ts) that returns the image path for a given faction and unit type, or `null` if unknown.

**Add this property and method to the `UnitList` component:**

```typescript
// Map of known unit type images keyed by "faction-prettyUnitType" (lowercase)
private knownUnitImages = new Set([
  'humans-knight',
  'humans-archer',
  'humans-paladin',
  'humans-witch',
  'demons-abomination',
  'demons-demonologist',
  'demons-imp',
  'demons-succubus',
]);

/**
 * Get the image path for a unit type, or null if unknown.
 */
getUnitImagePath(factionIndex: number, unitTypeIndex: number | null): string | null {
  if (unitTypeIndex === null) return null;
  const factionName = this.availableFactions()[factionIndex]?.toLowerCase() ?? '';
  const unitTypeName = this.availableUnittypes()[factionIndex]?.[unitTypeIndex]?.toLowerCase() ?? '';
  const key = `${factionName}-${unitTypeName}`;
  return this.knownUnitImages.has(key) ? `/assets/images/64px/${key}.png` : null;
}
```

---

### Step 2: Create Faction Cycle Methods

Add methods to cycle the faction index forward (right button) and backward (left button).

**Add to [`unit-list.ts`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.ts):**

```typescript
/**
 * Cycle our faction index in the given direction.
 */
onOurFactionCycle(direction: number) {
  const factions = this.availableFactions();
  const count = factions.length;
  if (count === 0) return;

  const current = this.ourFactionIndex();
  const next = (current + direction + count) % count;
  this.ourFactionIndex.set(next);
}

/**
 * Cycle opponent faction index in the given direction.
 */
onOpponentFactionCycle(direction: number) {
  const factions = this.availableFactions();
  const count = factions.length;
  if (count === 0) return;

  const current = this.opponentFactionIndex();
  const next = (current + direction + count) % count;
  this.opponentFactionIndex.set(next);
}
```

---

### Step 3: Create Warrior Type Cycle Method

Add a method to cycle the warrior type for a given slot. The cycle order is:
`0 → 1 → 2 → ... → null (None) → 0 → ...`

**Add to [`unit-list.ts`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.ts):**

```typescript
/**
 * Cycle the warrior type for a given slot in the given direction.
 * Direction: -1 for left (previous), +1 for right (next).
 */
onWarriorTypeCycle(slotIndex: number, direction: number) {
  const factionIdx = this.ourFactionIndex();
  const typeNames = this.availableUnittypes()[factionIdx] ?? [];
  const totalOptions = typeNames.length + 1; // +1 for "None"
  if (totalOptions === 0) return;

  const current = [...this.ourWarriorTypes()];
  const currentValue = current[slotIndex];

  // Convert current value to a position in the cycle:
  // null (None) is at position "typeNames.length"
  let position = currentValue === null ? typeNames.length : currentValue;

  // Calculate new position
  let newPosition = (position + direction + totalOptions) % totalOptions;

  // Convert back: if newPosition === typeNames.length, it's null
  const newValue = newPosition === typeNames.length ? null : newPosition;

  // Validation: prevent setting to null if all other slots are already null
  if (newValue === null) {
    const allOthersNull = current.every((v, i) => i !== slotIndex && v === null);
    if (allOthersNull) return;
  }

  current[slotIndex] = newValue;
  this.ourWarriorTypes.set(current);
}
```

---

### Step 4: Replace the HTML Template

Replace the contents of [`unit-list.html`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.html) with the new switcher-based layout.

**New template:**

```html
<div class="unit-list-container">
  <div class="game-zone">
    <h2>Select Warriors</h2>
    <div class="factions-layout">

      <!-- Our Side -->
      <div>
        <h3>Our Warriors</h3>

        <div class="faction-switcher">
          <label>Our Faction</label>
          <div class="switcher-buttons">
            <button class="cycle-btn" aria-label="Previous faction" (click)="onOurFactionCycle(-1)">&#9664;</button>
            <span class="switcher-label">{{ availableFactions()[ourFactionIndex()] }}</span>
            <button class="cycle-btn" aria-label="Next faction" (click)="onOurFactionCycle(1)">&#9654;</button>
          </div>
        </div>

        <div class="warrior-slots">
          @for (slot of [0, 1, 2, 3]; track slot) {
            <div class="warrior-switcher">
              <div class="switcher-buttons">
                <button class="cycle-btn" aria-label="Previous warrior type" (click)="onWarriorTypeCycle(slot, -1)">&#9664;</button>

                <div class="switcher-label">
                  @let unitIndex = ourWarriorTypes()[slot];
                  @if (unitIndex !== null) {
                    @let imagePath = getUnitImagePath(ourFactionIndex(), unitIndex);
                    @if (imagePath) {
                      <img [src]="imagePath" [alt]="getUnitTypeName(ourFactionIndex(), unitIndex)" class="unit-icon" />
                    }
                    <span>{{ getUnitTypeName(ourFactionIndex(), unitIndex) }}</span>
                  } @else {
                    <span class="none-label">None</span>
                  }
                </div>

                <button class="cycle-btn" aria-label="Next warrior type" (click)="onWarriorTypeCycle(slot, 1)">&#9654;</button>
              </div>
            </div>
          }
        </div>
      </div>

      <!-- Opponent Side -->
      <div>
        <h3>Enemy Warriors</h3>

        <div class="faction-switcher">
          <label>Opponent Faction</label>
          <div class="switcher-buttons">
            <button class="cycle-btn" aria-label="Previous faction" (click)="onOpponentFactionCycle(-1)">&#9664;</button>
            <span class="switcher-label">{{ availableFactions()[opponentFactionIndex()] }}</span>
            <button class="cycle-btn" aria-label="Next faction" (click)="onOpponentFactionCycle(1)">&#9654;</button>
          </div>
        </div>
      </div>

    </div>
  </div>

  <!-- Confirm button -->
  <button class="btn-advance-stage" [disabled]="!isValidSelection" (click)="onConfirmSelection()">
    Confirm Selection
  </button>
</div>
```

---

### Step 5: Add SCSS Styles for the Switcher

Add the following styles to [`unit-list.scss`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.scss) to style the new switcher buttons.

```scss
// Faction switcher layout
.faction-switcher {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1.5rem;

  label {
    font-weight: bold;
    font-size: 1rem;
  }
}

// Warrior slots container
.warrior-slots {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

// Individual warrior switcher
.warrior-switcher {
  .switcher-buttons {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }
}

// Shared switcher button row
.switcher-buttons {
  display: flex;
  align-items: center;
  gap: 0.5rem;

  .switcher-label {
    min-width: 150px;
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font-size: 1rem;

    .unit-icon {
      width: 32px;
      height: 32px;
      image-rendering: pixelated;
      image-rendering: -moz-crisp-edges;
      image-rendering: crisp-edges;
    }

    .none-label {
      color: #888;
      font-style: italic;
    }
  }
}

// Cycle buttons (left and right arrows)
.cycle-btn {
  background: $card-bg-color;
  border: 1px solid $contour-color;
  border-radius: $border-radius;
  color: $text-color;
  font-size: 1.2rem;
  padding: 0.25rem 0.5rem;
  cursor: pointer;
  transition: background-color 0.2s ease;

  &:hover {
    background-color: $contour-color;
    color: $card-bg-color;
  }

  &:active {
    transform: scale(0.95);
  }
}
```

---

## Summary of Changes

| File | Change |
|------|--------|
| [`unit-list.ts`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.ts) | Add `knownUnitImages` Set, `getUnitImagePath()`, `onOurFactionCycle()`, `onOpponentFactionCycle()`, `onWarriorTypeCycle()` |
| [`unit-list.html`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.html) | Replace all `<select>` elements with button-based switchers using `&#9664;` and &#9654; arrows |
| [`unit-list.scss`](GrandAutoBattler_Angular_UI/src/app/components/unit-list/unit-list.scss) | Add styles for `.faction-switcher`, `.warrior-switcher`, `.switcher-buttons`, `.cycle-btn`, `.unit-icon` |

## Notes

- The old methods `onOurFactionChange()`, `onOpponentFactionChange()`, and `onWarriorTypeChange()` can be removed after verification, since they are no longer referenced by the template.
- The image mapping uses a `Set` of known keys. If the server returns an unknown unit type, no image is rendered (only the text name).
- The cycle logic wraps around: pressing right on the last option goes to the first, and pressing left on the first option goes to the last.
- Validation logic (preventing all warriors from being `None`) is preserved in `onWarriorTypeCycle()`.
