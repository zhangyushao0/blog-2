use leptos::*;
use leptos_router::*;
#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="navbar bg-base-200 flex justify-between">
            <div>
                <A href="/" class="btn btn-ghost text-5xl">
                    "🐙"
                </A>
                <span class="text-sm hidden sm:flex">"章鱼烧的小站"</span>
            </div>
            <div class="flex-1 justify-center">
                <div class="form-control">
                    <input
                        type="text"
                        placeholder="🔍搜索"
                        class="input input-bordered w-24 md:w-auto"
                    />
                </div>
            </div>
            <div class="dropdown dropdown-end">
                <div tabindex="0" role="button" class="btn btn-ghost sm:hidden">
                    "☰"
                </div>
                <ul
                    tabindex="0"
                    class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-24"
                >
                    <li>
                        <A href="/blog">"博客"</A>
                    </li>
                    <li>
                        <A href="/about">"关于我"</A>
                    </li>
                    <li>
                        <label class="swap">
                            <input type="checkbox"/>
                            <div class="swap-on">"🔆"</div>
                            <div class="swap-off">"🌙"</div>
                        </label>
                    </li>
                </ul>
            </div>
            <div class="hidden sm:flex">
                <ul class="menu menu-horizontal px-1">
                    <li>
                        <A href="/blog">"博文"</A>
                    </li>
                    <li>
                        <A href="about">"关于我"</A>
                    </li>
                    <li>
                        <label class="swap">
                            <input type="checkbox"/>
                            <div class="swap-on">"🔆"</div>
                            <div class="swap-off">"🌙"</div>
                        </label>
                    </li>
                </ul>
            </div>
        </div>
    }
}
