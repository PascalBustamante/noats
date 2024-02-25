use yew::prelude::*;
use stylist::styled_component::*;

#[styled_component(Auth)]
pub fn auth() -> HTML {
   let stylesheet = css!{#"
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

   "#};
   html! {
       <section class={stylesheet}>
           <h1>Register</h1>
           <input class="form-control" type="text" placeholder="Username" />
           <input class="form-control" type="password" placeholder="Password" />
           <input class="form-control" type="password" placeholder="Confirm Password" />
           <button class="btn btn-primary" onclick={link.callback(|_| Msg::Register)}>Register</button>
       </section>
   };
}