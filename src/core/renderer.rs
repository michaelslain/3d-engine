use crate::core::camera::Camera;
use crate::scene::Scene;
use bytemuck;
use std::sync::Arc;
use wgpu;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    window: Arc<Window>,
    scene: Scene,
    camera: Camera,
    vertex_buffer: Option<wgpu::Buffer>,
    vertex_capacity: usize,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(scene: Scene, camera: Camera) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        window.set_title("3D Engine");

        let size = window.inner_size();
        let window = Arc::new(window);
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::default(),
            })
            .await
            .unwrap();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        // Create vertex buffer layout
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 3]>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        };

        // Create shader modules
        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Line Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/line.vert.wgsl").into()),
        });
        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Line Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/line.frag.wgsl").into()),
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: Some("vs_main"),
                buffers: &[vertex_buffer_layout],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        (
            Renderer {
                device,
                queue,
                surface,
                config,
                window,
                scene,
                camera,
                vertex_buffer: None,
                vertex_capacity: 0,
                render_pipeline,
            },
            event_loop,
        )
    }

    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // --- Projected triangle edges from scene ---
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        for object in self.scene.get_objects() {
            let projected_mesh = self.camera.project_object(object);
            let mesh_vertices = projected_mesh.get_vertices();
            vertices.extend(mesh_vertices.iter().map(|v| [v.x, v.y, v.z]));
        }
        for (i, v) in vertices.iter().take(6).enumerate() {
            println!(
                "Projected vertex {}: [{:.2}, {:.2}, {:.2}]",
                i, v[0], v[1], v[2]
            );
        }
        if vertices.is_empty() {
            println!("No triangles to draw.");
        }
        // --- Dynamic vertex buffer allocation ---
        let needed_capacity = vertices.len();
        let needed_bytes = std::mem::size_of::<[f32; 3]>() * needed_capacity;
        if self.vertex_buffer.is_none() || self.vertex_capacity < needed_capacity {
            self.vertex_buffer = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Line Vertex Buffer"),
                size: needed_bytes as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
            self.vertex_capacity = needed_capacity;
        }
        if let Some(buffer) = &self.vertex_buffer {
            self.queue
                .write_buffer(buffer, 0, bytemuck::cast_slice(&vertices));
        }
        // --- End projected edges ---

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            if let Some(buffer) = &self.vertex_buffer {
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..vertices.len() as u32, 0..1); // Draw all lines
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run_app(&mut self).unwrap();
    }
}

impl winit::application::ApplicationHandler<()> for Renderer {
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                _event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size.width, physical_size.height);
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // Application was resumed, nothing special needed
    }
}
