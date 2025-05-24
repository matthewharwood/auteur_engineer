**Instructions:**
I'm building a fullstack web Rustlang web framework.
- Server First
- Progressive enhancement
- axum
- html
- css
- js
- rust nightly
- Keats/tera for templating
- Web Components

Knowledge includes "Framework Notes.txt" that has pseudo-code for an e2e fullstack example for a blog. Any future request please consider the fullstack that uses:
- Handlers
- Gateways
- Adapters
- Views
- Components

**1. Handlers**
Request entrypoint for the web application. HTTP endpoints exposed by your blog (e.g., `/`, `/posts/{post_slug}`, `/admin/new-post`) will be mapped to specific handler functions. In Axum, these are typically asynchronous functions that take extractors (like `State`, `Path`, `Query`, `Form`, `Json`) as arguments and return a type that implements `IntoResponse`.

Directory Structure (Suggestion): `src/handlers/`
* `src/handlers/blog_handlers.rs` (for public-facing blog routes)
* `src/handlers/admin_handlers.rs` (for administrative routes)
* `src/handlers/mod.rs`

Responsibilities of a Handler:
1. Interpret Incoming HTTP Request: Extract data using Axum extractors (`Path(slug)`, `Query(params)`, `Form(data)`, `Json(payload)`, `Headers`).
2. Call Gateways: Determine which Gateway methods to invoke to interact with the database.
3. Process Gateway Response:
    * For API endpoints, might directly return a JSON response.
    * For HTML rendering, pass data to an Adapter or directly to a View.
4. Return a Response:
    * HTML pages: Call a View's render method.
    * API endpoints: Serialize data to JSON.

Example: Handler for Displaying a Single Blog Post
```rust
// src/handlers/blog_handlers.rs
use std::sync::Arc;
use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tera::Context as TeraContext; // Or keats::Context
use tracing::{error, instrument};
use uuid::Uuid;
use crate::{
    AppState,
    gateways::blog_gateway::{BlogGateway, BlogPostError, BlogPost},
    views,
    adapters::blog_adapter,
};

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub trace_id: Uuid,
}

#[instrument(skip(app_state, req_ctx))]
pub async fn get_blog_post_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(req_ctx): Extension<Arc<RequestContext>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match app_state.blog_gateway.get_post_by_slug(&slug).await {
        Ok(Some(post)) => {
            match blog_adapter::adapt_blog_post_to_view_model(&post) {
                Ok(view_model) => {
                    let mut tera_ctx = TeraContext::new();
                    tera_ctx.insert("post", &view_model);
                    tera_ctx.insert("trace_id", &req_ctx.trace_id.to_string());
                    match app_state.templates.render("post_detail.html", &tera_ctx) {
                        Ok(rendered_html) => Html(rendered_html).into_response(),
                        Err(e) => {
                            error!("Template rendering error for slug {}: {:?}", slug, e);
                            (StatusCode::INTERNAL_SERVER_ERROR, "Error rendering page").into_response()
                        }
                    }
                }
                Err(e) => {
                    error!("Error adapting blog post with slug {}: {:?}", slug, e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Error processing post data").into_response()
                }
            }
        }
        Ok(None) => {
            match app_state.templates.render("404.html", &TeraContext::new()) {
                Ok(rendered_html) => (StatusCode::NOT_FOUND, Html(rendered_html)).into_response(),
                Err(e) => {
                    error!("Template rendering error for 404 page: {:?}", e);
                    (StatusCode::NOT_FOUND, "Page not found").into_response()
                }
            }
        }
        Err(e) => {
            error!("Database error fetching post with slug {}: {:?}", slug, e);
            match app_state.templates.render("error.html", &TeraContext::new()) {
                Ok(rendered_html) => (StatusCode::INTERNAL_SERVER_ERROR, Html(rendered_html)).into_response(),
                Err(e_render) => {
                    error!("Template rendering error for error page: {:?}", e_render);
                    (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred").into_response()
                }
            }
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct CreatePostPayload {
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
}

#[derive(serde::Serialize, Debug)]
pub struct CreatePostResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
}

#[instrument(skip(app_state, payload))]
pub async fn create_blog_post_api_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(_req_ctx): Extension<Arc<RequestContext>>, // req_ctx often used for logging/auth
    axum::Json(payload): axum::Json<CreatePostPayload>,
) -> impl IntoResponse {
    if payload.title.is_empty() || payload.content.is_empty() {
        return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "Title and content are required"}))).into_response();
    }
    match app_state.blog_gateway.create_post(&payload.title, &payload.content, payload.author_id).await {
        Ok(new_post) => {
            let response_data = CreatePostResponse {
                id: new_post.id,
                title: new_post.title,
                slug: new_post.slug,
            };
            (StatusCode::CREATED, axum::Json(response_data)).into_response()
        }
        Err(e) => {
            error!("Error creating blog post: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "Could not create post"}))).into_response()
        }
    }
}

pub fn blog_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/posts/:slug", get(get_blog_post_handler))
        .route("/api/posts", axum::routing::post(create_blog_post_api_handler))
        .with_state(app_state)
        .layer(axum::middleware::from_fn(inject_request_context))
}

async fn inject_request_context(mut req: axum::extract::Request, next: axum::middleware::Next) -> Result<Response, StatusCode> {
    let trace_id = Uuid::new_v4();
    let req_ctx = Arc::new(RequestContext { trace_id });
    req.extensions_mut().insert(req_ctx);
    Ok(next.run(req).await)
}
```

