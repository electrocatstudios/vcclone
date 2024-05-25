use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::wasm_bindgen::prelude::Closure;
use js_sys::Date;

use gloo_console::log;

use crate::assets::fireball;
use crate::assets::lightning_bolt::LightningBolt;
use crate::assets::{backwall::Backwall, wall::*,fireball::Fireball};
use crate::characters::enemy_wizard::EnemyWizard;
use crate::player::player::Player;
use crate::utils::Point;
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
    backwall: Backwall,
    walls: Vec::<Wall>,
    enemies: Vec::<EnemyWizard>,
    lightning_bolts: Vec::<LightningBolt>,
    player: Player,
    fireballs: Vec::<Fireball>,
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

        // Create walls
        let backwall = Backwall::new(GAME_WIDTH/2.0, GAME_HEIGHT/2.0);

        let mut vec_walls = Vec::<Wall>::new();

        vec_walls.push(Wall::new(GAME_WIDTH/4.0, GAME_HEIGHT/2.0, WallType::Left));
        vec_walls.push(Wall::new(GAME_WIDTH * 0.75, GAME_HEIGHT/2.0, WallType::Right));
        vec_walls.push(Wall::new(GAME_WIDTH/2.0, GAME_HEIGHT/4.0, WallType::Top));
        vec_walls.push(Wall::new(GAME_WIDTH/2.0, GAME_HEIGHT * 0.75, WallType::Bottom));
                
        Self{
            last_x: 0.0,
            last_y: 0.0,
            last_action: "".to_string(),
            last_update: Date::now(),
            backwall: backwall,
            walls: vec_walls,
            enemies: Vec::<EnemyWizard>::new(),
            lightning_bolts: Vec::<LightningBolt>::new(),
            player: Player::new(),
            fireballs: Vec::<Fireball>::new(),
            canvas: NodeRef::default(),
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
                if self.player.fire_cooldown <= 0.0 {
                    // let fireball = Fireball::new(self.last_x, self.last_y);
                    self.player.fire(self.last_x, self.last_y);
                    // self.fireballs.push(fireball);
                }

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
    fn game_update(&mut self) {
        let cur_time = Date::now();
        let diff = cur_time - self.last_update;

        self.last_update = cur_time;

        self.player.update(diff);
    
        for lb in self.lightning_bolts.iter_mut() {
            lb.update(diff);
        }
        self.lightning_bolts.retain(|lb| {
            lb.is_alive
        });


        match self.player.get_fired() {
            Some(strike_point) => {
                // log!("We have successfully got_fired ", strike_point.x, strike_point.y);
                for enemy in self.enemies.iter_mut() {
                    if enemy.check_hit(strike_point.x, strike_point.y) {
                        // log!("We have hit the enemy");
                        self.lightning_bolts.push(
                            LightningBolt::new(
                            Point::new(GAME_WIDTH/2.0, GAME_HEIGHT),
                            Point::new(strike_point.x, strike_point.y)
                            )
                        );
                        break;
                    }
                }
            },
            None => {}
        }

        // for fb in self.fireballs.iter_mut() {
        //     fb.update(diff);
    
        //     // Check if hit the back wall
        //     // check_fireball_strike(fb);
        // }

        // self.fireballs.retain(|fireball| {
        //     fireball.is_alive
        // });

        // Check for presence of wizard
        if self.enemies.len() < 1 {
            self.enemies.push(EnemyWizard::new(0.0, 100.0));
            self.enemies.push(EnemyWizard::new(100.0, 100.0));
            self.enemies.push(EnemyWizard::new(25.0, 100.0));
            self.enemies.push(EnemyWizard::new(75.0, 100.0));
        }

        for enemy in self.enemies.iter_mut() {
            enemy.update(diff);
        }

        self.enemies.sort_by( |a,b| {
            let a_val: i32 = (a.loc.y * 100.0) as i32;
            let b_val: i32 = (b.loc.y * 100.0) as i32;
            a_val.cmp(&b_val)
        });

        'outer: for enemy in self.enemies.iter_mut() {
            for fireball in self.fireballs.iter_mut() {
                if true || fireball.check_collision(enemy) {
                    fireball.is_alive = false;
                    // log!("Outer loop hit enemy - set enemy on fire");
                    enemy.hit_by_object();
                    break 'outer;
                }
            }
        }
        // self.fireballs.retain(|fireball| {
        //     fireball.is_alive
        // });

    }

    fn render(&mut self) {
        self.game_update();

        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let mut ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();

        ctx.set_fill_style(&JsValue::from("rgb(200,200,255)"));
        ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        // Draw game elements
        // ---- Scenery
        for wall in self.walls.iter() {
            wall.render(&mut ctx);
        }
        self.backwall.render(&mut ctx);

        // ----- Game Assets
        for enemy in self.enemies.iter() {
            enemy.render(&mut ctx);
        }

        for fb in self.fireballs.iter() {
            fb.render(&mut ctx);
        }
        for lb in self.lightning_bolts.iter_mut() {
            lb.render(&mut ctx);
        }
        // ----- Player Assets
        
        // ----- HUD

        // End Draw game elements

        // Debug Information
        ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
        ctx.set_font("12px serif");
        let loc_string = "X: ".to_owned() + self.last_x.to_string().as_str() + ", Y: " + self.last_y.to_string().as_str();
        let _ = ctx.fill_text(loc_string.as_str(), 10.0, 15.0);
        // End Debug Information

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}


fn check_fireball_strike(fireball: &mut Fireball) {
    // Check for wall strikes
    let (x,_y) = fireball.get_loc();
    if x < 340.0 {
        // Check if hit the left wall
        let distance_to_wall = (x/340.0) * 100.0;
        let fireball_dist = fireball.get_distance();
        if fireball_dist >= distance_to_wall {
            fireball.hit_object();
        }
    } else if x > 460.0 {
        // Check if hit the right wall
        let distance_to_wall = ((GAME_WIDTH - x) / (GAME_WIDTH - 460.0)) * 100.0;
        let fireball_dist = fireball.get_distance();
        if fireball_dist >= distance_to_wall {
            fireball.hit_object();
        }
    }
    if fireball.get_distance() >= 100.0 {
        fireball.hit_object();
    }
}
