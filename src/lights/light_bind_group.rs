use crate::prelude::*;

pub struct LightBindGroup {
    buffer: wgpu::Buffer,
    uniform: LightUniform,
    layout: wgpu::BindGroupLayout,
    bind: wgpu::BindGroup,
}

impl LightBindGroup {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform = LightUniform::new([-2., 2., -2.], [1., 1., 1.]);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                }
            ]
        });

        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("light Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ]
        });

        Self {
            uniform,
            buffer,
            layout,
            bind,
        }
    } 
}

impl BindGroup for LightBindGroup {
    fn update(&mut self, _device: &wgpu::Device, queue: &wgpu::Queue) {
        self.uniform.position = (
            Quat::from_axis_angle(Vec3::Y, 0.1_f32.to_radians()) * Vec3::from_array(self.uniform.position)
        ).to_array();     

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    } 

    fn group_bind(&self) -> &wgpu::BindGroup {
        &self.bind        
    }

    fn group_layout(&self) -> &wgpu::BindGroupLayout {
       &self.layout 
    }
}


