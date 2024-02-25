use web_sys::{CanvasRenderingContext2d,HtmlImageElement};
use rand::Rng;

use crate::utils::{get_angle_between_points, Point}; 
use crate::{GAME_WIDTH,GAME_HEIGHT};

use gloo_console::log;

pub struct EnemyWizard {
    pub loc: Point,
    dest_loc: Point,
    wait_time: f64,
    move_frame: i32,
    time_to_frame: f64,
    image: HtmlImageElement,
    walk_image: HtmlImageElement,
}

const ENEMY_WIZARD_IMAGE: &str = "./assets/images/enemy_wizard.png";
const ENEMY_WIZARD_WALK_IMAGE: &str = "./assets/images/enemy_wizard_walk.png";

const ENEMY_WIZARD_FRAME_WIDTH: f64 = 100.0;
const ENEMY_WIZARD_FRAME_HEIGHT: f64 = 150.0;
const ENEMY_WIZARD_SPEED: f64 = 0.03;
const ENEMY_WIZARD_BACKWALL_HEIGHT: f64 = 30.0;
const ENEMY_WIZARD_BACKWALL_WIDTH: f64 = 20.0;

const ENEMY_WIZARD_MIN_WAIT: f64 = 1000.0;
const ENEMY_WIZARD_MAX_WAIT: f64 = 4000.0;

const ENEMY_WIZARD_DESTINATION_ACCEPT: f64 = 0.2;
const ENEMY_WIZARD_WALK_FRAME_DELAY: f64 = 100.0;

impl EnemyWizard {
    pub fn new(x: f64, y: f64) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(ENEMY_WIZARD_IMAGE);

        let walk_image = HtmlImageElement::new().unwrap();
        walk_image.set_src(ENEMY_WIZARD_WALK_IMAGE);

        EnemyWizard {
            loc: Point::new(x, y),
            dest_loc: Point::new(x, y),
            image: image,
            wait_time: 1000.0,
            move_frame: 0,
            time_to_frame: 0.0,
            walk_image: walk_image
        }
    }
    
    pub fn update(&mut self, delta: f64) {
        if self.wait_time > 0.0 {
            self.wait_time -= delta;
            return;
        }

        if self.loc.x == self.dest_loc.x || self.loc.y == self.dest_loc.y {
            // Create new dest location
            let mut rng = rand::thread_rng();
            let new_x = rng.gen_range(0..100) as f64;
            let new_y = rng.gen_range(0..80) as f64;
            self.dest_loc.x = new_x;
            self.dest_loc.y = new_y;

            self.wait_time = rng.gen_range(ENEMY_WIZARD_MIN_WAIT..ENEMY_WIZARD_MAX_WAIT);
            return;
        }

        let angle_to_dest = get_angle_between_points(&self.loc, &self.dest_loc);
        
        self.loc.x += angle_to_dest.sin() * (delta * ENEMY_WIZARD_SPEED);
        self.loc.y -= angle_to_dest.cos() * (delta * ENEMY_WIZARD_SPEED);

        if (self.loc.x - self.dest_loc.x).abs() < ENEMY_WIZARD_DESTINATION_ACCEPT 
           && (self.loc.y - self.dest_loc.y).abs() < ENEMY_WIZARD_DESTINATION_ACCEPT {
            self.loc.x = self.dest_loc.x;
            self.loc.y = self.dest_loc.y;
        }

        self.time_to_frame -= delta;
        if self.time_to_frame < 0.0 {
            self.time_to_frame += ENEMY_WIZARD_WALK_FRAME_DELAY;
            self.move_frame += 1;
            if self.move_frame as f64 * ENEMY_WIZARD_FRAME_WIDTH >= self.walk_image.width() as f64 {
                self.move_frame = 0;
            }
        }

    }

    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        let output_width = ENEMY_WIZARD_BACKWALL_WIDTH + ((self.loc.y / 100.0) * (ENEMY_WIZARD_FRAME_WIDTH - ENEMY_WIZARD_BACKWALL_WIDTH));
        let output_height = ENEMY_WIZARD_BACKWALL_HEIGHT + ((self.loc.y / 100.0) * (ENEMY_WIZARD_FRAME_HEIGHT - ENEMY_WIZARD_BACKWALL_HEIGHT));
        
        let hall_width = 120.0 + ((self.loc.y / 100.0) * 680.0);
        let offset_from_left = (self.loc.x / 100.0) * hall_width; 

        let x: f64 = (340.0 - ((self.loc.y / 100.0) * 340.0)) + offset_from_left;         
        let y: f64 = ((GAME_HEIGHT / 2.0) + (self.loc.y * 2.5) + 50.0) - output_height;

        // log!("Enemy position ", self.loc.x, self.loc.y, x, y);
        let mut offset = 0.0;
        let image = if self.wait_time <= 0.0 {
            offset = self.move_frame as f64 * ENEMY_WIZARD_FRAME_WIDTH;
            &self.walk_image
        } else {
            &self.image
        };

        let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image, 
            offset, 
            0.0, 
            ENEMY_WIZARD_FRAME_WIDTH, 
            ENEMY_WIZARD_FRAME_HEIGHT, 
            x - (output_width/2.0), 
            y, 
            output_width, 
            output_height
        );

    }

    pub fn get_loc(&self) -> (f64, f64) {
        (self.loc.x, self.loc.y)
    }

    pub fn get_distance(&mut self) -> f64 {
       100.0 - self.loc.y
    }
}