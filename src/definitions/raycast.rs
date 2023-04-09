use std::rc::Rc;

use blue_engine::{imports::glm, Camera};

pub struct Raycast {
    current_ray: glm::Vec3,
    projection_matrix: glm::Mat4,
    view_matrix: glm::Mat4,
    mouse_x_y: (f32, f32),
}
impl Raycast {
    pub fn new(camera: &Camera) -> Self {
        let view_matrix = camera.build_view_matrix();
        let projection_matrix = camera.build_projection_matrix();
        Self {
            projection_matrix,
            view_matrix,
            current_ray: glm::Vec3::new(0f32, 0f32, 0f32),
            mouse_x_y: (0.0, 0.0),
        }
    }

    pub fn get_current_ray(&self) -> glm::Vec3 {
        self.current_ray
    }

    pub fn update(
        &mut self,
        camera: &Camera,
        input: &blue_engine::InputHelper,
        window_size: &blue_engine::PhysicalSize<u32>,
    ) {
        let mouse_position = input.mouse();
        if mouse_position.is_some() {
            self.mouse_x_y = mouse_position.unwrap();
        }

        self.view_matrix = camera.build_view_matrix();
        self.current_ray = self.calculate_mouse_ray(window_size);
    }

    pub fn calculate_mouse_ray(&self, window_size: &blue_engine::PhysicalSize<u32>) -> glm::Vec3 {
        let normalized_coordinates = self.get_normalized_device_coordinates(window_size);
        let clip_coordinates = glm::vec3(normalized_coordinates.x, normalized_coordinates.y, -1f32);
        let eye_coordinates = self.to_eye_coordinates(clip_coordinates);
        self.to_world_coordinates(eye_coordinates)
        //let ray = self.projection_matrix * self.view_matrix * clip_coordinates;
    }

    pub fn to_world_coordinates(&self, eye_coordinates: glm::Vec3) -> glm::Vec3 {
        let inverted_view = glm::inverse(&self.view_matrix);
        let ray_world = inverted_view.transform_vector(&eye_coordinates);
        let mouse_ray = glm::Vec3::new(ray_world.x, ray_world.y, ray_world.z);
        mouse_ray.normalize()
    }

    pub fn to_eye_coordinates(&self, clip_coordinates: glm::Vec3) -> glm::Vec3 {
        let inverted_projection = glm::inverse(&self.projection_matrix);
        let eye_coordinates = inverted_projection.transform_vector(&clip_coordinates);
        glm::Vec3::new(eye_coordinates.x, eye_coordinates.y, -1f32)
    }

    pub fn get_normalized_device_coordinates(
        &self,
        window_size: &blue_engine::PhysicalSize<u32>,
    ) -> glm::Vec2 {
        let x = (self.mouse_x_y.0 * 2f32) / window_size.width as f32 - 1f32;
        let y = -((self.mouse_x_y.1 * 2f32) / window_size.height as f32 - 1f32);

        glm::vec2(x, y)
    }
}
