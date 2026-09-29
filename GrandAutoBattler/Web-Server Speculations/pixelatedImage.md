## Explanation of Your Current Implementation and Solution

### What your current CSS does:

**[`.game-bg`](src/app/components/new-game/new-game.scss:36)** (lines 36-42): Container with `max-width: 40rem; max-height: 20rem`, `position: relative` (creates positioning context), and `overflow: hidden`.

**[`.game-bg img`](src/app/components/new-game/new-game.scss:44)** (lines 44-50): Image fills container via `object-fit: cover` with `image-rendering: pixelated` for sharp pixel edges.

**[`.game-bg::after`](src/app/components/new-game/new-game.scss:52)** (lines 52-70): Pseudo-element overlay with `position: absolute; inset: 0` that covers the entire container with a grid pattern.

### Root Cause of Your Problem

The `::after` pseudo-element is attached to the `.game-bg` *container*, not the image. Since `inset: 0` pins it to all edges of the container, the grid covers everything inside that container — including space outside the image.

### Proposed Solution: Reusable `app-pixel-picture` Component

Create a standalone component at `src/app/components/pixel-picture/` with:

**Inputs:** `src`, `alt`, `width`, `height`, `pixelSize`, `dimmed`

**Structure:**
```
.pixel-picture (outer container, max dimensions)
  └── .pixel-picture__image-wrapper (position:relative, overflow:hidden)
        ├── img (the actual image)
        └── .pixel-picture__grid (absolute overlay, clipped by wrapper)
```

**Key difference:** The grid overlay is a real element inside a wrapper with `overflow: hidden`, so it gets clipped to the image bounds instead of spilling outside.

**Usage:**
```html
<app-pixel-picture
  src="path/to/image.png"
  alt="description"
  width="40rem"
  height="20rem"
  pixelSize="2px"
/>
```

This gives you full control over dimensions, pixel grid density, and dimming — all as a reusable component droppable into any other component.