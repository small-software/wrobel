use stylist::yew::styled_component;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub main_content: String,
    pub links: String,
    pub sidebar: String,
}

#[function_component]
pub fn TreeColumns(props: &Props) -> Html {
    html! {

        <div class={css!(r#"
                display: -webkit-box;      /* OLD - iOS 6-, Safari 3.1-6 */
                display: -moz-box;         /* OLD - Firefox 19- (buggy but mostly works) */
                display: -ms-flexbox;      /* TWEENER - IE 10 */
                display: -webkit-flex;     /* NEW - Chrome */
                display: flex;             /* NEW, Spec - Opera 12.1, Firefox 20+ */
        "#)}>
            <section class="main-content" role="main">
                {props.main_content.into()}
            </section>

            <nav class="main-nav" role="navigation">
                {props.links.into()}
            </nav>

            <aside class="main-sidebar" role="complementary">
                {props.sidebar.into()}
            </aside>
        </div>
    }
}