mod components {
    pub mod place_form;
    pub mod text_input;
    pub mod button;
    pub mod nav_bar;
    pub mod notification;
}

mod pages {
    pub mod calculator;
    pub mod home;
    pub mod not_found;
    pub mod report;
}

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;


use crate::components::nav_bar::Navbar;
use crate::pages::home::Home;
use crate::pages::calculator::Calculator;
use crate::pages::not_found::NotFound;
use crate::pages::report::Report;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/calc")]
    Calculator,
    #[at("/report/:origin/:destination")]
    Report {origin: String, destination: String},
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component]
fn App() -> Html {
    html! {
    <BrowserRouter>
        <div> 
            <Navbar />
            <main>
                <Switch<Route> render={switch} />
            </main>
        </div>
    </BrowserRouter>
    }

}

fn switch(routes: Route) -> Html {
     match routes {
        Route::Home => {
            html! {
                <Home />
            }
        },
        Route::Calculator => {
            html! {
                <Calculator />
            }
        },
        Route::Report {origin, destination} => {
            html!(<Report origin={origin} destination={destination}/>)
        }
        Route::NotFound => {
            html! { <NotFound /> }
        }
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
