use crate::prelude::*;

pub trait BindGroup {
    fn update(&mut self, _device: &wgpu::Device, _queue: &wgpu::Queue) {
        todo!()
    }

    fn group_layout(&self) -> &wgpu::BindGroupLayout;
    fn group_bind(&self) -> &wgpu::BindGroup;

    fn set(&self, group: u32, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(group, self.group_bind(), &[]);
    }
}
