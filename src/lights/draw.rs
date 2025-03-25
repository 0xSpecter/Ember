use crate::prelude::*;

pub trait DrawLight<'a> {
    fn draw_light_mesh(
        &mut self,
        mesh: &'a Mesh,
    );

    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        instances: Range<u32>,
    );

    fn draw_light_model(
        &mut self,
        model: &'a Model,
    );

    fn draw_light_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
    );
}

impl<'a, 'b> DrawLight<'b> for wgpu::RenderPass<'a> where 'b: 'a {
    fn draw_light_mesh(
        &mut self,
        mesh: &'b Mesh,
    ) {
        self.draw_light_mesh_instanced(mesh, 0..1);
    }

    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        instances: Range<u32>,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_light_model(
        &mut self,
        model: &'b Model,
    ) {
        self.draw_light_model_instanced(model, 0..1);
    }

    fn draw_light_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
    ) {
        for mesh in &model.meshes {
            self.draw_light_mesh_instanced(mesh, instances.clone());
        }
    }
}