Application State (`AppState`):
Holds shared state like DB connection pool and template engine instance.
```rust
// src/main.rs (or a dedicated app_state.rs)
use sqlx::PgPool;
use tera::Tera; // Or keats::Keats
use std::sync::Arc;
use crate::gateways::blog_gateway::PostgresBlogGateway;

pub struct AppState {
    pub db_pool: PgPool,
    pub templates: Tera, // Or Keats
    pub blog_gateway: Arc<PostgresBlogGateway>,
}
```

**2. Gateways**
Abstract data sources (e.g., PostgreSQL). Provide a clean API for handlers.

Directory Structure (Suggestion): `src/gateways/`
* `src/gateways/blog_gateway.rs`
* `src/gateways/mod.rs`

Responsibilities:
1. Define Data Access Methods: CRUD operations.
2. Interact with Database: Use `sqlx` for queries.
3. Map Results to Structs: Convert DB rows to Rust structs.
4. Handle Database Errors.
   Gateways should NOT be called from Adapters or Views.

Example: Blog Post Gateway
```rust
// src/gateways/blog_gateway.rs
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{error, instrument};
use std::sync::Arc;

#[derive(Debug, Clone, FromRow, serde::Serialize, serde::Deserialize)]
pub struct BlogPost {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author_id: Uuid,
}

#[derive(Debug, thiserror::Error)]
pub enum BlogPostError {
    #[error("Database query failed: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("Post not found")]
    NotFound,
    #[error("Failed to generate slug")]
    SlugGenerationError,
    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),
}

#[async_trait::async_trait]
pub trait BlogGateway: Send + Sync + 'static {
    async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>, BlogPostError>;
    async fn get_all_posts(&self, limit: i64, offset: i64) -> Result<Vec<BlogPost>, BlogPostError>;
    async fn create_post(&self, title: &str, content: &str, author_id: Uuid) -> Result<BlogPost, BlogPostError>;
}

#[derive(Clone)]
pub struct PostgresBlogGateway {
    db_pool: Arc<PgPool>,
}

impl PostgresBlogGateway {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
    fn generate_slug(title: &str) -> String {
        title.to_lowercase().replace(' ', "-").chars().filter(|c| c.is_alphanumeric() || *c == '-').collect()
    }
}

#[async_trait::async_trait]
impl BlogGateway for PostgresBlogGateway {
    #[instrument(skip(self))]
    async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>, BlogPostError> {
        sqlx::query_as!(
            BlogPost,
            "SELECT id, title, slug, content, published_at, created_at, updated_at, author_id FROM blog_posts WHERE slug = $1",
            slug
        )
        .fetch_optional(self.db_pool.as_ref())
        .await
        .map_err(BlogPostError::from)
    }

    #[instrument(skip(self))]
    async fn get_all_posts(&self, limit: i64, offset: i64) -> Result<Vec<BlogPost>, BlogPostError> {
        sqlx::query_as!(
            BlogPost,
            "SELECT id, title, slug, content, published_at, created_at, updated_at, author_id FROM blog_posts WHERE published_at IS NOT NULL ORDER BY published_at DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(self.db_pool.as_ref())
        .await
        .map_err(BlogPostError::from)
    }

    #[instrument(skip(self, title, content))]
    async fn create_post(&self, title: &str, content: &str, author_id: Uuid) -> Result<BlogPost, BlogPostError> {
        let slug = Self::generate_slug(title);
        if slug.is_empty() {
            return Err(BlogPostError::SlugGenerationError);
        }
        let new_post = sqlx::query_as!(
            BlogPost,
            "INSERT INTO blog_posts (title, slug, content, author_id) VALUES ($1, $2, $3, $4) RETURNING id, title, slug, content, published_at, created_at, updated_at, author_id",
            title,
            slug,
            content,
            author_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(BlogPostError::from)?;
        Ok(new_post)
    }
}
```

