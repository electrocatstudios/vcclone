use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::wasm_bindgen::prelude::Closure;

pub enum GameMsg {
    Render
}

pub struct GameControl {
    canvas: NodeRef,
    callback: Closure<dyn FnMut()>,
}

impl Component for GameControl {
    type Message = GameMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback = Closure::wrap(Box::new(move || comp_ctx.send_message(GameMsg::Render)) as Box<dyn FnMut()>);

        ctx.link().send_message(GameMsg::Render);

        Self{
            canvas: NodeRef::default(),
            callback: callback
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameMsg::Render => {
                self.render();
                true
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="game_canvas">
                <canvas id="canvas"
                    style={"margin: 0px; width: 100vw; height: 90vh; left: 0px; top:0px;"}
                    ref={self.canvas.clone()}>
                </canvas>
            </div>
        }
    }
}


impl GameControl {
    fn render(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();

        ctx.set_fill_style(&JsValue::from("rgb(200,200,255)"));
        ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from("rgb(255,0,0)"));
        ctx.move_to(50.0, 50.0);
        ctx.line_to(200.0, 200.0);
        ctx.stroke();

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}