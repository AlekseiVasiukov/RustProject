# Glossy Buttons & Background GIF Flare Animations Guide

## Table of Contents
1. [Glossy Plastic/Glass Combatant Cards](#1-glossy-plasticglass-combatant-cards)
2. [Background GIF Flare/Fade Animations](#2-background-gif-flarefade-animations)

---

## 1. Glossy Plastic/Glass Combatant Cards

### Overview

This guide demonstrates how to transform the `.combatant-card` and `.unit-card` elements in `enemies.html` and `battle-journal.html` into elegant "polished plastic" or "glass" buttons using pure CSS. The effect relies on three core techniques:

- **Gloss (highlight)**: A semi-transparent gradient overlay that simulates light reflection
- **Bevel (edge lighting)**: Layered `box-shadow` with `inset` keywords to create 3D edge geometry
- **Internal shadow**: Soft `inset` shadows that give depth and curvature

### CSS Custom Properties (The "Knobs" to Play With)

The industry standard for可调 (tunable) visual design is to expose CSS custom properties. These act as sliders you can tweak to find your sweet spot.

```scss
// Add to your component SCSS (e.g., enemies.scss or a shared mixin)
.glossy-card {
  // --GLOSS PARAMETERS--
  --gloss-top-opacity: 0.45;        // Opacity of the top highlight (0.0 = invisible, 1.0 = solid white)
  --gloss-bottom-opacity: 0.0;      // Opacity at the bottom of the gloss band (fade-out edge)
  --gloss-height: 50%;              // Vertical extent of the gloss band (30% = thin strip, 80% = wide wash)
  --gloss-blur: 0px;                // Blur applied to the gloss edge (0px = sharp, 8px = soft diffusion)

  // --BEVEL PARAMETERS--
  --bevel-light-size: 2px;          // Thickness of the light edge highlight (1px = hairline, 4px = bold)
  --bevel-light-color: rgba(255, 255, 255, 0.35);  // Color of the top-left catchlight
  --bevel-dark-size: 3px;           // Thickness of the dark edge shadow
  --bevel-dark-color: rgba(0, 0, 0, 0.45);          // Color of the bottom-right shadow edge

  // --INNER SHADOW PARAMETERS--
  --inner-shadow-size: 6px;         // Spread/blurred radius of the inner shadow (0px = none, 12px = deep cavity)
  --inner-shadow-color: rgba(0, 0, 0, 0.25);        // Color of the inner shadow

  // --BASE PARAMETERS--
  --base-bg-start: #2a2a2a;         // Top gradient color of the card background
  --base-bg-end: #1a1a1a;           // Bottom gradient color (slightly darker for depth)
  --border-color: rgba(255, 255, 255, 0.08);  // Subtle edge border
  --border-radius: 10px;            // Rounded corners
}
```

### Implementation

#### Step 1: Create the Glossy Base

The gloss effect is built using a `::before` pseudo-element layered on top of the card. This pseudo-element contains a linear gradient that transitions from semi-transparent white at the top to fully transparent at the bottom.

```scss
// In enemies.scss, replace/enhance the .combatant-card styles
.combatant-card {
  position: relative;               // Required for ::before positioning
  overflow: hidden;                 // Clips the gloss to card boundaries
  border: 1px solid var(--border-color, rgba(255, 255, 255, 0.08));
  border-radius: var(--border-radius, 10px);
  padding: 12px;
  margin-bottom: 10px;

  // Base background gradient for the plastic body
  background: linear-gradient(
    to bottom,
    var(--base-bg-start, #2a2a2a) 0%,
    var(--base-bg-end, #1a1a1a) 100%
  );

  // Bevel + inner shadow via layered box-shadow
  box-shadow:
    // Top-left light bevel (catchlight)
    inset var(--bevel-light-size, 2px) var(--bevel-light-size, 2px) 0 var(--bevel-light-color, rgba(255, 255, 255, 0.35)),
    // Bottom-right dark bevel (shadow edge)
    inset calc(-1 * var(--bevel-dark-size, 3px)) calc(-1 * var(--bevel-dark-size, 3px)) 0 var(--bevel-dark-color, rgba(0, 0, 0, 0.45)),
    // Inner ambient shadow for depth/cavity feel
    inset 0 0 var(--inner-shadow-size, 6px) var(--inner-shadow-color, rgba(0, 0, 0, 0.25)),
    // Outer drop shadow for floating effect (optional)
    0 2px 6px rgba(0, 0, 0, 0.4);
}
```

#### Step 2: Add the Gloss Highlight

The `::before` pseudo-element creates the "glass" reflection band. It sits at the top of the card and fades downward.

```scss
.combatant-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: var(--gloss-height, 50%);
  background: linear-gradient(
    to bottom,
    rgba(255, 255, 255, var(--gloss-top-opacity, 0.45)) 0%,
    rgba(255, 255, 255, var(--gloss-bottom-opacity, 0.0)) 100%
  );
  border-radius: var(--border-radius, 10px) var(--border-radius, 10px) 0 0;
  pointer-events: none;             // Allows clicks to pass through to the card
  transition:
    height 0.25s ease,
    opacity 0.25s ease;
}
```

**How the gloss gradient works:**
- The top of the `::before` element is semi-transparent white (simulating incoming light)
- The bottom fades to fully transparent (simulating the light dying off on the surface)
- The `height` controls how far down the card the gloss reaches

#### Step 3: Side Accent Borders (Gold/Magenta)

```scss
.combatant-card.golden::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 4px;
  height: 100%;
  background: gold;
  border-radius: var(--border-radius, 10px) 0 0 var(--border-radius, 10px);
}

.combatant-card.magenta::after {
  content: '';
  position: absolute;
  bottom: 0;
  right: 0;
  width: 4px;
  height: 100%;
  background: magenta;
  border-radius: 0 var(--border-radius, 10px) var(--border-radius, 10px) 0;
}
```

#### Step 4: Hover State — Brighter and Narrower Gloss

When the user hovers, the gloss band should become more intense but cover less area, simulating a "sharper" reflection.

```scss
.combatant-card:hover {
  // On hover, tweak the custom properties
  --gloss-top-opacity: 0.65;       // Brighter (was 0.45)
  --gloss-height: 35%;             // Narrower (was 50%)
  --bevel-light-color: rgba(255, 255, 255, 0.5);  // Stronger catchlight

  // Slight lift effect
  transform: translateY(-1px);
  transition:
    transform 0.2s ease,
    box-shadow 0.2s ease;
  box-shadow:
    inset var(--bevel-light-size, 2px) var(--bevel-light-size, 2px) 0 var(--bevel-light-color, rgba(255, 255, 255, 0.5)),
    inset calc(-1 * var(--bevel-dark-size, 3px)) calc(-1 * var(--bevel-dark-size, 3px)) 0 var(--bevel-dark-color, rgba(0, 0, 0, 0.45)),
    inset 0 0 var(--inner-shadow-size, 6px) var(--inner-shadow-color, rgba(0, 0, 0, 0.25)),
    0 4px 10px rgba(0, 0, 0, 0.5);
}
```

#### Step 5: Active (Press) State — Wider and More Matte Gloss

When pressed, the gloss spreads wider but becomes more diffuse (lower opacity), simulating the surface flattening and light scattering.

```scss
.combatant-card:active {
  --gloss-top-opacity: 0.25;       // More matte/diffuse (was 0.45)
  --gloss-height: 70%;             // Wider spread (was 50%)
  --inner-shadow-size: 3px;        // Less depth (surface flattened)
  --bevel-light-size: 1px;         // Thinner catchlight

  transform: translateY(1px);
  box-shadow:
    inset var(--bevel-light-size, 1px) var(--bevel-light-size, 1px) 0 var(--bevel-light-color, rgba(255, 255, 255, 0.2)),
    inset calc(-1 * var(--bevel-dark-size, 3px)) calc(-1 * var(--bevel-dark-size, 3px)) 0 var(--bevel-dark-color, rgba(0, 0, 0, 0.45)),
    inset 0 0 var(--inner-shadow-size, 3px) var(--inner-shadow-color, rgba(0, 0, 0, 0.25)),
    0 1px 3px rgba(0, 0, 0, 0.4);
}
```

### Parameter Cheat Sheet

| Parameter | Effect of Increasing | Typical Range | Sweet Spot |
|-----------|---------------------|---------------|------------|
| `--gloss-top-opacity` | Brighter/more visible highlight | `0.0` to `1.0` | `0.30` - `0.55` |
| `--gloss-bottom-opacity` | Softer fade-out edge at bottom | `0.0` to `0.5` | `0.0` (hard edge) or `0.05` (soft fade) |
| `--gloss-height` | How far down the gloss reaches | `20%` to `90%` | `40%` - `55%` |
| `--bevel-light-size` | Thickness of top-left highlight | `0px` to `6px` | `1px` - `3px` |
| `--bevel-dark-size` | Thickness of bottom-right shadow | `0px` to `6px` | `2px` - `4px` |
| `--inner-shadow-size` | Depth of the cavity effect | `0px` to `16px` | `4px` - `8px` |
| `--inner-shadow-color` alpha | Intensity of the inner shadow | `0.0` to `0.6` | `0.15` - `0.35` |

### Applying to Battle Journal Cards

Apply the same pattern to `.unit-card` in `battle-journal.scss`:

```scss
.unit-card {
  // Copy the same glossy-card structure from above
  position: relative;
  overflow: hidden;
  // ... same box-shadow, ::before, :hover, :active patterns
}
```

### Mermaid Diagram: Gloss Layer Stack

```
Card Element (position: relative)
├── Background Gradient (base plastic body)
├── box-shadow (inset bevels + inner shadow)
├── ::before (absolute, gloss highlight layer)
├── ::after (absolute, side accent border)
└── Content (name, stats, HP text)
```

---

## 2. Background GIF Flare/Fade Animations

### Overview

You want a background GIF image under the combatant details card that:

1. **On wide screens (>= 800px):** Plays a one-time "flare" animation on initial appearance — starts oversaturated/bright white and smoothly transitions to normal appearance, then stays static.
2. **On narrow screens (< 800px):** Loops infinitely with horizontal panning (left-to-right), fading to BLACK at the right edge, then instantly resetting to the left and fading in from black.

### Technique A: Desktop — One-Time "Coming Out of Light" Flare

The industry approach uses CSS `@keyframes` to animate CSS filter properties (`brightness`, `contrast`, `saturate`) combined with `opacity`. The image starts as a "washed-out white" state and settles into normal appearance.

#### Implementation

```scss
// In your details-card component SCSS
.details-background-gif {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
  z-index: -1;  // Behind the content

  // Initial state: oversaturated, over-bright, slightly blurred
  animation: flareIn 1.5s ease-out forwards;
}

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
```

#### Parameter Explanation

| Filter Function | What it Does | Effect at High Values | Effect at Low Values |
|----------------|-------------|----------------------|---------------------|
| `brightness(N)` | Multiplies pixel brightness | `3.0` = blinding white | `0.2` = nearly black |
| `saturate(N)` | Multiplies color saturation | `3.0` = neon vivid | `0.0` = grayscale |
| `contrast(N)` | Stretches the tonal range | `3.0` = harsh, crushed blacks | `0.0` = flat gray |
| `blur(Npx)` | Gaussian blur on the image | `8px` = heavy smear | `0px` = sharp |

#### How the Animation Works

The keyframe progression simulates a "flash of light" settling into clarity:

1. **0%**: The image is extremely bright (`brightness(3.0)`), over-saturated, with collapsed contrast and blur — essentially a white haze
2. **30%**: Still bright but starting to resolve — the "light" begins to fade
3. **70%**: Almost normal, just a slight "afterglow"
4. **100%**: Fully settled at normal appearance

The `forwards` fill-mode ensures the image stays in the final state after the animation completes.

#### Triggering Only on Modal Open

To ensure the animation only plays when the modal first appears, trigger it via a CSS class added by Angular:

```typescript
// In your details-card component TypeScript
@HostBinding('class.modal-visible') modalVisible = false;

ngAfterViewInit() {
  // Add class to trigger animation
  this.modalVisible = true;
}
```

```scss
// Only animate when the class is present
.details-background-gif.modal-visible {
  animation: flareIn 1.5s ease-out forwards;
}
```

### Technique B: Mobile — Infinite Pan with Fade-to-Black Edges

On narrow screens (< 800px), the widescreen GIF (16:9) won't fit vertically. The strategy is to create a horizontal scrolling window with fade-to-black masks on the left and right edges.

#### Implementation

```scss
// Container that clips the image and provides the fade masks
.gif-pan-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;  // Clips the moving image
  z-index: -1;

  // Black fade gradient overlays on both edges
  &::before,
  &::after {
    content: '';
    position: absolute;
    top: 0;
    height: 100%;
    width: 30%;  // Width of the fade zone on each side
    z-index: 1;  // Above the image
    pointer-events: none;
  }

  &::before {
    left: 0;
    background: linear-gradient(to right, black 0%, transparent 100%);
  }

  &::after {
    right: 0;
    background: linear-gradient(to left, black 0%, transparent 100%);
  }
}

// The moving GIF image inside the container
.gif-pan-container .panning-gif {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  width: auto;
  // The GIF is wider than the container, so it moves
  animation: panLeftToRight 8s linear infinite;
}

@keyframes panLeftToRight {
  0% {
    transform: translateX(0);
  }
  85% {
    transform: translateX(calc(100vw - 100%));  // Move until right edge approaches
  }
  92% {
    transform: translateX(calc(100vw - 100%));  // Hold at right edge
    opacity: 0;  // Fade to black
  }
  93% {
    transform: translateX(0);  // Instant reset to left
    opacity: 0;
  }
  100% {
    transform: translateX(0);
    opacity: 1;  // Fade in from black
  }
}
```

#### How the Pan-and-Fade Works

```
Timeline of one loop cycle (8 seconds):

[0% ============================ 85% === 92%| 93% ============ 100%]
 |<-------- pan left-to-right -------->| FADE|reset| FADE-IN |
                                                              at left
```

1. **0% to 85%**: The GIF image smoothly pans from the left edge toward the right edge
2. **85% to 92%**: Image holds position at the right edge while `opacity` fades to 0 (blends into black background)
3. **92% to 93%**: While invisible (opacity 0), instantly snap back to the left edge
4. **93% to 100%**: Image fades back in from black at the left edge, ready to begin panning again

The gradient masks on `::before` and `::after` ensure the edges always blend gracefully into the black background, even during the visible pan.

#### Media Query Integration

```scss
// Wide screens: one-time flare animation
@media (min-width: 801px) {
  .details-background-gif {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    z-index: -1;
    animation: flareIn 1.5s ease-out forwards;
  }
}

// Narrow screens: infinite pan with fade edges
@media (max-width: 800px) {
  .gif-pan-container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    z-index: -1;

    &::before,
    &::after {
      content: '';
      position: absolute;
      top: 0;
      height: 100%;
      width: 30%;
      z-index: 1;
      pointer-events: none;
    }

    &::before {
      left: 0;
      background: linear-gradient(to right, black 0%, transparent 100%);
    }

    &::after {
      right: 0;
      background: linear-gradient(to left, black 0%, transparent 100%);
    }

    .panning-gif {
      position: absolute;
      top: 0;
      left: 0;
      height: 100%;
      width: auto;
      animation: panLeftToRight 8s linear infinite;
    }
  }
}
```

### Cinematic Gamma/Contrast/Brightness Controls

For finer control over the "cinematic" feel of the flare animation, you can introduce additional CSS custom properties:

```scss
.details-background-gif {
  // --CINEMATIC FILTER PARAMETERS--
  --flare-brightness-peak: 3.0;     // How bright the initial flash is
  --flare-saturation-peak: 3.0;     // How vivid the initial flash is
  --flare-contrast-low: 0.2;        // How flat/washed-out the initial state is
  --flare-blur-start: 4px;          // How blurry the image starts
  --settle-duration: 1.5s;          // How long the transition to normal takes
  --settle-easing: ease-out;        // Timing function (ease-out = fast then slow)
}

@keyframes flareIn {
  0% {
    filter: brightness(var(--flare-brightness-peak))
            saturate(var(--flare-saturation-peak))
            contrast(var(--flare-contrast-low))
            blur(var(--flare-blur-start));
    opacity: 0;
  }
  100% {
    filter: brightness(1.0) saturate(1.0) contrast(1.0) blur(0px);
    opacity: 1;
  }
}
```

### Alternative: Gradient Transition Overlay

If you want a "white gradient wipe" effect instead of filter-based approach, use an overlay element:

```scss
.details-card-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
  // Starts as solid white, animates to transparent
  background: radial-gradient(ellipse at center, rgba(255,255,255,0.9) 0%, rgba(255,255,255,0.4) 60%, transparent 100%);
  animation: overlayFadeOut 1.5s ease-out forwards;
  pointer-events: none;
}

@keyframes overlayFadeOut {
  0% {
    opacity: 1;
    transform: scale(1.2);  // Slightly oversized for a "burst" effect
  }
  100% {
    opacity: 0;
    transform: scale(1.0);
  }
}
```

This layer sits between the GIF and the content, providing a clean "flash of light" without modifying the image filters.

---

## Summary of Key Techniques

### Glossy Buttons
| Technique | CSS Property | Purpose |
|-----------|-------------|---------|
| Gloss highlight | `::before` + `linear-gradient` | Simulates light reflection on glass/plastic |
| Bevel | `box-shadow: inset` (directional) | Creates 3D edge geometry (top-light, bottom-dark) |
| Inner shadow | `box-shadow: inset 0 0 Npx` | Adds depth/cavity feel to the card surface |
| Hover | Reduce `--gloss-height`, increase `--gloss-top-opacity` | Sharper, brighter reflection |
| Active/Press | Increase `--gloss-height`, decrease `--gloss-top-opacity` | Wider, more diffuse (matte) reflection |

### Background GIF Animations
| Technique | CSS Property | Purpose |
|-----------|-------------|---------|
| Flare-in | `filter: brightness() saturate() contrast() blur()` | Simulates emerging from white light |
| Opacity settling | `opacity: 0 -> 1` | Image fades into existence |
| Pan animation | `transform: translateX()` | Horizontal movement for mobile |
| Edge fade | `::before`/`::after` + `linear-gradient` | Masks image edges to black |
| Loop reset | `opacity: 0` + instant `translateX(0)` | Invisible snap-back for seamless repeat |

---

## File References

- Combatant cards in enemies: [`enemies.html`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.html:11)
- Combatant cards in battle journal: [`battle-journal.html`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.html:26)
- Enemies SCSS: [`enemies.scss`](GrandAutoBattler_Angular_UI/src/app/components/enemies/enemies.scss:1)
- Battle journal SCSS: [`battle-journal.scss`](GrandAutoBattler_Angular_UI/src/app/components/battle-journal/battle-journal.scss:1)
- Global variables: [`_variables.scss`](GrandAutoBattler_Angular_UI/src/styles/_variables.scss:1)
- Global mixins: [`mixins.scss`](GrandAutoBattler_Angular_UI/src/styles/mixins.scss:1)
- Global styles: [`styles.scss`](GrandAutoBattler_Angular_UI/src/styles/styles.scss:1)
