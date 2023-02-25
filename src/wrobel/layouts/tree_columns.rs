use stylist::{css, StyleSource};
use stylist::yew::Global;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub main_content: String,
    pub links: String,
    pub sidebar: String,
}

#[function_component]
pub fn TreeColumns(props: &Props) -> Html {
    let s: StyleSource = css!(r#"
            .page-wrap {
                display: -webkit-box;      /* OLD - iOS 6-, Safari 3.1-6 */
                display: -moz-box;         /* OLD - Firefox 19- (buggy but mostly works) */
                display: -ms-flexbox;      /* TWEENER - IE 10 */
                display: -webkit-flex;     /* NEW - Chrome */
                display: flex;             /* NEW, Spec - Opera 12.1, Firefox 20+ */
            }
            .main-content {
                width: 60%;
            }
            .main-nav,
            .main-sidebar {
                -webkit-box-flex: 1;      /* OLD - iOS 6-, Safari 3.1-6 */
                -moz-box-flex: 1;         /* OLD - Firefox 19- */
                width: 20%;               /* For old syntax, otherwise collapses. */
                -webkit-flex: 1;          /* Chrome */
                -ms-flex: 1;              /* IE 10 */
                flex: 1;                  /* NEW, Spec - Opera 12.1, Firefox 20+ */
            }"#);


    html! {
        <>
        <Global css={s} />
        //class={css!(r#""#)}
        <div class="page-wrap">
            <section class="main-content" role="main">
                {props.main_content.to_owned()}
            </section>

            <nav class="main-nav" role="navigation">
                {props.links.to_owned()}
            </nav>

            <aside class="main-sidebar" role="complementary">
                {props.sidebar.to_owned()}
            </aside>
        </div>
        </>
    }
}