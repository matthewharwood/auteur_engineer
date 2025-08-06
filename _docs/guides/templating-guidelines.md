# Tera Template Style Guide

A guide to the Tera template engine syntax, inspired by Jinja2 and Django templates.

## 1. Basic Template Syntax

Tera uses three types of delimiters:

| Purpose | Delimiters | Example |
|---------|------------|---------|
| Expressions (rendered value) | `{{ ... }}` | `{{ name }}` prints the value of name |
| Statements (logic/flow control) | `{% ... %}` | `{% if price < 10 %}...{% endif %}` |
| Comments | `{# ... #}` | `{# A comment #}` is ignored |

### Raw Blocks

Prevent Tera from rendering expressions:

```html
{% raw %}
  Hello {{ name }}
{% endraw %}
```
This renders literally as `Hello {{ name }}`.

### Whitespace Control

Remove whitespace around delimiters using `-`:

```html
{% set my_var = 2 -%}
{{ my_var }}
```
The `-` in `{% ... -%}` trims the newline after the statement. Also applies to `{{- ... -}}` and `{#- ... -#}`.

## 2. Data Structures

### Literals
- **Booleans**: `true`/`false` (case-insensitive)
- **Numbers**: Integers and floats
- **Strings**: Double quotes, single quotes, or backticks
- **Arrays**: `[1, 2, 3,]` (trailing comma allowed)

### Variables and Notation

Access nested fields with dot notation:
```html
{{ product.name }}
{{ products[0].price }}
```

**IMPORTANT: IDENTIFIERS CANNOT APPEAR AFTER THE DOT** - use literal indices or field names only.

Square-bracket notation allows dynamic indexing:
```html
{{ product['name'] }}
{{ product[my_field] }}
```
**THE INDEX MUST EVALUATE TO A STRING OR INTEGER - OTHERWISE TERA THROWS AN ERROR**

### Expressions

**MISUSING OPERATIONS ON NON-NUMERIC TYPES WILL ERROR**

- **Math**: `+`, `-`, `*`, `/`, `%` (precedence: `+`/`-` < `*`/`/`/`%`)
- **Comparisons**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Logical**: `and`, `or`, `not`
- **Concatenation**: `~` concatenates strings/numbers **TRYING TO CONCATENATE OTHER TYPES IS AN ERROR**
- **Membership**: `in`/`not in` tests existence in array/string/map

## 3. Manipulating Data During Rendering

### Assignments

```html
{% set greeting = "hello" %}
{% set sum = 1 + 4 %}
{% set copy = some_var %}
{% set list = [1, true, some_var | round] %}
```

Assignments inside loops/macros are scoped. **IF YOU NEED TO SET A VALUE GLOBALLY INSIDE A LOOP, USE `set_global`**:

```html
{% set_global total = 0 %}
{% for i in values %}
  {% set_global total = total + i %}
{% endfor %}
```

### Filters

Filters modify values using `|`:
```html
{{ name | lower | replace(from="doctor", to="Dr.") }}
```

**FILTERS OPERATE ON SPECIFIC TYPES - APPLYING TO INCOMPATIBLE TYPES CAUSES AN ERROR**

When combining filters with arithmetic, **FILTER APPLICATION HAS LOWEST PRECEDENCE**. Write `{{ a | length + 1 }}` instead of `{{ 1 + a | length }}`.

### Filter Blocks

```html
{% filter upper %}
  Hello
{% endfilter %}
```
Outputs: `HELLO`

## 4. Control Structures

### If Statements

```html
{% if price < 10 or always_show %}
  Price is {{ price }}.
{% elif price > 1000 and not rich %}
  That's expensive!
{% else %}
  N/A
{% endif %}
```

**EACH `if` MUST END WITH `{% endif %}`**

Undefined variables are falsy.

### For Loops

```html
{% for product in products %}
  {{ loop.index }}. {{ product.name }}
{% endfor %}
```

Loop variables:
- `loop.index`: Current iteration (1-indexed)
- `loop.index0`: Current iteration (0-indexed)
- `loop.first`: True on first iteration
- `loop.last`: True on last iteration

Iterate over maps: `{% for key, value in products %}`

### Loop Control

```html
{% for product in products %}
  {% if product.id == target_id %}{% break %}{% endif %}
  {{ loop.index }}. {{ product.name }}
{% endfor %}
```

### Include

```html
{% include "template.html" %}
```

**DO NOT CONCATENATE A DYNAMIC NAME WITH `~` - THIS CAUSES A COMPILATION ERROR**

