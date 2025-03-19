use crate::prelude::*;

impl Texture {
    pub fn from_rgba(
        width: u32, 
        height: u32, 
        color: [u8; 4],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Texture> {
        let mut img = image::ImageBuffer::new(width, height);
        for x in 0..width {
            for y in 0..height {
                img.put_pixel(x, y, image::Rgba(color));
            }
        }

        let dynamic_img = image::DynamicImage::ImageRgba8(img);
        Texture::from_image(device, queue, &dynamic_img, Some("Color Texture"))
    }

    pub fn debug(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Texture> {
        let mut img = image::ImageBuffer::new(2, 2);
        let debug_color_1 = image::Rgba([0, 0, 0, 255]);
        let debug_color_2 = image::Rgba([255, 0, 140, 255]);

        img.put_pixel(0, 0, debug_color_1);
        img.put_pixel(1, 0, debug_color_2);
        img.put_pixel(0, 1, debug_color_2);
        img.put_pixel(1, 1, debug_color_1);

        let dynamic_img = image::DynamicImage::ImageRgba8(img);
        Texture::create_texture(device, queue, &dynamic_img, Some("Debug Texture"), wgpu::AddressMode::Repeat, 25.0)
    }
}
