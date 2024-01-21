use yew::prelude::*;
use web_sys;
use crate::components::button::Button;



#[function_component(Home)]
pub fn home() -> Html {
    let redirect_to_calculator_page = Callback::from(|_| {
        let window = web_sys::window().expect("can not load window");
        window
        .location()
                    .set_href("/calc")
                    .expect("failed to set the href");
    });
    html! {
        <div>
            <section class="headline">
                <h1>{"Eco Friendly Transportation planning"}</h1>
                <p>{"Please enter Origin and destination to get you carbon foot print"}</p>
                <Button 
                    name={"Get Started"} 
                    handle_click= {redirect_to_calculator_page} 
                    class_style={"get-started".to_string()}
                />
            </section>
        </div>
    }
}