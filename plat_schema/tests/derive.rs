use plat_schema::Schema;
use plat_schema_macros::PlatSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PlatSchema)]
struct Post {
    id: Option<u32>,
    title: String,
}

#[test]
fn name_returns_struct_name() {
    assert_eq!(Post::name(), "Post");
}
