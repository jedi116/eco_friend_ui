use web_sys::*;
use yew::prelude::*;
use wasm_bindgen::*;


#[function_component(Navbar)]
pub fn nav_bar () -> Html {
    html! {
        <div class="page-wrapper">
            <div class="nav-wrapper">
                <div class="grad-bar"></div>
                    <nav class="navbar">
                        <img src="img/eco-friendly-logo.png" alt="Company Logo"/>
                        <ul class="nav no-search">
                            <li class="nav-item"><a href="/">{"Home"}</a></li>
                            <li class="nav-item"><a href="/calc">{"Calculator"}</a></li>
                        </ul>
                    </nav>
                </div>
            </div>
    }
}