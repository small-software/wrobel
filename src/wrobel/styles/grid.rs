use stylist::{css, StyleSource};
use stylist::yew::Global;
use yew::prelude::*;


#[function_component]
pub fn Grid() -> Html {
    let s: StyleSource = css!(r#"
                body {
                    font-family: 'Open Sans', sans-serif;
                    color: #141414;
                    text-align: center;
                    margin:0px;
                }

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }
                .row {
                    display:flex;
                    gap: 10px;
                    margin-bottom: 10px;
                }
                .column {
                    background-color: lightgray;
                    padding: 1rem 1rem;
                    flex: 1;
                }
                .col-two-thirds {
                    flex: 2;
                }
                .content-area {
                    margin: 0 auto;
                    width: 100%;
                    display: flex;
                    flex-wrap: wrap;
                    justify-content: center;
                }

                @media only screen and (max-width: 768px) {
                    .row {
                        flex-wrap: wrap;
                    }
                    .column {
                        flex-basis: 100%;
                    }
                }
            "#);

    html! {
        <Global css={s} />
    }
}