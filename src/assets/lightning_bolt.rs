use js_sys::wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use rand::Rng;
use gloo_console::log;
 
use crate::utils::Point;

pub struct LightningBolt {
    ttl: f64,
    pub is_alive: bool,
    segments: Vec::<Point>,
    start: Point,
    end: Point
}

const LIGHTNING_BOLT_TTL: f64 = 1000.0;
const LIGHTNING_BOLT_NUM_SEGS: i32 = 8;
const LIGHTNING_BOLT_MAX_WIGGLE: f64 = 20.0;

impl LightningBolt {
    pub fn new(start: Point, end: Point) -> Self {
        let mut segments = Vec::<Point>::new();
        let x_diff = end.x - start.x;
        let x_seg_diff = x_diff / LIGHTNING_BOLT_NUM_SEGS as f64;

        let y_diff = start.y - end.y;
        let y_seg_diff = y_diff / LIGHTNING_BOLT_NUM_SEGS as f64;
        let mut rng = rand::thread_rng();
                
        for i in 1..LIGHTNING_BOLT_NUM_SEGS {
            if i == LIGHTNING_BOLT_NUM_SEGS - 1 {
                segments.push(Point::new(end.x, end.y));
            } else {
                let x_norm = (i as f64 * x_seg_diff) + start.x;
                let new_x_seed = rng.gen_range(0.0..LIGHTNING_BOLT_MAX_WIGGLE);
                let new_x = x_norm + (new_x_seed - (LIGHTNING_BOLT_MAX_WIGGLE / 2.0));

                let y_norm = (LIGHTNING_BOLT_NUM_SEGS - i) as f64 * y_seg_diff + end.y;
                let new_y_seed = rng.gen_range(0.0..LIGHTNING_BOLT_MAX_WIGGLE);
                let new_y = y_norm + (new_y_seed - (LIGHTNING_BOLT_MAX_WIGGLE / 2.0));
                
                segments.push(
                    Point::new(
                        new_x,
                        new_y 
                    )
                )
            }
        }

        LightningBolt {
            ttl: LIGHTNING_BOLT_TTL,
            is_alive: true,
            segments: segments,
            start: start,
            end: end
        }
    }
    
    pub fn update(&mut self, delta: f64) {
        self.ttl -= delta;
        if self.ttl <= 0.0 {
            self.is_alive = false;
        }
    }

    pub fn render(&self, ctx: &mut CanvasRenderingContext2d) {
        let perc = self.ttl / LIGHTNING_BOLT_TTL;
        let col_str = format!("rgba(255,224,50,{})", perc);
        
        let ss = ctx.stroke_style();
        let lw = ctx.line_width();
        let _ = ctx.begin_path();
        
        let _ = ctx.set_line_width(5.0);
        let _ = ctx.set_stroke_style(&JsValue::from(col_str));
        let _ = ctx.move_to(self.start.x, self.start.y);

        for seg in self.segments.iter() {
            let _ = ctx.line_to(seg.x , seg.y );
        }

        let _ = ctx.stroke();  
        
        let _ = ctx.set_line_width(lw);
        let _ = ctx.set_stroke_style(&ss);   
        
    }
}