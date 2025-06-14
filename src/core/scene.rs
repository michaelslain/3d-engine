use crate::core::light::Light;
use crate::core::object::Object;

pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_lights(&self) -> &Vec<Box<dyn Light>> {
        &self.lights
    }

    pub fn add_light(&mut self, light: impl Light + 'static) {
        self.lights.push(Box::new(light));
    }

    pub fn update(&mut self, delta_time: f32) {
        for object in &mut self.objects {
            object.update(delta_time);
        }
    }
}
