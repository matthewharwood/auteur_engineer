#[cfg(test)]
mod html_snapshots {
    use super::*;
    use insta::assert_snapshot;
    use tera::{Context, Tera};
    use crate::schema_v2::{Page, RobotsMeta, SeoMetadata};

    /// Render the SEO macro and snapshot the HTML.
    #[test]
    fn seo_macro_renders_expected_html() {
        /* ---- build the same Page instance --------------------------- */
        let page = Page {
            id: None,
            metadata: SeoMetadata {
                title:        Some("Hello".into()),
                description:  Some("desc".into()),
                canonical:    Some("https://example.com".parse().unwrap()),
                viewport:     None,
                images:       vec![],
                published_time:  None,
                modified_time:   None,
                expiration_time: None,
                authors:         vec![],
                section:         None,
                tags:            vec![],
                robots:      Some(RobotsMeta::default()),
                open_graph:  None,
                twitter:     None,
                alternates:  vec![],
                schema_org:  None,
            },
        };

        /* ---- minimal in-memory Tera env ----------------------------- */
        let mut tera = Tera::default();
        
        // 1. bring in the macro file
        tera.add_raw_template(
            "macros/seo.html",
            include_str!("templates/seo/macros.html"),
        )
            .unwrap();

        // 2. a one-liner template that just calls the macro
        tera.add_raw_template(
            "snippet.html",
            r#"{% import "macros/seo.html" as macros -%}
           {{ macros::seo(meta=meta) -}}
        "#,
        ).unwrap();

        /* ---- render ------------------------------------------------- */
        let mut ctx = Context::new();
        ctx.insert("meta", &page.metadata);

        let html = tera.render("snippet.html", &ctx).unwrap();

        /* ---- snapshot ---------------------------------------------- */
        assert_snapshot!("seo_macro__minimal_case", html);
    }
}
