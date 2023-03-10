
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
                display: -webkit-box;           /* OLD - iOS 6-, Safari 3.1-6 */
                display: -moz-box;              /* OLD - Firefox 19- (doesn't work very well) */
                display: -ms-flexbox;           /* TWEENER - IE 10 */
                display: -webkit-flex;          /* NEW - Chrome */
                display: flex;                  /* NEW, Spec - Opera 12.1, Firefox 20+ */
            }

            .main-content {
              -webkit-box-ordinal-group: 2;   /* OLD - iOS 6-, Safari 3.1-6 */
              -moz-box-ordinal-group: 2;      /* OLD - Firefox 19- */
              -ms-flex-order: 2;              /* TWEENER - IE 10 */
              -webkit-order: 2;               /* NEW - Chrome */
              order: 2;                       /* NEW, Spec - Opera 12.1, Firefox 20+ */

              width: 60%;                     /* No flex here, other cols take up remaining space */

              -moz-box-flex: 1;               /* Without this, Firefox 19- expands to widest paragraph, overrides width */

              background: white;
            }

            .main-nav {
              -webkit-box-ordinal-group: 1;   /* OLD - iOS 6-, Safari 3.1-6 */
              -moz-box-ordinal-group: 1;      /* OLD - Firefox 19- */
              -ms-flex-order: 1;              /* TWEENER - IE 10 */
              -webkit-order: 1;               /* NEW - Chrome */
              order: 1;                       /* NEW, Spec - Opera 12.1, Firefox 20+ */

              -webkit-box-flex: 1;            /* OLD - iOS 6-, Safari 3.1-6 */
              -moz-box-flex: 1;               /* OLD - Firefox 19- */
              width: 20%;                     /* For old syntax, otherwise collapses. */
              -webkit-flex: 1;                /* Chrome */
              -ms-flex: 1;                    /* IE 10 */
              flex: 1;                        /* NEW, Spec - Opera 12.1, Firefox 20+ */

              background: #ccc;
            }

            .main-sidebar {
              -webkit-box-ordinal-group: 3;   /* OLD - iOS 6-, Safari 3.1-6 */
              -moz-box-ordinal-group: 3;      /* OLD - Firefox 19- */
              -ms-flex-order: 3;              /* TWEENER - IE 10 */
              -webkit-order: 3;               /* NEW - Chrome */
              order: 3;                       /* NEW, Spec - Opera 12.1, Firefox 20+ */

              -webkit-box-flex: 1;            /* OLD - iOS 6-, Safari 3.1-6 */
              -moz-box-flex: 1;               /* Firefox 19- */
              width: 20%;                     /* For OLD syntax, otherwise collapses. */
              -ms-flex: 1;                    /* TWEENER - IE 10 */
              -webkit-flex: 1;                /* NEW - Chrome */
              flex: 1;                        /* NEW, Spec - Opera 12.1, Firefox 20+ */

              background: #ccc;
            }

            .main-content,
            .main-sidebar,
            .main-nav {
              padding: 1em;
            }

            body {
              padding: 2em;
              background: #79a693;
            }
            * {
              -webkit-box-sizing: border-box;
              -moz-box-sizing: border-box;
              box-sizing: border-box;
            }

            h1, h2 {
              font: bold 2em Sans-Serif;
              margin: 0 0 1em 0;
            }
            h2 {
              font-size: 1.5em;
            }
            p {
              margin: 0 0 1em 0;
            }"#);

    html! {
        <>
        <Global css={s} />
        //class={css!(r#""#)}
        <div class="page-wrap">

            <nav class="main-nav" role="navigation">
                {props.links.to_owned()}
            </nav>
            <section class="main-content" role="main">
                {props.main_content.to_owned()}
            </section>
            <aside class="main-sidebar" role="complementary">
                {props.sidebar.to_owned()}
            </aside>
        </div>
        </>
    }
}