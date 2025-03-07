use crate::prelude::*;

pub trait BindGroup {
    fn update(&mut self, _device: &wgpu::Device) {
        todo!()
    }

    fn group_index(&self) -> u32;
    fn group_layout(&self) -> &wgpu::BindGroupLayout;
    fn group_bind(&self) -> &wgpu::BindGroup;

    fn set(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(self.group_index(), self.group_bind(), &[]);
    }
}
