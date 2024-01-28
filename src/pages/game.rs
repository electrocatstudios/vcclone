use yew::prelude::*;

pub enum GameMsg {}

pub struct GameControl {}

impl Component for GameControl {
    type Message = GameMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"We are on the game page"}</h1>
            </div>
        }
    }
}