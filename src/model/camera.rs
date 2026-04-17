use euclid::Transform3D;

pub struct Camera {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub projection: Transform3D<f32, (), ()>,
    pub view: Transform3D<f32, (), ()>,
    pub width: f32,
    pub height: f32,
    pub dirty: bool,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: [0.0, 40.0, -40.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            projection: Transform3D::identity(),
            view: Transform3D::identity(),
            width: 0.0,
            height: 0.0,
            dirty: false
        }
    }

    pub fn move_camera(&mut self, x: f32, y: f32, z: f32) {
        self.position[0] = x;
        self.position[1] = y;
        self.position[2] = z;
        self.dirty = true;
    }

    pub fn _get_position(&self) -> [f32; 3] {
        self.position
    }

    pub fn look_at(&mut self, x: f32, y: f32, z: f32) {
        self.target[0] = x;
        self.target[1] = y;
        self.target[2] = z;
    }

    pub fn set_up(&mut self, x: f32, y: f32, z: f32) {   
        self.up[0] = x;
        self.up[1] = y;
        self.up[2] = z;
    }

    pub fn setup(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        let field_of_view = std::f32::consts::PI / 4.0;
        let aspect = self.width / self.height;
        let z_near = 0.1;
        let z_far = 100.0;
        
        // Not sure if we need to reset this but just in case
        self.projection = Transform3D::identity();

        // Maths copied from: https://github.com/toji/gl-matrix/blob/master/src/mat4.js\
        // Apply perspective
        let f = 1.0 / (field_of_view / 2.0).tan();
        self.projection.m11 = f / aspect;
        self.projection.m22 = f;
        self.projection.m34 = -1.0;

        let nf = 1.0 / (z_near - z_far);
        self.projection.m33 = (z_far + z_near) * nf;
        self.projection.m43 = (2.0 * z_far * z_near) * nf; 

        // mat4.lookAt(this.viewMatrix, eye, center, up);
    }

    pub fn update(&mut self, _delta: f32) {
    }

    pub fn refresh(&mut self, delta: f64) {
        self.update(delta as f32);

        // Apply the look at
        // let epsilon: f32 = 0.000000000001;
        let eye: [f32; 3] = [self.position[0], self.position[1], self.position[2]];
        let center: [f32; 3] = [self.target[0], self.target[1], self.target[2]];
        let up: [f32; 3] = [self.up[0], self.up[1], self.up[2]];
        
        // if eye[0] - center[0] < epsilon &&
        //     eye[1] - center[1] < epsilon &&
        //     eye[2] - center[2] < epsilon {
        //     self.view = Transform3D::identity();
        //     return;
        // }
        let mut z = [eye[0] - center[0], eye[1] - center[1], eye[2] - center[2]];
        let len = 1.0 / (z[0] * z[0] + z[1] * z[1] + z[2] * z[2]).sqrt();
        z[0] *= len;
        z[1] *= len;
        z[2] *= len;

        let mut x = [
            up[1] * z[2] - up[2] * z[1], 
            up[2] * z[0] - up[0] * z[2], 
            up[0] * z[1] - up[1] * z[0]
        ];
        let len = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
        if len == 0.0 {
            x[0] = 0.0;
            x[1] = 0.0;
            x[2] = 0.0;
        } else {
            let len = 1.0 / len;
            x[0] *= len;
            x[1] *= len;
            x[2] *= len;
        }

        let mut y = [
            z[1] * x[2] - z[2] * x[1], 
            z[2] * x[0] - z[0] * x[2], 
            z[0] * x[1] - z[1] * x[0]
        ];
        let len = (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]).sqrt();
        if len == 0.0 {
            y[0] = 0.0;
            y[1] = 0.0;
            y[2] = 0.0;
        } else {
            let len = 1.0 / len;
            y[0] *= len;
            y[1] *= len;
            y[2] *= len;
        }
        self.view.m11 = x[0];
        self.view.m21 = x[1];
        self.view.m31 = x[2];
        self.view.m41 = -(x[0] * eye[0] + x[1] * eye[1] + x[2] * eye[2]);
        
        self.view.m12 = y[0];
        self.view.m22 = y[1];
        self.view.m32 = y[2];
        self.view.m42 = -(y[0] * eye[0] + y[1] * eye[1] + y[2] * eye[2]);

        self.view.m13 = z[0];
        self.view.m23 = z[1];
        self.view.m33 = z[2];
        self.view.m43 = -(z[0] * eye[0] + z[1] * eye[1] + z[2] * eye[2]);

        self.view.m14 = 0.0;
        self.view.m24 = 0.0;
        self.view.m34 = 0.0;
        self.view.m44 = 1.0;      
    }

    pub fn update_screen_dimensions(&mut self, width: f32, height: f32) {
        if width != self.width || height != self.height {
            self.setup(width, height);
        }
    }

}