**3. Adapters**
Intermediary layer transforming Gateway data into ViewModels for Views/templates. Encapsulate presentation logic.

Directory Structure (Suggestion): `src/adapters/`
* `src/adapters/blog_adapter.rs`
* `src/adapters/mod.rs`

Responsibilities:
1. Data Transformation: Convert Gateway structs (e.g., `BlogPost`) to ViewModels. Format dates, process Markdown, truncate text.
2. ViewModel Construction: Create and populate ViewModels for templates.
3. No Direct Rendering Logic: Prepare data for rendering, don't produce HTML.

Example: Blog Post Adapter
```rust
// src/adapters/blog_adapter.rs
use crate::gateways::blog_gateway::BlogPost;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BlogPostViewModel {
    pub title: String,
    pub slug: String,
    pub html_content: String,
    pub formatted_published_at: Option<String>,
    pub author_name: String, // Placeholder
}

#[derive(Debug, Serialize)]
pub struct BlogPostListItemViewModel {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub formatted_published_at: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("Failed to process markdown: {0}")]
    MarkdownError(String),
    #[error("Data formatting error")]
    FormattingError,
}

pub fn adapt_blog_post_to_view_model(post: &BlogPost) -> Result<BlogPostViewModel, AdapterError> {
    let mut html_output = String::new();
    // In a real scenario, use pulldown_cmark or similar for Markdown to HTML:
    // let parser = pulldown_cmark::Parser::new(&post.content);
    // pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output.push_str(&post.content); // Simplified: assuming content is HTML or plain text

    Ok(BlogPostViewModel {
        title: post.title.clone(),
        slug: post.slug.clone(),
        html_content: html_output,
        formatted_published_at: post.published_at.map(format_datetime),
        author_name: "Jane Doe".to_string(), // Placeholder: fetch actual author name
    })
}

pub fn adapt_blog_post_to_list_item_view_model(post: &BlogPost) -> Result<BlogPostListItemViewModel, AdapterError> {
    let summary = truncate_text(&post.content, 150);
    Ok(BlogPostListItemViewModel {
        title: post.title.clone(),
        slug: post.slug.clone(),
        summary,
        formatted_published_at: post.published_at.map(format_datetime),
    })
}

fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.format("%B %e, %Y").to_string()
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        text.chars().take(max_chars).collect::<String>() + "..."
    }
}
```

**4. Views (and ViewModels)**
Generate final HTML markup using Tera/Keats templates populated with ViewModels.

Directory Structure (Suggestion):
* `src/views/`
    * `src/views/blog_views.rs`
    * `src/views/mod.rs`
* `templates/`
    * `templates/base.html`
    * `templates/post_detail.html`
    * `templates/post_list.html`
    * `templates/components/post_list_item.html`
    * `templates/404.html`, `templates/error.html`
    * `templates/macros/forms.html`

Responsibilities:
1. Define ViewModels: Rust structs (`serde::Serialize`) for template context (often in adapters or `view_models` module).
2. HTML Templates (Tera/Keats): `.html` files with template syntax for data display, control flow, includes, macros, inheritance.
3. Render Method (Conceptual): Axum handlers often call `tera.render()`. Helper functions in views module can prepare `TeraContext`.

ViewModel examples: `BlogPostViewModel`, `BlogPostListItemViewModel` (see Adapters).

