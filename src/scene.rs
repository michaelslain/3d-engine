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

    pub fn update(&mut self) {}

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}
