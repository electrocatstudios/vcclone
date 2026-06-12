use yew::prelude::*;

use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL};
use web_sys::wasm_bindgen::{JsCast, prelude::Closure};
use js_sys::Date;

use gloo_console;

use crate::assets::{firebolt::Firebolt, skybox::Skybox};
use crate::pages::viewmanager::ViewManager;
use crate::player::keymanager::KeyManager;
use crate::characters::enemy::{self, Enemy};
use crate::utils::Location3D;

use crate::player::player::Player;
use crate::{GAME_HEIGHT, GAME_WIDTH};

pub enum GameMsg {
    MouseDown((f64,f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    KeyDown(String),
    KeyUp(String),
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
    key_manager: KeyManager,
    enemies: Vec::<Enemy>,
    // lightning_bolts: Vec::<LightningBolt>,
    player: Player,
    // fireballs: Vec::<Fireball>,
    firebolts: Vec::<Firebolt>,
    callback: Closure<dyn FnMut()>,
}

impl Component for GameControl {
    type Message = GameMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback = Closure::wrap(Box::new(move || comp_ctx.send_message(GameMsg::Render)) as Box<dyn FnMut()>);


        let mut vec_enemies = Vec::<Enemy>::new();
        vec_enemies.push(Enemy::new(Location3D { x: 0.0, y: 0.3, z: -7.0 }));
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
            key_manager: KeyManager::new(),
            enemies: vec_enemies,
            // lightning_bolts: Vec::<LightningBolt>::new(),
            player: Player::new(),
            // fireballs: Vec::<Fireball>::new(),
            firebolts: Vec::<Firebolt>::new(),
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
                let firebolt = self.player.cast_firebolt();
                self.firebolts.push(firebolt);
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
                let x_diff = evt.0 - self.last_x;
                let y_diff = evt.1 - self.last_y;

                self.last_x = evt.0;
                self.last_y = evt.1;

                self.player.look(x_diff as f32, y_diff as f32); 
                // self.last_action = "Mouse Move".to_string();
                true
            },
            GameMsg::Render => {
                self.render();
                true
            },
            GameMsg::KeyDown(key) => {
                self.key_manager.handle_key_down(key);
                true
            },
            GameMsg::KeyUp(key) => {
                self.key_manager.handle_key_up(key);
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
            GameMsg::MouseMove(( evt.page_x() as f64, evt.page_y() as f64) )
        });
        let onkeydown = ctx.link().callback(move |evt:KeyboardEvent| {
            GameMsg::KeyDown(evt.key())
        });
        let onkeyup = ctx.link().callback(move |evt:KeyboardEvent| {
            GameMsg::KeyUp(evt.key())
        });

        html! {
            <div class="game_canvas">
                <canvas id="canvas"
                    style={"margin: 0px; width: 800px; height: 600px; left: 0px; top:0px;"}
                    ref={self.node_ref.clone()}
                    onkeydown={onkeydown}
                    onkeyup={onkeyup}
                    onmousedown={onmousedown}
                    onmouseup={onmouseup}
                    onmousemove={onmousemove}
                    tabindex="0"
                    >
                </canvas>
                <div id="debug_info">{"Debug"}</div>
                <div id="sight"></div>
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
    fn game_update(&mut self) {
        let now = Date::now();
        let delta = now - self.last_update;
        self.last_update = now;

        self.player.update(delta, &self.key_manager);
        let player_loc = self.player.get_location();

        self.view_manager.camera.move_camera(player_loc.x, player_loc.y, player_loc.z);

        let look_at = self.player.get_look_rotation();
    
        let x = player_loc.x - look_at.0;
        let y = player_loc.y - (look_at.1 - std::f32::consts::PI * 0.5); // Add 90 degrees to look downwards by default

        self.view_manager.camera.look_at(x, y, 0.0);

        // let debug = 
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(el) = document.get_element_by_id("debug_info") {
                    
                    let first_line = format!("Player Location: ({:.2}, {:.2}, {:.2}) Look At: ({:.2}, {:.2})", player_loc.x, player_loc.y, player_loc.z, look_at.0, look_at.1);
                    let enemy = self.enemies.first();
                    let second_line = if let Some(enemy) = enemy {
                        let enemy_loc = enemy.get_location();
                        let enemy_dest = enemy.get_destination();
                        let enemy_rot = enemy.get_rotation();
                        let enemy_target_angle = enemy.get_target_angle();
                        
                        let enemy_line = format!("Enemy Location: ({:.2}, {:.2}, {:.2}) -> ({:.2}, {:.2}, {:.2}) Rotation: {:.2} Target Angle: {:.2}", enemy_loc.x, enemy_loc.y, enemy_loc.z, enemy_dest.x, enemy_dest.y, enemy_dest.z, enemy_rot, enemy_target_angle);
                        // let text = format!("{} <br> {} <br> Player Rotation: ({:.2}, {:.2})", first_line, enemy_line, self.player.rotation.0, self.player.rotation.1);
                        // el.set_inner_html(&text);
                        enemy_line
                    } else {
                        format!("No enemies")
                    };
                    // let second_line = format!("Player Rotation: ({:.2}, {:.2})", self.player.rotation.0, self.player.rotation.1);
                    let text = format!("{} <br> {}", first_line, second_line);
                    el.set_inner_html(&text);
                }
            }
        }

        for firebolt in self.firebolts.iter_mut() {
            firebolt.update(delta);
        }
        for enemy in self.enemies.iter_mut() {
            enemy.update(delta);
        }

    }

    fn render(&mut self) {
        match &mut self.gl {
            Some(_) => {},
            None => {
                gloo_console::log!("WebGL context not ready");
                return;
            }
        }
        self.game_update();

        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        for firebolt in self.firebolts.iter_mut() {
            firebolt.setup(gl, self.view_manager.width as f32, self.view_manager.height as f32);
        }
        for enemy in self.enemies.iter_mut() {
            enemy.setup(gl, self.view_manager.width as f32, self.view_manager.height as f32);
        }

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
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        // Enable the depth test
        gl.enable(GL::DEPTH_TEST);
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        
        // Clear the color buffer bit
        gl.clear(GL::COLOR_BUFFER_BIT);

        self.skybox.render(gl, self.view_manager.u_time as f64, &self.view_manager.camera);

        for firebolt in self.firebolts.iter_mut() {
            firebolt.render(gl, self.view_manager.u_time as f64, &self.view_manager.camera);
        }
        for enemy in self.enemies.iter_mut() {
            enemy.render(gl, self.view_manager.u_time as f64, &self.view_manager.camera);
        }
        // Debug Information
        // ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
        // ctx.set_font("12px serif");
        // let loc_string = "X: ".to_owned() + self.last_x.to_string().as_str() + ", Y: " + self.last_y.to_string().as_str();
        // let _ = ctx.fill_text(loc_string.as_str(), 10.0, 15.0);
        // End Debug Information

        self.firebolts.retain(|fb| !fb.is_expired());

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}
