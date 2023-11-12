use yew::prelude::*;
use crate::components::place_form::PlaceForm;


#[function_component(Calculator)]
pub fn calculator() -> Html {
    html! {
        <div>
            <PlaceForm />
        </div>
    }
}