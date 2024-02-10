use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use rand::Rng;
use gloo_console::log;

use crate::utils::Point;

enum FireballSelect {
    Left,
    Right
}

pub struct Fireball {
    loc: Point,
    ttl: f64,
    pub is_alive: bool,
    cur_frame: i32,
    time_to_frame: f64,
    current_select: FireballSelect,
    image: HtmlImageElement,
    image2: HtmlImageElement
}

const FIREBALL_RIGHT_IMAGE_NAME: &str = "./assets/images/fireball_right.png";
const FIREBALL_LEFT_IMAGE_NAME: &str = "./assets/images/fireball_left.png";
const FIREBALL_IMAGE_WIDTH: f64 = 70.0;
const FIREBALL_IMAGE_HEIGHT: f64 = 100.0;
const FIREBALL_TIME_TO_LIVE: f64 = 3000.0;
const FIREBALL_TIME_BETWEEN_FRAMES: f64 = 60.0;

impl Fireball {
    pub fn new(x: f64, y: f64) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(FIREBALL_RIGHT_IMAGE_NAME);

        let image2 = HtmlImageElement::new().unwrap();
        image2.set_src(FIREBALL_LEFT_IMAGE_NAME);

        let mut rng = rand::thread_rng();
        let fbs = match rng.gen_range(0..2) {
            0 => FireballSelect::Left,
            1 => FireballSelect::Right,
            _ => FireballSelect::Left, 
        };

        Fireball {
            loc: Point::new(x, y),
            ttl: FIREBALL_TIME_TO_LIVE,
            is_alive: true,
            cur_frame: 0,
            time_to_frame: FIREBALL_TIME_BETWEEN_FRAMES,
            current_select: fbs,
            image: image,
            image2: image2
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.ttl -= delta;
        if self.ttl <= 0.0 {
            self.is_alive = false;
        }

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
        let output_width = (self.ttl / FIREBALL_TIME_TO_LIVE) * FIREBALL_IMAGE_WIDTH;
        let output_height = (self.ttl / FIREBALL_TIME_TO_LIVE) * FIREBALL_IMAGE_HEIGHT;
        
        let x = self.loc.x - (output_width / 2.0);
        let y = self.loc.y - (output_height / 2.0);
        let img = match self.current_select {
            FireballSelect::Left => {
                &self.image2
            },
            FireballSelect::Right => {
                &self.image
            }
        };

        let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            img, 
            self.cur_frame as f64 * FIREBALL_IMAGE_WIDTH, 
            0.0, 
            FIREBALL_IMAGE_WIDTH, 
            FIREBALL_IMAGE_HEIGHT, 
            x, 
            y, 
            output_width, 
            output_height
        );

    }
}