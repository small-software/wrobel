use yew::prelude::*;

#[function_component(MainLayout)]
pub fn main_layout() -> Html {
    html! {
        <div class="page-wrap">
            <section class="main-content" role="main">
                {"Main content: first in source order"}
            </section>

            <nav class="main-nav" role="navigation">
                {"Links"}
            </nav>

            <aside class="main-sidebar" role="complementary">
                {"Sidebar"}
            </aside>
        </div>
    }
}