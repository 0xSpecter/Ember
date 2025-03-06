use crate::prelude::*;
use std::io::{
    Cursor,
    BufReader,
};

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>
}

impl Model {
    pub fn load_model(
        file_name: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> anyhow::Result<Self> {
        let obj_cursor = Cursor::new(stuff_path(file_name)?);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::load_obj_buf(
            &mut obj_reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| {
                let mat_text = stuff_path(&p.to_str().unwrap()).unwrap();
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            },
        )?;

        let materials = obj_materials?.into_iter().map(|m| {
            let diffuse_texture = Texture::from_file(&m.diffuse_texture.unwrap(), device, queue).unwrap();
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                label: None,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler)
                    },
                ]
            });

            Material::new(m.name, diffuse_texture, bind_group)
        }).collect::<Vec<Material>>();

        let meshes = models.into_iter().map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3).map(|index| {
                ModelVertex::convert(m.clone(), index)
            }).collect::<Vec<ModelVertex>>();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", file_name)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            Mesh {
                name: file_name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        }).collect::<Vec<Mesh>>();

        Ok(Self {
            meshes,
            materials,
        })
    }

    pub fn default_bind_group(stage: wgpu::ShaderStages, device: &wgpu::Device, label: Option<&str>) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: stage,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false,
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: stage,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None
                },
            ]
        })
    }
}
