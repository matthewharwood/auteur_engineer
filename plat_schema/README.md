# plat_schema

I'm trying to make a few macros that will be used to create sql queries that write to a surrealdb using only rust type.
It will be deeply similar to Sanity's data types as a lot of the fields of a struct will be decorated with metadata that I will use to additionally make form fields in html using tera

TODO:
- Give me a name for my Schema macro that is terse and still readable 

Here we have some types
```rust
#[derive(Serialize, Deserialize, Schema)]
    pub struct Post {
    pub id: Option<Thing>,
    pub title: InputField,
    pub blocks: Vec<Block>,
}
pub struct InputField {
    pub label: String,
    pub hint: String,
}

pub struct InputArea {
    pub label: String,
    pub hint: String,
}
pub struct InputDate {
    pub label: String,
    pub hint: String,
}
   
pub enum Block {
    Header(Header),
    Footer(Footer),
}

#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Header {
    pub content: InputArea,
}
#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Footer {
    pub copyright: InputDate,
}
```





