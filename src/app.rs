use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::error_template::{AppError, ErrorTemplate};
use crate::page::about::AboutPage;
use crate::page::blog::BlogPage;
use crate::page::home::HomePage;
use crate::page::post::PostPage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.

    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/blog-2.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>

            <main class="flex flex-col min-h-screen">
                <NavBar/>
                <div class="flex flex-grow">
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/about" view=AboutPage/>
                        <Route path="/blog" view=BlogPage/>
                        <Route path="/blog/:id" view=PostPage/>
                    </Routes>
                </div>
                <Footer/>
            </main>
        </Router>
    }
}
