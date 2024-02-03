use yew::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::wasm_bindgen::prelude::Closure;
use std::vec::{self, Vec};

use gloo_console::log;

use crate::assets::{backwall::Backwall, wall::{Wall,WallType}};

pub enum GameMsg {
    MouseDown((f64,f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    Render,
    Null
}

pub struct GameControl {
    last_x: f64,
    last_y: f64,
    last_action: String,
    canvas: NodeRef,
    backwall: Backwall,
    walls: Vec::<Wall>,
    callback: Closure<dyn FnMut()>,
}

const GAME_WIDTH: f64 = 800.0;
const GAME_HEIGHT: f64 = 600.0;

impl Component for GameControl {
    type Message = GameMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback = Closure::wrap(Box::new(move || comp_ctx.send_message(GameMsg::Render)) as Box<dyn FnMut()>);

        ctx.link().send_message(GameMsg::Render);
    
        let backwall = Backwall::new(GAME_WIDTH/2.0, GAME_HEIGHT/2.0);
        
        let mut vec_walls = Vec::<Wall>::new();

        vec_walls.push(Wall::new(GAME_WIDTH/4.0, GAME_HEIGHT / 2.0, WallType::Left));
        vec_walls.push(Wall::new(GAME_WIDTH * 0.75, GAME_HEIGHT / 2.0, WallType::Right));
        vec_walls.push(Wall::new(GAME_WIDTH/2.0, GAME_HEIGHT/4.0, WallType::Top));
        vec_walls.push(Wall::new(GAME_WIDTH/2.0, GAME_HEIGHT * 0.75, WallType::Bottom));
       
        Self{
            last_x: 0.0,
            last_y: 0.0,
            last_action: "".to_string(),
            canvas: NodeRef::default(),
            backwall: backwall,
            walls: vec_walls,
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
            GameMsg::Null => {
                true
            }
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
            // let element = match evt.target() {
            //     Some(el) => el,
            //     None => {
            //         return GameMsg::Null
            //     }
            // };

            GameMsg::MouseMove(( evt.client_x() as f64, evt.client_y() as f64) )
        });
        

        html! {
            <div class="game_canvas" style={"text-align: left;"}>
                <canvas id="canvas"
                    style={"margin: 0px; width: 800px; height: 600px; left: 0px; top:0px;"}
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

        let mut ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();

        ctx.set_fill_style(&JsValue::from("rgb(200,200,255)"));
        ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        for wall in self.walls.iter() {
            wall.render(&mut ctx);
        }

        self.backwall.render(&mut ctx);



        // DEBUG
        ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
        ctx.set_font("12px serif");
        // let _ = ctx.fill_text(self.last_action.as_str(), 300.0, 300.0);
        let loc_string = "X: ".to_owned() + self.last_x.to_string().as_str() + ", Y: " + self.last_y.to_string().as_str();
        let _ = ctx.fill_text(loc_string.as_str(), 0.0, 20.0);
        ctx.stroke();
        // END_DEBUG
        
        
        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }

    //    ctx.begin_path();
    //     ctx.set_stroke_style(&JsValue::from("rgb(255,0,0)"));
    //     ctx.move_to(50.0, 50.0);
    //     ctx.line_to(200.0, 200.0);
    //     ctx.stroke();

    //     ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
    //     ctx.set_font("48px serif");
    //     let _ = ctx.fill_text(self.last_action.as_str(), 300.0, 300.0);

    //     let loc_string = "X: ".to_owned() + self.last_x.to_string().as_str() + ", Y: " + self.last_y.to_string().as_str();
    //     let _ = ctx.fill_text(loc_string.as_str(), 300.0, 350.0);
     
}