use yew::prelude::*;

pub enum HomeMsg {}

pub struct HomeControl {}

impl Component for HomeControl {
    type Message = HomeMsg;
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
                <h1>{"Wizard Game"}</h1>
                <a href="/game">
                    <div class="link_button">
                        {"Play Game"}
                    </div>
                </a>
            </div>
        }
    }
}