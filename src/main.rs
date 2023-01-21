use std::{num::NonZeroU32, vec};

use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
struct Camera {
    zoom_level: f32,
    offset_x: f32,
    offset_y: f32,
}

struct GOL {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    front_texture: wgpu::Texture,
    back_texture: wgpu::Texture,
    gol_shader_module: wgpu::ShaderModule,
    blit_shader_module: wgpu::ShaderModule,
    blit_sampler: wgpu::Sampler,
    camera: Camera,
    camera_buffer: wgpu::Buffer,
}

impl GOL {
    fn new(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = pollster::block_on(async {
            instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    ..Default::default()
                })
                .await
        })
        .unwrap();

        let (device, queue) = pollster::block_on(async {
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                        ..Default::default()
                    },
                    None,
                )
                .await
        })
        .unwrap();

        let window_size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &surface_config);

        let front_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            size: wgpu::Extent3d {
                width: window_size.width,
                height: window_size.height,
                depth_or_array_layers: 1,
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST,
        });

        let mut data = vec![0; (window_size.width * window_size.height * 4) as usize];
        let width = window_size.width as usize;
        let off_x = 400;
        let off_y = 300;

        data[4 * (width * (off_y + 0) + (off_x + 1)) + 0] = u8::MAX;
        data[4 * (width * (off_y + 0) + (off_x + 1)) + 1] = u8::MAX;
        data[4 * (width * (off_y + 0) + (off_x + 1)) + 2] = u8::MAX;
        data[4 * (width * (off_y + 0) + (off_x + 1)) + 3] = u8::MAX;

        data[4 * (width * (off_y + 1) + (off_x + 3)) + 0] = u8::MAX;
        data[4 * (width * (off_y + 1) + (off_x + 3)) + 1] = u8::MAX;
        data[4 * (width * (off_y + 1) + (off_x + 3)) + 2] = u8::MAX;
        data[4 * (width * (off_y + 1) + (off_x + 3)) + 3] = u8::MAX;

        data[4 * (width * (off_y + 2) + (off_x + 0)) + 0] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 0)) + 1] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 0)) + 2] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 0)) + 3] = u8::MAX;

        data[4 * (width * (off_y + 2) + (off_x + 1)) + 0] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 1)) + 1] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 1)) + 2] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 1)) + 3] = u8::MAX;

        data[4 * (width * (off_y + 2) + (off_x + 4)) + 0] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 4)) + 1] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 4)) + 2] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 4)) + 3] = u8::MAX;

        data[4 * (width * (off_y + 2) + (off_x + 5)) + 0] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 5)) + 1] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 5)) + 2] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 5)) + 3] = u8::MAX;

        data[4 * (width * (off_y + 2) + (off_x + 6)) + 0] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 6)) + 1] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 6)) + 2] = u8::MAX;
        data[4 * (width * (off_y + 2) + (off_x + 6)) + 3] = u8::MAX;

        let back_texture = device.create_texture_with_data(
            &queue,
            &wgpu::TextureDescriptor {
                label: None,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                mip_level_count: 1,
                sample_count: 1,
                size: wgpu::Extent3d {
                    width: window_size.width,
                    height: window_size.height,
                    depth_or_array_layers: 1,
                },
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::COPY_DST,
            },
            &data,
        );

        let gol_shader_module = device.create_shader_module(wgpu::include_wgsl!("gol_shader.wgsl"));
        let blit_shader_module =
            device.create_shader_module(wgpu::include_wgsl!("blit_shader.wgsl"));

        let blit_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let camera = Camera {
            zoom_level: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::bytes_of(&camera),
            label: None,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            surface,
            surface_config,
            device,
            queue,
            front_texture,
            back_texture,
            gol_shader_module,
            blit_shader_module,
            blit_sampler,
            camera,
            camera_buffer,
        }
    }

    fn step_simulation(&mut self) -> wgpu::CommandEncoder {
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                format: wgpu::TextureFormat::Rgba8Unorm,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::ReadOnly,
                                format: wgpu::TextureFormat::Rgba8Unorm,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                    ],
                });
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &self
                            .front_texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &self
                            .back_texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
            ],
        });
        let compute_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });
        let compute_pipeline =
            self.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: Some(&compute_pipeline_layout),
                    module: &self.gol_shader_module,
                    entry_point: "main",
                });
        {
            let mut compute_pass =
                command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(
                self.surface_config.width,
                self.surface_config.height,
                1,
            );
        }

        command_encoder
    }

    fn resize(&mut self, physical_size: &winit::dpi::PhysicalSize<u32>) {
        self.surface_config.width = physical_size.width;
        self.surface_config.height = physical_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    fn render(&mut self) {
        let command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        self.render_with_command_encoder(command_encoder)
    }

    fn render_with_command_encoder(&mut self, mut command_encoder: wgpu::CommandEncoder) {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(_) => return,
        };
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let blit_bind_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            visibility: wgpu::ShaderStages::VERTEX,
                            binding: 0,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            binding: 1,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            binding: 2,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });
        let blit_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&blit_bind_layout],
                    push_constant_ranges: &[],
                });
        let blit_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&blit_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &self.blit_shader_module,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.blit_shader_module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.surface_config.format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        let blit_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &blit_bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        self.camera_buffer.as_entire_buffer_binding(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &self
                            .back_texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.blit_sampler),
                },
            ],
        });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::default()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&blit_pipeline);
            render_pass.set_bind_group(0, &blit_bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let mut gol = GOL::new(&window);
    let mut paused = true;
    let mut last_position = winit::dpi::PhysicalPosition::<f64>::default();
    let mut left_button_state = winit::event::ElementState::Released;
    let mut right_button_state = winit::event::ElementState::Released;

    event_loop.run(
        move |event, _event_loop_window_target, control_flow| match event {
            winit::event::Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    winit::event::WindowEvent::Resized(physical_size) => gol.resize(&physical_size),
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        gol.resize(new_inner_size)
                    }
                    winit::event::WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        modifiers,
                    } => {
                        if button == winit::event::MouseButton::Left {
                            left_button_state = state;
                        } else if button == winit::event::MouseButton::Right {
                            right_button_state = state;
                        }
                    }
                    winit::event::WindowEvent::MouseWheel {
                        device_id,
                        delta,
                        phase,
                        modifiers,
                    } => {
                        if let winit::event::MouseScrollDelta::LineDelta(delta_x, delta_y) = delta {
                            gol.camera.zoom_level *= if delta_y < 0.0 { 0.9 } else { 1.1 };
                            gol.queue.write_buffer(
                                &gol.camera_buffer,
                                0,
                                bytemuck::bytes_of(&gol.camera),
                            );
                        }
                    }
                    winit::event::WindowEvent::CursorMoved {
                        device_id,
                        position,
                        modifiers: _,
                    } => {
                        if left_button_state == winit::event::ElementState::Pressed {
                            gol.queue.write_texture(
                                wgpu::ImageCopyTextureBase {
                                    texture: &gol.back_texture,
                                    aspect: wgpu::TextureAspect::All,
                                    mip_level: 0,
                                    origin: wgpu::Origin3d {
                                        x: u32::min(
                                            position.x as u32,
                                            gol.surface_config.width - 1,
                                        ),
                                        y: u32::min(
                                            position.y as u32,
                                            gol.surface_config.height - 1,
                                        ),
                                        z: 0,
                                    },
                                },
                                &[u8::MAX, u8::MAX, u8::MAX, u8::MAX],
                                wgpu::ImageDataLayout {
                                    bytes_per_row: NonZeroU32::new(4),
                                    ..Default::default()
                                },
                                wgpu::Extent3d {
                                    width: 1,
                                    height: 1,
                                    depth_or_array_layers: 1,
                                },
                            )
                        }

                        if right_button_state == winit::event::ElementState::Pressed {
                            gol.camera.offset_x += (last_position.x - position.x) as f32
                                / gol.surface_config.width as f32 * 2.0;
                            gol.camera.offset_y += (last_position.y - position.y) as f32
                                / gol.surface_config.height as f32 * 2.0;
                            gol.queue.write_buffer(
                                &gol.camera_buffer,
                                0,
                                bytemuck::bytes_of(&gol.camera),
                            );
                        }

                        last_position = position;
                    }
                    winit::event::WindowEvent::ReceivedCharacter(c) => {
                        if c == ' ' {
                            paused = !paused;
                        }
                    }
                    _ => {}
                }
            }
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                if !paused {
                    let command_encoder = gol.step_simulation();
                    gol.render_with_command_encoder(command_encoder);
                    std::mem::swap(&mut gol.front_texture, &mut gol.back_texture);
                } else {
                    gol.render()
                }
            }
            winit::event::Event::MainEventsCleared => window.request_redraw(),
            _ => {}
        },
    )
}
