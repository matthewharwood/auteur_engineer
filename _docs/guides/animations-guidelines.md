# Frontend motion guidelines

## Motion & Animation Instructions for Front‑End Engineers (SEO‑friendly Web Components)

---

## Web Components and Progressive Enhancement

* **Use light DOM with `is` syntax for SEO.**
  When defining custom elements, prefer extending existing semantic HTML elements via the `is` attribute rather than shadow DOM.

  Example:

  ```html
  <button is="fancy-button"></button>
  ```

  This keeps your element inside the light DOM, ensuring search engines can parse meaningful markup and reduces the risk of inaccessible content.
  Avoid deep shadow DOM for content that needs to be indexed or crawled by SEO bots.

* **Fallback semantics.**
  Provide default content in light DOM so that if custom element registration or JavaScript fails, the base element still conveys intent.

  For instance:

  ```html
  <form is="custom-form">
    <!-- Fallback form content -->
  </form>
  ```

* **Progressive enhancement.**
  Start with a basic experience that works with static HTML and CSS. Add interactivity and animations using CSS first, then layer JavaScript (e.g., AnimeJS) for richer motion when needed. This ensures that pages remain usable when JavaScript is disabled or fails to load.

---

## High‑Performance CSS Animations (Web Dev Guide)

* **Prefer `transform` and `opacity`.**
  When moving, rotating or resizing elements, animate the `transform` property (`translate`, `rotate`, `scale`) and `opacity`. These properties operate in the compositor stage and don’t trigger layout or paint.

* **Avoid properties that trigger layout or paint.**
  Before animating any property other than `transform` and `opacity`, check its impact on the rendering pipeline.

  Properties like:

  ```css
  top, left, width, height, box-shadow
  ```

  cause layout or paint and will almost certainly drop frames. Use:

  ```css
  transform: translate();
  transform: scale();
  ```

  instead of animating `top/left` or `width/height`.

* **Use `will-change` sparingly.**
  Promote elements to their own compositor layer only when you have evidence (via DevTools) that the animation needs it.
  Overusing `will-change` can cause memory overhead and hurt performance.

  Use dynamically:

  ```js
  element.style.willChange = 'transform';
  // Later remove
  element.style.willChange = '';
  ```

* **Debug with DevTools.**
  Use Chrome or Firefox DevTools to:

    * Identify animations that trigger layout or paint
    * Check for nonzero rendering in the Performance summary
    * Analyze dropped frames
    * Use paint flashing and the paint profiler

* **Force layer creation only if needed.**
  As a fallback for browsers without `will-change` support:

  ```css
  transform: translateZ(0);
  ```

---

## RAIL Model — User‑Centric Performance Goals

The RAIL model breaks down user interaction into four phases: **Response**, **Animation**, **Idle**, and **Load**. Use it to plan animation performance.

### Response (R): Handle input quickly

* **Goal:** Complete transitions triggered by user input within 100 ms.
* **Guidelines:**

    * Process events in under 50 ms.
    * Provide feedback (e.g., spinner) if longer.
    * Use remaining time in 100 ms window for background work.

### Animation (A): Produce frames efficiently

* **Goal:** Render each animation frame in \~10 ms.
* **Guidelines:**

    * Theoretical budget = 16 ms (60fps)
    * Browser uses \~6 ms, leaving \~10 ms for your code.
    * Do minimal work in animation frame callbacks.
    * Precompute heavy calculations beforehand.

### Idle (I): Utilize downtime

* **Goal:** Maximize idle time for the main thread.
* **Guidelines:**

    * Schedule non‑critical work using:

      ```js
      requestIdleCallback(() => {
        // Non-critical work
      });
      ```
    * Idle tasks ≤ 50 ms
    * Allow user input to interrupt idle work.

### Load (L): Fast initial rendering

* **Goal:**

    * First load: Interactive within 5 s on 3G mobile.
    * Return visit: < 2 s.

* **Guidelines:**

    * Lazy‑load images
    * Split JS bundles
    * Use service worker to cache

---

### RAIL Summary

* **Respond to input:** ≤ 100 ms
* **Draw frames:** ≤ 10 ms
* **Idle tasks:** ≤ 50 ms
* **Interactive:** ≤ 5 s
* These targets help decide when to use CSS, JS, or deferred work.

---

## Avoid Non‑Composited Animations to Reduce CLS

* **Non‑composited animations**: Trigger style/layout/paint, leading to CLS.

* **Composited animations**: Use only:

  ```css
  transform, opacity
  ```

  Avoid triggering reflow/layout. These are excluded from CLS calculations.

* **How to ensure composited animations:**

    * Only use `transform` and `opacity`
    * Keep layer count low
    * Use DevTools to debug
    * Reference:

        * [Lighthouse non-composited animations](https://developer.chrome.com/docs/lighthouse/performance/non-composited-animations)

---

## Using AnimeJS for Fallback & Complex Motion

* **Use CSS animations first.**

  ```css
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  ```

* **Use AnimeJS** when:

    * You need chaining, sequencing, interactivity.

  ```js
  import anime from 'animejs';

  anime({
    targets: '.box',
    translateX: 250,
    opacity: 0.5,
    duration: 800,
    easing: 'easeInOutQuad'
  });
  ```

* **Ensure fallback.**

    * Load AnimeJS conditionally.
    * Use `prefers-reduced-motion` media query:

      ```css
      @media (prefers-reduced-motion: reduce) {
        * {
          animation: none !important;
          transition: none !important;
        }
      }
      ```

---

## General Best Practices for Motion & Animation

* **Plan around user perception.** Use RAIL time budgets.
* **Minimize file sizes.** Lazy‑load assets.
* **Respect user preferences.**

    * Honor `prefers-reduced-motion`.
* **Test on real devices.** Use DevTools, Lighthouse, throttling.
* **Document fallback behavior.** Be explicit about what happens if JS fails or motion is disabled.

---

By following these instructions, your team can deliver smooth, SEO‑friendly animations that respect users’ devices and preferences while maintaining high performance and minimal layout shifts.

---

## Resources

* [High‑Performance CSS Animations — Web.dev](https://web.dev/articles/animations-guide)
* [RAIL Model — Web.dev](https://web.dev/articles/rail/)
* [Non-Composited Animations — Chrome Dev](https://developer.chrome.com/docs/lighthouse/performance/non-composited-animations)

