# Atomic Design Styleguide for CSS and HTML

This styleguide explains how to build robust, consistent and maintainable user interfaces using Atomic Design principles and a living design system. Each section gives practical guidance ("do this") with working HTML and CSS examples. Citations reference Brad Frost's Atomic Design book.

## 1. Why a Design System?

Modern web projects must support many devices and screen sizes. Page-centric thinking fails because it treats each screen as a bespoke deliverable. A **design system** solves this by providing reusable patterns and a shared vocabulary. The benefits include:

- **Consistency** – Patterns look and behave the same everywhere, reducing cognitive load
- **Shared vocabulary** – Designers and developers speak the same language (e.g., "molecule," "organism")
- **Education & onboarding** – New team members learn how components work via living documentation
- **Testing and speed** – Patterns can be tested in isolation and composed quickly
- **Longevity** – A living style guide evolves with the product

👉 **Do this:** Treat your design system as a product. Document patterns with examples, code snippets and guidelines. Use consistent class names and document how patterns combine.

## 2. Atomic Design Methodology

Brad Frost proposes five hierarchical stages. You can move between parts and wholes, separating structure from content.

| Stage | What it is | Example elements |
|-------|-----------|------------------|
| **Atoms** | Basic HTML elements with unique properties and styling | Buttons, inputs, labels |
| **Molecules** | Simple groups of atoms forming functional units | Input + button group |
| **Organisms** | Groups of molecules/atoms forming a complex section | Header bar with logo and navigation |
| **Templates** | Layouts that arrange organisms to create page structure; content is generic | Blog page wireframe |
| **Pages** | Templates with real content and variations for testing | Article page with copy |

### 2.1 Atoms

Atoms should be well-named, self-contained and accessible. Use semantic HTML (e.g., `button`, `label`). Keep CSS minimal and avoid layout rules at this level.

```html
<!-- Button atom -->
<button class="btn btn--primary">Submit</button>
```

```css
/* Atoms: buttons */
.btn {
  font-family: inherit;
  font-size: 1rem;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  border: 1px solid transparent;
  cursor: pointer;
  transition: background 0.2s ease;
}

.btn--primary {
  background: #007acc;
  color: #fff;
}

.btn--primary:hover {
  background: #005ea8;
}
```

### 2.2 Molecules

A molecule combines atoms to form a unit of functionality. Use container classes to group atoms and manage alignment.

```html
<!-- Search form molecule: label, input and button atoms -->
<form class="search">
  <label class="search__label" for="q">Search:</label>
  <input class="search__input" id="q" type="text" placeholder="Type keywords" />
  <button class="btn btn--primary" type="submit">Go</button>
</form>
```

```css
/* Molecules: search form */
.search {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.search__label {
  font-weight: bold;
}

.search__input {
  flex: 1;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}
```

### 2.3 Organisms

Organisms are complex sections built from molecules and atoms. They define major regions of a page (header, footer, card group). Avoid content-specific styling; keep organisms flexible.

```html
<!-- Header organism: logo, nav, search -->
<header class="site-header">
  <h1 class="site-header__logo"><a href="/">MySite</a></h1>
  <nav class="site-header__nav">
    <ul class="menu">
      <li class="menu__item"><a href="#">Home</a></li>
      <li class="menu__item"><a href="#">Blog</a></li>
      <li class="menu__item"><a href="#">About</a></li>
    </ul>
  </nav>
  <form class="search">
    <label class="search__label" for="q-header">Search:</label>
    <input class="search__input" id="q-header" type="text" />
    <button class="btn btn--primary">Go</button>
  </form>
</header>
```

```css
/* Organisms: site header */
.site-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  background-color: #f7f7f7;
  border-bottom: 1px solid #ddd;
}

.site-header__logo a {
  color: #333;
  text-decoration: none;
  font-size: 1.5rem;
  font-weight: 700;
}

.site-header__nav .menu {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  gap: 1rem;
}

.site-header__nav .menu__item a {
  color: #333;
  text-decoration: none;
  font-weight: 500;
}

.site-header__nav .menu__item a:hover {
  text-decoration: underline;
}
```

### 2.4 Templates

Templates define structure and layout of pages; they do not include final copy. You can mark placeholders using `class="placeholder"` to show where content will go.

```html
<!-- Blog list template: organisms arranged in layout -->
<div class="blog-template">
  <header class="site-header">…</header>
  <main class="blog-template__content">
    <!-- Repeatable article organism placeholder -->
    <article class="article placeholder"></article>
    <article class="article placeholder"></article>
    <article class="article placeholder"></article>
  </main>
  <footer class="site-footer">…</footer>
</div>
```

```css
/* Templates: layout rules */
.blog-template__content {
  max-width: 60rem;
  margin: 0 auto;
  padding: 2rem;
  display: grid;
  gap: 2rem;
}

.placeholder {
  background-color: #fafafa;
  border: 1px dashed #ccc;
  min-height: 10rem;
}
```

### 2.5 Pages

Pages apply real content to templates. They illustrate how a pattern holds up with actual data, state changes and edge cases. Use your pattern library to assemble pages quickly.

👉 **Do this:** Test variations (long titles, missing images) on pages to ensure components are resilient. Don't hard-code content into organisms; supply content via server or CMS.

## 3. CSS Architecture and BEM Naming

Atomic design benefits from a disciplined CSS architecture. Follow these guidelines:

### 3.1 Use BEM naming

- **Block** – independent component (e.g., `card`)
- **Element** – part of a block (e.g., `card__title`)
- **Modifier** – variation (e.g., `card--featured`)

Using BEM keeps classes context-agnostic and avoids cascade issues.

```html
<article class="card card--featured">
  <h2 class="card__title">Featured article</h2>
  <p class="card__excerpt">This article is highlighted.</p>
</article>
```

```css
/* BEM example */
.card {
  border: 1px solid #ccc;
  padding: 1rem;
  border-radius: 4px;
}

.card__title {
  margin: 0 0 0.5rem;
  font-size: 1.25rem;
}

.card__excerpt {
  margin: 0;
  color: #555;
}

.card--featured {
  border-color: #007acc;
  background-color: #eef6fc;
}
```

### 3.2 Keep selectors shallow

Avoid deep nesting (`.header nav ul li a {}`) because it couples styles to markup. Use utility classes or BEM elements instead. Each class should be meaningful and reusable.

👉 **Do this:** Limit selectors to 2–3 levels deep, and don't style HTML tags directly (e.g., `.header a`), as that reduces flexibility.

### 3.3 Context-Agnostic but Contextual

Name patterns abstractly (e.g., `card`, not `feature-box`). At the same time, show patterns in context by including examples within layouts. Provide guidance on when and where to use them.

👉 **Do this:** Add "When to use" and "When not to use" sections for each pattern. Provide real-world examples (e.g., screenshot of a card used in a product page).

## 7. Additional Guidelines

- **Accessibility first** – Ensure patterns meet WCAG. Use semantic HTML, label controls, maintain colour contrast and enable keyboard navigation.
- **Performance matters** – Keep CSS lean. Use variables and design tokens to avoid duplication. Use `min()` and `max()` for responsive typography. Lazy-load large images.
- **Documentation** – Include design rationale, technical details and usage guidelines. Document dependencies and relationships between patterns (lineage).
- **Tooling** – Automate linting (Stylelint)

## Conclusion

By applying Atomic Design and maintaining a living design system, your team can produce consistent, scalable interfaces. Use atoms to build molecules and organisms, assemble templates and pages, and maintain everything within a documented pattern library. Keep the system evolving through governance, cross-team collaboration and a commitment to continual improvement.
