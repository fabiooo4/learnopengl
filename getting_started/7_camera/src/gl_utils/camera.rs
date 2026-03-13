use glm::{Vec3, vec3};

pub struct Camera {
    pub speed: f32,
    pub speed_mul: f32,
    pub is_sprinting: bool,

    pub fov: f32,

    pub pos: Vec3,
    pub front: Vec3,
    pub up: Vec3,

    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            speed: 2.5,
            speed_mul: 4.,
            is_sprinting: false,

            fov: 45.,

            pos: vec3(0., 0., 3.),
            front: vec3(0., 0., -1.),
            up: vec3(0., 1., 0.),

            pitch: 0.,
            yaw: -90.,
        }
    }
}

impl Camera {
    /// Toggles a faster camera speed
    pub fn toggle_sprint(&mut self) {
        if self.is_sprinting {
            self.speed /= self.speed_mul;
            self.is_sprinting = false;
        } else {
            self.speed *= self.speed_mul;
            self.is_sprinting = true;
        }
    }
}
