
# Define the tables
```
DEFINE TABLE posts  SCHEMAFULL PERMISSIONS NONE;
DEFINE TABLE blocks SCHEMAFULL PERMISSIONS NONE;


DEFINE FIELD id             ON blocks TYPE record<blocks>;
DEFINE FIELD body           ON blocks TYPE string;


DEFINE FIELD id             ON posts  TYPE record<posts>;
DEFINE FIELD title          ON posts  TYPE string;
DEFINE FIELD blocks         ON posts  TYPE array<record(blocks)> ;
```
# Creation of Records
```
-- blocks
CREATE blocks CONTENT { id: blocks:101, body: "First paragraph"  };
CREATE blocks CONTENT { id: blocks:102, body: "Second paragraph" };

-- posts 
CREATE posts CONTENT {
  id: posts:1,
  title: "Hello world",
  blocks: [blocks:101, blocks:102]        -- array of pointers
};

```
# Selection of Records
```
SELECT *, blocks.*         -- denormalised view
FROM posts                  -- or FROM posts:<id>
WHERE id = posts:1
FETCH blocks;               -- SurrealDB resolves each link

```
# Example output
```
{
  "id": "posts:1",
  "title": "Hello world",
  "blocks": [
    { "id": "blocks:101", "body": "First paragraph" },
    { "id": "blocks:102", "body": "Second paragraph" }
  ]
}
```

# Rust Handler
```rust


fn get_post() {
    let sql = "SELECT *, blocks.* FROM posts WHERE id = $id FETCH blocks";
    let mut conn = db.connect().await?;
    let res: Vec<PostRow> = conn.query(sql).bind(("posts:1",)).await?;
    let post_vm = adapt_post(res.remove(0))?;
}
```
# Post & Block Schema
All fields of a post will be used to create a CMS.  The CMS has 2 panels: Form Panel and Preview Panel. We will use Rust Types to define a JSONSchema
That will be used on the client to generate a form. Authoring content on that form will then be passed to backend to be saved and persisted.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<Thing>,
    pub title: Field,
    pub blocks: Vec<Block>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub label: String,
    pub hint: String,
    pub form_type: FormType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormType {
    Select,
    InputArea,
    InputText,
    InputDate,
}

pub enum HTMLElement {
    h1,
    h2,
    h3,
    h4,
    h5,
    h6,
    p,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Block {
    Header(Header),
    Footer(Footer),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub content: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footer {
    pub copyright: Field,
}

pub fn default_page_schema() -> Vec<Block> {
    vec![
        Block::Header(Header {
            content: Field {
                label: "Header".to_string(),
                hint: "".to_string(),
                form_type: FormType::InputArea,
            },
        }),
        Block::Footer(Footer {
            copyright: Field {
                label: "Copyright".to_string(),
                hint: "".to_string(),
                form_type: FormType::InputText,
            },
        }),
    ]
}
```