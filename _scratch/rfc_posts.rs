// I'm trying to make a few macros that will be used to create sql queries that write to a surrealdb using only rust type.
// It will be deeply similar to Sanity's data types as a lot of the fields of a struct will be decorated with metadata that I will use to additionally make form fields in html using tera
//
// TODO:
// - Give me a name for my Schema macro that is terse and still readable
//
// Here we have some types

#[derive(Serialize, Deserialize, Schema)]
pub struct Post {
    pub id: Option<Thing>,
    pub title: InputField,
    // pub blocks: Vec<Block>,
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

// A typed reference, serialized as:
// { "_type": "reference", "_ref": "<id>" }
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Reference<T> {
    #[serde(rename = "_type")]
    pub type_name: String, // always "reference"
    #[serde(rename = "_ref")]
    pub id: String, // the document ID
    #[serde(skip)]
    pub marker: std::marker::PhantomData<T>,
}

impl<T> Reference<T> {
    pub fn new(id: impl Into<String>) -> Self {
        Reference {
            type_name: "reference".into(),
            id: id.into(),
            marker: std::marker::PhantomData,
        }
    }
}

// BEGIN TRANSACTION;

// CREATE TABLE post SCHEMAFUL;
//   -- if this fails, nothing is created

// DEFINE FIELD title ON post TYPE object;
// DEFINE FIELD title.label ON post TYPE string;
// DEFINE FIELD title.hint  ON post TYPE string;

// COMMIT;
//
