use crate::core::object::Object;

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn update(&mut self, delta_time: f32) {
        for object in &mut self.objects {
            object.update(delta_time);
        }
    }
}
