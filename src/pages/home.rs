use yew::prelude::*;



#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <section class="headline">
                <h1>{"Eco Friendly Transportation planning"}</h1>
                <p>{"Please enter Origin and destination to get you carbon foot print"}</p>
            </section>
        </div>
    }
}