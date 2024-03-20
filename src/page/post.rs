use crate::page::blog::Metadata;
use leptos::*;
use leptos_router::*;
#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let post_html = create_resource(||(), move|_| async move { get_post(id()).await });
    view! {
        <Transition fallback=move || {
            view! {
                <div class="flex justify-center text-center w-full">
                    <span class="loading loading-spinner loading-md"></span>
                </div>
            }
        }>
            <article class="prose mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none">
                <div inner_html=post_html
                    .get()
                    .map(|html| match html {
                        Ok(html) => html,
                        Err(e) => format!("Error: {}", e),
                    })></div>
            </article>
        </Transition>
    }
}


#[server(GetPost, "/api")]
pub async fn get_post(link: String) -> Result<String, ServerFnError> {
    get_post_by_link(&link)
        .map(|post| post.content)
        .map_err(|e| ServerFnError::new(e.to_string()))
}

pub struct Post {
    content: String,
    metadata: Metadata,
}

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
    use serde_yaml;
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::path::PathBuf;
    use pulldown_cmark::{Parser, Event, Tag, Options, html};

    fn get_post_mata_by_path(path: &Path) -> Result<Metadata, ServerFnError> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // 使用splitn来确保我们正确地分割YAML块和Markdown内容
        let parts: Vec<&str> = contents.splitn(3, "---").collect();
        if parts.len() >= 3 {
            // 此时，parts[1]应该包含YAML内容
            let yaml_block = parts[1]; // 获取YAML内容
            let meta: Metadata = serde_yaml::from_str(yaml_block)?;
            Ok(meta)
        } else {
            Err(ServerFnError::new("No YAML block found in file"))
        }
    }

    fn get_post_by_link(link: &str) -> Result<Post, ServerFnError> {
        let path = PathBuf::from("public/blog");
        let entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .collect::<Vec<_>>();
        let mut post: Option<Post> = None;
        
        for entry in entries {
            let meta = get_post_mata_by_path(&entry.path())?;
            if meta.link == link {
                let mut file = fs::File::open(entry.path())?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                post = Some(Post {
                    content: contents,
                    metadata: meta,
                });
                break;
            }

        }
        if let Some(mut post) = post {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TASKLISTS);
            options.insert(Options::ENABLE_SMART_PUNCTUATION);
            
            let parser = Parser::new_ext(&post.content, options);
            
            let mut in_yaml = true;
            let mut html_output = String::new();
            let events: Vec<Event> = parser
            .filter(|event| {
                match event {
                    Event::Start(Tag::BlockQuote) | Event::Start(Tag::Paragraph) if in_yaml => {
                        in_yaml = false;
                        true
                    }
                    _ if in_yaml => false,
                    _ => true,
                }
            })
            .collect();
        
        html::push_html(&mut html_output, events.into_iter());
            post.content = html_output;
            Ok(post)

        } else {
            Err(ServerFnError::new("Post not found"))
        }

    }
        }}
