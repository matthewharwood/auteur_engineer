use serde::Serialize;

#[derive(Serialize)]
pub struct FieldSchema {
    pub name: &'static str,
    pub label: &'static str,
    pub form_type: &'static str,
}

#[derive(Serialize)]
pub struct BlockSchema {
    pub block_type: &'static str,
    pub fields: &'static [FieldSchema],
}

pub const HEADER_SCHEMA: BlockSchema = BlockSchema {
    block_type: "Header",
    fields: &[
        FieldSchema {
            name: "content",
            label: "Content",
            form_type: "InputArea",
        },
    ],
};

pub const FOOTER_SCHEMA: BlockSchema = BlockSchema {
    block_type: "Footer",
    fields: &[
        FieldSchema {
            name: "copyright",
            label: "Copyright",
            form_type: "InputText",
        },
    ],
};

pub const ALL_BLOCK_SCHEMAS: &[BlockSchema] = &[HEADER_SCHEMA, FOOTER_SCHEMA];

pub fn get_all_block_schemas() -> &'static [BlockSchema] {
    ALL_BLOCK_SCHEMAS
}
