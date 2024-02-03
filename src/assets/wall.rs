use gloo_console::log;
use web_sys::{CanvasRenderingContext2d,HtmlImageElement};

pub enum WallType {
    Top,
    Bottom,
    Left,
    Right
}

pub struct Wall {
    loc_x: f64,
    loc_y: f64,
    walltype: WallType,
    image: HtmlImageElement
}

const TOP_WALL_IMAGE_NAME: &str = "./assets/images/ceiling.png";
const BOTTOM_WALL_IMAGE_NAME: &str = "./assets/images/floor.png";
const LEFT_WALL_IMAGE_NAME: &str = "./assets/images/left_wall.png";
const RIGHT_WALL_IMAGE_NAME: &str = "./assets/images/right_wall.png";

impl Wall {
    pub fn new(x: f64, y: f64, walltype: WallType) -> Self {
        let image = match walltype {
            WallType::Top => {
                let image: HtmlImageElement = HtmlImageElement::new().unwrap();
                image.set_src(TOP_WALL_IMAGE_NAME);
                image
            },
            WallType::Bottom => {
                let image: HtmlImageElement = HtmlImageElement::new().unwrap();
                image.set_src(BOTTOM_WALL_IMAGE_NAME);
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
        
        Self {
            loc_x: x,
            loc_y: y,
            walltype: walltype,
            image: image
        }
    }

    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        let x = self.loc_x - (self.image.width() as f64/ 2.0);
        let y = self.loc_y - (self.image.height() as f64/ 2.0);
        // log!("backwall -> ", self.loc_x, self.loc_y, x, y);
        // log!(&self.image);

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