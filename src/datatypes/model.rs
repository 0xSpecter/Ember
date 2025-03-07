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
                println!("{:?}", p);
                let mat_text = stuff_path(p.to_str().unwrap()).unwrap();
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            },
        )?;

        let materials = obj_materials?.into_iter().map(|m| {
            println!("{m:?}");
            let mut diffuse_texture: Texture = if let Some(diffuse_texture) = &m.diffuse_texture {
                Texture::from_file(diffuse_texture, device, queue).unwrap()
            } else {
                Texture::default_texture(device, queue).unwrap()
            };

            diffuse_texture.attach_bind_group(0, device);
            Material::new(m.name, diffuse_texture)
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
}
