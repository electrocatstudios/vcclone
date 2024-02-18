use web_sys::{CanvasRenderingContext2d,HtmlImageElement};

use crate::utils::Point; 
use crate::{GAME_WIDTH,GAME_HEIGHT};

use gloo_console::log;

pub struct EnemyWizard {
    loc: Point,
    image: HtmlImageElement,
}

const ENEMY_WIZARD_IMAGE: &str = "./assets/images/enemy_wizard.png";
const ENEMY_WIZARD_FRAME_WIDTH: f64 = 100.0;
const ENEMY_WIZARD_FRAME_HEIGHT: f64 = 150.0;
const ENEMY_WIZARD_SPEED: f64 = 0.03;
const ENEMY_WIZARD_BACKWALL_HEIGHT: f64 = 30.0;
const ENEMY_WIZARD_BACKWALL_WIDTH: f64 = 20.0;

impl EnemyWizard {
    pub fn new(x: f64, y: f64) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(ENEMY_WIZARD_IMAGE);

        EnemyWizard {
            loc: Point::new(x, y),
            image: image
        }
    }
    
    pub fn update(&mut self, delta: f64) {
        self.loc.y -= delta * ENEMY_WIZARD_SPEED;
        if self.loc.y < 0.0 {
            self.loc.y = 0.0;
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

        let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &self.image, 
            0.0, 
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