Tera/Keats Template Examples:
`templates/base.html` (Base Layout):
```html
{# templates/base.html #}
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}My Awesome Blog{% endblock title %}</title>
    <link rel="stylesheet" href="/static/css/style.css"> {% block head_extra %}{% endblock head_extra %}
</head>
<body>
    <header>
        <h1><a href="/">My Awesome Blog</a></h1>
        <nav>
            <a href="/">Home</a> <a href="/archive">Archive</a>
            {% if current_user %}<a href="/admin">Admin</a> <a href="/logout">Logout ({{ current_user.username }})</a>{% else %}<a href="/login">Login</a>{% endif %}
        </nav>
    </header>
    <main>{% block content %}<p>Welcome to the blog!</p>{% endblock content %}</main>
    <footer>
        <p>&copy; {{ now() | date(format="%Y") }} My Blog. Powered by Rust & Axum!</p>
        {% if trace_id %}<p style="font-size:0.8em; color: #777;">Trace ID: {{ trace_id }}</p>{% endif %}
    </footer>
    <script src="/static/js/main.js" defer></script> {% block scripts_extra %}{% endblock scripts_extra %}
</body>
</html>
```

`templates/post_detail.html` (Single Post View):
```html
{# templates/post_detail.html #}
{% extends "base.html" %}
{% block title %}{{ post.title }} - My Awesome Blog{% endblock title %}
{% block content %}
<article class="blog-post-full">
    <header>
        <h2>{{ post.title }}</h2>
        <p class="meta">Published by {{ post.author_name }}
            {% if post.formatted_published_at %}on <time datetime="{{ post.raw_published_at }}">{{ post.formatted_published_at }}</time>{% else %}(Draft){% endif %}
        </p>
    </header>
    <div class="post-content">{{ post.html_content | safe }}</div>
    <section class="comments-section">
        <h3>Comments</h3>
        <comment-list post-slug="{{ post.slug }}"></comment-list>
        <add-comment-form post-slug="{{ post.slug }}"></add-comment-form>
    </section>
</article>
{% endblock content %}
{% block scripts_extra %}
    <script src="/static/js/components/comment-list.js" type="module"></script>
    <script src="/static/js/components/add-comment-form.js" type="module"></script>
{% endblock scripts_extra %}
```
Note: `post.raw_published_at` needs to be added to `BlogPostViewModel`.

`templates/macros/forms.html` (Tera Macros):
```html
{# templates/macros/forms.html #}
{% macro input(name, label, type="text", value="", placeholder="") %}
<div class="form-group">
    <label for="field-{{ name }}">{{ label }}</label>
    <input type="{{ type }}" name="{{ name }}" id="field-{{ name }}" value="{{ value }}" placeholder="{{ placeholder }}">
</div>
{% endmacro %}

{% macro textarea(name, label, value="", placeholder="", rows="5") %}
<div class="form-group">
    <label for="field-{{ name }}">{{ label }}</label>
    <textarea name="{{ name }}" id="field-{{ name }}" placeholder="{{ placeholder }}" rows="{{ rows }}">{{ value }}</textarea>
</div>
{% endmacro %}
```

Using Macros (e.g., `admin/new_post.html`):
```html
{# templates/admin/new_post.html #}
{% extends "base.html" %}
{% import "macros/forms.html" as forms %}
{% block title %}New Blog Post{% endblock title %}
{% block content %}
<h2>Create New Blog Post</h2>
<form action="/admin/posts" method="POST" id="new-post-form">
    {{ forms::input(name="title", label="Title", placeholder="Enter post title") }}
    {{ forms::textarea(name="content", label="Content", placeholder="Write your blog post content here (Markdown supported)") }}
    <button type="submit">Create Post</button>
</form>
<script>
document.getElementById('new-post-form')?.addEventListener('submit', async function(event) {
    event.preventDefault();
    const formData = new FormData(this);
    const data = Object.fromEntries(formData.entries());
    // data.author_id = "current-user-uuid-from-somewhere"; // Add if needed
    try {
        const response = await fetch('/api/posts', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(data),
        });
        if (response.ok) {
            const result = await response.json();
            window.location.href = `/posts/${result.slug}`;
        } else {
            const errorResult = await response.json();
            alert(`Error: ${errorResult.error || 'Could not create post.'}`);
        }
    } catch (error) {
        console.error('Form submission error:', error);
        alert('An unexpected error occurred.');
    }
});
</script>
{% endblock content %}
```

**5. Web Components (Progressive Enhancement)**
Client-side encapsulation of HTML, CSS, JS. Server renders basic HTML; JS upgrades/injects Web Components.

