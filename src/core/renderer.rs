use crate::core::camera::Camera;
use crate::core::scene::Scene;
use bytemuck;
use std::sync::Arc;
use std::time::Instant;
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
    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,
    last_frame_time: Instant,
}

impl Renderer {
    pub async fn new(scene: Scene, camera: Camera) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
                    .with_visible(true),
            )
            .unwrap();
        window.set_title("3D Engine");

        let size = window.inner_size();
        let window = Arc::new(window);
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(window.clone()).unwrap() };
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

        let surface_caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create light buffer
        let light_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Light Buffer"),
            size: std::mem::size_of::<[f32; 8]>() as u64, // Changed from 7 to 8 to match shader expectation (32 bytes)
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout for light
        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Light Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create bind group for light
        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Light Bind Group"),
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
        });

        // Create vertex buffer layout with normals
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 6]>() as u64, // 3 for position, 3 for normal
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position attribute
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normal attribute
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
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

        // Create pipeline layout
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&light_bind_group_layout],
                push_constant_ranges: &[],
            });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Pipeline"),
            layout: Some(&render_pipeline_layout),
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
                topology: wgpu::PrimitiveTopology::TriangleList,
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
                light_buffer,
                light_bind_group,
                last_frame_time: Instant::now(),
            },
            event_loop,
        )
    }

    pub fn render(&mut self) {
        let now = Instant::now();
        let delta_time = (now - self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        self.scene.update(delta_time);

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => return, // Skip rendering if surface is invalid
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Update light data
        if let Some(light) = self.scene.get_lights().first() {
            let light_data = [
                light.get_direction().x,
                light.get_direction().y,
                light.get_direction().z,
                0.0, // Padding to align to 32 bytes
                light.get_color().x,
                light.get_color().y,
                light.get_color().z,
                light.get_intensity(),
            ];
            self.queue
                .write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&light_data));
        }

        // --- Projected triangle edges from scene ---
        let mut vertices: Vec<[f32; 6]> = Vec::new(); // [x, y, z, nx, ny, nz]
        for object in self.scene.get_objects() {
            for tri in object.get_mesh().get_triangles() {
                let transformed_tri = object.transformed_triangle(tri.clone());

                // culling
                if transformed_tri
                    .get_normal()
                    .dot(transformed_tri.get_vertices()[0] - self.camera.get_position())
                    >= 0.0
                {
                    continue;
                }

                // projection
                let projected_tri = self.camera.project_triangle(transformed_tri.clone());
                let [v0, v1, v2] = projected_tri.get_vertices();
                let normal = transformed_tri.get_normal();

                // Add triangle vertices with normals
                vertices.push([v0.x, v0.y, v0.z, normal.x, normal.y, normal.z]);
                vertices.push([v1.x, v1.y, v1.z, normal.x, normal.y, normal.z]);
                vertices.push([v2.x, v2.y, v2.z, normal.x, normal.y, normal.z]);
            }
        }

        if vertices.is_empty() {
            // Skip rendering if there are no vertices
            self.queue.submit(Some(encoder.finish()));
            frame.present();
            return;
        }

        // --- Dynamic vertex buffer allocation ---
        let needed_capacity = vertices.len();
        let needed_bytes = std::mem::size_of::<[f32; 6]>() * needed_capacity;
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

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
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
                render_pass.set_bind_group(0, &self.light_bind_group, &[]);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..vertices.len() as u32, 0..1);
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
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size.width, physical_size.height);
                self.window.request_redraw();
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
        self.window.request_redraw();
    }
}
