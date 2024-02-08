use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::utils::Point;

pub struct Backwall {
    loc: Point,
    image: HtmlImageElement
}

const BACKWALL_IMAGE_NAME: &str = "./assets/images/backwall.png";

impl Backwall {
    pub fn new(x: f64, y: f64) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(BACKWALL_IMAGE_NAME);

        Backwall {
            loc: Point::new(x, y),
            image: image
        }
    }

    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        let x = self.loc.x - (self.image.width() as f64 / 2.0);
        let y = self.loc.y - (self.image.height() as f64 / 2.0);
        
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