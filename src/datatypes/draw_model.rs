use crate::prelude::*;

pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, material: &'a Material);
    fn draw_mesh_instanced(&mut self, mesh: &'a Mesh, material: &'a Material, instances: std::ops::Range<u32>);

    fn draw_model(&mut self, model: &'a Model);
    fn draw_model_instanced(&mut self, model: &'a Model, instances: std::ops::Range<u32>);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a> where 'b: 'a {
    fn draw_mesh(&mut self, mesh: &'b Mesh, material: &'a Material) {
        self.draw_mesh_instanced(mesh, material, 0..1_u32);
    } 

    fn draw_mesh_instanced(&mut self, mesh: &'b Mesh, material: &'b Material, instances: std::ops::Range<u32>) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        material.diffuse_texture.get_bind_group().set(0, self);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(&mut self, model: &'b Model) {
        self.draw_model_instanced(model, 0..1_u32);
    } 

    fn draw_model_instanced(&mut self, model: &'a Model, instances: std::ops::Range<u32>) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material]; 
            self.draw_mesh_instanced(mesh, material, instances.clone());
        }
    }
}
