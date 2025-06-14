// Fragment shader for line rendering
struct Light {
    direction: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
};

@group(0) @binding(0) var<uniform> light: Light;

@fragment
fn fs_main(@location(0) normal: vec3<f32>) -> @location(0) vec4<f32> {
    let n = normalize(normal);
    let dot_product = max(dot(n, -light.direction), 0.0);
    let diffuse = light.color * light.intensity * dot_product;
    let base_color = vec3<f32>(1.0, 1.0, 1.0);
    let final_color = base_color * diffuse;
    return vec4<f32>(final_color, 1.0);
} 