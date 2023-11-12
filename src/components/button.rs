use web_sys::*;
use yew::prelude::*;
use wasm_bindgen::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub name: String,
    pub handle_click: Callback<String>,
    pub class_style: String    
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let prop_handle_click = props.handle_click.clone();
    let prop_name_clone = props.name.clone();
    let handle_click = Callback::from(move |event: MouseEvent| {
        prop_handle_click.emit(prop_name_clone.clone());
    });
    html! {
        <button class={props.class_style.clone()} onclick={handle_click}>{props.name.clone()}</button>
    }
}