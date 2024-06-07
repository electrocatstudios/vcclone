use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use gloo_console::log;

use crate::utils::Point;

#[derive(PartialEq)]
pub enum WallType {
    Bottom,
    Top,
    Left,
    Right
}

pub struct Wall {
    loc: Point,
    wall_type: WallType,
    image: HtmlImageElement
}

const BOTTOM_WALL_IMAGE_NAME: &str = "./assets/images/floor_800x800.png";
const TOP_WALL_IMAGE_NAME: &str = "./assets/images/ceiling.png";
const LEFT_WALL_IMAGE_NAME: &str = "./assets/images/left_wall.png";
const RIGHT_WALL_IMAGE_NAME: &str = "./assets/images/right_wall.png";

const NUMBER_SEGMENTS: u32 = 100;

impl Wall {
    pub fn new(x: f64, y: f64, wall_type: WallType) -> Self {
        let image = match wall_type {
            WallType::Bottom => {
                let image: HtmlImageElement = HtmlImageElement::new().unwrap();
                image.set_src(BOTTOM_WALL_IMAGE_NAME);
                image
            },
            WallType::Top => {
                let image: HtmlImageElement = HtmlImageElement::new().unwrap();
                image.set_src(TOP_WALL_IMAGE_NAME);
                image
            },
            WallType::Left => {
                let image: HtmlImageElement = HtmlImageElement::new().unwrap();
                image.set_src(LEFT_WALL_IMAGE_NAME);
                image
            },
            WallType::Right => {
                let image: HtmlImageElement = HtmlImageElement::new().unwrap();
                image.set_src(RIGHT_WALL_IMAGE_NAME);
                image
            },
        };

        Wall {
            loc: Point::new(x, y),
            wall_type: wall_type,
            image: image,
        }
    }

    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        let x = self.loc.x - (self.image.width() as f64 / 2.0);
        let y = self.loc.y - (self.image.height() as f64 / 2.0);

        if self.wall_type == WallType::Bottom {
            let mut cur_h = self.loc.y;
            let mut frac = 1.0;
            for i in 1..NUMBER_SEGMENTS+1 {
            
                let seg_frac = 1.0 / NUMBER_SEGMENTS as f64; // 1/10th - 0.1
                // let frac = i as f64 / NUMBER_SEGMENTS as f64; // 0.1 
                // frac = frac * frac;
                // frac 0.25                
                let sh = seg_frac * self.image.height() as f64;
                let sy: f64 = (i-1) as f64 * sh;
                // 0
                // let new_y = self.loc.y + ((i-1) as f64 * sy);
                let new_h = sh * frac;
                let new_w = self.image.width() as f64 * frac;// * (600.0/800.0);
                let new_x = self.loc.x - (new_w / 2.0);
                if i == NUMBER_SEGMENTS {
                    log!("vals", cur_h, sh, self.image.height() as f64 * frac);
                
                }
                let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image, 
                    0.0, 
                    sy, 
                    self.image.width() as f64, 
                    sh, 
                    new_x, 
                    cur_h, 
                    new_w, 
                    new_h
                );
                cur_h -= new_h-1.0;
                frac *= (1.0 - (seg_frac)) * (1.0 - (seg_frac));// * (1.0 - (seg_frac));
            }
        } else {
            let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.image, 
                0.0, 
                0.0, 
                self.image.width() as f64, 
                self.image.height() as f64, 
                x, 
                y, 
                self.image.width() as f64, 
                self.image.height() as f64
            );
        }
    }
}