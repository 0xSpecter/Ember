use crate::prelude::*;

pub struct TimeBindGroup {
    pub buffer: wgpu::Buffer,
    pub bind: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
}

impl TimeBindGroup {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("time buffer"),
            contents: bytemuck::cast_slice(&[millis() as f32]),  
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Time bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None,
                },
            ], 
        });

        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Time bind group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
                }
            ],
        });

        
        Self {
            buffer,
            bind,
            layout,
        }
    }
}
impl BindGroup for TimeBindGroup {
    fn update(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) {
        self.buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("time buffer"),
            contents: bytemuck::cast_slice(&[millis() as f32]),  
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        self.bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Time bind group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(self.buffer.as_entire_buffer_binding()),
                }
            ],
        });
    }

    fn group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    fn group_bind(&self) -> &wgpu::BindGroup {
        &self.bind
    }
}
