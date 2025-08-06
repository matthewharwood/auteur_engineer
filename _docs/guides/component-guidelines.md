# Light DOM Web Components Style Guide with "is" Extension

## Core Principles

### 1. Extended Built-in Elements
- **Use "is" syntax** - Extend native HTML elements for maximum compatibility
- **Progressive enhancement by default** - Works without JavaScript
- **Native element semantics** - Preserves accessibility and form participation
- **SEO-optimized** - Search engines see standard HTML elements

### 2. Tera Template Integration
- **Server-rendered markup** - Components hydrate from Tera output
- **Data attributes for state** - Pass initial data via `data-*` attributes
- **Template macros** - Reusable component markup patterns
- **Hybrid rendering** - SSR with client-side reactivity

## Component Architecture

### Base Component Classes

```javascript
import { signal, effect, computed } from '/signal.js';


// Base class for extended elements
class LightDOMElement {
  constructor() {
    super();
    this._mounted = false;
    this._effects = [];
    this._signals = new Map();
    this._computeds = new Map();
  }
  
  connectedCallback() {
    if (!this._mounted) {
      this._mounted = true;
      this.hydrateFromServer();
      this.setupSignals();
      this.enhance();
      this.bindEvents();
      this.onMount();
    }
  }
  
  disconnectedCallback() {
    this._effects.forEach(dispose => dispose());
    this._effects = [];
    this.onUnmount();
  }
  
  // Lifecycle hooks
  hydrateFromServer() {}
  setupSignals() {}
  enhance() {}
  bindEvents() {}
  onMount() {}
  onUnmount() {}
  
  // Signal management
  createSignal(name, initialValue) {
    const sig = signal(initialValue);
    this._signals.set(name, sig);
    return sig;
  }
  
  watchAndUpdate(signalOrComputed, updateFn) {
    const dispose = effect(() => {
      updateFn(signalOrComputed.value);
    });
    this._effects.push(dispose);
  }
  
  // Hydration helpers
  getServerData(key) {
    return this.dataset[key];
  }
  
  getServerJSON(key) {
    try {
      return JSON.parse(this.dataset[key] || 'null');
    } catch {
      return null;
    }
  }
}

// Element-specific base classes
class ExtendedButton extends HTMLButtonElement {
  constructor() {
    super();
    Object.assign(this, LightDOMElement.prototype);
    LightDOMElement.call(this);
  }
}

class ExtendedForm extends HTMLFormElement {
  constructor() {
    super();
    Object.assign(this, LightDOMElement.prototype);
    LightDOMElement.call(this);
  }
}

class ExtendedArticle extends HTMLElement {
  constructor() {
    super();
    Object.assign(this, LightDOMElement.prototype);
    LightDOMElement.call(this);
  }
}
```

### Registration Pattern

```javascript
const defineExtension = (name, ElementClass, extendsTag) => {
  if (!customElements.get(name)) {
    customElements.define(name, ElementClass, { extends: extendsTag });
  }
  return ElementClass;
};

// Usage
defineExtension('ld-button', ExtendedButton, 'button');
defineExtension('ld-form', ExtendedForm, 'form');
defineExtension('ld-article', ExtendedArticle, 'article');
```

## Tera Template Integration

### Component Macros

```jinja2
{# templates/components/button.html #}
{% macro ld_button(text="", variant="primary", count=0, disabled=false) %}
<button 
  is="ld-button"
  class="btn btn--{{ variant }}"
  data-variant="{{ variant }}"
  data-count="{{ count }}"
  {% if disabled %}disabled{% endif %}
>
  <span class="btn__text">{{ text }}</span>
  <span class="btn__count" data-count-display>{{ count }}</span>
</button>
{% endmacro %}

{# templates/components/form.html #}
{% macro ld_form(action="", method="post", csrf_token="") %}
<form 
  is="ld-form"
  action="{{ action }}"
  method="{{ method }}"
  data-enhanced="true"
>
  <input type="hidden" name="csrf_token" value="{{ csrf_token }}">
  {{ caller() }}
  <div class="form__status" data-status hidden></div>
</form>
{% endmacro %}

{# templates/components/card.html #}
{% macro ld_card(product) %}
<article 
  is="ld-card"
  class="card"
  data-product='{{ product | tojson | safe }}'
  data-id="{{ product.id }}"
  itemscope 
  itemtype="https://schema.org/Product"
>
  <img 
    class="card__image" 
    src="{{ product.image }}" 
    alt="{{ product.name }}"
    itemprop="image"
  >
  <h3 class="card__title" itemprop="name">{{ product.name }}</h3>
  <div class="card__price" itemprop="offers" itemscope itemtype="https://schema.org/Offer">
    <span itemprop="priceCurrency" content="{{ product.currency }}">
      {{ product.currency_symbol }}
    </span>
    <span itemprop="price" content="{{ product.price }}">
      {{ product.price }}
    </span>
  </div>
  <p class="card__description" itemprop="description">
    {{ product.description }}
  </p>
  <button 
    is="ld-add-to-cart" 
    class="card__action"
    data-product-id="{{ product.id }}"
  >
    Add to Cart
  </button>
</article>
{% endmacro %}
```

### Page Templates Using Components

```jinja2
{# templates/pages/products.html #}
{% import "components/card.html" as components %}
{% extends "base.html" %}

{% block content %}
<div class="products-grid">
  {% for product in products %}
    {{ components::ld_card(product=product) }}
  {% endfor %}
</div>

{# Initial state for client-side store #}
<script type="application/json" id="initial-state">
{
  "user": {{ user | tojson | safe }},
  "cart": {{ cart | tojson | safe }},
  "theme": "{{ theme }}"
}
</script>
{% endblock %}
```

## Component Implementation Examples

