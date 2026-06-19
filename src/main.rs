use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;

mod pages;
mod model;
mod assets;
mod utils;
mod player;
mod consts;
mod characters;

pub const GAME_WIDTH: f64 = 800.0;
pub const GAME_HEIGHT: f64 = 600.0;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/game")]
    Game,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{
            <pages::home::HomeControl />
        },
        Route::Game => html!{
            <pages::game::GameControl />
        },
        Route::NotFound => html! {<h1>{"404 - Not Found"}</h1>},
    }
}

#[function_component(App)]
fn app_body() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> 
        </BrowserRouter>
    }
}

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"], js_name = invoke)]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn main() {
    yew::Renderer::<App>::new().render();
}
