use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <div class="hero bg-base-200 flex-grow">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold">"你好👋"</h1>
                    <p class="py-6">"这里是章鱼烧的小站"</p>
                </div>
            </div>
        </div>
    }
}
