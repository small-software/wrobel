use stylist::{css, StyleSource};
use stylist::yew::Global;
use yew::prelude::*;


#[function_component]
pub fn Menu() -> Html {

    let s: StyleSource = css!(r#"
                .link-remove-decorator {
                    a:link { text-decoration: none; }
                    a:visited { text-decoration: none; }
                    a:hover { text-decoration: none; }
                    a:active { text-decoration: none; }
            }
            "#);

    html! {
        <Global css={s} />
    }
}