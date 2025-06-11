use std::sync::RwLock;

#[derive(Clone)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
}

impl RenderConfig {
    pub const fn new(width: u32, height: u32, fov: f32) -> Self {
        Self { width, height, fov }
    }
}

static CONFIG: RwLock<RenderConfig> = RwLock::new(RenderConfig::new(800, 600, 90.0));

pub fn get_config() -> RenderConfig {
    CONFIG.read().unwrap().clone()
}

pub fn update_config(width: u32, height: u32, fov: f32) {
    let mut config = CONFIG.write().unwrap();
    *config = RenderConfig::new(width, height, fov);
}
