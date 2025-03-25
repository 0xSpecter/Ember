use crate::prelude::*;

pub struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: Pipeline,
    light_pipeline: Pipeline,

    camera: Camera,
    camera_controller: Box<dyn CameraController>,
    camera_buffer: wgpu::Buffer,
    camera_uniform: CameraUniform,
    camera_bind_group: wgpu::BindGroup,

    time_bind_group: TimeBindGroup,

    light: Light,

    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,

    obj_model: Model,

    depth_texture: Texture,
}


impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> State<'a> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&Default::default());
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None, 
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: Some("Hello"),
                memory_hints: Default::default(),
            },
            None,
        ).await.unwrap();

        // TBD -- Figure out what this does / is
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        // --
        
        let camera: Camera = Camera::std(&config);
        let camera_uniform = CameraUniform::from_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cam_buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Cam bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None
                }
            ],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cam bind group"),
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding()
                }
            ]
        });

        let time_bind_group = TimeBindGroup::new(&device);

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth texture");

        let light = Light::new(&device);

        let obj_model = Model::load_model(
            "cube.obj", 
            &device, 
            &queue, 
        ).unwrap();

        let instances: Vec<Instance> = (0..(std::f32::consts::PI*2.0) as i32).flat_map(|x| {
            (0..(std::f32::consts::PI*2.0) as i32).flat_map(move |y| {
                (0..(std::f32::consts::PI*2.0) as i32).map(move |z| {
                    let x = 4.0 * (x as f32 - 10. / 2.0);
                    let y = 4.0 * (y as f32 - 10. / 2.0);
                    let z = 4.0 * (z as f32 - 10. / 2.0);

                    let pos = vec3(x, y, z);

                    let rot = Quat::from_rotation_x(x);

                    Instance::new(pos, rot)
                })
            })
        }).collect::<Vec<Instance>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<InstanceRaw>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance raw buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        
        let camera_controller = Box::new(DroneCam::new());

        let render_pipeline = Pipeline::new(
            &device,
            &[
                &TextureBindGroup::dummy_layout(&device),
                &camera_bind_group_layout,
                time_bind_group.group_layout(),
                light.bind.group_layout(),
            ],
            &[
                ModelVertex::desc(), 
                InstanceRaw::desc()
            ],
            wgpu::include_wgsl!("../shaders/shader.wgsl"),
            config.format,
            Some(Texture::DEPTH_FORMAT),
        );

        let light_pipeline = Pipeline::new(
            &device,
            &[
                &camera_bind_group_layout,
                light.bind.group_layout(),
            ],
            &[
                ModelVertex::desc(), 
            ],
            wgpu::include_wgsl!("../shaders/light.wgsl"),
            config.format,
            Some(Texture::DEPTH_FORMAT),
        );

        State {
            instance,
            surface,
            device,
            queue,
            config,
            size,

            time_bind_group,

            render_pipeline,
            light_pipeline,

            camera,
            camera_controller,
            camera_uniform,
            camera_buffer,
            camera_bind_group,

            light,

            obj_model,

            instances,
            instance_buffer,

            depth_texture,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.depth_texture = Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    pub fn events(&mut self, input: &Input) {
        self.camera_controller.process_events(input);
    }

    pub fn update(&mut self, delta: Duration) {
        self.camera_controller.update(&mut self.camera, delta.as_secs_f32());
        self.camera_uniform.update(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));

        self.time_bind_group.update(&self.device, &self.queue);
        self.light.bind.update(&self.device, &self.queue);
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        let output = self.surface.get_current_texture()?; 
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(self.light_pipeline.pipe());
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        self.light.bind.set(1, &mut render_pass);
        render_pass.draw_light_model(
            &self.obj_model,
        );

        render_pass.set_pipeline(self.render_pipeline.pipe());
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        self.time_bind_group.set(2, &mut render_pass);
        self.light.bind.set(3, &mut render_pass);

        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.draw_model(&self.obj_model);
        render_pass.draw_model_instanced(&self.obj_model, 0..self.instances.len() as u32);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

