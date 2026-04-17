use yew::prelude::*;

use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL};
use web_sys::wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use js_sys::Date;

use gloo_console;

use crate::assets::skybox::Skybox;
use crate::pages::viewmanager::ViewManager;

use crate::{GAME_WIDTH,GAME_HEIGHT};

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
    last_update: f64,
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    node_ref: NodeRef,
    view_manager: ViewManager,
    skybox: Skybox,
    // enemies: Vec::<EnemyWizard>,
    // lightning_bolts: Vec::<LightningBolt>,
    // player: Player,
    // fireballs: Vec::<Fireball>,
    callback: Closure<dyn FnMut()>,
}

impl Component for GameControl {
    type Message = GameMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback = Closure::wrap(Box::new(move || comp_ctx.send_message(GameMsg::Render)) as Box<dyn FnMut()>);

        // Create walls        
        Self{
            last_x: 0.0,
            last_y: 0.0,
            last_action: "".to_string(),
            last_update: Date::now(),
            canvas: None,
            gl: None,
            node_ref: NodeRef::default(),
            view_manager: ViewManager::new(),
            skybox: Skybox::new(),
            // enemies: Vec::<EnemyWizard>::new(),
            // lightning_bolts: Vec::<LightningBolt>::new(),
            // player: Player::new(),
            // fireballs: Vec::<Fireball>::new(),
            callback: callback
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameMsg::MouseDown(evt) => {
                self.last_x = evt.0;
                self.last_y = evt.1;
                self.last_action = "Mouse down".to_string();
                // log!("Mouse down ", self.last_x, self.last_y );
                // if self.player.fire_cooldown <= 0.0 {
                //     // let fireball = Fireball::new(self.last_x, self.last_y);
                //     self.player.fire(self.last_x, self.last_y);
                //     // self.fireballs.push(fireball);
                // }

                true
            },
            GameMsg::MouseUp(evt) => {
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
                    style={"margin: 0px; width: 800px; height: 600px; left: 0px; top:0px;"}
                    ref={self.node_ref.clone()}
                    onmousedown={onmousedown}
                    onmouseup={onmouseup}
                    onmousemove={onmousemove}
                    >
                </canvas>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Grab context and other setup
        let c = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: GL = c
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        c.set_width(GAME_WIDTH as u32);
        c.set_height(GAME_HEIGHT as u32);

        self.canvas = Some(c);
        self.gl = Some(gl);

        if first_render {
            // self.reload();
            ctx.link().send_message(GameMsg::Render);
        }
    }
}


impl GameControl {
    fn render(&mut self) {
        match &mut self.gl {
            Some(_) => {},
            None => {
                gloo_console::log!("WebGL context not ready");
                return;
            }
        }
        let mut gl = self.gl.as_ref().expect("GL Context not initialized!");

        self.view_manager.update(gl);
        self.skybox.update(self.view_manager.delta, gl, self.view_manager.width as f32, self.view_manager.height as f32);
        
        gl.viewport(
            0,
            0,
            GAME_WIDTH as i32,
            GAME_HEIGHT as i32,
        );
        gl.clear_color(0.2, 0.8, 0.2, 1.0);
        gl.clear_depth(1.0);
        
        // Enable the depth test
        gl.enable(GL::DEPTH_TEST);

        // Clear the color buffer bit
        gl.clear(GL::COLOR_BUFFER_BIT);

        self.skybox.render(gl, self.view_manager.u_time as f64, &self.view_manager.camera);

        
        // Debug Information
        // ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
        // ctx.set_font("12px serif");
        // let loc_string = "X: ".to_owned() + self.last_x.to_string().as_str() + ", Y: " + self.last_y.to_string().as_str();
        // let _ = ctx.fill_text(loc_string.as_str(), 10.0, 15.0);
        // End Debug Information

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}
