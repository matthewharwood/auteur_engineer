# Anime.js v3.x.x → v4.x.x Migration Guide

Anime.js v4 introduces a modern, modular API that replaces the older v3 global API. This guide explains how to migrate existing v3 code to v4, shows side-by-side syntax differences and highlights performance and readability best practices. All examples use vanilla JavaScript with ES modules (`import … from 'animejs'`).

## Introduction

Anime.js v3 made it simple to animate CSS properties, transform values, JavaScript objects and SVG attributes using a single global `anime` object. Version 4 takes a modular approach: functions and classes are imported individually from `animejs`. The core animation engine has been rewritten and builds on the Web Animations API (WAAPI) for better performance. Several parameters and callbacks have been renamed or reorganised, and a few behaviours have changed.

If you are starting a new project you should use v4. When updating an existing v3 project, read through the Changes sections below and replace each old pattern with its v4 counterpart.

---

## 1. Core API changes

### 1.1 Importing the library

```js
// v3
import anime from 'animejs';

const animation = anime({ targets: 'div', opacity: 0.5 });
```

```js
// v4
import { animate } from 'animejs';

const animation = animate('div', { opacity: 0.5 });
```

The v4 engine also exposes other modules (`createTimeline`, `utils`, `svg`, `waapi`, etc.), which you import separately.

### 1.2 Targets

```js
// v3
anime({
  targets: 'div',
  opacity: 0.5
});
```

```js
// v4
animate('div', {
  opacity: 0.5
});
```

The `targets` property has been removed. Use the first argument of `animate()` instead.

### 1.3 Renamed property parameters

| v3       | v4        |
| -------- | --------- |
| endDelay | loopDelay |
| easing   | ease      |
| value    | to        |
| round    | modifier  |

Example:

```js
// v3
opacity: { value: 0.5, duration: 250 }

// v4
opacity: { to: 0.5, duration: 250 }
```

To round values in v4:

```js
modifier: utils.round(decimalLength)
```

### 1.4 Animation parameters

#### Direction

```js
// v3
direction: 'reverse'
// v4
reversed: true
```

```js
// v3
direction: 'alternate'
// v4
alternate: true
```

#### Loop semantics

* v3: `loop: 1` = one iteration
* v4: `loop: 1` = two iterations (1 initial + 1 repeat)

#### Other changes

* `autoplay` still exists, defaults to `true`
* Spring easing:

```js
import { createSpring, animate } from 'animejs';
const spring = createSpring({ mass: 1, stiffness: 80, damping: 10, velocity: 0 });
animate('.box', { x: 200, ease: spring });
```

* Custom easing functions now passed directly to `ease`

### 1.5 Keyframes and stagger

```js
// v3
opacity: [{ value: 0.5 }, { value: 1 }, { value: 0.5 }]

// v4
opacity: [{ to: 0.5 }, { to: 1 }, { to: 0.5 }]
```

#### Stagger options changes

* `direction` → `reversed`
* `easing` → `ease`

```js
import { animate, stagger } from 'animejs';

animate('.items', {
  y: stagger(10, { reversed: true, ease: 'inOutQuad' }),
  loop: 1,
  alternate: true
});
```

---

## 2. Timeline changes

### 2.1 Creating timelines

```js
// v3
const tl = anime.timeline();

// v4
import { createTimeline } from 'animejs';
const tl = createTimeline();
```

### 2.2 Defaults

```js
// v3
const tl = anime.timeline({ easing: 'easeOutQuad', duration: 250 });

// v4
const tl = createTimeline({
  defaults: { ease: 'outQuad', duration: 250 }
});
```

### 2.3 Loop in children

Children of a timeline can now define their own `loop` independently.

### 2.4 Syncing animations

```js
import { createTimeline, waapi } from 'animejs';
const tl = createTimeline();

const fade = waapi.animate('.box', { opacity: [1, 0.5], duration: 1000 });
ttl.sync(fade);
```

---

## 3. Controls (playback methods)

* `.play()` always plays forward
* `.resume()` continues in current direction
* `.reverse()` plays backward
* `.alternate()` alternates directions

---

## 4. Callbacks and promises

| v3                         | v4         |
| -------------------------- | ---------- |
| update                     | onUpdate   |
| begin                      | onBegin    |
| complete                   | onComplete |
| loopBegin/loopComplete     | onLoop     |
| change                     | onRender   |
| changeBegin/changeComplete | *removed*  |

#### Promises

```js
// v3
anime({ targets: target, ... }).finished.then(() => { ... });

// v4
animate(target, options).then(() => { ... });
```

---

## 5. SVG helpers

| v3                    | v4                             |
| --------------------- | ------------------------------ |
| anime.path(selector)  | svg.createMotionPath(selector) |
| anime.setDashoffset() | svg.createDrawable()           |

```js
import { animate, svg } from 'animejs';
const { translateX, translateY, rotate } = svg.createMotionPath('svg path');
animate('.ball', { translateX, translateY, rotate, duration: 2000 });

const drawable = svg.createDrawable('path');
animate(drawable, { draw: '0 1', duration: 1500 });
```

---

## 6. Easings

* `easing` → `ease`
* All built-ins lost `ease` prefix: `'easeOutQuad'` → `'outQuad'`
* Default: `'out(2)'`
* Springs via `createSpring()`
* Custom functions passed directly

---

## 7. Utilities & Engine

### 7.1 Utility functions moved to `utils`

| v3                 | v4             |
| ------------------ | -------------- |
| animation.remove() | utils.remove() |
| anime.get()        | utils.get()    |
| anime.set()        | utils.set()    |
| anime.random()     | utils.random() |

```js
modifier: utils.round(2)
```

### 7.2 Engine updates

* `engine.useDefaultMainLoop = false` to disable
* `engine.update()` must be called manually if loop is disabled
* `anime.running` removed
* `pauseOnDocumentHidden` moved to `engine.pauseOnDocumentHidden`
* `engine.speed`, `engine.fps`, `engine.precision` added

### 7.3 `Animatable` vs `utils.set()`

Use `utils.set()` for complex one-off settings.
Use `Animatable` (via `animate()` or `waapi.animate()`) for repeated updates to avoid parsing overhead.

---

## 8. New modules and features

* `waapi.animate()`: minimal, fast, no timeline/stagger
* `createDraggable()`: drag-and-drop support
* Scroll/scope modules for scroll-triggered animations

---

## 9. Performance & readability best practices

### 9.1 Use `transform` and `opacity`

Avoid layout-affecting properties like `top`, `left`, `width`.

### 9.2 Prefer WAAPI when possible

Use `waapi.animate()` for lightweight transitions.

### 9.3 Use `Animatable` for repeated updates

### 9.4 Pause animations when not visible

Enable `engine.pauseOnDocumentHidden` (default: `true`)

### 9.5 Centralise defaults and organise timelines

Use `defaults` to prevent repetition.

### 9.6 Use modular imports

Import only what's needed (`animate`, `stagger`, `utils`, etc.)

### 9.7 Write modern, readable code

* Use `const` and `let`
* Arrow functions
* Named durations, delays
* Split logic into functions

---

## Conclusion

Migrating from Anime.js v3 to v4 primarily involves adopting ES module imports and updating option names. The new API offers a cleaner, more modular design, improved performance through WAAPI, and better control via the engine and utils modules. By following this guide—renaming parameters (`endDelay` → `loopDelay`, `easing` → `ease`, etc.), replacing the global `anime` object with `animate()`, updating timelines, and observing best practices—you can upgrade your animations to v4 smoothly and take advantage of its improvements.
