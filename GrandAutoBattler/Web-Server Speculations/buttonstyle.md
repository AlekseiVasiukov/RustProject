Yes — the main issue is that in SCSS, `.hover` means **a child class named `hover`**, not the mouse-hover state.

Use `&:hover`:

```scss
.btn-advance-stage {
  background-color: yellow;
  transition:
    background-color 0.3s ease,
    opacity 0.2s ease,
    transform 0.3s ease;

  &:hover {
    background-color: white;
    opacity: 0.8;
    transform: scale(1.05);
  }
}
```

For the **soft, bubbly / wobbling effect** you're describing, I'd use `transform` rather than changing `width`. Transformations are smoother and don't cause layout reflow.

### A nice "soap bubble" hover

```scss
.btn-advance-stage {
  background-color: yellow;

  transition:
    background-color 0.3s ease,
    opacity 0.2s ease,
    transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);

  &:hover {
    background-color: white;
    opacity: 0.85;
    transform: scale(1.08);
  }

  &:active {
    transform: scale(0.94);
    transition-duration: 0.12s;
  }
}
```

The interesting part is:

```scss
cubic-bezier(0.34, 1.56, 0.64, 1)
```

The `1.56` makes it **overshoot**, giving you that slightly squishy/bouncy feeling.

### If you want an actual wobble

You can use a keyframe animation:

```scss
.btn-advance-stage {
  background-color: yellow;

  &:hover {
    animation: bubble-wobble 0.6s ease-out;
  }
}

@keyframes bubble-wobble {
  0% {
    transform: scale(1);
  }

  25% {
    transform: scale(1.08, 0.94);
  }

  45% {
    transform: scale(0.96, 1.05);
  }

  65% {
    transform: scale(1.04, 0.98);
  }

  82% {
    transform: scale(0.99, 1.01);
  }

  100% {
    transform: scale(1);
  }
}
```

That gives you roughly:

**normal → squish → stretch → squish → settle**

For a button, I generally prefer the first version because it feels more natural and doesn't repeatedly restart the animation while the mouse is moving around.

One other useful trick: if you want the button to **expand sideways without changing its height**, use:

```scss
&:hover {
  transform: scaleX(1.08);
}
```

Or a subtle press effect:

```scss
&:active {
  transform: scale(0.95);
}
```

So you can combine them:

```scss
.btn-advance-stage {
  background: yellow;
  transition:
    background-color 250ms ease,
    opacity 200ms ease,
    transform 400ms cubic-bezier(0.34, 1.56, 0.64, 1);

  &:hover {
    background: white;
    opacity: 0.85;
    transform: scale(1.06);
  }

  &:active {
    transform: scale(0.94);
    transition-duration: 100ms;
  }
}
```

That gives you a **slightly inflated button on hover + satisfying squish on press**, without affecting the surrounding layout.
