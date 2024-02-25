use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, Debug, PartialEq)]
pub struct Props {
    pub input: String,
    pub parent: &String,
}

#[styled_component(Line)] // maybe change it to paragragh?
pub fn line() -> HTML {
    let stylesheet = css!{#"
        
    "#};
    html! {
        <div class="line"></div>
    }
}