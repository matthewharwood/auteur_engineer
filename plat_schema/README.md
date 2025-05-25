# plat_schema

Provides the `Schema` trait and the `PlatSchema` derive macro.

```rust
use plat_schema::Schema;
use plat_schema_macros::PlatSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PlatSchema)]
struct Post {
    id: Option<u32>,
    title: String,
}

assert_eq!(Post::name(), "Post");
```
