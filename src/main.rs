use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod assets;

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

fn main() {
    yew::Renderer::<App>::new().render();
}
