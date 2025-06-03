# Auteur Engineer Examples

This repo contains a small Axum based website showing how to combine server-side rendering with progressive enhancement using Web Components and SurrealDB.

## Running

```
cargo run --package website
```

The server expects a SurrealDB instance running at `ws://127.0.0.1:8000` with the `root:root` credentials. The database and namespace `test` are created automatically.

## Counter Example Walkthrough

1. **Tera macro** – `templates/controls.html` defines `counter_form` which renders the HTML for a counter. The macro is used from `templates/counter.html`.
2. **Web components** – buttons use the `is` attribute with `extends` in the class definition. A single `counter-button` component handles increments and decrements via a `data-delta` attribute.
3. **jsAction attribute** – a button declares the event and method to invoke: `jsAction="click:change:counter-button"`. The component parses this attribute in `connectedCallback` to register the listener.
4. **Signals** – `useCounter` from `@preact/signals-core` stores per‑counter state. `<p is="art-counter-value">` displays the value reactively.
5. **WebSockets** – `<counter-ws>` connects to `/ws/counter/:id` and updates the signal whenever the server broadcasts a change.
6. **Forms** – the buttons live inside a standard HTML `<form>` so non‑JS clients still work. Submitting the form triggers SurrealDB updates via Axum handlers.

The pattern enables server first rendering with progressive enhancement: HTML works without JavaScript and becomes interactive once the scripts load.
