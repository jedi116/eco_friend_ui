use web_sys::*;
use yew::prelude::*;
use wasm_bindgen::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub handle_input: Callback<String>
}

#[function_component(TextInput)]
pub fn input_text(props: &TextInputProps) -> Html {
    let handle_onchange = props.handle_input.clone();
    let on_change = Callback::from(move |event: Event| {
        let value = event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
        handle_onchange.emit(value);
    });
    html! {
        <div class="input-container ic1">
            <input class="input" type="text" onchange={on_change}/>
            <div class="cut"></div>
            <label class="placeholder">{props.name.clone()}</label>
        </div>
    }

}