### Enhanced Button with Server State

```javascript
class LDButton extends ExtendedButton {
  hydrateFromServer() {
    // Read initial state from server
    this.initialCount = parseInt(this.getServerData('count') || '0');
    this.variant = this.getServerData('variant') || 'primary';
  }
  
  setupSignals() {
    this.count = this.createSignal('count', this.initialCount);
  }
  
  enhance() {
    // Add reactive updates to existing DOM
    const display = this.querySelector('[data-count-display]');
    if (display) {
      this.watchAndUpdate(this.count, (value) => {
        display.textContent = value;
        display.classList.toggle('has-count', value > 0);
      });
    }
  }
  
  bindEvents() {
    this.addEventListener('click', () => {
      this.count.value++;
    });
  }
}

defineExtension('ld-button', LDButton, 'button');
```

### Enhanced Form with Validation

```javascript
class LDForm extends ExtendedForm {
  hydrateFromServer() {
    this.endpoint = this.action;
    this.csrfToken = this.querySelector('[name="csrf_token"]')?.value;
  }
  
  setupSignals() {
    this.submitting = this.createSignal('submitting', false);
    this.errors = this.createSignal('errors', {});
  }
  
  enhance() {
    // Add client-side validation UI
    const statusEl = this.querySelector('[data-status]');
    
    this.watchAndUpdate(this.submitting, (isSubmitting) => {
      this.classList.toggle('is-submitting', isSubmitting);
      this.querySelectorAll('button[type="submit"]').forEach(btn => {
        btn.disabled = isSubmitting;
      });
    });
    
    this.watchAndUpdate(this.errors, (errors) => {
      // Display validation errors
      Object.entries(errors).forEach(([field, message]) => {
        const input = this.querySelector(`[name="${field}"]`);
        if (input) {
          input.classList.add('has-error');
          input.setAttribute('aria-invalid', 'true');
          
          let errorEl = input.parentElement.querySelector('.field-error');
          if (!errorEl) {
            errorEl = document.createElement('div');
            errorEl.className = 'field-error';
            input.parentElement.appendChild(errorEl);
          }
          errorEl.textContent = message;
        }
      });
    });
  }
  
  bindEvents() {
    this.addEventListener('submit', async (e) => {
      if (!this.dataset.enhanced) return;
      
      e.preventDefault();
      this.submitting.value = true;
      this.errors.value = {};
      
      try {
        const formData = new FormData(this);
        const response = await fetch(this.endpoint, {
          method: this.method,
          body: formData,
          headers: {
            'X-Requested-With': 'XMLHttpRequest'
          }
        });
        
        const data = await response.json();
        
        if (!response.ok) {
          this.errors.value = data.errors || {};
        } else {
          this.onSuccess(data);
        }
      } catch (error) {
        // Fallback to normal submission
        this.submit();
      } finally {
        this.submitting.value = false;
      }
    });
  }
  
  onSuccess(data) {
    // Override in subclasses
    this.dispatchEvent(new CustomEvent('ld:form:success', { 
      detail: data,
      bubbles: true 
    }));
  }
}

defineExtension('ld-form', LDForm, 'form');
```

### Product Card with Store Integration

```javascript
import { store } from './store.js';

class LDCard extends ExtendedArticle {
  hydrateFromServer() {
    this.product = this.getServerJSON('product');
    this.productId = this.getServerData('id');
  }
  
  setupSignals() {
    this.inCart = computed(() => 
      store.cart.value.some(item => item.id === this.productId)
    );
  }
  
  enhance() {
    const addButton = this.querySelector('[is="ld-add-to-cart"]');
    if (addButton) {
      this.watchAndUpdate(this.inCart, (isInCart) => {
        addButton.textContent = isInCart ? 'In Cart' : 'Add to Cart';
        addButton.disabled = isInCart;
      });
    }
  }
}

class LDAddToCart extends ExtendedButton {
  hydrateFromServer() {
    this.productId = this.getServerData('productId');
  }
  
  bindEvents() {
    this.addEventListener('click', async () => {
      const card = this.closest('[is="ld-card"]');
      const product = card?.product;
      
      if (product) {
        store.addToCart(product);
        
        // Optimistic update
        this.textContent = 'Added!';
        setTimeout(() => {
          this.textContent = 'In Cart';
        }, 1000);
      }
    });
  }
}

defineExtension('ld-card', LDCard, 'article');
defineExtension('ld-add-to-cart', LDAddToCart, 'button');
```

## State Management with SSR

### Store Initialization from Server

```javascript
// store.js
import { signal, computed } from '/signal.js';

class Store {
  constructor() {
    // Initialize from server-rendered state
    const initialState = this.getInitialState();
    
    this.user = signal(initialState.user);
    this.cart = signal(initialState.cart);
    this.theme = signal(initialState.theme);
    
    // Computed values
    this.cartCount = computed(() => this.cart.value.length);
    this.isAuthenticated = computed(() => !!this.user.value);
    
    // Sync theme with DOM
    this.watchTheme();
  }
  
  getInitialState() {
    const stateEl = document.getElementById('initial-state');
    if (stateEl) {
      try {
        return JSON.parse(stateEl.textContent);
      } catch {}
    }
    return { user: null, cart: [], theme: 'light' };
  }
  
  watchTheme() {
    effect(() => {
      document.documentElement.dataset.theme = this.theme.value;
    });
  }
  
  async addToCart(product) {
    // Optimistic update
    this.cart.value = [...this.cart.value, product];
    
    // Sync with server
    try {
      await fetch('/api/cart/add', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ productId: product.id })
      });
    } catch (error) {
      // Rollback on failure
      this.cart.value = this.cart.value.filter(p => p.id !== product.id);
    }
  }
}

export const store = new Store();
```