Example: `<comment-list>` Web Component
`static/js/components/comment-list.js`:
```javascript
// static/js/components/comment-list.js
class CommentList extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.postSlug = this.getAttribute('post-slug');
        this._comments = [];
    }
    connectedCallback() {
        this.render();
        this.fetchComments();
        document.addEventListener('commentAdded', (event) => {
            if (event.detail.postSlug === this.postSlug) this.fetchComments();
        });
    }
    async fetchComments() {
        if (!this.postSlug) return;
        try {
            // const response = await fetch(`/api/posts/${this.postSlug}/comments`);
            // if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
            // this._comments = await response.json();
            // --- Mock Data ---
            await new Promise(resolve => setTimeout(resolve, 500));
            if (this.postSlug === "my-first-post") {
                this._comments = [
                    { id: 'c1', author: 'Alice', text: 'Great post!', timestamp: '2025-05-10T10:00:00Z' },
                    { id: 'c2', author: 'Bob', text: 'Very informative.', timestamp: '2025-05-10T11:30:00Z' },
                ];
            } else {
                this._comments = [{id: 'c3', author: 'Charlie', text: 'Nice read!', timestamp: '2025-05-11T09:00:00Z'}];
            }
            // --- End Mock Data ---
            this.render();
        } catch (error) {
            console.error('Failed to fetch comments:', error);
            this.shadowRoot.innerHTML = `<p>Error loading comments.</p><style>${this.getStyles()}</style>`;
        }
    }
    render() {
        const styles = this.getStyles();
        let commentsHtml = '<p>Loading comments...</p>';
        if (this._comments && this._comments.length > 0) {
            commentsHtml = `<ul>${this._comments.map(comment => `
                <li class="comment-item">
                    <strong class="comment-author">${comment.author}</strong>
                    <p class="comment-text">${comment.text}</p>
                    <small class="comment-timestamp">${new Date(comment.timestamp).toLocaleString()}</small>
                </li>`).join('')}</ul>`;
        } else if (this._comments) {
            commentsHtml = '<p>No comments yet. Be the first to comment!</p>';
        }
        this.shadowRoot.innerHTML = `<style>${styles}</style><div class="comment-list-container">${commentsHtml}</div>`;
    }
    getStyles() {
        return `
            :host { display: block; margin-top: 20px; font-family: sans-serif; }
            .comment-list-container { border: 1px solid #eee; padding: 15px; border-radius: 5px; background-color: #f9f9f9; }
            ul { list-style-type: none; padding: 0; }
            .comment-item { border-bottom: 1px dashed #ddd; padding: 10px 0; }
            .comment-item:last-child { border-bottom: none; }
            .comment-author { color: #333; font-weight: bold; }
            .comment-text { margin: 5px 0; color: #555; }
            .comment-timestamp { font-size: 0.8em; color: #777; }
        `;
    }
}
customElements.define('comment-list', CommentList);
```

`static/js/components/add-comment-form.js`:
```javascript
// static/js/components/add-comment-form.js
class AddCommentForm extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.postSlug = this.getAttribute('post-slug');
    }
    connectedCallback() {
        this.render();
        this.shadowRoot.querySelector('form').addEventListener('submit', this.handleSubmit.bind(this));
    }
    async handleSubmit(event) {
        event.preventDefault();
        const form = event.target;
        const commentText = form.elements.commentText.value;
        const authorName = form.elements.authorName.value;
        if (!commentText.trim() || !authorName.trim()) {
            alert('Author and comment text are required.');
            return;
        }
        try {
            // const response = await fetch(`/api/posts/${this.postSlug}/comments`, {
            //     method: 'POST',
            //     headers: { 'Content-Type': 'application/json' },
            //     body: JSON.stringify({ author: authorName, text: commentText }),
            // });
            // if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
            // const newComment = await response.json();
            // --- Mock API Call ---
            await new Promise(resolve => setTimeout(resolve, 300));
            console.log('Mock comment submitted:', { author: authorName, text: commentText, postSlug: this.postSlug });
            // --- End Mock API Call ---
            form.reset();
            document.dispatchEvent(new CustomEvent('commentAdded', {
                detail: { postSlug: this.postSlug /*, newComment */ }
            }));
        } catch (error) {
            console.error('Failed to submit comment:', error);
            alert('Error submitting comment.');
        }
    }
    render() {
        this.shadowRoot.innerHTML = `
            <style>
                :host { display: block; margin-top: 20px; font-family: sans-serif; }
                form { background-color: #f0f0f0; padding: 15px; border-radius: 5px; }
                .form-group { margin-bottom: 10px; }
                label { display: block; margin-bottom: 5px; font-weight: bold; }
                input[type="text"], textarea { width: calc(100% - 22px); padding: 10px; border: 1px solid #ccc; border-radius: 3px; box-sizing: border-box; }
                textarea { resize: vertical; }
                button { padding: 10px 15px; background-color: #007bff; color: white; border: none; border-radius: 3px; cursor: pointer; }
                button:hover { background-color: #0056b3; }
            </style>
            <form>
                <div class="form-group">
                    <label for="authorName">Your Name:</label>
                    <input type="text" id="authorName" name="authorName" required>
                </div>
                <div class="form-group">
                    <label for="commentText">Comment:</label>
                    <textarea id="commentText" name="commentText" rows="4" required></textarea>
                </div>
                <button type="submit">Submit Comment</button>
            </form>
        `;
    }
}
customElements.define('add-comment-form', AddCommentForm);
```

**6. Database Setup (PostgreSQL with sqlx)**
Dependencies in `Cargo.toml`:
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tera = "1"     # Or keats = "0.11"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
sqlx = { version = "0.7", features = [ "runtime-tokio-rustls", "postgres", "uuid", "chrono", "macros" ] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
tower-http = { version = "0.5", features = ["trace", "cors"] }
dotenvy = "0.15"
# pulldown-cmark = "0.9" # Optional for Markdown
# slug = "0.1"           # Optional for slugs
```

Database Migrations (`sqlx-cli`):
1. Install: `cargo install sqlx-cli`
2. Setup: `sqlx database setup --database-url postgres://user:pass@host/db_name`
3. Create migration: `sqlx migrate add create_blog_posts_table`

Example Migration (`migrations/{timestamp}_create_blog_posts_table.sql`):
```sql
-- migrations/{timestamp}_create_blog_posts_table.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE authors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE blog_posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    author_id UUID NOT NULL REFERENCES authors(id) ON DELETE CASCADE
);

CREATE INDEX idx_blog_posts_slug ON blog_posts(slug);
CREATE INDEX idx_blog_posts_author_id ON blog_posts(author_id);
CREATE INDEX idx_blog_posts_published_at ON blog_posts(published_at DESC NULLS LAST);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp_blog_posts
BEFORE UPDATE ON blog_posts
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TRIGGER set_timestamp_authors
BEFORE UPDATE ON authors
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();
```
Run migrations: `sqlx migrate run --database-url your_db_url`

**7. main.rs - Tying it all together**
```rust
// src/main.rs
use axum::{Router, Server};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tera::Tera;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;
use std::env;

mod handlers;
mod gateways;
mod adapters;
mod views;

use gateways::blog_gateway::{PostgresBlogGateway, BlogGateway};
use handlers::blog_handlers;

pub struct AppState {
    db_pool: sqlx::PgPool,
    templates: Tera,
    blog_gateway: Arc<dyn BlogGateway>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "example_blog=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to create Postgres connection pool");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");

    let templates = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Tera parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let blog_gateway_impl = Arc::new(PostgresBlogGateway::new(Arc::new(db_pool.clone())));
    let blog_gateway: Arc<dyn BlogGateway> = blog_gateway_impl; // Type erasure for AppState

    let app_state = Arc::new(AppState {
        db_pool,
        templates,
        blog_gateway,
    });

    let app = Router::new()
        .merge(blog_handlers::blog_router(app_state.clone()))
        // .fallback(handler_404) // Add global 404 if needed
        .layer(CorsLayer::new().allow_methods(tower_http::cors::Any).allow_origin(tower_http::cors::Any))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
    Ok(())
}

// Example global 404 handler:
// use axum::{extract::State, http::StatusCode, response::Html, response::IntoResponse};
// async fn handler_404(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
//     match app_state.templates.render("404.html", &tera::Context::new()) {
//         Ok(rendered_html) => (StatusCode::NOT_FOUND, Html(rendered_html)).into_response(),
//         Err(_) => (StatusCode::NOT_FOUND, "Page Not Found").into_response(),
//     }
// }
```