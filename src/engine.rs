use crate::core::camera::Camera;
use crate::core::renderer::Renderer;
use crate::core::scene::Scene;
use pollster::block_on;

pub struct Engine {
    scene: Scene,
    camera: Camera,
}

impl Engine {
    pub fn new(scene: Scene, camera: Camera) -> Self {
        Engine { scene, camera }
    }

    pub fn run(self) {
        block_on(self.run_async());
    }

    pub async fn run_async(self) {
        let (renderer, event_loop) = Renderer::new(self.scene, self.camera).await;
        renderer.run(event_loop);
    }
}