- Use `ignore missing` to render nothing if file absent
- Provide candidates: `{% include ["custom/header.html", "header.html"] %}`
- **NEVER MIX `include` WITH `extends`**

### Macros

```html
{% macro input(label, type="text") %}
  <label>
    {{ label }}
    <input type="{{ type }}" />
  </label>
{% endmacro input %}
```

Import: `{% import "macros.html" as macros %}`

Call: `{{ macros::input(label="Name", type="text") }}`

**MACROS REQUIRE KEYWORD ARGUMENTS - POSITIONAL ARGUMENTS ARE NOT ALLOWED**

- Call same-file macros with `self` namespace: `{{ self::factorial(n=5) }}`
- **MACROS MUST BE DEFINED AT TOP LEVEL**
- **DO NOT USE GLOBAL TEMPLATE VARIABLES DIRECTLY INSIDE MACROS**

### Template Inheritance

Base template:
```html
<!DOCTYPE html>
<html>
<head>
  {% block head %}
    <link rel="stylesheet" href="style.css" />
    <title>{% block title %}{% endblock title %} – My Webpage</title>
  {% endblock head %}
</head>
<body>
  <div id="content">{% block content %}{% endblock content %}</div>
  <div id="footer">
    {% block footer %}
      &copy; Copyright 2008 by <a href="http://domain.invalid/">you</a>.
    {% endblock footer %}
  </div>
</body>
</html>
```

Child template:
```html
{% extends "base.html" %}

{% block title %}Index{% endblock title %}

{% block head %}
  {{ super() }}
  <style>.important { color: #336699; }</style>
{% endblock head %}

{% block content %}
  <h1>Index</h1>
  <p class="important">Welcome to my awesome homepage.</p>
{% endblock content %}
```

**THE `{% extends %}` TAG MUST BE THE FIRST STATEMENT IN THE CHILD FILE**

- Use `{{ super() }}` to include parent block content
- Content outside blocks in child templates is ignored

## 5. Built-in Utilities

### Key Filters

| Filter | Purpose | Example |
|--------|---------|---------|
| `lower`/`upper` | Change case | `{{ name \| upper }}` |
| `replace(from, to)` | Replace substring | `{{ name \| replace(from="Robert", to="Bob") }}` |
| `trim`/`trim_start`/`trim_end` | Remove whitespace | |
| `truncate(length, end)` | Truncate with suffix | |
| `join(sep)` | Join array elements | |
| `length` | Get length | |
| `reverse`/`sort`/`unique` | Array operations | |
| `slice(start, end)` | Slice arrays (negative indices allowed) | |
| `filter(attribute, value)` | Filter array items | |
| `safe` | Mark as safe **MUST BE THE FINAL FILTER** | |
| `default(value)` | Provide default **CHECKS ONLY EXISTENCE, NOT TRUTHINESS** | |

### Tests

Use in `if` statements:
```html
{% if my_number is odd %}Odd{% endif %}
{% if my_number is not odd %}Even{% endif %}
```

### Functions

```html
{{ url_for(name="home") }}
{% for i in range(end=5) %}...{% endfor %}
```

## 6. Passing Data from Server to Template

```rust
use tera::{Tera, Context};

fn main() {
    // Load templates from the `templates` directory
    let tera = Tera::new("templates/**/*").unwrap();
    
    // Build context with variables
    let mut context = Context::new();
    context.insert("title", "First Steps with Tera");
    context.insert("site_name", "example.com");
    context.insert("message", "Hello, world!");
    
    // Render the template
    let rendered = tera.render("base.html", &context).unwrap();
    println!("{}", rendered);
}
```

## 7. Common Pitfalls

**CRITICAL RULES TO AVOID ERRORS:**

- **MUST NOT** call macros without namespace/parentheses - use `namespace::macro_name()` with keyword arguments
- **ALWAYS** close control structures (`if`/`for`/`block`/`macro`) with appropriate `endif`/`endfor`/`endblock`/`endmacro`
- **DO NOT** dynamically build template paths in `{% include %}` - path must be literal
- **DO NOT** reference global variables directly inside macros
- **NEVER** mix `include` and `extends` in same template
- **BE CAREFUL** with `default` filter - it only tests existence, not truthiness (empty strings or 0 won't trigger default)
- **ENSURE** Context keys exactly match template variables - mismatched names cause errors
- **ENSURE** values exist when using variable indexing - non-string/non-integer index throws error