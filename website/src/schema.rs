use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum FormType {
    InputArea,
    InputText,
    InputDate,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Field {
    pub label: &'static str,
    pub hint: &'static str,
    pub form_type: FormType,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Header {
    pub content: Field,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Footer {
    pub copyright: Field,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Block {
    Header(Header),
    Footer(Footer),
}

pub const TITLE_FIELD: Field = Field {
    label: "Title",
    hint: "Title of the page",
    form_type: FormType::InputText,
};

pub const HEADER_BLOCK: Block = Block::Header(Header {
    content: Field {
        label: "Header",
        hint: "",
        form_type: FormType::InputArea,
    },
});

pub const FOOTER_BLOCK: Block = Block::Footer(Footer {
    copyright: Field {
        label: "Copyright",
        hint: "",
        form_type: FormType::InputText,
    },
});

pub const PAGE_SCHEMA: &[Block] = &[HEADER_BLOCK, FOOTER_BLOCK];

