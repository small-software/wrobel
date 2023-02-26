use stylist::{css, StyleSource};
use stylist::yew::Global;
use yew::prelude::*;


#[function_component]
pub fn Btn() -> Html {

    let s: StyleSource = css!(r#"
                .btn {
                    padding: .3em .8em;
                    border: 1px solid #446d88;
                    background: #58a linear-gradient(#77a0bb, #58a);
                    /* border-radius: .2em; */
                    box-shadow: 0 .05em .25em gray;
                    color: white;
                    text-shadow: 0 -.05em .05em #335166;
                    font-size 125%;
                    line-height: 1.5;
                }
            "#);

    html! {
        <Global css={s} />
    }
}