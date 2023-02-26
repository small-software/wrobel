use stylist::{css, StyleSource};
use stylist::yew::Global;
use yew::prelude::*;


#[function_component]
pub fn Grid() -> Html {
    // base on https://www.youtube.com/watch?v=eu4TRpAhTpM
    let s: StyleSource = css!(r#"
                body {
                    font-family: 'Open Sans', sans-serif;
                    color: #141414;
                    text-align: center;
                }
                .row {
                    margin:0px;
                    padding:0px;
                }
                w-header {
                    padding: 1rem 2rem 4rem;
                }

                .w-container {
                    max-width: 1200px;
                    margin: 0 auto;
                }
                .w-row {
                    display:flex;
                    gap: 10px;
                    margin-bottom: 10px;
                }
                .w-column {
                    background-color: lightgray;
                    padding: 1rem 1rem;
                    flex: 1;
                }
                .w-col-two-thirds {
                    flex: 2;
                }
                .w-content-area {
                    width: 100%;
                    display: flex;
                    flex-wrap: wrap;
                    justify-content: center;
                    color: white;
                    background: black;
                }

                @media only screen and (max-width: 768px) {
                    .w-row {
                        flex-wrap: wrap;
                    }
                    .w-column {
                        flex-basis: 100%;
                    }
                }
            "#);

    html! {
        <Global css={s} />
    }
}