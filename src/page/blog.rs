use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize,Deserializer};
#[component]
pub fn BlogPage() -> impl IntoView {
    let (page,set_page) = create_signal(1);
    let blog = create_resource(move ||page.get(),get_blog);    
    view! {
        <div class=" flex-grow ">
            <div class="flex flex-col h-full bg-base-300 ">
                <div class="container mx-auto px-4 py-8 flex-grow">
                    <div class="flex flex-wrap bg-base-200 rounded-box h-full space-y-4 w-full">
                        <Transition fallback=move || {
                            view! {
                                <div class="flex justify-center text-center w-full">
                                    <span class="loading loading-spinner loading-md"></span>
                                </div>
                            }
                        }>
                            {blog
                                .get()
                                .map(|posts| match posts {
                                    Ok(posts) => {
                                        posts
                                            .into_iter()
                                            .map(|post| {
                                                view! {
                                                    <div class="flex-auto w-full card bg-base-200 shadow-xl">

                                                        <div class="card-body">
                                                            <A
                                                                class=" hover:underline"
                                                                href=format!("/blog/{}", post.link)
                                                            >
                                                                <h2 class="card-title">{post.title}</h2>
                                                                <h2 class="card-title text-sm font-light">{post.date}</h2>
                                                                <p>{post.summary}</p>
                                                            </A>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                            .collect_view()
                                    }
                                    Err(e) => {
                                        view! { <p>"Error: " {format!("{:?}", e)}</p> }.into_view()
                                    }
                                })}

                        </Transition>
                    </div>
                </div>
                <div class="join flex-none justify-center items-end">
                    <div class="mb-10">
                        <button
                            class="join-item btn"
                            on:click=move |_| {
                                set_page
                                    .update(|p| {
                                        if *p > 1 {
                                            *p -= 1;
                                        }
                                    })
                            }
                        >

                            "«"
                        </button>
                        <button class="join-item btn">{page}</button>
                        <button
                            class="join-item btn"
                            on:click=move |_| set_page.update(|p| *p += 1)
                        >
                            "»"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}


#[server(GetBlog, "/api")]
pub async fn get_blog(page:u32) -> Result<Vec<Metadata>, ServerFnError> {
    get_post_meta_by_page(page)
}





#[derive(Serialize, Debug,Clone)]
pub struct Metadata{
    title:String,
    date:String,
    summary:String,
    pub link:String
}

// 实现Deserialize，以便可以自定义反序列化过程
impl<'de> Deserialize<'de> for Metadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 首先，定义一个只包含`title`的临时结构体，因为这是我们将从YAML中直接反序列化的唯一字段
        #[derive(Deserialize)]
        struct TempMetadata {
            title: String,
            date:String,
            summary:String,
        }

        // 反序列化到临时结构体
        let temp_meta = TempMetadata::deserialize(deserializer)?;

        // 然后，基于title生成link
        let link = temp_meta.title.to_lowercase().replace(" ", "-") + "-"+ temp_meta.date.as_str();
        // 最后，返回填充完整的Metadata实例
        Ok(Metadata {
            title: temp_meta.title,
            date: temp_meta.date,
            summary: temp_meta.summary,
            link,
        })
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature="ssr")]{ 
        use pulldown_cmark::{html, Parser};
        use serde_yaml;
        use std::fs;
        use std::io::Read;
        use std::path::Path;
        use std::path::PathBuf;
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
        fn get_post_meta_by_page(page: u32) -> Result<Vec<Metadata>, ServerFnError> {
            let path = PathBuf::from("public/blog");
            let posts_per_page = 6usize;
            let mut posts: Vec<Metadata> = Vec::new();
        
            let entries = fs::read_dir(path)?
                .filter_map(Result::ok)
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
                .collect::<Vec<_>>();
        
            let start = (page as usize).saturating_sub(1) * posts_per_page;
            let end = start + posts_per_page;
        
            for entry in entries.into_iter().skip(start).take(posts_per_page) {
                let meta = get_post_mata_by_path(&entry.path())?;
                posts.push(meta);

                }
                Ok(posts)
            }
        
           
        }
    }


