use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use rand::Rng;
use gloo_console::log;

use crate::{characters::enemy_wizard::EnemyWizard, utils::Point};

enum FireballSelect {
    Left,
    Right
}

pub struct Fireball {
    loc: Point,
    ttl: f64,
    pub distance: f64,
    is_dying: bool,
    pub is_alive: bool,
    cur_frame: i32,
    time_to_frame: f64,
    current_select: FireballSelect,
    image: HtmlImageElement,
    image2: HtmlImageElement,
    scorch_image: HtmlImageElement
}

const FIREBALL_RIGHT_IMAGE_NAME: &str = "./assets/images/fireball_right.png";
const FIREBALL_LEFT_IMAGE_NAME: &str = "./assets/images/fireball_left.png";
const FIREBALL_SCORCH_IMAGE_NAME: &str = "./assets/images/scorch.png";
const FIREBALL_IMAGE_WIDTH: f64 = 70.0;
const FIREBALL_IMAGE_HEIGHT: f64 = 100.0;
const FIREBALL_TIME_TO_LIVE: f64 = 3000.0;
const FIREBALL_TIME_BETWEEN_FRAMES: f64 = 60.0;
const FIREBALL_TIME_TO_FULL_DISTANCE: f64 = 2000.0; // Time to travel entire corridor
const FIREBALL_COLLISION_DISTANCE: f64 = 2.0;

impl Fireball {
    pub fn new(x: f64, y: f64) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(FIREBALL_RIGHT_IMAGE_NAME);

        let image2 = HtmlImageElement::new().unwrap();
        image2.set_src(FIREBALL_LEFT_IMAGE_NAME);

        let image3 = HtmlImageElement::new().unwrap();
        image3.set_src(FIREBALL_SCORCH_IMAGE_NAME);

        let mut rng = rand::thread_rng();
        let fbs = match rng.gen_range(0..2) {
            0 => FireballSelect::Left,
            1 => FireballSelect::Right,
            _ => FireballSelect::Left, 
        };

        Fireball {
            loc: Point::new(x, y),
            ttl: FIREBALL_TIME_TO_LIVE,
            distance: 100.0,
            is_dying: false,
            is_alive: true,
            cur_frame: 0,
            time_to_frame: FIREBALL_TIME_BETWEEN_FRAMES,
            current_select: fbs,
            image: image,
            image2: image2,
            scorch_image: image3
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.ttl -= delta;
        if self.ttl <= 0.0 && self.is_dying {
            self.is_alive = false;
        } else if self.ttl <= 0.0 && !self.is_dying {
            self.is_dying = true;
            self.ttl = FIREBALL_TIME_TO_LIVE;
        }

        self.distance = (self.ttl/FIREBALL_TIME_TO_LIVE) * 100.0;

        self.time_to_frame -= delta;
        if self.time_to_frame < 0.0 {
            self.time_to_frame += FIREBALL_TIME_BETWEEN_FRAMES;
            self.cur_frame += 1;
            if self.cur_frame as f64 * FIREBALL_IMAGE_WIDTH >= self.image.width() as f64 {
                self.cur_frame = 0;
                let mut rng = rand::thread_rng();
                self.current_select = match rng.gen_range(0..2) {
                    0 => FireballSelect::Left,
                    1 => FireballSelect::Right,
                    _ => FireballSelect::Left, 
                };
            }
        }
    }

    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        if !self.is_alive {
            return;
        }

        let (output_width,output_height) = if self.is_dying {
            (
                0.5 * FIREBALL_IMAGE_WIDTH,
                0.5 * FIREBALL_IMAGE_HEIGHT
            )
        } else {
            (
                (self.ttl / FIREBALL_TIME_TO_LIVE) * FIREBALL_IMAGE_WIDTH,
                (self.ttl / FIREBALL_TIME_TO_LIVE) * FIREBALL_IMAGE_HEIGHT
            )
        };

        let x = self.loc.x - (output_width / 2.0);
        let y = self.loc.y - (output_height / 2.0);
        let img = if self.is_dying {
            &self.scorch_image
        } else {
            match self.current_select {
                FireballSelect::Left => {
                    &self.image2
                },
                FireballSelect::Right => {
                    &self.image
                }
            }
        };
        
        let offset = if self.is_dying {
            0.0
        } else {
            self.cur_frame as f64 * FIREBALL_IMAGE_WIDTH
        };

        let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            img, 
            offset, 
            0.0, 
            FIREBALL_IMAGE_WIDTH, 
            FIREBALL_IMAGE_HEIGHT, 
            x, 
            y, 
            output_width, 
            output_height
        );

    }

    pub fn hit_object(&mut self) {
        self.is_dying = true;
    }

    pub fn get_loc(&self) -> (f64, f64) {
        (self.loc.x, self.loc.y)
    }

    pub fn get_distance(&mut self) -> f64 {
        if self.is_dying {
            return 0.0;
        }
        let time_alive = FIREBALL_TIME_TO_LIVE - self.ttl;
        
        (time_alive / FIREBALL_TIME_TO_FULL_DISTANCE) * 100.0
    }

    pub fn check_collision(&mut self, object: &EnemyWizard) -> bool {
        let half_width = object.get_width() / 2.0;
        if self.loc.x > object.loc.x - half_width && self.loc.x < object.loc.x + half_width {
            // In correct x location
            if (self.get_distance() - object.loc.y).abs() < FIREBALL_COLLISION_DISTANCE {
                log!("Fireball hit enemy");
                self.hit_object();
                return true;
            }
        }
        false
    }
}