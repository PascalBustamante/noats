use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, Debug, PartialEq)]
pub struct Props {
} 

#[styled_component()]
pub fn text_editor() -> HTML {
    let stylesheet = css!{#"

    "#};
    html! {
        <section class={stylesheet}>
            <h1>Hello, world!</h1>
        </div>
    }
}