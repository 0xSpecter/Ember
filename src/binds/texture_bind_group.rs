use crate::prelude::*;

pub struct TextureBindGroup {
    pub group: u32,
    pub bind: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
}

impl TextureBindGroup {
    pub fn new(group: u32, device: &wgpu::Device, texture: &Texture) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind group layoyut"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension:  wgpu::TextureViewDimension::D2, 
                        multisampled: false
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ]
        });

        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ]
        });

        
        Self {
            group,
            bind,
            layout,
        }
    }

    pub fn dummy_layout(group: u32, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind group layoyut"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension:  wgpu::TextureViewDimension::D2, 
                        multisampled: false
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ]
        })
    }
}
impl BindGroup for TextureBindGroup {
    fn update(&mut self, device: &wgpu::Device) {
        todo!();
    }

    fn group_index(&self) -> u32 {
        self.group
    }

    fn group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    fn group_bind(&self) -> &wgpu::BindGroup {
        &self.bind
    }
}
