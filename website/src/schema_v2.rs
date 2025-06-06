use chrono::{DateTime, Utc};
use schemars::{schema_for, JsonSchema, SchemaGenerator};
use schemars::schema::{Schema, SchemaObject};
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;
use url::Url;


fn textarea_widget_schema(g: &mut SchemaGenerator) -> Schema {
    let mut schema: SchemaObject = String::json_schema(g).into_object();

    // add our extra key
    schema
        .extensions
        .insert("widget".to_string(), json!("textarea"));

    Schema::Object(schema)
}
#[allow(dead_code)]
fn select_widget_schema(g: &mut SchemaGenerator) -> Schema {
    let mut schema: SchemaObject = String::json_schema(g).into_object();
    schema.extensions.insert("widget".to_string(), json!("select"));
    Schema::Object(schema)
}



#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Page {
    #[schemars(skip)]
    pub id: Option<Thing>,
    pub metadata: SeoMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[allow(dead_code)]
pub enum Category {
    #[schemars(title = "Sci-Fi")]
    SciFi,
    #[schemars(title = "Western")]
    Western,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SeoMetadata {
    // ─── core -----------------------------------------------------------
    #[schemars(schema_with = "textarea_widget_schema")]
    pub title:        Option<String>,
    #[schemars(schema_with = "textarea_widget_schema")]
    pub description:  Option<String>,
    pub canonical:    Option<Url>,
    pub viewport:     Option<String>,

    // ─── hero images ----------------------------------------------------
    #[serde(default)]
    pub images:       Vec<Url>,              // 0 … n images, default = empty

    // ─── publication data ----------------------------------------------
    pub published_time:  Option<DateTime<Utc>>,
    pub modified_time:   Option<DateTime<Utc>>,
    pub expiration_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub authors:     Vec<Url>,
    pub section:     Option<String>,
    #[serde(default)]
    pub tags:        Vec<String>,

    // ─── advanced blocks -----------------------------------------------
    pub robots:      Option<RobotsMeta>,
    pub open_graph:  Option<OpenGraph>,
    pub twitter:     Option<TwitterCard>,
    #[serde(default)]
    pub alternates:  Vec<Hreflang>,
    pub schema_org:  Option<serde_json::Value>,
}

// ─── TWITTER CARD ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwitterCard {
    pub card:        Option<TwitterCardType>,
    pub site:        Option<String>,
    pub creator:     Option<String>,
    pub title:       Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub images:      Vec<Url>,
    pub image_alt:   Option<String>,
    pub player:      Option<TwitterPlayer>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum TwitterCardType {
    Summary,
    SummaryLargeImage,
    App,
    Player,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwitterPlayer {
    pub url: Url,
    pub width: u32,
    pub height: u32,
    pub stream: Option<Url>,
}

// ─── ROBOTS ─────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RobotsMeta {
    pub index:              Option<bool>,
    pub follow:             Option<bool>,
    pub archive:            Option<bool>,
    pub max_snippet:        Option<u32>,
    pub max_image_preview:  Option<ImagePreviewSize>,
    pub max_video_preview:  Option<u32>,
}
impl Default for RobotsMeta {
    fn default() -> Self {
        Self {
            index: Some(true),
            follow: Some(true),
            archive: Some(true),
            max_snippet: None,
            max_image_preview: None,
            max_video_preview: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ImagePreviewSize {
    None,
    Standard,
    Large,
}

// ─── I18N / HREFLANG ────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Hreflang {
    pub lang: String,
    pub url: Url,
}

// ─── OPEN GRAPH ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OpenGraph {
    pub og_type: OgType,
    pub url: Option<Url>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub locale: Option<String>,
    pub site_name: Option<String>,
    #[serde(default)]
    pub images: Vec<OgImage>,
    #[serde(default)]
    pub videos: Vec<OgVideo>,
    pub article: Option<OgArticle>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum OgType {
    Website,
    Article,
    Video,
    Music,
    Book,
    Profile,
    Other(String),
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OgImage {
    pub url: Url,
    pub secure_url: Option<Url>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub alt: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OgVideo {
    pub url: Url,
    pub secure_url: Option<Url>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OgArticle {
    pub published_time: Option<DateTime<Utc>>,
    pub modified_time: Option<DateTime<Utc>>,
    pub expiration_time: Option<DateTime<Utc>>,
    pub authors: Vec<Url>,
    pub section: Option<String>,
    pub tags: Vec<String>,
}

// endregion ------ SEO METADATA --------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn schema_contains_metadata_property() {
        let schema = schema_for!(Page);
        let value = serde_json::to_value(schema).unwrap();
        // Ensure the generated schema has a "metadata" field at the top level
        let props = value
            .get("properties")
            .expect("schema should have properties");
        assert!(
            props.get("metadata").is_some(),
            "metadata field missing from schema"
        );
    }

    #[test]
    fn sample_page_serializes_against_schema() {
        // A minimal Page instance
        let page = Page {
            id: None,
            metadata: SeoMetadata {
                /* core --------------------------------------------------------- */
                title:        Some("Hello".into()),
                description:  Some("desc".into()),
                canonical:    Some("https://example.com".parse().unwrap()),
                viewport:     None,

                /* hero images -------------------------------------------------- */
                images:       vec![],

                /* publication data -------------------------------------------- */
                published_time:  None,
                modified_time:   None,
                expiration_time: None,
                authors:         vec![],
                section:         None,
                tags:            vec![],

                /* advanced ----------------------------------------------------- */
                robots:      Some(RobotsMeta::default()),
                open_graph:  None,
                twitter:     None,
                alternates:  vec![],
                schema_org:  None,
            },
        };


        // Serialize to JSON; ensure it round-trips (basic smoke test)
        let json = serde_json::to_value(&page).unwrap();
        let page_back: Page = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(page_back.metadata.title.as_deref(), Some("Hello"));

        // Extra sanity: JSON object has `metadata` key with `title`
        assert_eq!(json!({"metadata": {"title": "Hello", "description":"desc","robots":{"index":true,"follow":true,"archive":true}}})["metadata"]["title"], "Hello");
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use schemars::schema_for;

        #[test]
        fn title_field_has_widget_extension() {
            let schema = schema_for!(SeoMetadata);
            let v      = serde_json::to_value(schema).unwrap();
            assert_eq!(v["properties"]["title"]["widget"], "textarea");
            assert_eq!(v["properties"]["description"]["widget"], "textarea");
        }
    }
}

