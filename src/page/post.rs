use crate::page::blog::Metadata;
use leptos::*;
use leptos_router::*;
#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
}

pub struct Post {
    content: String,
    metadata: Metadata,
}

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
    use pulldown_cmark::{html, Parser};
    use serde_yaml;
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::path::PathBuf;
    fn get_post_by_link(link: &str) -> Result<Post, ServerFnError> {
        let path = PathBuf::from("public/blog");

    }
        }}
