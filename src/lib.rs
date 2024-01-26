mod camera;
mod drawing;
mod nbody_sim;

pub use nbody_sim::*;
use std::fmt::Debug;
use std::mem;
use std::num::NonZeroU64;
use std::sync::Arc;
use wgpu::{Adapter, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Buffer, BufferAddress, BufferBinding, BufferBindingType, BufferUsages, Color, ColorTargetState, ColorWrites, CompositeAlphaMode, Device, DeviceDescriptor, Features, FragmentState, include_wgsl, Instance, InstanceDescriptor, InstanceFlags, Label, Limits, LoadOp, Operations, PipelineLayoutDescriptor, PowerPreference, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModule, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp, Surface, SurfaceConfiguration, TextureFormat, TextureUsages, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState};
use wgpu::LoadOp::Load;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::window::Window;
use camera::Camera;
use crate::camera::CameraUniform;
use crate::drawing::Circle;

pub struct State {
    pub window: Arc<Window>,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,

    pub device: Device,
    pub queue: Queue,

    pub instances: Vec<Circle>,
    pub instance_buffer: Buffer,

    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_bind_group: BindGroup,
    pub camera_buffer: Buffer,

    pub render_pipeline: RenderPipeline,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            flags: InstanceFlags::default(),
            ..Default::default()
        });


        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface: None,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Default::default(),
            },
            None,
        ).await.unwrap();

        let (surface, surface_config) = setup_surface(&instance, window.clone(), &adapter, &device);

        let (instances, instance_buffer) =
            drawing::initialize_circle_and_vertex_bufs(&device);

        let (camera,
            camera_uniform,
            camera_bind_group,
            camera_bind_group_layout,
            camera_buffer) = camera::setup_camera(&surface, &device, &surface_config);

        let pipeline_layout = device.create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let render_pipeline = device.create_render_pipeline(
            &RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        drawing::Circle::desc()
                    ],
                },
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[
                        Some(ColorTargetState {
                            format: surface_config.format,
                            blend: Some(BlendState::ALPHA_BLENDING),
                            write_mask: ColorWrites::ALL,
                        })
                    ],
                }),
                multiview: None,
            });

        Self {
            window,
            surface,
            surface_config,

            device,
            queue,


            // vertex_buffer,
            // vertex_len,

            instances,
            instance_buffer,

            camera,
            camera_uniform,
            camera_bind_group,
            camera_buffer,

            render_pipeline,
        }
    }

    pub fn resize(&mut self, physical_size: PhysicalSize<u32>) {
        self.surface_config.width = physical_size.width;
        self.surface_config.height = physical_size.height;
        self.surface.configure(&self.device, &self.surface_config);

        self.camera.aspect = self.surface_config.width as f32 / self.surface_config.height as f32;
        self.camera_uniform.update_view_proj(&self.camera);
        // println!("{:?}", physical_size);
    }

    pub fn update(&mut self) {
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn update_circles(&mut self, new_circles: Vec<Circle>) {
        self.instances = new_circles.clone();
        self.instance_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Circle Buffer"),
            contents: bytemuck::cast_slice(new_circles.as_slice()),
            usage: BufferUsages::VERTEX,
        });
    }

    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[
                Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Discard,
                    },
                })
            ],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        render_pass.draw(0..3, 0..self.instances.len() as u32);
        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        return;
    }
}

pub fn setup_surface(instance: &Instance, window: Arc<Window>, adapter: &Adapter, device: &Device) -> (Surface<'static>, SurfaceConfiguration) {
    let surface = instance.create_surface(window.clone()).unwrap();

    let surface_capabilities = surface.get_capabilities(&adapter);
    let surface_format = surface_capabilities
        .formats
        .iter()
        .copied()
        .filter(|f| f.is_srgb())
        .next()
        .unwrap_or(surface_capabilities.formats[0]);

    let window_size = window.inner_size();
    let surface_config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::Immediate,
        desired_maximum_frame_latency: 2,
        alpha_mode: Default::default(),
        view_formats: vec![],
    };

    surface.configure(&device, &surface_config);
    (surface, surface_config)
}