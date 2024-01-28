use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::wasm_bindgen::prelude::Closure;

use gloo_console::log;

pub enum GameMsg {
    MouseDown((f64,f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    Render
}

pub struct GameControl {
    last_x: f64,
    last_y: f64,
    last_action: String,
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
            last_x: 0.0,
            last_y: 0.0,
            last_action: "".to_string(),
            canvas: NodeRef::default(),
            callback: callback
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameMsg::MouseDown(evt) => {
                log!("Mouse down ", evt.0, evt.1);
                self.last_x = evt.0;
                self.last_y = evt.1;
                self.last_action = "Mouse down".to_string();
                true
            },
            GameMsg::MouseUp(evt) => {
                log!("Mouse up ", evt.0, evt.1);
                self.last_x = evt.0;
                self.last_y = evt.1;
                self.last_action = "Mouse Up".to_string();
                true
            },
            GameMsg::MouseMove(evt) => {
                self.last_x = evt.0;
                self.last_y = evt.1;
                self.last_action = "Mouse Move".to_string();
                true
            },
            GameMsg::Render => {
                self.render();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousedown = ctx.link().callback(move |evt:MouseEvent| {
            GameMsg::MouseDown(( evt.page_x() as f64, evt.page_y() as f64) )
        });
        let onmouseup = ctx.link().callback(move |evt:MouseEvent| {
            GameMsg::MouseUp(( evt.page_x() as f64, evt.page_y() as f64) )
        });
        let onmousemove = ctx.link().callback(move |evt:MouseEvent| {
            GameMsg::MouseMove(( evt.page_x() as f64, evt.page_y() as f64) )
        });
        

        html! {
            <div class="game_canvas">
                <canvas id="canvas"
                    style={"margin: 0px; width: 100vw; height: 90vh; left: 0px; top:0px;"}
                    ref={self.canvas.clone()}
                    onmousedown={onmousedown}
                    onmouseup={onmouseup}
                    onmousemove={onmousemove}
                    >
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

        ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
        ctx.set_font("48px serif");
        let _ = ctx.fill_text(self.last_action.as_str(), 300.0, 300.0);

        let loc_string = "X: ".to_owned() + self.last_x.to_string().as_str() + ", Y: " + self.last_y.to_string().as_str();
        let _ = ctx.fill_text(loc_string.as_str(), 300.0, 350.0);

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}